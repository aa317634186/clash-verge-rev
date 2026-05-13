//! Android JNI bridge module for Rust-Kotlin communication.
//! Provides callback functions that can be invoked from the Kotlin/Java side
//! to notify the Rust backend about VPN state changes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jboolean;

use crate::logging;
use crate::utils::logging::Type;

/// Called from Kotlin when VPN service has started successfully.
/// This notifies the Rust side that the VPN tunnel is active.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_clashvergerev_clashverge_vpn_MihomoCore_onVpnStarted(
    _env: JNIEnv,
    _class: JClass,
) {
    logging!(
        info,
        Type::Core,
        "Android VPN service started (JNI callback)"
    );
    // Update shared atomic state so get_vpn_status returns the correct value
    crate::cmd::android::VPN_RUNNING.store(true, std::sync::atomic::Ordering::Relaxed);
    if let Some(app) = crate::APP_HANDLE.get() {
        use tauri::Emitter;
        let _ = app.emit("vpn-state-changed", "started");
    }
}

/// Called from Kotlin when VPN service has stopped.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_clashvergerev_clashverge_vpn_MihomoCore_onVpnStopped(
    _env: JNIEnv,
    _class: JClass,
) {
    logging!(
        info,
        Type::Core,
        "Android VPN service stopped (JNI callback)"
    );
    // Update shared atomic state so get_vpn_status returns the correct value
    crate::cmd::android::VPN_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
    if let Some(app) = crate::APP_HANDLE.get() {
        use tauri::Emitter;
        let _ = app.emit("vpn-state-changed", "stopped");
    }
}

/// Called from Kotlin when an error occurs in the VPN service.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_clashvergerev_clashverge_vpn_MihomoCore_onVpnError<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    error_msg: JString<'a>,
) {
    let error: String = env
        .get_string(&error_msg)
        .map(|s| s.into())
        .unwrap_or_else(|_| "Unknown error".to_string());

    logging!(
        error,
        Type::Core,
        "Android VPN error (JNI callback): {}",
        error
    );
    // VPN is not running after an error
    crate::cmd::android::VPN_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
    if let Some(app) = crate::APP_HANDLE.get() {
        use tauri::Emitter;
        let _ = app.emit("vpn-state-changed", format!("error:{error}"));
    }
}

/// Called from Kotlin to check if mihomo core is running.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_clashvergerev_clashverge_vpn_MihomoCore_isCoreRunning(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    // Check if core manager reports a running state
    use crate::core::CoreManager;
    use crate::core::manager::RunningMode;

    let mode = CoreManager::global().get_running_mode();
    matches!(*mode, RunningMode::Service | RunningMode::Sidecar) as jboolean
}

/// Called from Kotlin when the split tunnel app list is updated.
/// Receives a JSON array string of package names (e.g. `["com.app1","com.app2"]`).
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_github_clashvergerev_clashverge_vpn_MihomoCore_onSplitTunnelAppsUpdated<
    'a,
>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    apps_json: JString<'a>,
) {
    let json_str: String = env
        .get_string(&apps_json)
        .map(|s| s.into())
        .unwrap_or_else(|_| "[]".to_string());

    logging!(
        info,
        Type::Core,
        "Split tunnel apps updated (JNI callback): {}",
        json_str
    );

    // Parse JSON array of package names
    let apps: Vec<String> = serde_json::from_str(&json_str).unwrap_or_default();

    // Update shared state
    if let Ok(mut state) = crate::cmd::android::SPLIT_TUNNEL_APPS.write() {
        *state = apps;
    }
}
