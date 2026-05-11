/**
 * Platform detection utilities for mobile/desktop differentiation.
 */

/**
 * Returns the current operating system identifier.
 * On Android, uses both user-agent detection and the build-time OS_PLATFORM define
 * (which is set to "android" when building with `tauri android build`).
 */
export function getOS():
  | "android"
  | "ios"
  | "windows"
  | "macos"
  | "linux"
  | "unknown" {
  const ua = navigator.userAgent;
  const platform = OS_PLATFORM;

  // User-agent check takes priority for runtime accuracy in WebView
  if (/android/i.test(ua) || platform === "android") return "android";
  if (/iPad|iPhone|iPod/.test(ua)) return "ios";
  if (ua.includes("Mac OS X") || platform === "darwin") return "macos";
  if (/win64|win32/i.test(ua) || platform === "win32") return "windows";
  if (/linux/i.test(ua) || platform === "linux") return "linux";

  return "unknown";
}

/**
 * Returns true if running on Android.
 */
export function isAndroid(): boolean {
  return getOS() === "android";
}

/**
 * Returns true if running on a mobile platform (Android or iOS).
 */
export function isMobile(): boolean {
  const os = getOS();
  return os === "android" || os === "ios";
}
