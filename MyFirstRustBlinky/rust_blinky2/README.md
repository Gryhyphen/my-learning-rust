# How to flash

In order to flash the thing correctly, first must run

`cargo run --bin blinky`

then run

`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/rust_blinky2/target/thumbv6m-none-eabi/debug/blinky.elf' -fx`