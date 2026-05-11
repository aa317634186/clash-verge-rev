/**
 * Platform detection utilities for mobile/desktop differentiation.
 */

/**
 * Returns the current operating system identifier.
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

  if (/android/i.test(ua) || platform === "android") return "android";
  if (/iPad|iPhone|iPod/.test(ua)) return "ios";
  if (ua.includes("Mac OS X") || platform === "darwin") return "macos";
  if (/win64|win32/i.test(ua) || platform === "win32") return "windows";
  if (/linux/i.test(ua)) return "linux";

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
