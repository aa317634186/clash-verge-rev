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
