use crate::*;

pub use smart_leds::RGB8;

use nrf52833_hal::pwm;
use smart_leds_trait::SmartLedsWrite;
use ws2812_nrf52833_pwm::{self as ws2812, Ws2812};

type AmbientPin = gpio::p1::P1_02<gpio::Output<gpio::PushPull>>;

pub struct WuKongAmbient<PWM, PwmDelay>
where
    PWM: pwm::Instance,
{
    ambient: Ws2812<PWM, PwmDelay>,
    rgb_colors: [RGB8; 4],
}

/// Error during ambient driver operation.
pub enum Error<PWM, PwmDelay> {
    /// WS2812 error.
    Ws2812Error(ws2812::Error<PWM, PwmDelay>),
    /// Bad index.
    IndexError(usize),
}

impl<PWM, PwmDelay> core::fmt::Debug for Error<PWM, PwmDelay> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Ws2812Error(err) => write!(f, "WS2812 error: {:?}", err),
            Error::IndexError(index) => write!(f, "index error: {}", index),
        }
    }
}

impl<PWM, PwmDelay> WuKongAmbient<PWM, PwmDelay>
where
    PWM: pwm::Instance,
    PwmDelay: delay::DelayNs,
{
    pub fn new(pwm: PWM, delay: PwmDelay, pin: AmbientPin) -> Result<Self, Error<PWM, PwmDelay>> {
        let ambient = Ws2812::new(pwm, delay, pin.degrade());
        let rgb_colors = [RGB8::default(); 4];
        let mut ambient = Self { ambient, rgb_colors };
        ambient.send_colors()?;
        Ok(ambient)
    }

    fn send_colors(&mut self) -> Result<(), Error<PWM, PwmDelay>> {
        let leds = self.rgb_colors;
        self.ambient.write(leds).map_err(|e| Error::Ws2812Error(e))
    }

    pub fn set_color(&mut self, index: usize, color: RGB8) -> Result<(), Error<PWM, PwmDelay>> {
        if index >= self.rgb_colors.len() {
            return Err(Error::IndexError(index));
        }
        self.rgb_colors[index] = color;
        self.send_colors()
    }
}
