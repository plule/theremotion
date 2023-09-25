use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use derive_more::Into;
use staff::Interval;

/// Interval between to floating notes
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Into)]
pub struct IntervalF(pub f32);

impl IntervalF {
    /// Returns the semitones of this [`IntervalF`].
    pub fn semitones(&self) -> f32 {
        self.0
    }
    /// Absolute interval.
    pub fn abs(&self) -> Self {
        Self(self.semitones().abs())
    }
    /// Clamp this interval between a minimal and a maximal value
    pub fn clamp(&self, min: IntervalF, max: IntervalF) -> Self {
        Self(self.semitones().clamp(min.semitones(), max.semitones()))
    }
}

impl From<Interval> for IntervalF {
    fn from(value: Interval) -> Self {
        Self(value.semitones() as f32)
    }
}

impl From<IntervalF> for f64 {
    fn from(value: IntervalF) -> Self {
        value.semitones().into()
    }
}

impl Neg for IntervalF {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.semitones())
    }
}

impl Display for IntervalF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.semitones())
    }
}

impl Add for IntervalF {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.semitones() + rhs.semitones())
    }
}

impl Sub for IntervalF {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.semitones() - rhs.semitones())
    }
}

impl Div for IntervalF {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.semitones() / rhs.semitones()
    }
}

impl Mul<f32> for IntervalF {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.semitones() * rhs)
    }
}
