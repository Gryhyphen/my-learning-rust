#![no_std]
#![no_main]
use panic_halt as _;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

//use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::init;
use embassy_time::Timer;

#[cortex_m_rt::entry]
fn coolMain() -> ! {
    let peripherals = init(Default::default());

    let mut onboard_led = Output::new(peripherals.PIN_25, Level::Low);
    onboard_led.set_high();

    loop {}
}

/*
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = init(Default::default());

    let mut  base_led = Output::new(peripherals.PIN_25, Level::High);
    base_led.set_high();

    let led = Output::new(peripherals.PIN_15, Level::Low);

    spawner
        .spawn(blink_task(led))
        .unwrap();
}

#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
    }
}
    */

/* async fn blink_task<'a>(mut led: Output<'a>) {
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
} */