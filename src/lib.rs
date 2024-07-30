#![no_std]

use embedded_hal::{delay, i2c};

pub struct WuKong<D: delay::DelayNs, T: i2c::I2c> {
    i2c: T,
    delay: D,
}

pub enum LightMode {
    Breath,
    Off,
}

pub enum ServoType {
    Degrees180,
    Degrees270,
    Degrees360,
}

impl<D: delay::DelayNs, T: i2c::I2c> WuKong<D, T> {
    pub const I2C_ADDR: u8 = 0x10;

    pub fn new(delay: D, i2c: T) -> Self {
        Self { i2c, delay }
    }

    pub fn set_light_mode(&mut self, light_mode: LightMode) -> Result<(), T::Error> {
        match light_mode {
            LightMode::Breath => {
                let buf = [0x11, 0x00, 0x00, 0x00];
                self.i2c.write(Self::I2C_ADDR, &buf)?;

                self.delay.delay_ms(100);

                let buf = [0x12, 150];
                self.i2c.write(Self::I2C_ADDR, &buf)?;
            }
            LightMode::Off => todo!(),
        }
        Ok(())
    }
}
