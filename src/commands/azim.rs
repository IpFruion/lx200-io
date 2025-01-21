use nom::{bytes::complete::tag, IResult};

use crate::{
    error::Error,
    formats::{degs::Degrees, mins::Minutes},
};

pub struct CurrentAzimuth;

impl CurrentAzimuth {
    pub const fn request(&self) -> [u8; 4] {
        [b':', b'G', b'A', b'#']
    }

    pub fn response(self, input: &[u8]) -> Result<Azimuth, Error> {
        let (input, azim) = Azimuth::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(azim)
    }
}

pub struct Azimuth {
    pub degrees: Degrees,
    pub minutes: Minutes,
}

impl Azimuth {
    fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, degrees) = Degrees::from_bytes(input)?;
        let (input, _) = tag([223u8])(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        Ok((input, Azimuth { degrees, minutes }))
    }
}
