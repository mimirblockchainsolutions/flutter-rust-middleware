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

**note:** ***these need to be the location of the directories or your variables should reflect
the directories they are actually located at***

```
Mac:
  export ANDROID_HOME=/Users/$USER/Library/Android/sdk

  export NDK_HOME=$ANDROID_HOME/ndk-bundle

Linux:
  must add

```

Then, you need to run the ndk script to build your compile targets from the root folder of the project

`./ndk.sh`

Next, change to the cargo directory

`cd cargo`

and run the cargo build script

`./build.sh`

finally, you can use **flutter** to build the project for android or ios from the root directory of the project:

***(you must build with flutter before opening with android studio or Xcode)***

```
For iOS:
  flutter build ios

For Android:
  flutter build apk
```


# TODO
- Streamline where possible
- Remove all unnecessary files except for configs with necessary linking pointers to built libraries and dependencies
- Turn into flutter library if it's possible to include the cross building process
