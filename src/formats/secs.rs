use core::str;
use std::fmt::Display;

use nom::{bytes::complete::take, combinator::map_res, IResult};

use super::FormatError;

pub struct Seconds(u8);

impl Seconds {
    pub const fn new(val: u8) -> Option<Self> {
        if val > 59 {
            return None;
        }
        Some(Seconds(val))
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
            Ok(Seconds(v))
        })(input)
    }
}

impl Display for Seconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
