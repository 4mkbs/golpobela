#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_DIR="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/Android/Sdk}}"
NDK_DIR="${ANDROID_NDK_HOME:-${NDK_HOME:-$SDK_DIR/ndk/25.2.9519653}}"
KEYSTORE="$HOME/.android/golpo-release.p12"
KEY_PASSWORD="$HOME/.android/golpo-release.password"
ANDROID_PROJECT="$PROJECT_DIR/target/dx/golpo/release/android/app"
APK_SIGNER="$SDK_DIR/build-tools/36.0.0/apksigner"

if ! command -v dx >/dev/null 2>&1; then
  echo "Dioxus CLI not found. Install it with: cargo install dioxus-cli --locked" >&2
  exit 1
fi

if [[ ! -d "$NDK_DIR" ]]; then
  echo "Android NDK not found at: $NDK_DIR" >&2
  echo "Install it with: sdkmanager --install 'ndk;25.2.9519653'" >&2
  exit 1
fi

if [[ ! -f "$KEYSTORE" || ! -f "$KEY_PASSWORD" ]]; then
  echo "Release signing key is missing from $HOME/.android" >&2
  exit 1
fi

export ANDROID_HOME="$SDK_DIR"
export ANDROID_SDK_ROOT="$SDK_DIR"
export ANDROID_NDK_HOME="$NDK_DIR"
export NDK_HOME="$NDK_DIR"
export PATH="$PATH:$SDK_DIR/platform-tools:$SDK_DIR/cmdline-tools/latest/bin"

cd "$PROJECT_DIR"
if [[ -d "$ANDROID_PROJECT/app/src/main/jniLibs" ]]; then
  find "$ANDROID_PROJECT/app/src/main/jniLibs" -mindepth 2 -maxdepth 2 -type f -name 'libmain.so' -delete
fi

dx bundle --android --release --target aarch64-linux-android --package-types apk
(
  cd "$ANDROID_PROJECT"
  ./gradlew --no-daemon :app:assembleRelease
)

UNSIGNED_APK="$ANDROID_PROJECT/app/build/outputs/apk/release/app-release-unsigned.apk"
if [[ ! -f "$UNSIGNED_APK" ]]; then
  echo "Gradle did not produce the expected release APK: $UNSIGNED_APK" >&2
  exit 1
fi

mkdir -p "$PROJECT_DIR/dist"
SIGNED_APK="$PROJECT_DIR/dist/GolpoBela-arm64-release.apk"
"$APK_SIGNER" sign \
  --ks "$KEYSTORE" \
  --ks-key-alias golpo-release \
  --ks-pass "file:$KEY_PASSWORD" \
  --out "$SIGNED_APK" \
  "$UNSIGNED_APK"
"$APK_SIGNER" verify --verbose --print-certs "$SIGNED_APK"
echo "Signed APK: $SIGNED_APK"
