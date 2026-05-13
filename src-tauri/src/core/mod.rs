#[cfg(target_os = "android")]
pub mod android_bridge;
pub mod async_proxy_query;
pub mod backup;
pub mod event_driven_proxy;
pub mod handle;
#[cfg(not(target_os = "android"))]
pub mod hotkey;
pub mod logger;
pub mod manager;
mod notification;
pub mod service;
pub mod sysopt;
pub mod timer;
#[cfg(not(target_os = "android"))]
pub mod tray;
pub mod validate;
pub mod win_uwp;

pub use self::{event_driven_proxy::EventDrivenProxyManager, manager::CoreManager, timer::Timer};
