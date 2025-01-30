use nom::bytes::complete::tag;

#[cfg(feature = "roms")]
use crate::formats::secs::Seconds;
use crate::{
    error::Error,
    formats::{degs::SignedDegrees, mins::Minutes},
};

pub const GET_DECLANATION: &[u8] = b":GD#";
pub const GET_OBJECT_DECLANATION: &[u8] = b":Gd#";
#[cfg(not(feature = "roms"))]
pub const GET_DECLANATION_SIZE: usize = b"sDD\xDFMM#".len();
#[cfg(feature = "roms")]
pub const GET_DECLANATION_SIZE: usize = b"sDD\xDFMM'SS#".len();

pub struct Declination {
    pub degrees: SignedDegrees,
    pub minutes: Minutes,
    #[cfg(feature = "roms")]
    pub arc_seconds: Seconds,
}

impl Declination {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, degrees) = SignedDegrees::from_bytes(input)?;
        let (input, _) = tag(b"\xDF")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Declination { degrees, minutes })
    }

    /// A Mapug-Astronomy post indicated that for the 3.34L ROMS the format for Declination is `sDD*MM'SS#`.
    #[cfg(feature = "roms")]
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, degrees) = SignedDegrees::from_bytes(input)?;
        let (input, _) = tag(b"\xDF")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (input, _) = tag("'")(input)?;
        let (input, arc_seconds) = Seconds::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Declination {
            degrees,
            minutes,
            arc_seconds,
        })
    }

    pub fn set_request(&self) -> Vec<u8> {
        let mut bytes = b":Sd ".to_vec();
        bytes.extend(self.degrees.to_string().as_bytes());
        bytes.push(b'\xDF');
        bytes.extend(self.minutes.to_string().as_bytes());
        bytes.push(b'#');
        bytes
    }
}
