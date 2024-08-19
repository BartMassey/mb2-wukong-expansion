/*!
Interface for Wukong DC motor drivers. The Wukong has two DC
motor outputs that are controlled by the I2C bus. The
interface here mostly follows that of the MicroPython and
PXT implementations.
*/

use crate::bus;

use nrf52833_hal::twim;

/// Motor to be controlled.
#[derive(Debug, Clone, Copy)]
pub enum Motor {
    /// Motor 1.
    M1,
    /// Motor 2.
    M2,
}

impl<TWIM> bus::WuKongBus<TWIM>
where
    TWIM: twim::Instance,
{
    /// Set the given `motor` to the given `speed` (-100..=100).
    ///
    /// # Errors
    ///
    /// Returns an error if the I2C write fails.
    pub fn set_motor_speed(&mut self, motor: Motor, speed: i8) -> Result<(), bus::Error> {
        let motor = match motor {
            Motor::M1 => 1,
            Motor::M2 => 2,
        };
        let sign = if speed >= 0 { 1 } else { 2 };
        // XXX Fixme: Return error on overspeed.
        let speed = speed.abs().min(100);
        let buf = [motor, sign, speed as u8, 0];
        self.i2c.write(Self::I2C_ADDR, &buf)?;
        Ok(())
    }
}
