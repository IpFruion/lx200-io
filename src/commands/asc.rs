use std::fmt::Display;

use nom::{bytes::complete::tag, combinator::opt, IResult};

use crate::{
    error::Error,
    formats::{
        hrs::Hours,
        mins::{Minutes, Tenths},
    },
};

pub struct RightAscension;

impl RightAscension {
    pub const fn request(&self) -> [u8; 4] {
        [b':', b'G', b'R', b'#']
    }

    //TODO: A Mapug-Astronomy post indicated that for the 3.34L ROMS the format is  +HH:MM:SS# for RA
    // And the format for Declination is  sDD*MM'SS#.  Thus adding seconds and arc seconds.
    pub fn response(self, input: &[u8]) -> Result<(Hours, Minutes, Tenths), Error> {
        let (input, _) = tag("+")(input)?;
        let (input, hours) = Hours::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (input, _) = tag(".")(input)?;
        let (input, tenths_minutes) = Tenths::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok((hours, minutes, tenths_minutes))
    }
}

pub struct Ascension {
    pub hours: Hours,
    pub minutes: Minutes,
    pub tenths_minutes: Tenths,
}

impl Ascension {
    pub fn from_bytes(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = opt(tag("+"))(input)?;
        let (input, hours) = Hours::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (input, _) = tag(".")(input)?;
        let (input, tenths_minutes) = Tenths::from_bytes(input)?;
        let (input, _) = tag("#")(input)?;
        Ok((
            input,
            Ascension {
                hours,
                minutes,
                tenths_minutes,
            },
        ))
    }
}

impl Display for Ascension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}.{}#",
            self.hours, self.minutes, self.tenths_minutes
        )
    }
}
