# Middleware

Boilerplate code to patch Flutter with Rust on iOS and Android.

## Getting Started

The first thing you need to do is set a few environment variables to help the process.
```
export ANDROID_HOME=/Users/$USER/Library/Android/sdk

export NDK_HOME=$ANDROID_HOME/ndk-bundle
```
Secondly, you need to run the ndk script to build your compile targets from the root folder of the project

`./ndk.sh`

Next, change to the cargo directory

`cd cargo`

and run the build script

`./build.sh`

finally, you can build the project from the root directory of the project

```
For iOS:
  flutter build iOS

For Android:
  flutter build apk
```
