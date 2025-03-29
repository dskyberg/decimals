#![doc = include_str!("../README.md")]

pub trait Decimals {
    /// Round the fractional part of a float to `fract_decimals`.  This is not simply a
    /// string trim function.
    fn decimals(&self, fract_decimals: u32) -> Self;
}

impl Decimals for f32 {
    fn decimals(&self, fract_decimals: u32) -> Self {
        let y = 10i32.pow(fract_decimals) as f32;
        (*self * y).round() / y
    }
}

impl Decimals for f64 {
    fn decimals(&self, fract_decimals: u32) -> Self {
        let y = 10i64.pow(fract_decimals) as f64;
        (*self * y).round() / y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimals() {
        assert_eq!(1.23456789.decimals(2), 1.23);
        assert_eq!(1.23456789.decimals(3), 1.235);
    }
}
