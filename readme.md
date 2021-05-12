# Positivelys mono-repo

## Running Locally

The AppContext has an  `environment` property which can be defined as "development" or an empty string. When set to
"development" then the views are retrieved from a local server rather than the file system. This is done so
changes can be made to the views without having to recompile or restart the emulators. A local server must be running
to serve up the views. Inside `rust-code` start a development server `basic-http-server -a 0.0.0.0:3000`.

