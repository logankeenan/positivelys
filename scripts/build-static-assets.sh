#!/bin/sh

rm -rf assets/static
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

#build scss
sass app/scss/styles.scss assets/app/css/styles.css

# copy over fonts
cp -rf app/fonts assets/app/

# copy over assets to Android
rm -rf android/app/src/main/assets/*
mkdir android/app/src/main/assets/assets
mkdir android/app/src/main/assets/assets/static
cp -rf assets/static/ android/app/src/main/assets/assets/static/


mkdir android/app/src/main/assets/assets/app
cp -rf assets/app/ android/app/src/main/assets/assets/app