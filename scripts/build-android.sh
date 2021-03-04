#!/bin/sh

cd rust-android
cargo clean

if [ -n "$1" ] && [ $1 = '--release' ]
then
  cargo build --target aarch64-linux-android --release
  cargo build --target armv7-linux-androideabi --release
else
  cargo build --target aarch64-linux-android
  cargo build --target armv7-linux-androideabi
  cargo build --target i686-linux-android
fi

cd ../

LIBS_DIR="android/app/src/main/jniLibs"

rm -rf ${LIBS_DIR};

mkdir ${LIBS_DIR}
mkdir ${LIBS_DIR}/arm64-v8a
mkdir ${LIBS_DIR}/armeabi-v7a
mkdir ${LIBS_DIR}/x86

if [ -n "$1" ] && [ $1 = '--release' ]
then
  cp rust-android/target/aarch64-linux-android/release/librust_android.so  ${LIBS_DIR}/arm64-v8a/librust_android.so
  cp rust-android/target/armv7-linux-androideabi/release/librust_android.so  ${LIBS_DIR}/armeabi-v7a/librust_android.so
  cp rust-android/target/i686-linux-android/release/librust_android.so  ${LIBS_DIR}/x86/librust_android.so
else
  cp rust-android/target/aarch64-linux-android/debug/librust_android.so  ${LIBS_DIR}/arm64-v8a/librust_android.so
  cp rust-android/target/armv7-linux-androideabi/debug/librust_android.so  ${LIBS_DIR}/armeabi-v7a/librust_android.so
  cp rust-android/target/i686-linux-android/debug/librust_android.so  ${LIBS_DIR}/x86/librust_android.so
fi


