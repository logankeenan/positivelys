#!/bin/sh

rm -rf assets/*

#copy over js to static assets
mkdir assets/static/
mkdir assets/static/js
cp node_modules/jquery/dist/jquery.slim.js assets/static/js

# copy over assets to Android
rm -rf android/app/src/main/assets/*
mkdir android/app/src/main/assets/assets
mkdir android/app/src/main/assets/assets/static
mkdir android/app/src/main/assets/assets/static/js

cp -rf assets/static/js/ android/app/src/main/assets/assets/static/js/