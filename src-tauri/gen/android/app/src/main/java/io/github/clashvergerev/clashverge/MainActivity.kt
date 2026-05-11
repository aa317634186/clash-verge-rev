package io.github.clashvergerev.clashverge

import android.content.Intent
import android.os.Bundle
import app.tauri.activity.TauriActivity
import io.github.clashvergerev.clashverge.vpn.VpnManager

class MainActivity : TauriActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        handleVpnPermissionRequest(intent)
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        handleVpnPermissionRequest(intent)
    }

    /**
     * Handles VPN permission requests from Quick Settings tile or other sources.
     */
    private fun handleVpnPermissionRequest(intent: Intent?) {
        if (intent?.getBooleanExtra("request_vpn_permission", false) == true) {
            val prefs = getSharedPreferences("vpn_prefs", MODE_PRIVATE)
            val configPath = prefs.getString("config_path", null) ?: return
            VpnManager.prepare(this, configPath)
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == VpnManager.VPN_REQUEST_CODE) {
            if (resultCode == RESULT_OK) {
                VpnManager.start(this, VpnManager.pendingConfigPath ?: return)
            }
        }
    }
}
