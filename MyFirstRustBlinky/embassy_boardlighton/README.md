

# Notes - 1

Okay I've gotten the embassy hal crate for the rp2040 to work,
which also has in-built support for the bootloader.

However, I haven't gotten the embassy executor / embassy
main entry point to work. I've so far had to revert
to the cortext-m-rt entry point and the
cortext-m critical section implementation.

Also, I'm still getting multidrop rp2040 issues,
however this doesn't appear to be preventing me from flashing
the chip when I'm using cortex-m

# Notes - 2

I have since slowly been getting closer to the source of the issue.
We are now using the embassy critical section implementation.

This then leaves us looking at the embassy-executor as the culprit.

For some reason the main function isn't being invoked correctly when
we use the embassy main macro.

May try see if we can manually start the executor from the
cortex minimal runtime (cortex-m-rt).

But worth just seeing if there is an easy poke to get the following
minimal example to work:

```rust

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = init(Default::default());

    let mut onboard_led = Output::new(peripherals.PIN_25, Level::Low);
    onboard_led.set_high();
}

```

An example of how to start an executor from the cortex-m-rt entry point:

https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/multicore.rs

I'm only really planning on using one core anyway, but damn it is weird that
you have to use the cortex-m-rt entry for multicores, and this isn't abstracted
away. Cause doesn't that mean if you want to run tasks on different cores, for example,
for parallel programming, there is no nice api?


# Good scripts

## The Best Script

This script builds the application, and if successful, deploys it to the rp2040.

Note - I had to make it executable using `chmod +x /workspaces/myFirstBlinky/MyFirstRustBlinky/embassy_blinky/quickdirtybuild.sh`

Note - The -E is required to keep the current environment context when the script is running
	   otherwise the script can't find things like the toolchain and rust.

```shell
sudo -E ./quickdirtybuild.sh
```

## Other scripts

```shell

sudo openocd -s tcl \
-f interface/cmsis-dap.cfg \
-f target/rp2040.cfg \
-c "adapter speed 5000" \
-c "program /workspaces/myFirstBlinky/MyFirstRustBlinky/embassy_blinky/target/thumbv6m-none-eabi/debug/embassy_blinky verify reset exit"

```


```shell

echo '# SPDX-License-Identifier: GPL-2.0-or-later

# RP2040 is a microcontroller with dual Cortex-M0+ core.
# https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html

# The device requires multidrop SWD for debug.
transport select swd

source [find target/swj-dp.tcl]

if { [info exists CHIPNAME] } {
	set _CHIPNAME $CHIPNAME
} else {
	set _CHIPNAME rp2040
}

if { [info exists WORKAREASIZE] } {
	set _WORKAREASIZE $WORKAREASIZE
} else {
	set _WORKAREASIZE 0x10000
}

if { [info exists CPUTAPID] } {
	set _CPUTAPID $CPUTAPID
} else {
	set _CPUTAPID 0x01002927
}

# Set to '1' to start rescue mode
if { [info exists RESCUE] } {
	set _RESCUE $RESCUE
} else {
	set _RESCUE 0
}

# Set to '0' or '1' for single core configuration, 'SMP' for -rtos hwthread
# handling of both cores, anything else for isolated debugging of both cores
if { [info exists USE_CORE] } {
	set _USE_CORE $USE_CORE
} else {
	set _USE_CORE SMP
}
set _BOTH_CORES [expr { $_USE_CORE != 0 && $_USE_CORE != 1 }]

swj_newdap $_CHIPNAME cpu -expected-id $_CPUTAPID

# The rescue debug port uses the DP CTRL/STAT bit DBGPWRUPREQ to reset the
# PSM (power on state machine) of the RP2040 with a flag set in the
# VREG_AND_POR_CHIP_RESET register. Once the reset is released
# (by clearing the DBGPWRUPREQ flag), the bootrom will run, see this flag,
# and halt. Allowing the user to load some fresh code, rather than loading
# the potentially broken code stored in flash
if { $_RESCUE } {
	dap create $_CHIPNAME.rescue_dap -chain-position $_CHIPNAME.cpu -dp-id $_CPUTAPID -instance-id 0xf -ignore-syspwrupack
	init

	# Clear DBGPWRUPREQ
	$_CHIPNAME.rescue_dap dpreg 0x4 0x00000000

	# Verifying CTRL/STAT is 0
	set _CTRLSTAT [$_CHIPNAME.rescue_dap dpreg 0x4]
	if {[expr {$_CTRLSTAT & 0xf0000000}]} {
		echo "Rescue failed, DP CTRL/STAT readback $_CTRLSTAT"
	} else {
		echo "Now restart OpenOCD without RESCUE flag and load code to RP2040"
	}
	shutdown
}

# core 0
if { $_USE_CORE != 1 } {
	dap create $_CHIPNAME.dap0 -chain-position $_CHIPNAME.cpu -dp-id $_CPUTAPID -instance-id 0
	set _TARGETNAME_0 $_CHIPNAME.core0
	target create $_TARGETNAME_0 cortex_m -dap $_CHIPNAME.dap0 -coreid 0
	# srst does not exist; use SYSRESETREQ to perform a soft reset
	$_TARGETNAME_0 cortex_m reset_config sysresetreq
}

# core 1
if { $_USE_CORE != 0 } {
	dap create $_CHIPNAME.dap1 -chain-position $_CHIPNAME.cpu -dp-id $_CPUTAPID -instance-id 1
	set _TARGETNAME_1 $_CHIPNAME.core1
	target create $_TARGETNAME_1 cortex_m -dap $_CHIPNAME.dap1 -coreid 1
	$_TARGETNAME_1 cortex_m reset_config sysresetreq
}

if {[string compare $_USE_CORE SMP] == 0} {
	$_TARGETNAME_0 configure  -rtos hwthread
	$_TARGETNAME_1 configure  -rtos hwthread
	target smp $_TARGETNAME_0 $_TARGETNAME_1
}

if { $_USE_CORE == 1 } {
	set _FLASH_TARGET $_TARGETNAME_1
} else {
	set _FLASH_TARGET $_TARGETNAME_0
}
# Backup the work area. The flash probe runs an algorithm on the target CPU.
# The flash is probed during gdb connect if gdb memory_map is enabled (by default).
$_FLASH_TARGET configure -work-area-phys 0x20010000 -work-area-size $_WORKAREASIZE -work-area-backup 1
set _FLASHNAME $_CHIPNAME.flash
flash bank $_FLASHNAME rp2040_flash 0x10000000 0 0 0 $_FLASH_TARGET

if { $_BOTH_CORES } {
	# Alias to ensure gdb connecting to core 1 gets the correct memory map
	flash bank $_CHIPNAME.alias virtual 0x10000000 0 0 0 $_TARGETNAME_1 $_FLASHNAME

	# Select core 0
	targets $_TARGETNAME_0
}' | sudo tee /usr/share/openocd/scripts/target/rp2040.cfg > /dev/null