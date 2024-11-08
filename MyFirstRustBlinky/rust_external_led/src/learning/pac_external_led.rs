pub fn blinky() -> ! {
    let peripherals = rp2040_pac::Peripherals::take().unwrap();
    
    enable_io_bank_0(&peripherals.RESETS);
    configure_pin_to_sio(&peripherals.IO_BANK0,15);
    turn_pin_on(&peripherals.SIO,15);
    
    loop {}
}

// When the RP2040 starts, the IO Bank0 peripheral is disabled. To enable it, developers have to use the Reset's peripheral RESET register.
fn enable_io_bank_0(resets: &rp2040_pac::RESETS) {
    const IO_BANK_0_BIT: u32 = 1 << 5;

    // 1 << 5 is the bit value for resetting the IO bank in this register.
    resets.reset().modify(|r, w| unsafe { w.bits(r.bits() & !(IO_BANK_0_BIT)) });
    while resets.reset_done().read().bits() & (IO_BANK_0_BIT) == 0 {}
}

// Configure the FUNCSEL for GPIO pin to SIO mode.
fn configure_pin_to_sio(io_bank0: &rp2040_pac::IO_BANK0, pin_number: usize) {
    io_bank0.gpio(pin_number).gpio_ctrl().modify(|_, w| unsafe { w.funcsel().bits(5) });
}

fn turn_pin_on(sio: &rp2040_pac::SIO, pin_number: usize) {
    sio.gpio_oe().modify(|r, w| unsafe { w.bits(1 << pin_number) });
    sio.gpio_out().modify(|r, w| unsafe { w.bits(1 << pin_number) });
}