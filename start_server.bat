@echo off

@REM Starts a local web-server that serves the contents of the `doc/` folder,
@REM which is the folder to where the web version is compiled.

echo "open http://localhost:3004"

(cd docs && basic-http-server --addr 127.0.0.1:3004 .)
@REM (cd docs && python3 -m http.server 8080 --bind 127.0.0.1)
