
## Notes

So I have discovered that cortex-m is required because it implements the critical_section needed to compile this code. 


`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/rust_pac/target/thumbv6m-none-eabi/debug/rust_pac_blinky.elf' -fx`