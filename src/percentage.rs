use std::{convert::TryFrom, num::ParseFloatError, str::FromStr};

use ordered_float::NotNan;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(NotNan<f64>);

impl Percentage {
    /// Means `100%`
    pub const ONE: Percentage = unsafe { Percentage(NotNan::new_unchecked(1.0)) };

    /// Means `0%`
    pub const ZERO: Percentage = unsafe { Percentage(NotNan::new_unchecked(0.0)) };

    pub fn value(&self) -> f64 {
        *self.0
    }

    pub fn nearest_int(&self, total: usize) -> usize {
        (total as f64 * *self.0).round() as usize
    }

    /// Allows to bypass checks made in the usual construction (using `TryFrom`).
    ///
    /// # Safety
    ///
    /// Behaviour is undefined in case `v` is not finite (e.g. NaN, INFINITY).
    pub unsafe fn new_unchecked(v: f64) -> Self {
        Self(NotNan::new_unchecked(v))
    }
}

impl TryFrom<f64> for Percentage {
    type Error = PercentageError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(unsafe { NotNan::new_unchecked(value) }))
        } else {
            Err(PercentageError::NotFinite(value))
        }
    }
}

impl FromStr for Percentage {
    type Err = PercentageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<f64>()?;

        Self::try_from(value)
    }
}

#[derive(Debug, Error)]
pub enum PercentageError {
    #[error("Number must be finite to be a valid percentage")]
    NotFinite(f64),
    #[error(transparent)]
    FloatError(#[from] ParseFloatError),
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn percentage_works() {
        let valid = [0.5, 0.7, 0.25, 0.0, 1.0, 0.1, -0.5, 20.0, 1.5];

        for test in valid {
            assert!(
                Percentage::try_from(test).is_ok(),
                "{} should be valid",
                test
            );
        }

        let invalid = [f64::NAN, f64::INFINITY];

        for test in invalid {
            assert!(
                Percentage::try_from(test).is_err(),
                "{} shouldn't be valid",
                test
            );
        }
    }

    #[test]
    fn nearest_int_works() {
        let tests = [
            (0.5, 20, 10),
            (0.5, 15, 8),
            (0.3, 9, 3),
            (0.0, 10, 0),
            (1.0, 10, 10),
        ];

        for test in tests {
            let percent: Percentage = test.0.try_into().unwrap();
            assert_eq!(percent.nearest_int(test.1), test.2);
        }
    }
}
