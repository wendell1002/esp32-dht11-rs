#![no_std]
#![no_main]
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use esp_println::println;

use esp_hal::analog::adc::{Adc, AdcCalBasic, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{AnyPin, Flex, Level, Output, OutputConfig};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::peripherals::Interrupt::TIMER1;
use esp_hal::peripherals::{Peripherals, ADC1, GPIO0, GPIO3};

use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    let delay = Delay::new();
    let mut dht11 = DHT11::new(peripherals.GPIO2, delay);

    loop {
        match dht11.read() {
            Ok(m) => log::info!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                m.temperature,
                m.humidity
            ),
            Err(error) => log::error!("An error occurred while trying to read sensor: {:?}", error),
        }
        delay.delay_millis(1000);
    }
}
