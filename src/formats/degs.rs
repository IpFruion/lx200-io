use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

/// Represents `DDD` from `000-359`
pub struct Degrees(u16);

impl Degrees {
    pub const fn new(val: u16) -> Option<Self> {
        if val > 359 {
            return None;
        }
        Some(Degrees(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(2usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<u16>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if v > 359 {
                return Err(FormatError::BoundaryReached);
            }
            Ok(Degrees(v))
        })(input)
    }
}

impl Display for Degrees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03}", self.0)
    }
}

// Represents `sDD` from `-90 to 90`
#[derive(Debug)]
pub struct SignedDegrees(i8);

impl SignedDegrees {
    pub const fn new(val: i8) -> Option<Self> {
        if val > 90 || val < -90 {
            return None;
        }
        Some(SignedDegrees(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(3usize), |value| {
            let v = str::from_utf8(value)?;
            let val = v
                .parse::<i8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if !(-90..=90).contains(&val) {
                return Err(FormatError::BoundaryReached);
            }
            Ok(SignedDegrees(val))
        })(input)
    }
}

impl Display for SignedDegrees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:+02}", self.0)
    }
}

impl PartialEq<i8> for SignedDegrees {
    fn eq(&self, other: &i8) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq for SignedDegrees {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
