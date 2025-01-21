use nom::bytes::complete::tag;

use crate::{
    error::Error,
    formats::{degs::SignedDegrees, mins::Minutes},
};

pub struct CurrentDeclanation;

impl CurrentDeclanation {
    pub const fn request(&self) -> [u8; 4] {
        [b':', b'G', b'D', b'#']
    }

    //TODO: A Mapug-Astronomy post indicated that for the 3.34L ROMS the format is  +HH:MM:SS# for RA
    // And the format for Declination is  sDD*MM'SS#.  Thus adding seconds and arc seconds.
    pub fn response(self, input: &[u8]) -> Result<(SignedDegrees, Minutes), Error> {
        let (input, _) = tag("+")(input)?;
        let (input, degrees) = SignedDegrees::from_bytes(input)?;
        let (input, _) = tag([223u8])(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok((degrees, minutes))
    }
}
