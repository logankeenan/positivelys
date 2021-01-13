#!/bin/sh

cd ./rust-ios

cargo clean
cbindgen src/lib.rs -l c > rust-ios.h

if [ -n "$1" ] && [ $1 = '--release' ]
then
  cargo lipo --release
else
  cargo lipo
fi



cd ../ios
rm -rf rust-ios

mkdir -p rust-ios/libs
mkdir -p rust-ios/include

cd ../
cp rust-ios/rust-ios.h ios/rust-ios/include


if [ -n "$1" ] && [ $1 = '--release' ]
then
  cp rust-ios/target/universal/release/librust_ios.a ios/rust-ios/libs
else
  cp rust-ios/target/universal/debug/librust_ios.a ios/rust-ios/libs
fi



