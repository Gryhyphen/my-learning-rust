



`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/direct_unsafe_blinky/target/thumbv6m-none-eabi/debug/direct_unsafe_blinky.elf' -fx`


## UPDATE 1:

Okay I finally got this to work. But now I am confuzelled by why both
SET and just directly setting the value on the register works. Need to learn why


## UPDATE 2:
To get SWD working, need to have the following installed in the container

Need to `sudo apt install openocd`
and `sudo apt install gdb-multiarch`


Then to flash run (we don't need the elf extension anymore too!)

```shell

sudo openocd -s tcl \
-f interface/cmsis-dap.cfg \
-f target/rp2040.cfg \
-c "adapter speed 5000" \
-c "program /workspaces/myFirstBlinky/MyFirstRustBlinky/direct_unsafe_blinky/target/thumbv6m-none-eabi/debug/direct_unsafe_blinky.elf verify reset exit"

```

## How to debug

Then to debug

```shell

# First start the open ocd server
sudo openocd -s tcl \
-f interface/cmsis-dap.cfg \
-f target/rp2040.cfg \
-c "adapter speed 5000"

```

Then run the "Cortex-Debug with external OpenOCD" debug configuration.

Right now unable to directly flash the pico through vscode / directly start-up the openocd debug server through vscode. This is because within this dev container environment, currently these operations are required to be run with sudo level permissions.

Without sudo, these commands do not have sufficent permissions to read... whatever files or ports that allow it to connect to the pico.

Some of these issues definitely are related to my dev container environment. I'm sure there are ways to tweak it to grant sufficent permissions so that the commands could be run without sudo, and thus can be run in vscode (without running vscode in sudo mode). But idk and it's a minor inconvience for me rn.

