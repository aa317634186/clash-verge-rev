# Clash Verge ProGuard Rules

# Keep VPN service and related classes
-keep class io.github.clashvergerev.clashverge.vpn.** { *; }

# Keep Quick Settings tile service
-keep class io.github.clashvergerev.clashverge.tile.** { *; }

# Keep boot receiver
-keep class io.github.clashvergerev.clashverge.receiver.** { *; }

# Keep all native (JNI) methods
-keepclassmembers class * {
    native <methods>;
}

# Keep Tauri framework classes
-dontwarn app.tauri.**
-keep class app.tauri.** { *; }

# Keep classes referenced from AndroidManifest.xml
-keep public class * extends android.app.Service
-keep public class * extends android.content.BroadcastReceiver
-keep public class * extends android.service.quicksettings.TileService
