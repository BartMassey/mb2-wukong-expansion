#![no_std]

use embedded_hal::delay;
use nrf52833_hal::{self as hal, gpio, pac::twim0, twim};

pub struct WuKong<D: delay::DelayNs, T: twim::Instance> {
    i2c: twim::Twim<T>,
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

pub type SCL = gpio::p0::P0_26<gpio::Input<gpio::Floating>>;
pub type SDA = gpio::p1::P1_00<gpio::Input<gpio::Floating>>;

impl<D, T> WuKong<D, T>
where
    D: delay::DelayNs,
    T: twim::Instance,
{
    pub const I2C_ADDR: u8 = 0x10;

    pub fn new(delay: D, i2c: T, scl: SCL, sda: SDA) -> Self {
        let pins = twim::Pins {
            scl: scl.degrade(),
            sda: sda.degrade(),
        };
        let freq = twim0::frequency::FREQUENCY_A::K100;
        let i2c = twim::Twim::new(i2c, pins, freq);
        Self { i2c, delay }
    }

    pub fn set_light_mode(&mut self, light_mode: LightMode) -> Result<(), hal::twim::Error> {
        match light_mode {
            LightMode::Breath => {
                let buf = [0x11, 0, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;

                self.delay.delay_ms(100);

                let buf = [0x12, 150, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;
            }
            LightMode::Off => todo!(),
        }
        Ok(())
    }
}
