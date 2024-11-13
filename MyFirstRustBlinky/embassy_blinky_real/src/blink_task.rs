use embassy_time::Timer;
use embassy_rp::gpio::{AnyPin, Level, Output};


#[embassy_executor::task]
pub async fn blink_task(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low);

    loop {
        led.set_high();
        Timer::after_secs(1).await;
        led.set_low();
        Timer::after_secs(1).await;
    }
}