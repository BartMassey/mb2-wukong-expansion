use crate::*;

pub enum LightMode {
    Breath,
    Off,
    Intensity(u8),
}

impl<D, T> WuKong<D, T>
where
    D: delay::DelayNs,
    T: twim::Instance,
{
    pub fn set_light_mode(&mut self, light_mode: LightMode) -> Result<(), hal::twim::Error> {
        match light_mode {
            LightMode::Breath => {
                let buf = [0x11, 0, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;

                self.delay.delay_ms(100);

                let buf = [0x12, 150, 0, 0];
                self.i2c.write(Self::I2C_ADDR, &buf)?;
            }
            light_mode => {
                let intensity = match light_mode {
                    LightMode::Off => 0,
                    // XXX Fixme: return an error on overdrive.
                    LightMode::Intensity(intensity) => intensity.min(100),
                    LightMode::Breath => unreachable!(),
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
