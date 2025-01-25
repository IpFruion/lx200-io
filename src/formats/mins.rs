use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

/// Represents `MM` from `00-59`
#[derive(Debug)]
pub struct Minutes(u8);

impl Minutes {
    pub const fn new(val: u8) -> Option<Self> {
        if val > 59 {
            return None;
        }
        Some(Minutes(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(2usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<u8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if v > 59 {
                return Err(FormatError::BoundaryReached);
            }
            Ok(Minutes(v))
        })(input)
    }
}

impl Display for Minutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl PartialEq<u8> for Minutes {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq for Minutes {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

// Represents `T` from `0-9`
pub struct Tenths(u8);

impl Tenths {
    pub const fn new(val: u8) -> Option<Self> {
        if val > 9 {
            return None;
        }
        Some(Tenths(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(1usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<u8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if v > 9 {
                return Err(FormatError::BoundaryReached);
            }
            Ok(Tenths(v))
        })(input)
    }
}

impl Display for Tenths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:01}", self.0)
    }
}
