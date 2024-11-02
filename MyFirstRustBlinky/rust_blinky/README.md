
## New stuff

Make sure libudev-sys is installed

`sudo apt-get install libudev-dev`


## OLD STUFF
Use the command 

`cargo build --target thumbv6m-none-eabi`

Then rename the rust_blinky file to rust_blinky.elf

And to deploy
`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/rust_blinky/target/thumbv6m-none-eabi/debug/rust_blinky' -fx`

Looks like everytime you unplug the device, you need to reboot the dev container.

Also, you need to hold down the bootsel button while plugging in the pico to put it into bootsel mode.


`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/rust_blinky/target/thumbv6m-none-eabi/release/blinky.elf' -fx`