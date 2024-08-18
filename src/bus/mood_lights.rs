use crate::bus;

use embedded_hal::delay;
use nrf52833_hal::twim;

pub enum MoodLights {
    Breath,
    Off,
    Intensity(u8),
}

impl<TWIM> bus::WuKongBus<TWIM>
where
    TWIM: twim::Instance,
{
    pub fn set_mood_lights<Delay>(
        &mut self,
        delay: &mut Delay,
        mood_lights: MoodLights,
    ) -> Result<(), bus::Error>
    where
        Delay: delay::DelayNs,
    {
        match mood_lights {
            MoodLights::Breath => {
                let buf = [0x11, 0, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;

                delay.delay_ms(100);

                let buf = [0x12, 150, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;
            }
            mood_lights => {
                let intensity = match mood_lights {
                    MoodLights::Off => 0,
                    // XXX Fixme: return an error on overdrive.
                    MoodLights::Intensity(intensity) => intensity.min(100),
                    MoodLights::Breath => unreachable!(),
                };
                let buf = [0x12, intensity, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;

                delay.delay_ms(100);

                let buf = [0x11, 160, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;
            }
        }
        Ok(())
    }
}
