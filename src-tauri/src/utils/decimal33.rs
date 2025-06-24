use std::fmt;
use std::str::FromStr;

/// A fixed-point decimal structure storing values with a precision of 3 decimal places.
/// The value is stored as a `u32` scaled by 1000. For example, 55.201 is stored as 55201.
/// The maximum representable value is 999.999.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode)]
pub struct Decimal33 {
    scaled_int: u32, // value in thousandths, e.g. 55201 = 55.201
}

impl Decimal33 {
    pub const PRECISION: u32 = 1000; // 3 decimal places
    pub const MAX_SCALED: u32 = 999_999; // max value: 999.999

    /// Creates a new `Decimal33` from an `f32` value.
    ///
    /// The float value is scaled by `PRECISION` (1000) and floored.
    ///
    /// # Errors
    /// Returns `DecimalError::NegativeInput` if the `value` is negative.
    /// Returns `DecimalError::Overflow` if the scaled value exceeds `MAX_SCALED` (999,999).
    pub fn new(value: f32) -> Result<Self, DecimalError> {
        if value < 0.0 {
            return Err(DecimalError::NegativeInput);
        }
        let scaled = (value * Self::PRECISION as f32).floor() as u32;
        if scaled > Self::MAX_SCALED {
            return Err(DecimalError::Overflow(value));
        }
        Ok(Self { scaled_int: scaled })
    }

    /// Creates a new `Decimal33` from a pre-scaled `u32` integer value.
    /// The `scaled_int` is expected to be the desired decimal value multiplied by 1000.
    ///
    /// # Errors
    /// Returns `DecimalError::Overflow` if `scaled_int` exceeds `MAX_SCALED` (999,999).
    pub fn from_scaled(scaled_int: u32) -> Result<Self, DecimalError> {
        if scaled_int > Self::MAX_SCALED {
            return Err(DecimalError::Overflow(
                scaled_int as f32 / Self::PRECISION as f32,
            ));
        }
        Ok(Self { scaled_int })
    }

    /// Converts the `Decimal33` back to an `f32` value.
    pub fn to_f32(self) -> f32 {
        self.scaled_int as f32 / Self::PRECISION as f32
    }

    /// Returns the internal scaled `u32` integer value (value * 1000).
    pub fn scaled(&self) -> u32 {
        self.scaled_int
    }

    /// Adds another `Decimal33` to this one in place.
    ///
    /// The addition is saturating: if the sum of the scaled values exceeds `MAX_SCALED`,
    /// the internal `scaled_int` will be capped at `MAX_SCALED`.
    pub fn add(&mut self, other: Self) {
        self.scaled_int = self
            .scaled_int
            .saturating_add(other.scaled_int)
            .min(Self::MAX_SCALED);
    }
    /// Adds an `f32` value to this `Decimal33` in place.
    ///
    /// If `delta` is negative, this method effectively subtracts the absolute value of `delta`.
    /// The `delta` is scaled by `PRECISION` (1000) and floored before addition.
    ///
    /// The addition is saturating: if the sum of the scaled values exceeds `MAX_SCALED`,
    /// the internal `scaled_int` will be capped at `MAX_SCALED`.
    pub fn add_f32(&mut self, delta: f32) {
        if delta < 0.0 {
            self.subtract_f32(-delta);
            return;
        }

        let delta_scaled = (delta * Self::PRECISION as f32).floor() as u32;
        self.scaled_int = self.scaled_int
            .saturating_add(delta_scaled)
            .min(Self::MAX_SCALED);
    }


    /// Subtracts another `Decimal33` from this one in place.
    ///
    /// The subtraction is saturating: if `other.scaled_int` is greater than `self.scaled_int`,
    /// the internal `scaled_int` will become 0.
    pub fn subtract(&mut self, other: Self) {
        self.scaled_int = self.scaled_int.saturating_sub(other.scaled_int);
    }

