pub mod mood_lights;
pub mod motor;
pub mod servo;

use crate::*;

pub use mood_lights::*;
pub use motor::*;
pub use servo::*;

pub(crate) use nrf52833_hal::{pac::twim0, twim};

type SclPin = gpio::p0::P0_26<gpio::Input<gpio::Floating>>;
type SdaPin = gpio::p1::P1_00<gpio::Input<gpio::Floating>>;

pub struct WuKongBus<TWIM, I2cDelay> {
    i2c: twim::Twim<TWIM>,
    delay: I2cDelay,
    servo_max_angles: [Option<ServoAngle>; 8],
}

impl<TWIM, I2cDelay> WuKongBus<TWIM, I2cDelay>
where
    TWIM: twim::Instance,
    I2cDelay: delay::DelayNs,
{
    pub const I2C_ADDR: u8 = 0x10;

    pub fn new(
        i2c: TWIM,
        delay: I2cDelay,
        scl: SclPin,
        sda: SdaPin,
    ) -> Self {
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
