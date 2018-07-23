#!/bin/bash
export CARGO_DIR=`pwd`

rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
cargo lipo --release

cd ../android/app/src/main
mkdir jniLibs
mkdir jniLibs/arm64
mkdir jniLibs/armeabi
mkdir jniLibs/x86

ln -s $CARGO_DIR/target/aarch64-linux-android/release/libmiddleware.so jniLibs/arm64/libmiddleware.so
ln -s $CARGO_DIR/target/armv7-linux-androideabi/release/libmiddleware.so jniLibs/armeabi/libmiddleware.so
ln -s $CARGO_DIR/target/i686-linux-android/release/libmiddleware.so jniLibs/x86/libmiddleware.so

cd ../../../../ios
ln -s $CARGO_DIR/target/universal/release/libmiddleware.a Flutter/libmiddleware.a
