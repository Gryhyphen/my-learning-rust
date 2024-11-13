#![no_std]
#![no_main]
use panic_halt as _;

use embassy_executor::{Spawner, Executor};
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::init;
use embassy_time::Timer;
use static_cell::StaticCell;


static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();

#[cortex_m_rt::entry]
fn coolMain() -> ! {
    // Low priority executor: runs in thread mode, using WFE/SEV
    let executor = EXECUTOR_LOW.init(Executor::new());
    
    executor.run(|spawner| {
        spawner.spawn(embassyMain(spawner)).unwrap();
    });
}

#[embassy_executor::task]
async fn embassyMain(spawner: Spawner) {
    let peripherals = init(Default::default());

    let mut onboard_led = Output::new(peripherals.PIN_25, Level::Low);
    onboard_led.set_high();

    // If I don't have this loop here, the light will only flicker on for a moment
    // Then disapear
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