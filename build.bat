@echo off
cargo build --release
cd target/release
start.
echo press any key to close
pause > nul
exit