use crate::{
    config,
    helpers::{DriveDirection, TurnDirection},
    libwallaby::libwallaby,
    motor::Motor,
};
use std::{cmp, thread, time::Duration};

/// A controller for managing two motors acting as wheels, to allow for easily
/// moving them in unison.
///
/// Usually, the left and right motors won't be perfectly in alignment; you can
/// multiply their speeds individually with the `left_offset` and `right_offset`
/// properties. Note that these properties are very sensitive; you probably only
/// need to adjust them by a small amount to get your wheels going straight!
#[derive(Debug, Clone, Copy)]
pub struct Wheels {
    pub left_motor: Motor,
    pub right_motor: Motor,
    pub left_offset: f64,
    pub right_offset: f64,
}

impl Wheels {
    /// Move both wheels the specified distance (in cm) in the specified
    /// direction.
    ///
    /// Note that this function blocks until finished; if you don't want to
    /// block the current thread (eg. to control multiple motors at once), call
    /// it on a background thread.
    pub fn drive(self, direction: DriveDirection, cm: f64) {
        // Calculate the velocities for each motor separately, incorporating the
        // offset
        let left_velocity =
            Wheels::calculate_velocity(self.left_motor.speed, direction, self.left_offset);
        let right_velocity =
            Wheels::calculate_velocity(self.right_motor.speed, direction, self.right_offset);

        // Whichever motor travels slower, sleep for that amount of time
        let slower_velocity = cmp::min(left_velocity, right_velocity);
        let sleep_time = Motor::block_duration(cm, slower_velocity);

        // Drive the motors in unison and sleep
        self.drive_in_unison(left_velocity, right_velocity, sleep_time);
    }

    /// Turn both wheels the specified amount (in degrees) in the specified
    /// direction.
    ///
    /// Note that this function blocks until finished; if you don't want to
    /// block the current thread (eg. to control multiple motors at once), call
    /// it on a background thread.
    pub fn turn(self, direction: TurnDirection, deg: f64) {
        // Determine which direction each wheel should move depending on the
        // direction of the turn
        let (left_direction, right_direction) = match direction {
            TurnDirection::Left => (DriveDirection::Reverse, DriveDirection::Forward),
            TurnDirection::Right => (DriveDirection::Forward, DriveDirection::Reverse),
        };

        // Calculate the velocities for each motor separately, ignoring the
        // offset because it is not applicable while turning
        let left_velocity = Wheels::calculate_velocity(self.left_motor.speed, left_direction, 1.);
        let right_velocity =
            Wheels::calculate_velocity(self.right_motor.speed, right_direction, 1.);

        // Determine how long to sleep while the turn occurs
        let cm = config::wheels::turn_amount(deg);
        let sleep_time = Motor::block_duration(cm, left_velocity);

        // Drive the motors in unison and sleep
        self.drive_in_unison(left_velocity, right_velocity, sleep_time);
    }

    fn drive_in_unison(self, left_velocity: i32, right_velocity: i32, sleep_time: Duration) {
        unsafe {
            libwallaby().move_at_velocity(self.left_motor.port, left_velocity);
            libwallaby().move_at_velocity(self.right_motor.port, right_velocity);

            thread::sleep(sleep_time);

            // See Motor::force_stop
            libwallaby().move_at_velocity(self.left_motor.port, 0);
            libwallaby().move_at_velocity(self.right_motor.port, 0);
            thread::sleep(Duration::from_millis(50));
            libwallaby().off(self.left_motor.port);
            libwallaby().off(self.right_motor.port);
        }
    }

    /// Replacement for `Motor`'s velocity calculation incorporating the wheel
    /// offset.
    fn calculate_velocity(speed: f64, direction: DriveDirection, offset: f64) -> i32 {
        let mut velocity = (speed * config::motor::PWM_TICKS * offset) as i32;

        if direction == DriveDirection::Reverse {
            velocity *= -1;
        }

        velocity
    }
}
