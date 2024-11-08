#![no_std]
#![no_main]

use core::ptr::{write_volatile, read_volatile};
use cortex_m_rt::entry;
use panic_halt as _;

mod learning;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;


#[entry]
fn main() -> ! {
    learning::pac_external_led::blinky();
}