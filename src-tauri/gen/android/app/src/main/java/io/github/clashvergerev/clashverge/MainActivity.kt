package io.github.clashvergerev.clashverge

import android.content.Intent
import android.os.Bundle
import app.tauri.activity.TauriActivity
import io.github.clashvergerev.clashverge.vpn.VpnManager

class MainActivity : TauriActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
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
