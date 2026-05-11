package io.github.clashvergerev.clashverge.vpn

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.ServiceInfo
import android.net.VpnService
import android.os.Build
import android.os.ParcelFileDescriptor
import android.util.Log
import io.github.clashvergerev.clashverge.MainActivity

class ClashVpnService : VpnService() {

    companion object {
        private const val TAG = "ClashVpnService"
        private const val NOTIFICATION_ID = 1
        private const val CHANNEL_ID = "clash_vpn"
        const val ACTION_START = "io.github.clashvergerev.clashverge.vpn.START"
        const val ACTION_STOP = "io.github.clashvergerev.clashverge.vpn.STOP"
        const val EXTRA_CONFIG_PATH = "config_path"

        @Volatile
        var isRunning: Boolean = false
            private set
    }

    private var tunInterface: ParcelFileDescriptor? = null

    override fun onCreate() {
        super.onCreate()
        createNotificationChannel()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        when (intent?.action) {
            ACTION_STOP -> {
                shutdown()
                return START_NOT_STICKY
            }
            ACTION_START, null -> {
                val configPath = intent?.getStringExtra(EXTRA_CONFIG_PATH)
                if (configPath == null) {
                    Log.e(TAG, "No config path provided")
                    stopSelf()
                    return START_NOT_STICKY
                }
                startForegroundNotification()
                configure(configPath)
                return START_STICKY
            }
            else -> return START_NOT_STICKY
        }
    }

    private fun configure(configPath: String) {
        val builder = Builder()
            .setMtu(9000)
            .addAddress("172.19.0.1", 30)
            .addRoute("0.0.0.0", 0)
            .addRoute("::", 0)
            .addDns("1.1.1.1")
            .addDns("8.8.8.8")
            .setSession("Clash Verge")

        // Apply split tunneling rules
        SplitTunnelManager.applyToVpnBuilder(builder, this)

        tunInterface = builder.establish()
        if (tunInterface == null) {
            Log.e(TAG, "Failed to establish TUN interface")
            stopSelf()
            return
        }

        val tunFd = tunInterface!!.fd
        val started = MihomoCore.start(configPath, tunFd)
        if (!started) {
            Log.e(TAG, "Failed to start mihomo core")
            tunInterface?.close()
            tunInterface = null
            stopSelf()
            return
        }

        isRunning = true
        Log.i(TAG, "VPN service started successfully")
    }

    private fun shutdown() {
        isRunning = false
        MihomoCore.stop()
        tunInterface?.close()
        tunInterface = null
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
        Log.i(TAG, "VPN service stopped")
    }

    override fun onDestroy() {
        shutdown()
        super.onDestroy()
    }

    override fun onRevoke() {
        Log.i(TAG, "VPN permission revoked")
        shutdown()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Clash Verge VPN",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "VPN service status"
                setShowBadge(false)
            }
            val manager = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            manager.createNotificationChannel(channel)
        }
    }

    private fun startForegroundNotification() {
        val stopIntent = Intent(this, ClashVpnService::class.java).apply {
            action = ACTION_STOP
        }
        val stopPendingIntent = PendingIntent.getService(
            this,
            0,
            stopIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val openIntent = Intent(this, MainActivity::class.java).apply {
            flags = Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK
        }
        val openPendingIntent = PendingIntent.getActivity(
            this,
            0,
            openIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val notification = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ID)
        } else {
            @Suppress("DEPRECATION")
            Notification.Builder(this)
        }.apply {
            setContentTitle("Clash Verge VPN")
            setContentText("Connected")
            setSmallIcon(android.R.drawable.ic_lock_lock)
            setContentIntent(openPendingIntent)
            setOngoing(true)
            addAction(
                Notification.Action.Builder(
                    null,
                    "Stop",
                    stopPendingIntent
                ).build()
            )
        }.build()

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            startForeground(
                NOTIFICATION_ID,
                notification,
                ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE
            )
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
    }
}
