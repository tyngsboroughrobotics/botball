use crate::{config, helpers::DriveDirection, libwallaby::libwallaby};
use std::{thread, time::Duration};

/// Represents a motor connected to the robot. You can control it with the
/// `drive` function. The default speed for the motor is 1.0 and must be between
/// 0 and 1 (1 being the fastest).
#[derive(Debug, Clone, Copy)]
pub struct Motor {
    pub port: i32,
    pub speed: f64,
}

impl Motor {
    /// Move the motor the specified distance (in cm) in the specified direction.
    ///
    /// Note that this function blocks until finished; if you don't want to
    /// block the current thread (eg. to control multiple motors at once), call
    /// it on a background thread.
    pub fn drive(self, direction: DriveDirection, cm: f64) {
        // Calculate the raw velocity the motor should travel from the current
        // speed
        let mut velocity = (self.speed * config::motor::PWM_TICKS) as i32;

        if direction == DriveDirection::Reverse {
            velocity *= -1;
        }

        // Move the motor at the calculated velocity
        unsafe {
            libwallaby().move_at_velocity(self.port, velocity);
        }

        // Calculate how long it will take the motor to travel
        let block_duration = Motor::block_duration(cm, velocity);

        // Block until the motor has finished traveling the specified distance
        thread::sleep(block_duration);

        // Turn off the motor
        self.force_stop();
    }

    /// When `libwallaby::off` is called, it just stops sending power to the
    /// motor. This has the side effect of making the motor spin for a little
    /// while longer than intended, throwing off our precise distance
    /// measurements. By instead setting the motor's velocity to 0 first, the
    /// motor will stop immediately as desired. To conserve power we still turn
    /// it off after a short delay.
    fn force_stop(self) {
        unsafe {
            libwallaby().move_at_velocity(self.port, 0);
        }

        thread::sleep(Duration::from_millis(50));

        unsafe {
            libwallaby().off(self.port);
        }
    }

    pub(crate) fn block_duration(cm: f64, velocity: i32) -> Duration {
        Duration::from_millis(
            ((config::motor::PWM_TICKS * config::motor::TRAVEL_TIME_1_CM * cm / velocity as f64)
                .abs()
                * 1000.) as u64,
        )
    }
}
