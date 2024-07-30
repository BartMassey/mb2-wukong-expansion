#![no_std]

pub mod lightmode;
pub mod motor;
pub mod servo;

pub use lightmode::*;
pub use motor::*;
pub use servo::*;

use embedded_hal::delay;
use nrf52833_hal::{gpio, pac::twim0, twim};

pub struct WuKong<D: delay::DelayNs, T: twim::Instance> {
    i2c: twim::Twim<T>,
    delay: D,
    servo_max_angles: [Option<ServoAngle>; 8],
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
        let servo_max_angles = Default::default();
        Self { i2c, delay, servo_max_angles }
    }

}
