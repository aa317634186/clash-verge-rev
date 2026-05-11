pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.PREFER_SETTINGS)
    repositories {
        google()
        mavenCentral()
    }
}

include(":app")

val tauriAndroidPackagePath = System.getenv("TAURI_ANDROID_PACKAGE_PATH")
    ?: error("TAURI_ANDROID_PACKAGE_PATH environment variable is not set")
include(":tauri-android")
project(":tauri-android").projectDir = file(tauriAndroidPackagePath)
