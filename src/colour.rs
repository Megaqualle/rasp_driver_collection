use rppal::i2c::*;

// Colour registers
const CDATAL: u8 = 0x14;
const CDATAH: u8 = 0x15;
const RDATAL: u8 = 0x16;
const RDATAH: u8 = 0x17;
const GDATAL: u8 = 0x18;
const GDATAH: u8 = 0x19;
const BDATAL: u8 = 0x1a;
const BDATAH: u8 = 0x1b;

// Configuration registers
const COMMAND_BIT: u8 = 0x80;
const REG_ENABLE: u8 = 0x00;
const REG_ATIME: u8 = 0x01;
const REG_WTIME: u8 = 0x03;

// Enable register configuration
const REG_ENABLE_SAI: u8 = 0x40;
const REG_ENABLE_AIEN: u8 = 0x10;
const REG_ENABLE_WEN: u8 = 0x08;
const REG_ENABLE_AEN: u8 = 0x02;
const REG_ENABLE_PON: u8 = 0x01;

// Time register configuration
const REG_ATIME_2_4: u8 = 0xff;
const REG_ATIME_24: u8 = 0xf6;
const REG_ATIME_101: u8 = 0xdb;
const REG_ATIME_154: u8 = 0xc0;
const REG_ATIME_700: u8 = 0x00;
const REG_WTIME_2_4: u8 = 0xff;
const REG_WTIME_204: u8 = 0xab;
const REG_WTIME_614: u8 = 0x00;

// Gain configuration
const REG_CONTROL_AGAIN_1: u8 = 0x00;
const REG_CONTROL_AGAIN_4: u8 = 0x01;
const REG_CONTROL_AGAIN_16: u8 = 0x02;
const REG_CONTROL_AGAIN_60: u8 = 0x03;

#[derive(Debug)]
pub struct Colour {
    i2c: I2c,
    clear: u8,
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    pub fn new(i2c_addr: u16) -> Colour {
        let mut i2c = I2c::new()
            .expect("I2C failed to initialize");
        i2c.set_slave_address(i2c_addr)
            .expect("Device not found");
        i2c.smbus_write_byte(REG_ENABLE | COMMAND_BIT, REG_ENABLE_AEN | REG_ENABLE_PON)
            .expect("Failed to write to enable register");
        Colour {
            i2c,
            clear: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn update(&mut self) -> &mut Colour {
        let clear_low = self.i2c.smbus_read_byte(COMMAND_BIT | CDATAL)
            .expect("Failed to read CDATAL register");
        let clear_high = self.i2c.smbus_read_byte(COMMAND_BIT | CDATAH)
            .expect("Failed to read CDATAH register");
        let red_low = self.i2c.smbus_read_byte(COMMAND_BIT | RDATAL)
            .expect("Failed to read RDATAL register");
        let red_high = self.i2c.smbus_read_byte(COMMAND_BIT | RDATAH)
            .expect("Failed to read RDATAH register");
        let green_low = self.i2c.smbus_read_byte(COMMAND_BIT | GDATAL)
            .expect("Failed to read GDATAL register");
        let green_high = self.i2c.smbus_read_byte(COMMAND_BIT | GDATAH)
            .expect("Failed to read GDATAH register");
        let blue_low = self.i2c.smbus_read_byte(COMMAND_BIT | BDATAL)
            .expect("Failed to read BDATAL register");
        let blue_high = self.i2c.smbus_read_byte(COMMAND_BIT | BDATAH)
            .expect("Failed to read BDATAH register");

        self.clear = clear_low;
        self.red = red_low;
        self.green = green_low;
        self.blue = blue_low;

        //println!("CL: {},\nCH: {}", clean_low, clean_high);
        self
    }

    pub fn get_clear(&self) -> u8 {
        self.clear
    }
    pub fn get_red(&self) -> u8 {
        self.red
    }
    pub fn get_green(&self) -> u8 {
        self.green
    }
    pub fn get_blue(&self) -> u8 {
        self.blue
    }
}
