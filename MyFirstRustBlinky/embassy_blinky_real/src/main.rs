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
    
    // If I don't have this loop here, the light will only flicker on for a moment
    // Then disapear
    loop {} 
}