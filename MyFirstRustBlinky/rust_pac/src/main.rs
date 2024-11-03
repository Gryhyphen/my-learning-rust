#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;


const ONBOARD_LED_PIN: u32 = 1 << 25;

#[entry]
fn main() -> ! {
    // Get a reference to all the peripherals
    let peripherals = rp2040_pac::Peripherals::take().unwrap();

    let sio = peripherals.SIO;

    // set the `pin` pin as output
    sio.gpio_oe_set().write(|w| unsafe { w.bits(ONBOARD_LED_PIN) });
    // set the `pin` to value `0`
    sio.gpio_out_clr().write(|w| unsafe { w.bits(ONBOARD_LED_PIN) });
    // set the `pin` to value `1`
    sio.gpio_out_set().write(|w| unsafe { w.bits(ONBOARD_LED_PIN) });


    // When the RP2040 starts, the IO Bank0 peripheral is disabled. To enable it, developers have to use the Reset's peripheral RESET register.
    let reset = peripherals.RESETS;

    // 1 << 5 is the bit value for restting the IO bank in this register.
    reset
        .reset()
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 5)) });
    while reset.reset_done().read().bits() & (1 << 5) == 0 {}

    // Configure the FUNCSEL for GPIO pin 25 to SIO mode.
    let io_bank0 = peripherals.IO_BANK0;

    // write the value 5 to the FUNCSEL field
    // Note: This actually writes the value 5 in FUNCSEL
    //       and 0s in all the other field. While this
    //       is fine for this example, usually a
    //       read, modify, write sequence should be used
    io_bank0
    .gpio(25)
    .gpio_ctrl()
    .write(|w| unsafe { w.bits(5) });

    // the correct way of doing this
    //io_bank0
    //.gpio(pin)
    //.gpio_ctrl()
    // the PAC crate provides fucntions for all the 
    // register's fields
    //.modify(|_, w| unsafe { w.funcsel().bits(5) });

    loop { }
}