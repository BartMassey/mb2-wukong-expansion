use crate::*;

pub enum ServoType {
    Degrees180,
    Degrees270,
    Degrees360,
}

impl<D, T> WuKong<D, T>
where
    D: delay::DelayNs,
    T: twim::Instance,
{
    pub fn set_servo(&mut self) -> Result<(), twim::Error> {
}
