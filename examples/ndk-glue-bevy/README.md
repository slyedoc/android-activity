
This is a minimal Bevy test app that runs on Android based on `NativeActivity` /
`ndk-glue` (it's just in this repo for convenience)

The same app can also be run on PC via `cargo run --features=desktop`

# Gradle Build
```
export ANDROID_NDK_HOME="path/to/ndk"
export ANDROID_HOME="path/to/sdk"

rustup target add aarch64-linux-android
cargo install cargo-ndk

cargo ndk -t arm64-v8a -o app/src/main/jniLibs/  build
./gradlew build
./gradlew installDebug
```

# Cargo APK Build
```
export ANDROID_NDK_HOME="path/to/ndk"
export ANDROID_SDK_HOME="path/to/sdk"

rustup target add aarch64-linux-android

cargo install cargo-apk
cargo apk run
```