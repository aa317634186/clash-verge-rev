package io.github.clashvergerev.clashverge.vpn

import android.content.Context
import android.net.VpnService
import android.util.Log

/**
 * Manages per-app proxy (split tunneling) configuration.
 * Supports whitelist mode (only selected apps use VPN) and
 * blacklist mode (all apps except selected use VPN).
 */
object SplitTunnelManager {

    private const val TAG = "SplitTunnelManager"
    private const val PREFS_NAME = "split_tunnel_prefs"
    private const val KEY_MODE = "mode"
    private const val KEY_APPS = "apps"

    enum class Mode {
        DISABLED,
        WHITELIST,
        BLACKLIST
    }

    fun getMode(context: Context): Mode {
        val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val modeStr = prefs.getString(KEY_MODE, Mode.DISABLED.name) ?: Mode.DISABLED.name
        return try {
            Mode.valueOf(modeStr)
        } catch (e: IllegalArgumentException) {
            Mode.DISABLED
        }
    }

    fun setMode(context: Context, mode: Mode) {
        val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        prefs.edit().putString(KEY_MODE, mode.name).apply()
    }

    fun getApps(context: Context): Set<String> {
        val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        return prefs.getStringSet(KEY_APPS, emptySet()) ?: emptySet()
    }

    fun addApp(context: Context, packageName: String) {
        val apps = getApps(context).toMutableSet()
        apps.add(packageName)
        setApps(context, apps)
    }

    fun removeApp(context: Context, packageName: String) {
        val apps = getApps(context).toMutableSet()
        apps.remove(packageName)
        setApps(context, apps)
    }

    fun setApps(context: Context, packages: Set<String>) {
        val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        prefs.edit().putStringSet(KEY_APPS, packages).apply()
    }

    /**
     * Applies split tunneling configuration to the VPN builder.
     * In WHITELIST mode, only the selected apps will route through VPN.
     * In BLACKLIST mode, all apps except selected will route through VPN.
     * In DISABLED mode, no per-app filtering is applied.
     */
    fun applyToVpnBuilder(builder: VpnService.Builder, context: Context) {
        val mode = getMode(context)
        if (mode == Mode.DISABLED) return

        val apps = getApps(context)
        if (apps.isEmpty()) return

        when (mode) {
            Mode.WHITELIST -> {
                for (packageName in apps) {
                    try {
                        builder.addAllowedApplication(packageName)
                    } catch (e: Exception) {
                        Log.w(TAG, "Failed to add allowed app: $packageName", e)
                    }
                }
            }
            Mode.BLACKLIST -> {
                for (packageName in apps) {
                    try {
                        builder.addDisallowedApplication(packageName)
                    } catch (e: Exception) {
                        Log.w(TAG, "Failed to add disallowed app: $packageName", e)
                    }
                }
            }
            Mode.DISABLED -> { /* no-op */ }
        }
    }
}
