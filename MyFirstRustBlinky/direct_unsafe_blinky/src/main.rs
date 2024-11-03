#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m_rt::entry;
use panic_halt as _;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const USER_BANK_IO_REGISTER_START_ADDRESS: u32 = 0x40014000;
const GPIO25_CTRL_OFFSET: u32 = 0x0cc;
// EAch register in the bank seems to be 32 bits long?

// SIO if I want to directly drive the pin from the processor
// PIO if I want to drive the pin from a programmable IO block.

const FUNCSEL_MODE_SIO: u32 = 5;
// FUNCEL is the first 4 bits of the GPIO25 offset (tho I'm not sure if it's left or right)

const SIO_BASE: u32 =  0xd0000000;

const GPIO_OUT_REGISTER_OFFSET: u32 = 0x010;
const GPIO_OE_REGISTER_OFFSET: u32 = 0x020;
const GPIO_OE_SET_OFFSET: u32 = 0x24; // Offset for gpio_oe_set
const GPIO_OUT_SET_OFFSET: u32 = 0x014;

// 25 bitshifted
const TWENTYFIVE : u32 = 1 << 25;

#[entry]
fn main() -> ! {
    const GPIO25_CTRL_REGISTER_ADDR: *mut u32 = (USER_BANK_IO_REGISTER_START_ADDRESS + GPIO25_CTRL_OFFSET)  as *mut u32;
    const GPIO_OE_ADDR: *mut u32 = (SIO_BASE + GPIO_OE_SET_OFFSET) as *mut u32;
    const GPIO_OUT_ADDR: *mut u32 = (SIO_BASE + GPIO_OUT_SET_OFFSET) as *mut u32;

    unsafe {
        write_volatile(GPIO25_CTRL_REGISTER_ADDR, FUNCSEL_MODE_SIO);
        write_volatile(GPIO_OE_ADDR, TWENTYFIVE);
        write_volatile(GPIO_OUT_ADDR, TWENTYFIVE);
    }

    loop {
    }
}