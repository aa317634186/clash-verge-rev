package io.github.clashvergerev.clashverge.vpn

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.net.VpnService

object VpnManager {

    const val VPN_REQUEST_CODE = 1001

    /**
     * Stores the config path while waiting for VPN permission result.
     */
    var pendingConfigPath: String? = null
        private set

    /**
     * Prepares the VPN service. If the system needs user consent, launches the
     * permission dialog. Returns true if VPN is already prepared, false if
     * waiting for user consent.
     */
    fun prepare(activity: Activity, configPath: String): Boolean {
        val intent = VpnService.prepare(activity)
        return if (intent != null) {
            pendingConfigPath = configPath
            activity.startActivityForResult(intent, VPN_REQUEST_CODE)
            false
        } else {
            pendingConfigPath = null
            true
        }
    }

    /**
     * Starts the VPN service with the given configuration path.
     */
    fun start(context: Context, configPath: String) {
        pendingConfigPath = null
        val intent = Intent(context, ClashVpnService::class.java).apply {
            action = ClashVpnService.ACTION_START
            putExtra(ClashVpnService.EXTRA_CONFIG_PATH, configPath)
        }
        context.startForegroundService(intent)
    }

    /**
     * Stops the VPN service.
     */
    fun stop(context: Context) {
        val intent = Intent(context, ClashVpnService::class.java).apply {
            action = ClashVpnService.ACTION_STOP
        }
        context.startService(intent)
    }

    /**
     * Restarts the VPN service with the given configuration path.
     */
    fun restart(context: Context, configPath: String) {
        stop(context)
        start(context, configPath)
    }

    /**
     * Returns whether the VPN service is currently running.
     */
    fun isRunning(): Boolean {
        return ClashVpnService.isRunning
    }
}
