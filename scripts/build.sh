#!/bin/sh


if [ -n "$1" ] && [ $1 = '--release' ]
then
  . ./scripts/build-android.sh --release
  . ./scripts/build-ios.sh --release
else
  . ./scripts/build-android.sh
  . ./scripts/build-ios.sh
fi
