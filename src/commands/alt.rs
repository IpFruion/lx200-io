use nom::{bytes::complete::tag, IResult};

use crate::{
    error::Error,
    formats::{degs::SignedDegrees, mins::Minutes},
};

pub struct CurrentAltitude;

impl CurrentAltitude {
    pub const fn request(&self) -> [u8; 4] {
        [b':', b'G', b'A', b'#']
    }

    //TODO: A Mapug-Astronomy post indicated that for the 3.34L ROMS the format is  +HH:MM:SS# for RA
    // And the format for Declination is  sDD*MM'SS#.  Thus adding seconds and arc seconds.
    pub fn response(self, input: &[u8]) -> Result<Altitude, Error> {
        let (input, alt) = Altitude::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(alt)
    }
}

pub struct Altitude {
    pub degrees: SignedDegrees,
    pub minutes: Minutes,
}

impl Altitude {
    fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, degrees) = SignedDegrees::from_bytes(input)?;
        let (input, _) = tag([223u8])(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        Ok((input, Altitude { degrees, minutes }))
    }
}
