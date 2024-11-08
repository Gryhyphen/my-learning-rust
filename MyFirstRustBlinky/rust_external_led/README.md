


```shell

sudo openocd -s tcl \
-f interface/cmsis-dap.cfg \
-f target/rp2040.cfg \
-c "adapter speed 5000" \
-c "program /workspaces/myFirstBlinky/MyFirstRustBlinky/rust_external_led/target/thumbv6m-none-eabi/debug/rust_external_led verify reset exit"

```