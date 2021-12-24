use crate::{config, helpers::scale, libwallaby::libwallaby};
use std::thread;

/// Represents a servo connected to the robot. You can control it using
/// `set_position`.
#[derive(Debug, Clone, Copy)]
pub struct Servo {
    pub port: i32,
}

impl Servo {
    /// The current position of the servo, between 0 and 1. The meaning of this
    /// position depends on how your servo is oriented.
    pub fn position(self) -> f64 {
        let raw_position = unsafe { libwallaby().get_servo_position(self.port) };

        scale(
            raw_position as f64,
            config::servo::RAW_POSITION_RANGE,
            0.0..=1.0,
        )
    }

    /// Set the position of this servo to a value between 0 and 1. The meaning
    /// of this position depends on how your servo is oriented.
    ///
    /// Note that this function blocks until finished; if you don't want to
    /// block the current thread (eg. to control multiple servos at once),
    /// change this value on a background thread.
    pub fn set_position(self, position: f64) {
        // Enable this servo
        unsafe {
            libwallaby().enable_servo(self.port);
        }

        // Move the servo to the specified position
        let raw_position = scale(position, 0.0..=1.0, config::servo::RAW_POSITION_RANGE);
        unsafe {
            libwallaby().set_servo_position(self.port, raw_position as i32);
        }

        // Wait until the servo finishes moving
        thread::sleep(config::servo::SLEEP_DURATION);

        // Disable this servo while we're not using it to save battery life
        unsafe {
            libwallaby().disable_servo(self.port);
        }
    }
}
