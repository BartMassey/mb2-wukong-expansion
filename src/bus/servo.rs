use crate::bus;

use nrf52833_hal::twim;

#[derive(Debug, Clone, Copy)]
pub struct ServoAngle(u16);

impl ServoAngle {
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

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Servo {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
}

type ServoMaxAngles = [Option<ServoAngle>; 8];

#[derive(Debug, Clone)]
pub struct ServoConfig {
    servo_max_angles: ServoMaxAngles,
}

impl ServoConfig {
    pub fn new<C, I>(config: C) -> Self
    where
        C: IntoIterator<Item = I>,
        I: Into<(Servo, ServoAngle)>,
    {
        let mut servo_max_angles: ServoMaxAngles = Default::default();
        for item in config.into_iter() {
            let (servo, servo_angle) = item.into();
            // XXX: Fix me, return an error for max_angle > 360° or < 1°
            let servo_angle = ServoAngle(servo_angle.0.max(1));
            assert!(servo_max_angles[servo as usize].is_none());
            servo_max_angles[servo as usize] = Some(servo_angle);
        }
        Self { servo_max_angles }
    }
}

impl<TWIM> bus::WuKongBus<TWIM>
where
    TWIM: twim::Instance,
{
    pub fn set_servo_angle(
        &mut self,
        config: &ServoConfig,
        servo: Servo,
        angle: ServoAngle,
    ) -> Result<(), twim::Error> {
        let max_angle = config.servo_max_angles[servo as usize].unwrap().0;
        // XXX: Fix me, way better error handling needed here.
        let angle = angle.0.min(max_angle);
        let scaled_angle = angle * 180 / max_angle;
        assert!(scaled_angle <= 180);
        let servo = servo as u8 + 3;

        let buf = [servo, scaled_angle as u8, 0, 0];
        self.i2c.write(Self::I2C_ADDR, &buf)
    }
}
