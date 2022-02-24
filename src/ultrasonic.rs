use std::thread::sleep;
use std::time;
use std::time::Duration;
use rppal::gpio::{Gpio, InputPin, OutputPin};

pub struct Ultrasonic {
    enable_pin: OutputPin,
    echo_pin: InputPin,
    distance: f64,
}

impl Ultrasonic {
    pub fn new(enable_pin: u8, echo_pin: u8) -> Ultrasonic {
        let gpio = Gpio::new().unwrap();

        let enable_pin = gpio.get(enable_pin).unwrap();
        let echo_pin = gpio.get(echo_pin).unwrap();

        let enable_pin = enable_pin.into_output();
        let echo_pin = echo_pin.into_input();

        Ultrasonic {
            enable_pin,
            echo_pin,
            distance: 0.0,
        }
    }

    pub fn get_distance(&mut self) -> f64 {
        self.enable_pin.set_high();
        sleep(Duration::from_micros(10));
        self.enable_pin.set_low();
        
        let mut start_time: u128 = 0;
        let mut end_time: u128 = 0;

        let start = time::Instant::now();
        let end = time::Instant::now();

        while self.echo_pin.is_low() {
            start_time = start.elapsed().as_nanos();
        }
        while self.echo_pin.is_high() /*|| end_time - start_time > 20000000*/{
            end_time = end.elapsed().as_nanos();
        }

        let time = end_time - start_time;

        let distance = time as f64 /1000.0 * (34300.0 / 1000000.0) / 2.0;
        self.distance = distance as f64;
        self.distance
    }

    pub fn distance(&self) -> f64 {
        return self.distance;
    }
}