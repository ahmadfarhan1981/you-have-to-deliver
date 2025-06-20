use std::ops::{AddAssign, SubAssign};
use std::fmt;
use std::str::FromStr;
use crate::utils::decimal33::{Decimal33, DecimalError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct DiscreteFloat33 {
    raw: Decimal33,
}

impl DiscreteFloat33 {
    /// Creates a new `DiscreteFloat` from an `f32` value.
    pub fn new(value: f32) -> Result<Self, DecimalError> {
        Ok(Self { raw: Decimal33::new(value)? })
    }

    /// Returns the floored integer part of the current value.
    /// This is the "effective" discrete output.
    pub fn value(&self) -> u32 {
        self.raw.scaled() / Decimal33::PRECISION
    }

    /// Returns the internal raw Decimal33 value.
    pub fn raw(&self) -> Decimal33 {
        self.raw
    }

    /// Adds a delta to the current value (saturating).
    pub fn add_f32(&mut self, delta: f32) {
        self.raw.add_f32(delta);
    }

    /// Subtracts a delta from the current value (saturating).
    pub fn subtract_f32(&mut self, delta: f32) {
        self.raw.subtract_f32(delta);
    }

    /// Returns the float difference needed to reach the next integer step.
    pub fn delta_to_next(&self) -> f32 {
        let next = (self.value() + 1) as f32;
        (next - self.raw.to_f32()).max(0.0)
    }
}

// Trait Implementations

impl From<f32> for DiscreteFloat33 {
    fn from(value: f32) -> Self {
        Self { raw: Decimal33::from(value) }
    }
}

impl AddAssign<f32> for DiscreteFloat33 {
    fn add_assign(&mut self, rhs: f32) {
        self.add_f32(rhs);
    }
}

impl SubAssign<f32> for DiscreteFloat33 {
    fn sub_assign(&mut self, rhs: f32) {
        self.subtract_f32(rhs);
    }
}

impl fmt::Display for DiscreteFloat33 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:.3})", self.value(), self.raw.to_f32())
    }
}

impl FromStr for DiscreteFloat33 {
    type Err = DecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { raw: Decimal33::from_str(s)? })
    }
}
