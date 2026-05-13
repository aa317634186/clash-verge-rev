use super::CmdResult;
#[cfg(not(target_os = "android"))]
use super::StringifyErr;
#[cfg(not(target_os = "android"))]
use crate::core::service::{self, SERVICE_MANAGER, ServiceStatus};
#[cfg(not(target_os = "android"))]
use smartstring::SmartString;

#[cfg(not(target_os = "android"))]
async fn execute_service_operation_sync(status: ServiceStatus, op_type: &str) -> CmdResult {
    if let Err(e) = SERVICE_MANAGER
        .lock()
        .await
        .handle_service_status(&status)
        .await
    {
        let emsg = format!("{} Service failed: {}", op_type, e);
        return Err(SmartString::from(emsg));
    }
    Ok(())
}

#[tauri::command]
pub async fn install_service() -> CmdResult {
    #[cfg(not(target_os = "android"))]
    {
        execute_service_operation_sync(ServiceStatus::InstallRequired, "Install").await
    }
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
}

#[tauri::command]
pub async fn uninstall_service() -> CmdResult {
    #[cfg(not(target_os = "android"))]
    {
        execute_service_operation_sync(ServiceStatus::UninstallRequired, "Uninstall").await
    }
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
}

#[tauri::command]
pub async fn reinstall_service() -> CmdResult {
    #[cfg(not(target_os = "android"))]
    {
        execute_service_operation_sync(ServiceStatus::ReinstallRequired, "Reinstall").await
    }
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
}

#[tauri::command]
pub async fn repair_service() -> CmdResult {
    #[cfg(not(target_os = "android"))]
    {
        execute_service_operation_sync(ServiceStatus::ForceReinstallRequired, "Repair").await
    }
    #[cfg(target_os = "android")]
    {
        Ok(())
    }
}

#[tauri::command]
pub async fn is_service_available() -> CmdResult<bool> {
    #[cfg(not(target_os = "android"))]
    {
        service::is_service_available().await.stringify_err()?;
        Ok(true)
    }
    #[cfg(target_os = "android")]
    {
        // On Android, VPN service is always available
        Ok(true)
    }
}
