use crate::{helpers::scale, libwallaby::libwallaby};

/// Represents a digital sensor connected to the robot.
#[derive(Debug, Clone, Copy)]
pub struct DigitalSensor {
    pub port: i32,
}

impl DigitalSensor {
    /// Returns a boolean value representing whether the sensor is activated or
    /// not.
    pub fn value(self) -> bool {
        unsafe { libwallaby().digital(self.port) != 0 }
    }
}

/// Represents an analog sensor connected to the robot.
#[derive(Debug, Clone, Copy)]
pub struct AnalogSensor {
    pub port: i32,
}

impl AnalogSensor {
    /// Returns a value between 0 and 1 representing the state of the sensor.
    pub fn value(self) -> f64 {
        scale(self.raw_value() as f64, 0.0..=4095.0, 0.0..=1.0)
    }

    /// Returns an integer value between 0 and 4096 representing the raw state
    /// of the sensor.
    pub fn raw_value(self) -> i32 {
        unsafe { libwallaby().analog(self.port) }
    }
}
