//! Android-specific utility functions
use anyhow::Result;
use std::path::PathBuf;

/// Get Android app internal files directory.
/// On Android, this is typically /data/data/<package>/files/.
/// The actual path is provided by the Tauri runtime or environment variable.
pub fn app_data_dir() -> Result<PathBuf> {
    let path = std::env::var("CLASH_VERGE_DATA_DIR")
        .unwrap_or_else(|_| "/data/data/io.github.clashvergerev.clashverge/files".to_string());
    Ok(PathBuf::from(path))
}

/// Get Android app cache directory.
pub fn app_cache_dir() -> Result<PathBuf> {
    let path = std::env::var("CLASH_VERGE_CACHE_DIR")
        .unwrap_or_else(|_| "/data/data/io.github.clashvergerev.clashverge/cache".to_string());
    Ok(PathBuf::from(path))
}

/// IPC path on Android (local socket in app directory).
pub fn ipc_path() -> Result<PathBuf> {
    let data_dir = app_data_dir()?;
    Ok(data_dir.join("verge-mihomo.sock"))
}

/// Ensure a safe directory for mihomo on Android (use app cache dir).
pub fn ensure_mihomo_safe_dir() -> Option<PathBuf> {
    app_cache_dir().ok()
}

/// Get Android app resources directory (native libs location).
pub fn app_resources_dir() -> Result<PathBuf> {
    let path = std::env::var("CLASH_VERGE_LIB_DIR")
        .unwrap_or_else(|_| "/data/data/io.github.clashvergerev.clashverge/lib".to_string());
    Ok(PathBuf::from(path))
}
