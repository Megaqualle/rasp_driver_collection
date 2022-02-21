use rppal::pwm::*;
pub struct Servo {
    pwm: Pwm,
    rot: f64,
}

impl Servo {
    pub fn new(channel: Channel) -> Servo {
        let pwm = Pwm::with_frequency(
            channel,
            1000.0,
            0.0,
            Polarity::Normal,
            true,
        ).unwrap();
        Servo {
            pwm,
            rot: 0.0,
        }
    }

    pub fn set_duty_cycle(&mut self, rot_new: f64) {
        let rot_new = crate::adjust_pwm_value(rot_new);
        self.pwm.set_duty_cycle(rot_new).unwrap();
        self.rot = rot_new;
    }

    pub fn get_rot(&self) -> f64 {
        self.rot
    }
}