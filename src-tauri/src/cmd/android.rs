//! Android-specific Tauri commands for VPN control
use super::CmdResult;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Emitter;

/// Shared VPN running state, updated by JNI callbacks from the Kotlin side.
pub static VPN_RUNNING: AtomicBool = AtomicBool::new(false);

/// Shared split tunnel app list, updated by JNI callback from the Kotlin side.
/// Kotlin calls `onSplitTunnelAppsUpdated` to populate this state.
pub static SPLIT_TUNNEL_APPS: RwLock<Vec<String>> = RwLock::new(Vec::new());

/// Signals the Android Kotlin layer to start VPN service with the given config path.
/// Note: This is an event-driven fire-and-forget command. The actual start result
/// is reported asynchronously via the "vpn-state-changed" event from JNI callbacks.
#[tauri::command]
pub async fn start_vpn(config_path: String) -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-vpn-start", config_path)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Signals the Android Kotlin layer to stop the VPN service.
/// Note: This is an event-driven fire-and-forget command. The actual stop result
/// is reported asynchronously via the "vpn-state-changed" event from JNI callbacks.
#[tauri::command]
pub async fn stop_vpn() -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-vpn-stop", ())
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Returns current VPN running status from shared atomic state.
/// The state is updated by JNI callbacks (onVpnStarted/onVpnStopped) from the Kotlin layer.
#[tauri::command]
pub async fn get_vpn_status() -> CmdResult<bool> {
    Ok(VPN_RUNNING.load(Ordering::Relaxed))
}

/// Requests VPN permission from the Android system via the Kotlin layer.
#[tauri::command]
pub async fn request_vpn_permission() -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-vpn-permission-request", ())
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Gets the list of apps configured for split tunneling.
/// Reads from a shared state populated by the Kotlin layer via JNI callback.
#[tauri::command]
pub async fn get_split_tunnel_apps() -> CmdResult<Vec<String>> {
    let apps = SPLIT_TUNNEL_APPS
        .read()
        .map_err(|e| format!("Failed to read split tunnel apps: {e}"))?;
    Ok(apps.clone())
}

/// Sets the list of apps for split tunneling.
#[tauri::command]
pub async fn set_split_tunnel_apps(apps: Vec<String>) -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-set-split-tunnel-apps", apps)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Sets the split tunnel mode: "whitelist", "blacklist", or "disabled".
#[tauri::command]
pub async fn set_split_tunnel_mode(mode: String) -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-set-split-tunnel-mode", mode)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Requests battery optimization exclusion from the Android system.
#[tauri::command]
pub async fn request_battery_optimization_exclusion() -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-request-battery-optimization-exclusion", ())
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Sets whether VPN should auto-start on device boot.
#[tauri::command]
pub async fn set_auto_start_on_boot(enabled: bool) -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-set-auto-start-on-boot", enabled)
        .map_err(|e| e.to_string())?;
    Ok(())
}
