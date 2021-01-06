#!/bin/sh

rm -rf assets/*
mkdir assets/static/

#copy over js to static assets
mkdir assets/static/js
cp node_modules/jquery/dist/jquery.slim.js assets/static/js
cp node_modules/popper.js/dist/umd/popper.js assets/static/js
cp node_modules/bootstrap/dist/js/bootstrap.js assets/static/js
cp node_modules/morphdom/dist/morphdom-umd.js assets/static/js

#copy over css to static assets
mkdir assets/static/css
cp node_modules/bootstrap/dist/css/bootstrap.css assets/static/css

# copy over assets to Android
rm -rf android/app/src/main/assets/*
mkdir android/app/src/main/assets/assets
mkdir android/app/src/main/assets/assets/static

mkdir android/app/src/main/assets/assets/static/js
cp -rf assets/static/js/ android/app/src/main/assets/assets/static/js/

mkdir android/app/src/main/assets/assets/static/css
cp -rf assets/static/css/ android/app/src/main/assets/assets/static/css/

