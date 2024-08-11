pub mod mood_lights;
pub mod motor;
pub mod servo;

use crate::*;

pub use mood_lights::MoodLights;
pub use motor::Motor;
pub use servo::{Servo, ServoAngle};

pub(crate) use nrf52833_hal::{pac::twim0, twim};


/// Error during bus driver operation.
pub enum Error {
    /// I2C error.
    I2cError(twim::Error),
}

impl From<twim::Error> for Error {
    fn from(err: twim::Error) -> Self {
        Self::I2cError(err)
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::I2cError(err) => write!(f, "I2C error: {:?}", err),
        }
    }
}

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
