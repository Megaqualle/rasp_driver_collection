use std::thread::sleep;
use std::time::Duration;
use rppal::i2c::*;

const I2CADDR: u16 = 0x70;

#[derive(Debug)]
pub struct Laser {
    i2c: I2c,
    distance: f64,
}

impl Laser {
    pub fn new(i2c_addr: u16) -> Laser {
        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(i2c_addr).unwrap();
        Laser::init(&i2c);
        sleep(Duration::from_millis(65));
        Laser {
            i2c,
            distance: 0.0,
        }
    }

    pub fn update(&mut self) {
        let new_distance = 0.0;
        self.distance = new_distance;
        todo!()
    }

    fn init(i2c: &I2c) {
        todo!()
    }
}
