#![no_std]
#![no_main]
use panic_halt as _;

use embassy_executor::{Spawner, Executor};
use embassy_rp::gpio::{AnyPin, Level, Output, Pin};
use embassy_rp::init;
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = init(Default::default());
    
    spawner
        .spawn(blink_task(peripherals.PIN_15.degrade()))
        .unwrap();
}

#[embassy_executor::task]
async fn blink_task(mut led: AnyPin) {
    let mut led = Output::new(led, Level::Low);

    loop {
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
    }
}