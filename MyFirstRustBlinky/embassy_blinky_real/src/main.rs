#![no_std]
#![no_main]
use panic_halt as _;

use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use embassy_rp::init;


mod blink_task;
use blink_task::blink_task;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = init(Default::default());

    let onboard_led = peripherals.PIN_25.degrade();
    
    spawner
        .spawn(blink_task(onboard_led))
        .unwrap();
}