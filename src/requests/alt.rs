use nom::bytes::complete::tag;

use crate::{
    error::Error,
    formats::{degs::SignedDegrees, mins::Minutes},
};

pub const GET_ALTITUDE: &[u8] = b":GA#";
pub const GET_ALTITUDE_SIZE: usize = b"sDD\xDFMM#".len();

pub struct Altitude {
    pub degrees: SignedDegrees,
    pub minutes: Minutes,
}

impl Altitude {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, degrees) = SignedDegrees::from_bytes(input)?;
        let (input, _) = tag(b"\xDF")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Altitude { degrees, minutes })
    }
}
