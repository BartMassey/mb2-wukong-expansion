/*!
Driver for Wukong servos. The Wukong has eight SVG servo
ports are controlled via the I2C bus. The interface here is
"inspired by" the MicroPython and PXT implementations, but
is a bit more flexible.
*/

use crate::bus;

use nrf52833_hal::twim;

/// Error during servo operation.
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Given servo index out of range.
    InvalidIndex(u8),
}

/// Angle in degrees (0..=360).
#[derive(Debug, Clone, Copy)]
pub struct ServoAngle(u16);

impl ServoAngle {
    /// Make a new servo angle.
    pub fn new(angle: u16) -> Self {
        angle.try_into().unwrap()
    }
}

impl From<ServoAngle> for u16 {
    fn from(angle: ServoAngle) -> Self {
        angle.0
    }
}

impl core::convert::TryFrom<u16> for ServoAngle {
    type Error = core::convert::Infallible;

    fn try_from(angle: u16) -> Result<Self, core::convert::Infallible> {
        // XXX FIXME Return an error for out-of-range angles.
        Ok(ServoAngle(angle.min(360)))
    }
}

/// Servo to be controlled (0..=8).
#[derive(Debug, Clone, Copy)]
pub struct Servo(u8);

impl Servo {
    /// Make a new servo id.  Uses one-based numbering: the
    /// first servo is `1`, not `0`.
    ///
    /// # Panics
    ///
    /// Panics when given an out-of-range ID.
    pub fn new(servo: u8) -> Self {
        servo.try_into().unwrap()
    }
}

impl From<Servo> for u8 {
    fn from(servo: Servo) -> Self {
        servo.0
    }
}

impl core::convert::TryFrom<u8> for Servo {
    type Error = Error;

    fn try_from(servo: u8) -> Result<Self, Error> {
        if !(1..=8).contains(&servo) {
            return Err(Error::InvalidIndex(servo));
        }
        Ok(Servo(servo - 1))
    }
}

type ServoMaxAngles = [Option<ServoAngle>; 8];

/// Configuration information for servos includes
/// per-servo enablement and max angles.
#[derive(Debug, Clone)]
pub struct ServoConfig {
    servo_max_angles: ServoMaxAngles,
}

impl ServoConfig {
    /// Make a new servo config from an iterator over servos
    /// and their max angles.
    ///
    /// # Panics
    ///
    /// Panics if a servo is repeated in the iterator.
    pub fn new<C, I>(config: C) -> Self
    where
        C: IntoIterator<Item = I>,
        I: Into<(Servo, ServoAngle)>,
    {
        let mut servo_max_angles: ServoMaxAngles = Default::default();
        for item in config.into_iter() {
            let (servo, servo_angle) = item.into();
            let servo = u8::from(servo) as usize;
            // XXX: Fix me, return an error for max_angle > 360° or < 1°
            let servo_angle = ServoAngle(servo_angle.0.max(1));
            assert!(servo_max_angles[servo].is_none());
            servo_max_angles[servo] = Some(servo_angle);
        }
        Self { servo_max_angles }
    }
}

impl<TWIM> bus::WuKongBus<TWIM>
where
    TWIM: twim::Instance,
{
    /// Set the given `servo` to the `given` angle,
    /// taking into account the given `config`.
    ///
    /// # Errors
    ///
    /// Returns an error if the I2C write fails.
    pub fn set_servo_angle(
        &mut self,
        config: &ServoConfig,
        servo: Servo,
        angle: ServoAngle,
    ) -> Result<(), bus::Error> {
        let servo = u8::from(servo);
        let max_angle = config.servo_max_angles[servo as usize].unwrap().0;
        // XXX: Fix me, way better error handling needed here.
        let angle = angle.0.min(max_angle);
        let scaled_angle = angle * 180 / max_angle;
        assert!(scaled_angle <= 180);
        let servo = servo + 3;

        let buf = [servo, scaled_angle as u8, 0, 0];
        self.i2c.write(Self::I2C_ADDR, &buf)?;
        Ok(())
    }
}
