#![no_std]
#![no_main]

use esp32_dht11_rs::DHT11;
use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use esp_println::println;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let delay = Delay::new();

    let mut dht11 = DHT11::new(peripherals.GPIO2, delay);

    loop {
        match dht11.read() {
            Ok(m) => println!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                m.temperature, m.humidity
            ),
            Err(error) => println!("An error occurred while trying to read sensor: {:?}", error),
        }
        delay.delay_millis(1000);
    }
}
