
## esp32-dht11-rs

esp32-dht11-rs is is a Rust crate that reads temperature and humidity data from the DHT11 sensors for esp32 series. 

This library is #![no_std] and depends  on embedded_hal and esp-hal.

## Usage
 
 
```rust
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

 ```

