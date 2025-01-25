use nom::bytes::complete::tag;

use crate::{
    error::Error,
    formats::{degs::Degrees, mins::Minutes},
};

pub const GET_AZMITH: &[u8] = b":GZ#";
pub const GET_AZMITH_SIZE: usize = b"DDD\xDFMM#".len();

pub struct Azimuth {
    pub degrees: Degrees,
    pub minutes: Minutes,
}

impl Azimuth {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, degrees) = Degrees::from_bytes(input)?;
        let (input, _) = tag(b"\xDF")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Azimuth { degrees, minutes })
    }
}
