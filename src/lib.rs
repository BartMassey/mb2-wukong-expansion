#![no_std]

#[cfg(feature = "ambient")]
pub mod ambient;
#[cfg(feature = "bus")]
pub mod bus;
#[cfg(feature = "buzzer")]
pub mod buzzer;

#[cfg(feature = "ambient")]
pub use ambient::{WuKongAmbient, RGB8};
#[cfg(feature = "mood_lights")]
pub use bus::MoodLights;
#[cfg(feature = "motor")]
pub use bus::Motor;
#[cfg(feature = "bus")]
pub use bus::WuKongBus;
#[cfg(feature = "servo")]
pub use bus::{Servo, ServoAngle, ServoConfig};
#[cfg(feature = "buzzer")]
pub use buzzer::WuKongBuzzer;
