use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

/// Represents `MM` from `01-12`
pub struct Months(u8);

impl Months {
    /// Creates a new hours as long as it falls between 00-24
    pub const fn new(val: u8) -> Option<Self> {
        if val < 1 || val > 12 {
            return None;
        }
        Some(Months(val))
    }

    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        map_res(take(2usize), |value| {
            let v = str::from_utf8(value)?;
            let v = v
                .parse::<u8>()
                .map_err(|err| FormatError::Num(Box::new(err)))?;
            if !(1..=12).contains(&v) {
                return Err(FormatError::BoundaryReached);
            }
            Ok(Months(v))
        })(input)
    }
}

impl Display for Months {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
