// Stuff

#![no_std]
#![no_main]
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;

// Some traits we need
use embedded_hal::digital::v2::OutputPin;


use cortex_m::peripheral as _;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
#[allow(unused_imports)]
use panic_halt as _;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
//const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut peripherals = rp2040_hal::pac::Peripherals::take().unwrap();

    let sio = rp2040_hal::sio::Sio::new(peripherals.SIO);

    // Set the pins to their default state
    let pins = rp2040_hal::gpio::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS
    );

    // Configure GPIO25 as an output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
    }
}