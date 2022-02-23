#[macro_use] extern crate log;

mod servo;
mod mpu6050;
mod laser;
mod colour;
mod motors;
mod ultrasonic;

pub use {
    servo::*,
    mpu6050::*,
    laser::*,
    colour::*,
    motors::*,
    ultrasonic::*,
};


pub fn adjust_pwm_value(mut value: f64) -> f64 {
    if value <= 0.0 || value >= 1.0 {
        if value >= 1.0 {
            warn!("[rasp_driver_collection/adjust_pwm_value]: PWM VALUE TOO HIGH, ADJUSTING DOWN!");
            value = 1.0;
        }
        else {
            warn!("[rasp_driver_collection/adjust_pwm_value]: PWM VALUE TOO LOW, ADJUSTING UP!");
            value = 0.0;
        }
    }
    return value;
}

pub struct Devices {
    servo: ServoDev,
    motors: MotorsDev,
    gyro: GyroDev,
    laser: LaserDev,
    colour: ColourDev,
}

pub struct ServoDev {}
pub struct MotorsDev {}
pub struct GyroDev {}
pub struct LaserDev {}
pub struct ColourDev {}

pub fn init() {}