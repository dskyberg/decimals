#![doc = include_str!("../README.md")]

pub trait Decimals {
    /// Round the fractional part of a float to `fract_decimals`.  This is not simply a
    /// string trim function.
    fn round(&self, num_decimals: u32) -> Self;
}

impl Decimals for f32 {
    fn round(&self, num_decimals: u32) -> Self {
        // from u/rundevelopment: f32 and f64 can only represent at most about 8 and 15 decimals respectively.
        // So clamp fract_decimals to that to prevent 10i32.pow(fract_decimals) from overflowing
        let num_decimals = num_decimals.clamp(0, 8);
        let y = 10i32.pow(num_decimals) as f32;
        (*self * y).round() / y
    }
}

impl Decimals for f64 {
    fn round(&self, num_decimals: u32) -> Self {
        // from u/rundevelopment: f32 and f64 can only represent at most about 8 and 15 decimals respectively.
        // So clamp fract_decimals to that to prevent 10i32.pow(fract_decimals) from overflowing
        let num_decimals = num_decimals.clamp(0, 15);
        let y = 10i64.pow(num_decimals) as f64;
        (*self * y).round() / y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimals() {
        assert_eq!(1.23456789.round(2), 1.23);
        assert_eq!(1.23456789.round(3), 1.235);
    }
}
