use std::ops::RangeInclusive;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriveDirection {
    Forward = 0,
    Reverse = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnDirection {
    Left = 0,
    Right = 1,
}

pub fn scale(n: f64, start: RangeInclusive<f64>, end: RangeInclusive<f64>) -> f64 {
    let (in_min, in_max) = (start.start(), start.end());
    let (out_min, out_max) = (end.start(), end.end());

    (n - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
