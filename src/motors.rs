use std::error::Error;
use rppal::{
    gpio::{Gpio,  OutputPin},
    pwm::{Pwm, Polarity, Channel},
};

pub struct Motors {
    pwm: Pwm,
    in1: OutputPin,
    in2: OutputPin,
    in3: OutputPin,
    in4: OutputPin,
    dir1: Direction,
    dir2: Direction,
    spd: f64,
}

impl Motors {
    pub fn new(channel: Channel, in1: u8, in2: u8, in3: u8, in4: u8, inital_spd: f64) -> Motors {
        let spd = crate::adjust_pwm_value(inital_spd);
        let gpio = Gpio::new().unwrap();
        let pwm = Pwm::with_frequency(
           channel,
           100.0,
           spd,
           Polarity::Normal,
           true,
        )
            .expect("Failed to create PWM object");

        let in1 = gpio.get(in1).unwrap();
        let in2 = gpio.get(in2).unwrap();
        let in3 = gpio.get(in3).unwrap();
        let in4 = gpio.get(in4).unwrap();

        let in1 = in1.into_output_high();
        let in2 = in2.into_output_high();
        let in3 = in3.into_output_high();
        let in4 = in4.into_output_high();

        Motors {
            pwm,
            in1,
            in2,
            in3,
            in4,
            dir1: Direction::Halt,
            dir2: Direction::Halt,
            spd,
        }
    }

    pub fn set_speed(&mut self, mut speed: f64) {
        /*if speed <= 0.0 || speed >= 1.0 {
            if speed >= 1.0 {
                speed = 1.0;
            }
            else {
                speed = 0.0;
            }
        }*/
        let speed = crate::adjust_pwm_value(speed);
        self.pwm.set_duty_cycle(speed).unwrap();
        self.spd = speed;
    }

    pub fn set_direction(&mut self, dir: Direction, motor: Motor) {
        if motor == Motor::Motor1 {
            match dir {
                Direction::Forwards => {
                    self.in1.set_high();
                    self.in2.set_low();
                    self.dir1 = Direction::Forwards;
                    },
                Direction::Backwards => {
                    self.in1.set_low();
                    self.in2.set_high();
                    self.dir1 = Direction::Backwards;
                    },
                Direction::Halt => {
                    self.in1.set_high();
                    self.in2.set_high();
                    self.dir1 = Direction::Halt;
                    },
            }
        }
        else if motor == Motor::Motor2 {
            match dir {
                Direction::Forwards => {
                    self.in3.set_high();
                    self.in4.set_low();
                    self.dir2 = Direction::Forwards;
                    },
                Direction::Backwards => {
                    self.in3.set_low();
                    self.in4.set_high();
                    self.dir2 = Direction::Backwards;
                    },
                Direction::Halt => {
                    self.in3.set_high();
                    self.in4.set_high();
                    self.dir2 = Direction::Halt;
                    },
            }
        }
    }

    pub fn get_spd(&self) -> f64 {
        self.spd
    }
}

pub enum Direction {
    Forwards,
    Backwards,
    Halt,
}

#[derive(PartialEq)]
pub enum Motor {
    Motor1,
    Motor2,
}