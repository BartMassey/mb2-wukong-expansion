use super::*;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct ServoAngle(pub u16);

impl From<ServoAngle> for u16 {
    fn from(angle: ServoAngle) -> Self {
        angle.0
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

impl<TWIM, I2cDelay> WuKongBus<TWIM, I2cDelay>
where
    TWIM: twim::Instance,
    I2cDelay: delay::DelayNs,
{
    pub fn init_servo(&mut self, servo: Servo, max_angle: ServoAngle) {
        // XXX: Fix me, return an error for max_angle > 360° or < 1°
        let max_angle = ServoAngle(max_angle.0.clamp(1, 360));
        self.servo_max_angles[servo as usize] = Some(max_angle);
    }

    pub fn set_servo_angle(&mut self, servo: Servo, angle: ServoAngle) -> Result<(), twim::Error> {
        // XXX: Fix me, way better error handling needed here.
        let max_angle = self.servo_max_angles[servo as usize].unwrap().0;
        let angle = angle.0.min(max_angle);
        let scaled_angle = angle * 180 / max_angle;
        assert!(scaled_angle <= 180);
        let servo = servo as u8 + 3;

        let buf = [servo, scaled_angle as u8, 0, 0];
        self.i2c.write(Self::I2C_ADDR, &buf)
    }
}
