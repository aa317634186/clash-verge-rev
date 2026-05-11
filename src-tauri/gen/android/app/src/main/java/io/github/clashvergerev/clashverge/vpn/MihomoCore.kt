package io.github.clashvergerev.clashverge.vpn

import android.util.Log

/**
 * JNI bridge to the mihomo (Clash.Meta) native library.
 * The native library (libmihomo.so) is loaded at class initialization
 * and provides the proxy core functionality.
 */
object MihomoCore {

    private const val TAG = "MihomoCore"

    init {
        try {
            System.loadLibrary("mihomo")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Failed to load libmihomo.so: ${e.message}")
        }
    }

    private external fun nativeStart(configPath: String, tunFd: Int): Boolean
    private external fun nativeStop()
    private external fun nativeIsRunning(): Boolean

    // JNI callbacks declared in android_bridge.rs - these are called from Kotlin
    // to notify the Rust backend about VPN state changes.
    @JvmStatic
    external fun onVpnStarted()
    @JvmStatic
    external fun onVpnStopped()
    @JvmStatic
    external fun onVpnError(errorMsg: String)

    /**
     * Starts the mihomo core with the given config file and TUN file descriptor.
     * Returns true if the core started successfully.
     */
    fun start(configPath: String, tunFd: Int): Boolean {
        return try {
            nativeStart(configPath, tunFd)
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Native method not available: ${e.message}")
            false
        }
    }

    /**
     * Stops the mihomo core.
     */
    fun stop() {
        try {
            nativeStop()
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Native method not available: ${e.message}")
        }
    }

    /**
     * Returns whether the mihomo core is currently running.
     */
    fun isRunning(): Boolean {
        return try {
            nativeIsRunning()
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "Native method not available: ${e.message}")
            false
        }
    }

    /**
     * Notifies the Rust backend that VPN has started.
     */
    fun notifyVpnStarted() {
        try {
            onVpnStarted()
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "onVpnStarted native method not available: ${e.message}")
        }
    }

    /**
     * Notifies the Rust backend that VPN has stopped.
     */
    fun notifyVpnStopped() {
        try {
            onVpnStopped()
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "onVpnStopped native method not available: ${e.message}")
        }
    }

    /**
     * Notifies the Rust backend that a VPN error occurred.
     */
    fun notifyVpnError(errorMsg: String) {
        try {
            onVpnError(errorMsg)
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "onVpnError native method not available: ${e.message}")
        }
    }
}
