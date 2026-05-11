use super::{CoreManager, RunningMode};
use crate::config::{Config, ConfigType, IVerge};
#[cfg(not(target_os = "android"))]
use crate::core::service::{SERVICE_MANAGER, ServiceStatus};
use crate::{core::logger::CLASH_LOGGER, logging, utils::logging::Type};
use anyhow::Result;
use smartstring::alias::String;

impl CoreManager {
    #[cfg(not(target_os = "android"))]
    pub async fn start_core(&self) -> Result<()> {
        self.prepare_startup().await?;

        match *self.get_running_mode() {
            RunningMode::Service => self.start_core_by_service().await,
            RunningMode::NotRunning | RunningMode::Sidecar => self.start_core_by_sidecar().await,
        }
    }

    /// On Android, start_core writes the mihomo config and signals the Kotlin
    /// layer to start the VPN service (which runs mihomo in-process).
    #[cfg(target_os = "android")]
    pub async fn start_core(&self) -> Result<()> {
        logging!(info, Type::Core, "Starting core on Android via VPN service");
        self.set_running_mode(RunningMode::Service);

        // Generate the runtime config file
        let run_path = Config::generate_file(ConfigType::Run).await?;
        let config_path = run_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Failed to convert config path to string"))?
            .to_string();

        // Signal the Android Kotlin layer to start VPN with this config
        if let Some(app) = crate::APP_HANDLE.get() {
            use tauri::Emitter;
            let _ = app.emit("android-vpn-start", &config_path);
        }

        Ok(())
    }

    #[cfg(not(target_os = "android"))]
    pub async fn stop_core(&self) -> Result<()> {
        CLASH_LOGGER.clear_logs().await;

        match *self.get_running_mode() {
            RunningMode::Service => self.stop_core_by_service().await,
            RunningMode::Sidecar => self.stop_core_by_sidecar(),
            RunningMode::NotRunning => Ok(()),
        }
    }

    #[cfg(target_os = "android")]
    pub async fn stop_core(&self) -> Result<()> {
        CLASH_LOGGER.clear_logs().await;
        logging!(info, Type::Core, "Stopping core on Android via VPN service");

        if let Some(app) = crate::APP_HANDLE.get() {
            use tauri::Emitter;
            let _ = app.emit("android-vpn-stop", ());
        }

        self.set_running_mode(RunningMode::NotRunning);
        Ok(())
    }

    pub async fn restart_core(&self) -> Result<()> {
        logging!(info, Type::Core, "Restarting core");
        self.stop_core().await?;

        #[cfg(not(target_os = "android"))]
        {
            if SERVICE_MANAGER.lock().await.init().await.is_ok() {
                let _ = SERVICE_MANAGER.lock().await.refresh().await;
            }
        }

        self.start_core().await
    }

    pub async fn change_core(&self, clash_core: &String) -> Result<(), String> {
        if !IVerge::VALID_CLASH_CORES.contains(&clash_core.as_str()) {
            return Err(format!("Invalid clash core: {}", clash_core).into());
        }

        Config::verge().await.edit_draft(|d| {
            d.clash_core = Some(clash_core.to_owned());
        });
        Config::verge().await.apply();

        let verge_data = Config::verge().await.latest_arc();
        verge_data.save_file().await.map_err(|e| e.to_string())?;

        let run_path = Config::generate_file(ConfigType::Run)
            .await
            .map_err(|e| e.to_string())?;

        self.apply_config(run_path)
            .await
            .map_err(|e| e.to_string().into())
    }

    #[cfg(not(target_os = "android"))]
    async fn prepare_startup(&self) -> Result<()> {
        #[cfg(target_os = "windows")]
        self.wait_for_service_if_needed().await;

        let value = SERVICE_MANAGER.lock().await.current();
        let mode = match value {
            ServiceStatus::Ready => RunningMode::Service,
            _ => RunningMode::Sidecar,
        };

        self.set_running_mode(mode);
        Ok(())
    }

    #[cfg(target_os = "windows")]
    async fn wait_for_service_if_needed(&self) {
        use crate::{config::Config, constants::timing};
        use backoff::{Error as BackoffError, ExponentialBackoff};

        let needs_service = Config::verge()
            .await
            .latest_arc()
            .enable_tun_mode
            .unwrap_or(false);

        if !needs_service {
            return;
        }

        let backoff = ExponentialBackoff {
            initial_interval: timing::SERVICE_WAIT_INTERVAL,
            max_interval: timing::SERVICE_WAIT_INTERVAL,
            max_elapsed_time: Some(timing::SERVICE_WAIT_MAX),
            multiplier: 1.0,
            randomization_factor: 0.0,
            ..Default::default()
        };

        let operation = || async {
            let mut manager = SERVICE_MANAGER.lock().await;

            if matches!(manager.current(), ServiceStatus::Ready) {
                return Ok(());
            }

            manager.init().await.map_err(BackoffError::transient)?;
            let _ = manager.refresh().await;

            if matches!(manager.current(), ServiceStatus::Ready) {
                Ok(())
            } else {
                Err(BackoffError::transient(anyhow::anyhow!(
                    "Service not ready"
                )))
            }
        };

        let _ = backoff::future::retry(backoff, operation).await;
    }
}
