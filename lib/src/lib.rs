#![allow(illegal_floating_point_literal_pattern)]

pub mod config;
pub mod helpers;
pub mod libwallaby;
pub mod motor;
pub mod sensors;
pub mod servo;
pub mod wheels;

pub mod bridge {
    use crate::{helpers::DriveDirection, motor::Motor, wheels::Wheels};

    #[no_mangle]
    extern "C" fn drive(
        left_port: f64,
        left_speed: f64,
        left_offset: f64,
        right_port: f64,
        right_speed: f64,
        right_offset: f64,
        direction: f64,
        distance: f64,
        units: f64,
    ) {
        let wheels = Wheels {
            left_motor: Motor {
                port: left_port as i32,
                speed: left_speed,
            },
            right_motor: Motor {
                port: right_port as i32,
                speed: right_speed,
            },
            left_offset,
            right_offset,
        };

        let direction = match direction as i32 {
            0 => DriveDirection::Forward,
            1 => DriveDirection::Reverse,
            _ => panic!("Invalid direction"),
        };

        let cm = distance * units;

        wheels.drive(direction, cm)
    }
}
