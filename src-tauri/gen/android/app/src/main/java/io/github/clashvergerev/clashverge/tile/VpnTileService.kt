package io.github.clashvergerev.clashverge.tile

import android.content.Context
import android.os.Build
import android.service.quicksettings.Tile
import android.service.quicksettings.TileService
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
            // Get the default config path from shared preferences
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
}
