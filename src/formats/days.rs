use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

/// Represents `DD` from `01-31`
pub struct Days(u8);

impl Days {
    /// Creates a new hours as long as it falls between 00-24
    pub const fn new(val: u8) -> Option<Self> {
        if val < 1 || val > 31 {
            return None;
        }
        Some(Days(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(2usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<u8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if !(1..=31).contains(&v) {
                return Err(FormatError::BoundaryReached);
            }
            Ok(Days(v))
        })(input)
    }
}

impl Display for Days {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
