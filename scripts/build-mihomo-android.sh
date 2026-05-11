#!/bin/bash
# Cross-compile mihomo as shared library for Android
# Prerequisites: Go 1.22+, Android NDK (set NDK_HOME)
# Usage: ./scripts/build-mihomo-android.sh [mihomo-source-dir]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
JNILIBS_DIR="$PROJECT_ROOT/src-tauri/gen/android/app/src/main/jniLibs"

MIHOMO_DIR="${1:-$PROJECT_ROOT/../mihomo}"

# Validate environment
if [ -z "${NDK_HOME:-}" ]; then
    echo "ERROR: NDK_HOME is not set."
    echo "Please set NDK_HOME to your Android NDK installation path."
    echo "Example: export NDK_HOME=\$ANDROID_HOME/ndk/27.2.12479018"
    exit 1
fi

if ! command -v go &> /dev/null; then
    echo "ERROR: Go is not installed or not in PATH."
    exit 1
fi

if [ ! -d "$MIHOMO_DIR" ]; then
    echo "ERROR: Mihomo source directory not found: $MIHOMO_DIR"
    echo "Usage: $0 [mihomo-source-dir]"
    echo "Default: ../mihomo (relative to project root)"
    exit 1
fi

TOOLCHAIN="$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin"

if [ ! -d "$TOOLCHAIN" ]; then
    # Try macOS path
    TOOLCHAIN="$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin"
    if [ ! -d "$TOOLCHAIN" ]; then
        echo "ERROR: NDK toolchain not found. Check NDK_HOME: $NDK_HOME"
        exit 1
    fi
fi

echo "=== Mihomo Android Cross-Compilation ==="
echo "NDK_HOME:    $NDK_HOME"
echo "Toolchain:   $TOOLCHAIN"
echo "Mihomo dir:  $MIHOMO_DIR"
echo "Output dir:  $JNILIBS_DIR"
echo ""

# Create output directories
mkdir -p "$JNILIBS_DIR/arm64-v8a"
mkdir -p "$JNILIBS_DIR/armeabi-v7a"
mkdir -p "$JNILIBS_DIR/x86_64"

build_for_arch() {
    local arch="$1"
    local goarch="$2"
    local cc="$3"
    local output_dir="$JNILIBS_DIR/$arch"
    local extra_env="${4:-}"

    echo "--- Building for $arch (GOARCH=$goarch) ---"

    (
        cd "$MIHOMO_DIR"
        env CGO_ENABLED=1 \
            GOOS=android \
            GOARCH="$goarch" \
            CC="$TOOLCHAIN/$cc" \
            $extra_env \
            go build -buildmode=c-shared \
            -trimpath \
            -ldflags="-s -w" \
            -o "$output_dir/libmihomo.so" \
            ./
    )

    # Remove the generated header file (not needed)
    rm -f "$output_dir/libmihomo.h"

    local size
    size=$(du -h "$output_dir/libmihomo.so" | cut -f1)
    echo "    Output: $output_dir/libmihomo.so ($size)"
    echo ""
}

# Build for arm64-v8a
build_for_arch "arm64-v8a" "arm64" "aarch64-linux-android24-clang"

# Build for armeabi-v7a
build_for_arch "armeabi-v7a" "arm" "armv7a-linux-androideabi24-clang" "GOARM=7"

# Build for x86_64
build_for_arch "x86_64" "amd64" "x86_64-linux-android24-clang"

echo "=== Build Summary ==="
echo ""
echo "Libraries compiled successfully:"
for abi in arm64-v8a armeabi-v7a x86_64; do
    if [ -f "$JNILIBS_DIR/$abi/libmihomo.so" ]; then
        size=$(du -h "$JNILIBS_DIR/$abi/libmihomo.so" | cut -f1)
        echo "  $abi/libmihomo.so ($size)"
    else
        echo "  $abi/libmihomo.so (MISSING)"
    fi
done
echo ""
echo "Done! Libraries are ready in: $JNILIBS_DIR"
