#![no_std]
#![no_main]

use core::ptr::{write_volatile, read_volatile};
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


// RESETS
// When the RP2040 starts, the IO Bank0 peripheral is disabled.
// To enable it, developers have to start the IO Bank0 reset process by writing
//  1 << 5 is the bit value for restting the IO bank
// Then wait till the reset_done for this operation returns true.

const RESETS_BASE: u32 =  0x4000c000; // Yes RESETS and not RESET

const RESET_OFFSET: u32 = 0;
const RESET_DONE_OFFSET: u32 = 0x8;

const IO_BANK0_BIT: u32 = 1 << 5;


#[entry]
fn main() -> ! {
    const RESET_ADDR: *mut u32 = (RESETS_BASE + RESET_OFFSET) as *mut u32;
    const RESET_DONE_ADDR: *mut u32 = (RESETS_BASE + RESET_DONE_OFFSET) as *mut u32;

    const GPIO25_CTRL_REGISTER_ADDR: *mut u32 = (USER_BANK_IO_REGISTER_START_ADDRESS + GPIO25_CTRL_OFFSET)  as *mut u32;
    
    const GPIO_OE_ADDR: *mut u32 = (SIO_BASE + GPIO_OE_REGISTER_OFFSET) as *mut u32;
    const GPIO_OUT_ADDR: *mut u32 = (SIO_BASE + GPIO_OUT_REGISTER_OFFSET) as *mut u32;

    const GPIO_OE_SET_ADDR: *mut u32 = (SIO_BASE + GPIO_OE_SET_OFFSET) as *mut u32;
    const GPIO_OUT_SET_ADDR: *mut u32 = (SIO_BASE + GPIO_OUT_SET_OFFSET) as *mut u32;


    unsafe {
        // Enable IO_BANK0
        // The read_volatile is VERY IMPORTANT
        // Because just setting it to reset the IO_BANK0_BIT did not work
        // (Asuumedly because it broke other perherials so I need to merge the bit masks/state)
        write_volatile(RESET_ADDR, read_volatile(RESET_ADDR) & !IO_BANK0_BIT);
        // Wait until the IO_BANK0 is initalized
        while read_volatile(RESET_DONE_ADDR) & IO_BANK0_BIT == 0 {}
        


        // Enable funcsel for gpio25 in IO Bank0
        write_volatile(GPIO25_CTRL_REGISTER_ADDR, FUNCSEL_MODE_SIO);
        // Out Enable (OE) SET
        //write_volatile(GPIO_OE_SET_ADDR, TWENTYFIVE);
        // Set bit SET
        //write_volatile(GPIO_OUT_SET_ADDR, TWENTYFIVE);

        // Out Enable
        write_volatile(GPIO_OE_ADDR, TWENTYFIVE);
        write_volatile(GPIO_OUT_ADDR, TWENTYFIVE);
    }

    loop {
    }
}