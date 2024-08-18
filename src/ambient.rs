pub use smart_leds::RGB8;

use nrf52833_hal::{gpio, pwm};
use smart_leds_trait::SmartLedsWrite;
use ws2812_nrf52833_pwm::{self as ws2812, Ws2812};

pub struct WuKongAmbient<PWM>
where
    PWM: pwm::Instance,
{
    ambient: Ws2812<{ 4 * 24 }, PWM>,
    rgb_colors: [RGB8; 4],
}

/// Error during ambient driver operation.
pub enum Error<PWM> {
    /// WS2812 error.
    Ws2812Error(ws2812::Error<PWM>),
    /// Bad index.
    IndexError(usize),
}

impl<PWM> core::fmt::Debug for Error<PWM> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Ws2812Error(err) => write!(f, "WS2812 error: {:?}", err),
            Error::IndexError(index) => write!(f, "index error: {}", index),
        }
    }
}

impl<PWM> WuKongAmbient<PWM>
where
    PWM: pwm::Instance,
{
    pub fn new<PinMode>(pwm: PWM, pin: gpio::p1::P1_02<PinMode>) -> Result<Self, Error<PWM>> {
        let ambient: Ws2812<{ 4 * 24 }, _> = Ws2812::new(pwm, pin.degrade());
        let rgb_colors = [RGB8::default(); 4];
        let mut ambient = Self {
            ambient,
            rgb_colors,
        };
        ambient.send_colors()?;
        Ok(ambient)
    }

    fn send_colors(&mut self) -> Result<(), Error<PWM>> {
        let leds = self.rgb_colors;
        self.ambient.write(leds).map_err(|e| Error::Ws2812Error(e))
    }

    pub fn set_color(&mut self, index: usize, color: RGB8) -> Result<(), Error<PWM>> {
        if index >= self.rgb_colors.len() {
            return Err(Error::IndexError(index));
        }
        self.rgb_colors[index] = color;
        self.send_colors()
    }
}
