#!/bin/bash

# Run cargo build
/usr/local/cargo/bin/cargo build

# Check if cargo build was successful
if [ $? -eq 0 ]; then
    echo "Build succeeded, flashing the binary to the board..."
    openocd -s tcl \
        -f interface/cmsis-dap.cfg \
        -f target/rp2040.cfg \
        -c "adapter speed 5000" \
        -c "program /workspaces/myFirstBlinky/MyFirstRustBlinky/embassy_boardlighton/target/thumbv6m-none-eabi/debug/embassy_boardlighton verify reset exit"
else
    echo "Build failed, please check the errors and try again."
fi