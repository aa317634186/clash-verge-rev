package io.github.clashvergerev.clashverge.receiver

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import io.github.clashvergerev.clashverge.vpn.VpnManager

class BootReceiver : BroadcastReceiver() {

    companion object {
        private const val TAG = "BootReceiver"
        private const val PREFS_NAME = "vpn_prefs"
        private const val KEY_AUTO_START = "auto_start_on_boot"
        private const val KEY_CONFIG_PATH = "config_path"
    }

    override fun onReceive(context: Context, intent: Intent) {
        if (intent.action != Intent.ACTION_BOOT_COMPLETED) return

        val prefs = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)
        val autoStart = prefs.getBoolean(KEY_AUTO_START, false)

        if (!autoStart) {
            Log.d(TAG, "Auto-start on boot is disabled")
            return
        }

        val configPath = prefs.getString(KEY_CONFIG_PATH, null)
        if (configPath == null) {
            Log.w(TAG, "Auto-start enabled but no config path found")
            return
        }

        Log.i(TAG, "Starting VPN service on boot")
        VpnManager.start(context, configPath)
    }
}
