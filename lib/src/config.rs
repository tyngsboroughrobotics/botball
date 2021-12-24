/// Stores the configuration for servos.
pub mod servo {
    use std::{ops::RangeInclusive, time::Duration};

    /// The lowest raw position the servo can travel without potentially causing
    /// damage.
    pub const MIN_RAW_POSITION: f64 = 98.;

    /// The highest raw position the servo can travel without potentially
    /// causing damage.
    pub const MAX_RAW_POSITION: f64 = 1947.;

    /// The range starting at `MIN_RAW_POSITION` and ending at `MAX_RAW_POSITION`.
    pub const RAW_POSITION_RANGE: RangeInclusive<f64> = MIN_RAW_POSITION..=MAX_RAW_POSITION;

    /// The amount of time to wait in between servo movements.
    ///
    /// For simplicity, the `Servo` implementation sleeps for the same amount of
    /// time for any distance traveled. In the future we might make this adjust
    /// to that amount, but a constant value works for us.
    pub const SLEEP_DURATION: Duration = Duration::from_millis(750);
}

/// Stores the configuration for motors.
pub mod motor {
    /// The number of PWM ticks that occur per second. Used to control the speed
    /// of the motor.
    pub const PWM_TICKS: f64 = 1500.;

    /// The amount of time (in seconds) it takes to travel 1 cm at full speed.
    /// Used to calculate how long to block while the motor is traveling.
    pub const TRAVEL_TIME_1_CM: f64 = 0.04;
}

/// Stores the configuration for wheel controllers.
pub mod wheels {
    /// The distance (in cm) forward the left wheel and backward the right wheel
    /// should travel when the wheels are performing a clockwise turn, and vice
    /// versa.
    ///
    /// This function has been generated using regression for the following turn
    /// amounts:
    ///
    ///   -  45 degrees = 45 * 1.175
    ///   -  90 degrees = 90 * 1.2425
    ///   -  180 degrees = 180 * 1.35
    ///   -  360 degrees = 360.1425
    pub fn turn_amount(degrees: f64) -> f64 {
        let multiplier = match degrees {
            45. => 1.175,
            90. => 1.2425,
            180. => 1.35,
            360. => 1.425,
            _ => {
                -2.781096509 * (10f64).powf(-6.) * degrees.powf(2.)
                    + 1.922759857 * (10f64).powf(-3.) * degrees
                    + 1.093333333
            }
        };

        degrees * multiplier / 10.
    }
}
