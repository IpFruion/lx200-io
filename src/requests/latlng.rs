use nom::bytes::complete::tag;

use crate::{
    error::Error,
    formats::{
        degs::{Degrees, SignedDegrees},
        mins::Minutes,
    },
};

pub const GET_LATITUDE: &[u8] = b":Gt#";
pub const GET_LONGITUDE: &[u8] = b":Gg#";
pub const GET_LATITUDE_SIZE: usize = b"sDD\xDFMM#".len();
pub const GET_LONGITUDE_SIZE: usize = b"DDD\xDFMM#".len();

pub struct Latitude {
    pub degrees: SignedDegrees,
    pub minutes: Minutes,
}

impl Latitude {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, degrees) = SignedDegrees::from_bytes(input)?;
        let (input, _) = tag(b"\xDF")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Latitude { degrees, minutes })
    }

    pub fn set_request(&self) -> Vec<u8> {
        let mut bytes = b":St ".to_vec();
        bytes.extend(self.degrees.to_string().as_bytes());
        bytes.push(b'\xDF');
        bytes.extend(self.minutes.to_string().as_bytes());
        bytes.push(b'#');
        bytes
    }
}

pub struct Longitude {
    pub degrees: Degrees,
    pub minutes: Minutes,
}

impl Longitude {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, degrees) = Degrees::from_bytes(input)?;
        let (input, _) = tag(b"\xDF")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Longitude { degrees, minutes })
    }

    pub fn set_request(&self) -> Vec<u8> {
        let mut bytes = b":Sg ".to_vec();
        bytes.extend(self.degrees.to_string().as_bytes());
        bytes.push(b'\xDF');
        bytes.extend(self.minutes.to_string().as_bytes());
        bytes.push(b'#');
        bytes
    }
}
