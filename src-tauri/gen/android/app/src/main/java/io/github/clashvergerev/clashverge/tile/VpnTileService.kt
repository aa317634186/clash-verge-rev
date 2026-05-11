package io.github.clashvergerev.clashverge.tile

import android.content.Context
import android.content.Intent
import android.net.VpnService
import android.os.Build
import android.service.quicksettings.Tile
import android.service.quicksettings.TileService
import io.github.clashvergerev.clashverge.MainActivity
import io.github.clashvergerev.clashverge.vpn.VpnManager

class VpnTileService : TileService() {

    override fun onStartListening() {
        super.onStartListening()
        updateTile()
    }

    override fun onClick() {
        super.onClick()
        if (VpnManager.isRunning()) {
            VpnManager.stop(this)
        } else {
            // Check if VPN permission is granted before attempting to start
            val prepareIntent = VpnService.prepare(this)
            if (prepareIntent != null) {
                // VPN permission not granted - launch MainActivity to handle consent
                val activityIntent = Intent(this, MainActivity::class.java).apply {
                    flags = Intent.FLAG_ACTIVITY_NEW_TASK
                    putExtra("request_vpn_permission", true)
                }
                startActivityAndCollapse(activityIntent)
                return
            }

            // Permission is granted, proceed to start VPN
            val prefs = getSharedPreferences("vpn_prefs", Context.MODE_PRIVATE)
            val configPath = prefs.getString("config_path", null)
            if (configPath != null) {
                VpnManager.start(this, configPath)
            }
        }
        updateTile()
    }

    private fun updateTile() {
        val tile = qsTile ?: return
        val isActive = VpnManager.isRunning()

        tile.state = if (isActive) Tile.STATE_ACTIVE else Tile.STATE_INACTIVE
        tile.label = "Clash Verge"

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            tile.stateDescription = if (isActive) "Connected" else "Disconnected"
        }

        tile.updateTile()
    }

    private fun startActivityAndCollapse(intent: Intent) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startActivityAndCollapse(android.app.PendingIntent.getActivity(
                this, 0, intent,
                android.app.PendingIntent.FLAG_UPDATE_CURRENT or android.app.PendingIntent.FLAG_IMMUTABLE
            ))
        } else {
            @Suppress("DEPRECATION")
            startActivityAndCollapse(intent)
        }
    }
}
