use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

// Represents `HH` from `00-24`
pub struct Hours(u8);

impl Hours {
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
