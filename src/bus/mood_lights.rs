use crate::*;
use super::*;

pub enum MoodLights {
    Breath,
    Off,
    Intensity(u8),
}

impl<TWIM, I2cDelay> WuKongBus<TWIM, I2cDelay>
where
    TWIM: twim::Instance,
    I2cDelay: delay::DelayNs,
{
    pub fn set_mood_lights(&mut self, mood_lights: MoodLights) -> Result<(), bus::Error> {
        match mood_lights {
            MoodLights::Breath => {
                let buf = [0x11, 0, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;

                self.delay.delay_ms(100);

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

                self.delay.delay_ms(100);

                let buf = [0x11, 160, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;
            }
        }
        Ok(())
    }
}
