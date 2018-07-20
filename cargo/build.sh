#!/bin/bash

cp cargo-config.toml ~/.cargo/config
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo lipo --release

cd ../android/app/src/main
mkdir jniLibs
mkdir jniLibs/arm64
mkdir jniLibs/armeabi
mkdir jniLibs/x86

ln -s /Users/$USER/Projects/middleware/cargo/target/aarch64-linux-android/release/libmiddleware.so jniLibs/arm64/libmiddleware.so
ln -s /Users/$USER/Projects/middleware/cargo/target/armv7-linux-androideabi/release/libmiddleware.so jniLibs/armeabi/libmiddleware.so
ln -s /Users/$USER/Projects/middleware/cargo/target/i686-linux-android/release/libmiddleware.so jniLibs/x86/libmiddleware.so