    /// Subtracts an `f32` value from this `Decimal33` in place.
    ///
    /// If `delta` is negative, this method effectively adds the absolute value of `delta`.
    /// The `delta` is scaled by `PRECISION` (1000) and floored before subtraction.
    ///
    /// The subtraction is saturating: if the scaled `delta` is greater than `self.scaled_int`,
    /// the internal `scaled_int` will become 0.
    pub fn subtract_f32(&mut self, delta: f32) {
        if delta < 0.0 {
            self.add_f32(-delta);
            return;
        }

        let delta_scaled = (delta * Self::PRECISION as f32).floor() as u32;
        self.scaled_int = self.scaled_int.saturating_sub(delta_scaled);
    }
}

use std::ops::{AddAssign, SubAssign};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::utils::errors::DecimalError;

impl AddAssign for Decimal33 {
    /// Performs saturating addition and assignment.
    /// `*self += rhs;` is equivalent to `self.add(rhs);`.
    fn add_assign(&mut self, rhs: Self) {
        self.add(rhs);
    }
}

impl SubAssign for Decimal33 {
    /// Performs saturating subtraction and assignment.
    /// `*self -= rhs;` is equivalent to `self.subtract(rhs);`.
    fn sub_assign(&mut self, rhs: Self) {
        self.subtract(rhs);
    }
}

impl FromStr for Decimal33 {
    type Err = DecimalError;

    /// Parses a string slice into a `Decimal33`.
    ///
    /// This first parses the string as an `f32` and then attempts to create a `Decimal33`
    /// from it using `Decimal33::new`.
    ///
    /// # Errors
    /// Returns `DecimalError::ParseFloatError` if the string cannot be parsed into an `f32`.
    /// Returns other `DecimalError` variants if `Decimal33::new` fails (e.g., negative input or overflow).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<f32>()?;
        Decimal33::new(value)
    }
}
impl fmt::Display for Decimal33 {
    /// Formats the `Decimal33` as a string with 3 decimal places.
    /// For example, a `Decimal33` representing 55.201 will be displayed as "55.201".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.to_f32())
    }
}

impl From<f32> for Decimal33 {
    /// Converts an `f32` into a `Decimal33`.
    ///
    /// # Panics
    /// Panics if the `f32` value is negative or out of the representable range for `Decimal33`
    /// (i.e., if `Decimal33::new(value)` would return an `Err`).
    fn from(value: f32) -> Self {
        Decimal33::new(value).unwrap()
    }
}

impl AddAssign<f32> for Decimal33 {
    fn add_assign(&mut self, rhs: f32) {
        /// Performs saturating addition of an `f32` and assignment.
        /// `*self += rhs_f32;` is equivalent to `self.add_f32(rhs_f32);`.
        self.add_f32(rhs);
    }
}

