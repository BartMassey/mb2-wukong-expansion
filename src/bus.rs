#[cfg(feature = "mood_lights")]
pub mod mood_lights;
#[cfg(feature = "motor")]
pub mod motor;
#[cfg(feature = "servo")]
pub mod servo;

#[cfg(feature = "mood_lights")]
pub use mood_lights::MoodLights;
#[cfg(feature = "motor")]
pub use motor::Motor;
#[cfg(feature = "servo")]
pub use servo::{Servo, ServoAngle, ServoConfig};

use nrf52833_hal::{gpio, pac::twim0, twim};

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

pub struct WuKongBus<TWIM> {
    i2c: twim::Twim<TWIM>,
}

impl<TWIM> WuKongBus<TWIM>
where
    TWIM: twim::Instance,
{
    pub const I2C_ADDR: u8 = 0x10;

    pub fn new(i2c: TWIM, scl: SclPin, sda: SdaPin) -> Self {
        let pins = twim::Pins {
            scl: scl.degrade(),
            sda: sda.degrade(),
        };
        let freq = twim0::frequency::FREQUENCY_A::K100;
        let i2c = twim::Twim::new(i2c, pins, freq);
        Self { i2c }
    }
}
