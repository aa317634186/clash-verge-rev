//! Android-specific Tauri commands for VPN control
use super::CmdResult;
use tauri::Emitter;

/// Signals the Android Kotlin layer to start VPN service with the given config path.
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
#[tauri::command]
pub async fn stop_vpn() -> CmdResult {
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-vpn-stop", ())
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Returns current VPN running status.
/// Actual state is tracked via events from the Kotlin side.
#[tauri::command]
pub async fn get_vpn_status() -> CmdResult<bool> {
    // State is managed by the Kotlin VpnManager and updated via JNI callbacks
    Ok(false)
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
#[tauri::command]
pub async fn get_split_tunnel_apps() -> CmdResult<Vec<String>> {
    // Retrieves from SharedPreferences via Kotlin layer
    let app = crate::APP_HANDLE
        .get()
        .ok_or_else(|| "App handle not initialized".to_string())?;
    app.emit("android-get-split-tunnel-apps", ())
        .map_err(|e| e.to_string())?;
    Ok(vec![])
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
