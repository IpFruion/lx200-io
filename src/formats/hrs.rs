use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

/// Represents `HH` from `00-24`
pub struct Hours(u8);

impl Hours {
    /// Creates a new hours as long as it falls between 00-24
    pub const fn new(val: u8) -> Option<Self> {
        if val > 24 {
            return None;
        }
        Some(Hours(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(2usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<u8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if v > 24 {
                return Err(FormatError::BoundaryReached);
            }
            Ok(Hours(v))
        })(input)
    }
}

impl Display for Hours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

/// Represents `sHH` from `-24-24`
pub struct SignedHours(i8);

impl SignedHours {
    #[inline]
    const fn is_invalid(val: i8) -> bool {
        val < -24 || val > 24
    }

    pub const fn new(val: i8) -> Option<Self> {
        if Self::is_invalid(val) {
            return None;
        }
        Some(SignedHours(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(3usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<i8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if Self::is_invalid(v) {
                return Err(FormatError::BoundaryReached);
            }
            Ok(SignedHours(v))
        })(input)
    }
}

impl Display for SignedHours {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+02}", self.0)
    }
}
