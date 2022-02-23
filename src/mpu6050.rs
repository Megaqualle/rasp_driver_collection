use rppal::i2c::*;

const GYRO_CONFIG: u8 = 0x1b;
const GYRO_SCALE_RANGE: u8 = 0x0;
const GYRO_X_REGISTER: u8 = 0x43;
const GYRO_Y_REGISTER: u8 = 0x45;
const GYRO_Z_REGISTER: u8 = 0x47;
const ACCEL_CONFIG: u8 = 0x1c;
const ACCEL_FORCE_CONFIG: u8 = 0x0;
const ACCEL_X_REGISTER: u8 = 0x3b;
const ACCEL_Y_REGISTER: u8 = 0x3d;
const ACCEL_Z_REGISTER: u8 = 0x3f;

#[derive(Debug)]
pub struct Mpu6050 {
    i2c: I2c,
    // Rotation of the gyro sensor
    x_rot: u8,
    y_rot: u8,
    z_rot: u8,
    // 
    x_mov: u8,
    y_mov: u8,
    z_mov: u8,
}

impl Mpu6050 {
    pub fn new(addr: u16) -> Mpu6050 {
        // Create the i2c object
        let mut i2c = I2c::new().expect("I2C failed to initialize");
        // Set the i2c slave address
        i2c.set_slave_address(addr).expect("Device not found");
        // Set the ACCEL_CONFIG register to 0
        i2c.smbus_write_byte(ACCEL_CONFIG, ACCEL_FORCE_CONFIG)
            .expect("Failed to write to ACCEL_CONFIG");
        i2c.smbus_write_byte(GYRO_CONFIG, GYRO_SCALE_RANGE)
            .expect("Failed to write to GYRO_CONFIG");
        Mpu6050 {
            i2c,
            x_rot: 1,
            y_rot: 1,
            z_rot: 1,
            x_mov: 1,
            y_mov: 1,
            z_mov: 1,
        }
    }
    pub fn update(&mut self) -> &Mpu6050 {
        self.set_rotation();
        self.set_position();
        self
    }
    pub fn reset(&self) {

    }

    pub fn get_x_rot(&self) -> u8 {
        return self.x_rot;
    }
    pub fn get_y_rot(&self) -> u8 {
        return self.y_rot;
    }
    pub fn get_z_rot(&self) -> u8 {
        return self.z_rot;
    }
    pub fn get_x_mov(&self) -> u8 {
        return self.x_mov;
    }
    pub fn get_y_mov(&self) -> u8 {
        return self.y_mov;
    }
    pub fn get_z_mov(&self) -> u8 {
        return self.z_mov;
    }

    fn set_rotation(&mut self) {
       self.x_rot = self.i2c.smbus_read_byte(ACCEL_X_REGISTER)
           .expect("Failed to read the ACCEL_XOUT register");
       self.y_rot = self.i2c.smbus_read_byte(ACCEL_Y_REGISTER)
           .expect("Failed to read the ACCEL_YOUT register");
       self.z_rot = self.i2c.smbus_read_byte(ACCEL_Z_REGISTER)
           .expect("Failed to read the ACCEL_ZOUT register");
    }

    fn set_position(&mut self) {
        self.x_mov = self.i2c.smbus_read_byte(GYRO_X_REGISTER)
            .expect("Failed to read the GYRO_XOUT register");
        self.y_mov = self.i2c.smbus_read_byte(GYRO_Y_REGISTER)
            .expect("Failed to read the GYRO_YOUT register");
        self.z_mov = self.i2c.smbus_read_byte(GYRO_Z_REGISTER)
            .expect("Failed to read the GYRO_ZOUT register");

    }
}