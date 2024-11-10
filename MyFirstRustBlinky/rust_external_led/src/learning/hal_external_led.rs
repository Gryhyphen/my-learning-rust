

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

use rp2040_hal::Clock;
use embedded_hal::digital::OutputPin;

pub fn blinky() -> ! {
    use rp2040_hal as hal;

    let mut peripherals = 
        hal::pac::Peripherals::take().unwrap();

    let core = 
        hal::pac::CorePeripherals::take().unwrap();

    // Setup watchdog needed for clock
    let mut watchdog =
        hal::Watchdog::new(peripherals.WATCHDOG);

    // Configure clocks
    let clocks =
        hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            peripherals.XOSC,
            peripherals.CLOCKS,
            peripherals.PLL_SYS,
            peripherals.PLL_USB,
            &mut peripherals.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        use cortex_m::delay::Delay as Delay;

        let mut delay = 
            Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

        // Configure GPIO - (Does the bank enable stuffs)
        let sio = hal::Sio::new(peripherals.SIO);

        let pins = hal::gpio::Pins::new(
            peripherals.IO_BANK0,
            peripherals.PADS_BANK0,
            sio.gpio_bank0,
            &mut peripherals.RESETS
        );

        let mut external_led_pin =
            pins.gpio15.into_push_pull_output();

    loop {
        external_led_pin.set_high().unwrap();
        delay.delay_ms(500);
        external_led_pin.set_low().unwrap();
        delay.delay_ms(500);
    };
}