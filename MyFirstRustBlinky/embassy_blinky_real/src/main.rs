#![no_std]
#![no_main]
use panic_halt as _;

use embassy_executor::{Spawner, Executor};
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::init;
use embassy_time::Timer;



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = init(Default::default());

    let mut onboard_led = Output::new(peripherals.PIN_25, Level::Low);
    onboard_led.set_high();
    
    spawner
        .spawn(blink_task(onboard_led))
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