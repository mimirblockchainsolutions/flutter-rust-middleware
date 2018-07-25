# Middleware

Boilerplate code to patch Flutter with Rust on iOS and Android.

## Prerequisites

- `rust`
- `flutter`
- `cargo-lipo`
- `Android`
- `Android NDK`
- `Xcode`

## Getting Started

The first thing you need to do is set a few environment variables to help the process.
```
export ANDROID_HOME=/Users/$USER/Library/Android/sdk

export NDK_HOME=$ANDROID_HOME/ndk-bundle
```
And change your cargo/.cargo/config file and change <project_dir> to the location of this project on your computer
```
[target.aarch64-linux-android]
ar = "/<project_dir>/flutter-rust-middleware/NDK/arm64/bin/aarch64-linux-android-ar"
linker = "/<project_dir>/middleware/NDK/arm64/bin/aarch64-linux-android-clang"

```


Then, you need to run the ndk script to build your compile targets from the root folder of the project

`./ndk.sh`

Next, change to the cargo directory

`cd cargo`

and run the cargo build script

`./build.sh`

finally, you can use flutter to build the project for android or ios from the root directory of the project

```
For iOS:
  flutter build iOS

For Android:
  flutter build apk
```


# TODO
- Streamline where possible
- Turn into flutter library if it's possible to include the cross building process
