# Android Build Guide

This document provides comprehensive instructions for building Clash Verge Rev for Android.

## Prerequisites

| Tool        | Version             | Notes                      |
| ----------- | ------------------- | -------------------------- |
| Android SDK | API 35              | Platform and build tools   |
| Android NDK | r27 (27.2.12479018) | For native compilation     |
| JDK         | 17                  | Required by Android Gradle |
| Rust        | 1.91+               | With Android targets       |
| Go          | 1.22+               | For cross-compiling mihomo |
| Node.js     | 22+                 | Frontend toolchain         |
| pnpm        | 9.x                 | Package manager            |

## Environment Setup

### 1. Install Android SDK and NDK

Install Android Studio or use the command-line tools:

```bash
# Set environment variables (add to ~/.bashrc or ~/.zshrc)
export ANDROID_HOME="$HOME/Android/Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/27.2.12479018"
export PATH="$ANDROID_HOME/platform-tools:$PATH"
```

Install required SDK packages:

```bash
sdkmanager "platform-tools" \
  "platforms;android-35" \
  "build-tools;35.0.0" \
  "ndk;27.2.12479018"
```

### 2. Install Rust Android Targets

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

### 3. Install Go

Download Go 1.22+ from https://go.dev/dl/ and ensure it is in your PATH:

```bash
go version  # Should show 1.22 or higher
```

### 4. Install Node.js and pnpm

```bash
# Install Node.js 22+ via your preferred method (nvm, fnm, etc.)
node --version  # Should show v22.x

# Install pnpm
corepack enable
corepack prepare pnpm@latest --activate
```

## Cross-Compiling Mihomo

Mihomo is the proxy core used by Clash Verge. On Android, it is compiled as a shared library (`libmihomo.so`) and loaded via JNI.

### Automated Build

Use the provided script:

```bash
# Clone mihomo source
git clone https://github.com/MetaCubeX/mihomo.git ../mihomo

# Run the build script
./scripts/build-mihomo-android.sh ../mihomo
```

### Manual Build

If you prefer to build manually:

#### ARM64 (arm64-v8a)

```bash
export NDK_HOME="$ANDROID_HOME/ndk/27.2.12479018"
export TOOLCHAIN="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"

cd /path/to/mihomo
CGO_ENABLED=1 \
GOOS=android \
GOARCH=arm64 \
CC="$TOOLCHAIN/aarch64-linux-android24-clang" \
  go build -buildmode=c-shared -trimpath -ldflags="-s -w" \
  -o libmihomo.so ./
```

#### ARMv7 (armeabi-v7a)

```bash
CGO_ENABLED=1 \
GOOS=android \
GOARCH=arm \
GOARM=7 \
CC="$TOOLCHAIN/armv7a-linux-androideabi24-clang" \
  go build -buildmode=c-shared -trimpath -ldflags="-s -w" \
  -o libmihomo.so ./
```

#### x86_64

```bash
CGO_ENABLED=1 \
GOOS=android \
GOARCH=amd64 \
CC="$TOOLCHAIN/x86_64-linux-android24-clang" \
  go build -buildmode=c-shared -trimpath -ldflags="-s -w" \
  -o libmihomo.so ./
```

### Placing the Libraries

Copy the compiled `.so` files to the correct JNI directories:

```
src-tauri/gen/android/app/src/main/jniLibs/
  arm64-v8a/libmihomo.so
  armeabi-v7a/libmihomo.so
  x86_64/libmihomo.so
```

## Building the App

### Install Dependencies

```bash
pnpm install
```

### Initialize Android Project (first time only)

```bash
pnpm tauri android init
```

This generates the Android project structure under `src-tauri/gen/android/`.

### Debug Build

```bash
pnpm tauri android build --debug
```

### Release Build

```bash
pnpm tauri android build
```

The output APK will be located at:

```
src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
```

## Development Workflow

### Hot-Reload Development

1. Connect an Android device via USB (with USB debugging enabled) or start an emulator.
2. Run the development server:

```bash
pnpm tauri android dev
```

This will:

- Start the Vite dev server with hot-reload
- Build and install the debug APK on the connected device
- Open the app with WebView connected to the dev server

### Using npm Scripts

Convenience scripts are available in `package.json`:

```bash
pnpm android:dev      # Start development with hot-reload
pnpm android:build    # Build release APK
pnpm android:init     # Initialize Android project structure
```

## Signing

### Generate a Keystore

```bash
keytool -genkey -v \
  -keystore clash-verge.jks \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -alias clash-verge
```

### Configure Signing in Gradle

Add to `src-tauri/gen/android/app/build.gradle.kts`:

```kotlin
android {
    signingConfigs {
        create("release") {
            storeFile = file("path/to/clash-verge.jks")
            storePassword = System.getenv("KEYSTORE_PASSWORD") ?: ""
            keyAlias = "clash-verge"
            keyPassword = System.getenv("KEY_PASSWORD") ?: ""
        }
    }
    buildTypes {
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")
        }
    }
}
```

For CI, store keystore credentials as GitHub Secrets:

- `ANDROID_KEYSTORE_BASE64` - Base64-encoded keystore file
- `KEYSTORE_PASSWORD` - Keystore password
- `KEY_PASSWORD` - Key password

## Troubleshooting

### NDK not found

Ensure `NDK_HOME` is set correctly and the NDK version matches:

```bash
echo $NDK_HOME
ls $NDK_HOME/toolchains/llvm/prebuilt/
```

### Rust compilation fails for Android target

Ensure the Android targets are installed:

```bash
rustup target list --installed | grep android
```

If missing, add them:

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

### Go cross-compilation fails

Verify CGO is enabled and the correct clang from NDK is being used:

```bash
# Test that the compiler exists
$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android24-clang --version
```

### Gradle build fails with "SDK not found"

Create or verify `src-tauri/gen/android/local.properties`:

```properties
sdk.dir=/path/to/your/Android/Sdk
```

### App crashes on launch

1. Check that `libmihomo.so` is present in the correct `jniLibs/<abi>/` directory.
2. Verify the library was built for the correct ABI matching your device/emulator.
3. Check logcat for detailed error messages:

```bash
adb logcat -s "ClashVerge" "MihomoCore" "AndroidRuntime"
```

### WebView not loading

Ensure the Vite build output is present. For debug builds, the dev server must be accessible from the device:

```bash
# Check device can reach host
adb reverse tcp:1420 tcp:1420
```