impl SubAssign<f32> for Decimal33 {
    fn sub_assign(&mut self, rhs: f32) {
        /// Performs saturating subtraction of an `f32` and assignment.
        /// `*self -= rhs_f32;` is equivalent to `self.subtract_f32(rhs_f32);`.
        self.subtract_f32(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    // Helper to create Decimal33 from scaled value for tests
    fn dec_scaled(scaled: u32) -> Decimal33 {
        Decimal33::from_scaled(scaled).expect("Failed to create Decimal33 from scaled value in test")
    }

    #[test]
    fn test_new_valid() {
        let d = Decimal33::new(123.456).unwrap();
        assert_eq!(d.scaled(), 123456);

        let d = Decimal33::new(0.0).unwrap();
        assert_eq!(d.scaled(), 0);

        let d = Decimal33::new(999.999).unwrap();
        assert_eq!(d.scaled(), 999999);
        let d = Decimal33::new(1.2341).unwrap();
        assert_eq!(d.scaled(), 1234);
    }

    #[test]
    fn test_new_negative_input() {
        let err = Decimal33::new(-1.0).unwrap_err();
        assert!(matches!(err, DecimalError::NegativeInput));
    }

    #[test]
    fn test_new_flooring() {
        // Values that should floor down but still be valid
        let d = Decimal33::new(999.9999).unwrap();
        assert_eq!(d.scaled(), 999999);

        let d = Decimal33::new(1.0009).unwrap();
        assert_eq!(d.scaled(), 1000);

        let d = Decimal33::new(0.0009).unwrap();
        assert_eq!(d.scaled(), 0);
    }

    #[test]
    fn test_new_overflow() {
        let err = Decimal33::new(1000.0).unwrap_err();
        assert!(matches!(err, DecimalError::Overflow(1000.0)));

        let err = Decimal33::new(1000.0001).unwrap_err();
        assert!(matches!(err, DecimalError::Overflow(1000.0001)));
    }

    #[test]
    fn test_from_scaled_valid() {
        let d = Decimal33::from_scaled(123456).unwrap();
        assert_eq!(d.scaled(), 123456);

        let d = Decimal33::from_scaled(0).unwrap();
        assert_eq!(d.scaled(), 0);

        let d = Decimal33::from_scaled(999999).unwrap();
        assert_eq!(d.scaled(), 999999);
    }

    #[test]
    fn test_from_scaled_overflow() {
        let err = Decimal33::from_scaled(1000000).unwrap_err();
        assert!(matches!(err, DecimalError::Overflow(_)));
    }

    #[test]
    fn test_to_f32() {
        let d = dec_scaled(123456);
        assert!((d.to_f32() - 123.456).abs() < EPSILON);

        let d = dec_scaled(0);
        assert!((d.to_f32() - 0.0).abs() < EPSILON);

        let d = dec_scaled(999999);
        assert!((d.to_f32() - 999.999).abs() < EPSILON);
    }

    #[test]
    fn test_scaled() {
        let d = dec_scaled(123456);
        assert_eq!(d.scaled(), 123456);
    }

    #[test]
    fn test_add() {
        let mut d1 = dec_scaled(100000); // 100.000
        let d2 = dec_scaled(200000); // 200.000
        d1.add(d2);
        assert_eq!(d1.scaled(), 300000); // 300.000

        let mut d1 = dec_scaled(800000); // 800.000
        let d2 = dec_scaled(300000); // 300.000
        d1.add(d2);
        assert_eq!(d1.scaled(), Decimal33::MAX_SCALED); // Saturates at 999.999
    }

    #[test]
    fn test_add_f32() {
        let mut d = dec_scaled(100000); // 100.000
        d.add_f32(200.500);
        assert_eq!(d.scaled(), 300500); // 300.500

        let mut d = dec_scaled(800000); // 800.000
        d.add_f32(300.000);
        assert_eq!(d.scaled(), Decimal33::MAX_SCALED); // Saturates at 999.999

        let mut d = dec_scaled(100000); // 100.000
        d.add_f32(0.0); // Adding zero
        assert_eq!(d.scaled(), 100000);

        let mut d = dec_scaled(100000); // 100.000
        d.add_f32(-50.0); // Adding negative (should subtract)
        assert_eq!(d.scaled(), 50000);
    }

    #[test]
    fn test_subtract() {
        let mut d1 = dec_scaled(300000); // 300.000
        let d2 = dec_scaled(100000); // 100.000
        d1.subtract(d2);
        assert_eq!(d1.scaled(), 200000); // 200.000

        let mut d1 = dec_scaled(100000); // 100.000
        let d2 = dec_scaled(300000); // 300.000
        d1.subtract(d2);
        assert_eq!(d1.scaled(), 0); // Saturates at 0

        let mut d1 = dec_scaled(100000); // 100.000
        let d2 = dec_scaled(0); // 0.000
        d1.subtract(d2);
        assert_eq!(d1.scaled(), 100000);
    }

    #[test]
    fn test_subtract_f32() {
        let mut d = dec_scaled(300000); // 300.000
        d.subtract_f32(100.500);
        assert_eq!(d.scaled(), 199500); // 199.500

        let mut d = dec_scaled(100000); // 100.000
        d.subtract_f32(300.000);
        assert_eq!(d.scaled(), 0); // Saturates at 0

        let mut d = dec_scaled(100000); // 100.000
        d.subtract_f32(0.0); // Subtracting zero
        assert_eq!(d.scaled(), 100000);

        let mut d = dec_scaled(100000); // 100.000
        d.subtract_f32(-50.0); // Subtracting negative (should add)
        assert_eq!(d.scaled(), 150000);
    }

    #[test]
    fn test_add_assign() {
        let mut d1 = dec_scaled(100000);
        let d2 = dec_scaled(200000);
        d1 += d2;
        assert_eq!(d1.scaled(), 300000);
    }

    #[test]
    fn test_sub_assign() {
        let mut d1 = dec_scaled(300000);
        let d2 = dec_scaled(100000);
        d1 -= d2;
        assert_eq!(d1.scaled(), 200000);
    }

    #[test]
    fn test_add_assign_f32() {
        let mut d = dec_scaled(100000);
        d += 200.500;
        assert_eq!(d.scaled(), 300500);
    }

    #[test]
    fn test_sub_assign_f32() {
        let mut d = dec_scaled(300000);
        d -= 100.500;
        assert_eq!(d.scaled(), 199500);
    }

    #[test]
    fn test_from_str_valid() {
        let d = "123.456".parse::<Decimal33>().unwrap();
        assert_eq!(d.scaled(), 123456);

        let d = "0".parse::<Decimal33>().unwrap();
        assert_eq!(d.scaled(), 0);

        let d = "999.999".parse::<Decimal33>().unwrap();
        assert_eq!(d.scaled(), 999999);
    }

    #[test]
    fn test_from_str_invalid() {
        let err = "abc".parse::<Decimal33>().unwrap_err();
        assert!(matches!(err, DecimalError::ParseFloatError(_)));

        let err = "".parse::<Decimal33>().unwrap_err();
        assert!(matches!(err, DecimalError::ParseFloatError(_)));

        let err = "-1.0".parse::<Decimal33>().unwrap_err();
        assert!(matches!(err, DecimalError::NegativeInput));

        let err = "1000.0".parse::<Decimal33>().unwrap_err();
        assert!(matches!(err, DecimalError::Overflow(1000.0)));
    }

    #[test]
    fn test_display() {
        let d = dec_scaled(123456);
        assert_eq!(format!("{}", d), "123.456");

        let d = dec_scaled(100000);
        assert_eq!(format!("{}", d), "100.000");

        let d = dec_scaled(500); // 0.500
        assert_eq!(format!("{}", d), "0.500");

        let d = dec_scaled(0);
        assert_eq!(format!("{}", d), "0.000");
    }

    #[test]
    fn test_from_f32_valid() {
        let d: Decimal33 = 123.456.into();
        assert_eq!(d.scaled(), 123456);

        let d: Decimal33 = 0.0.into();
        assert_eq!(d.scaled(), 0);

        let d: Decimal33 = 999.999.into();
        assert_eq!(d.scaled(), 999999);
    }

    #[test]
    #[should_panic]
    fn test_from_f32_negative_panics() {
        let _d: Decimal33 = (-1.0).into();
    }

    #[test]
    #[should_panic]
    fn test_from_f32_overflow_panics() {
        let _d: Decimal33 = 1000.0.into();
    }

    #[test]
    fn test_comparison() {
        let d1 = dec_scaled(100000);
        let d2 = dec_scaled(200000);
        let d3 = dec_scaled(100000);

        assert_eq!(d1, d3);
        assert_ne!(d1, d2);
        assert!(d1 < d2);
        assert!(d2 > d1);
        assert!(d1 <= d3);
        assert!(d2 >= d1);
    }
}
