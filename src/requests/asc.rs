use std::fmt::Display;

use nom::bytes::complete::tag;

#[cfg(feature = "roms")]
use crate::formats::secs::Seconds;
use crate::{
    error::Error,
    formats::{
        hrs::Hours,
        mins::{Minutes, Tenths},
    },
};

pub const GET_RIGHT_ASCENSION: &[u8] = b":GR#";

#[cfg(not(feature = "roms"))]
pub const GET_RIGHT_ASCENSION_SIZE: usize = b"+HH:MM.T#".len();
#[cfg(feature = "roms")]
pub const GET_RIGHT_ASCENSION_SIZE: usize = b"+HH:MM:SS#".len();

pub struct Ascension {
    pub hours: Hours,
    pub minutes: Minutes,
    #[cfg(not(feature = "roms"))]
    pub tenths_minutes: Tenths,
    #[cfg(feature = "roms")]
    pub seconds: Seconds,
}

impl Ascension {
    /// Parses [AscensionSeconds] from byte array corresponding to the [Right Ascension](https://en.wikipedia.org/wiki/Right_ascension) orientation.
    /// This correspondds to horizontal rotation when looking at the sky.
    ///
    /// *NOTE* A Mapug-Astronomy post indicated that for the 3.34L ROMS the format is +HH:MM:SS# for RA
    #[cfg(feature = "roms")]
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, _) = tag("+")(input)?;
        let (input, hours) = Hours::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, seconds) = Seconds::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Ascension {
            hours,
            minutes,
            seconds,
        })
    }

    /// Parses [Ascension] from byte array corresponding to the [Right Ascension](https://en.wikipedia.org/wiki/Right_ascension) orientation.
    /// This correspondds to horizontal rotation when looking at the sky.
    #[cfg(not(feature = "roms"))]
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, _) = tag("+")(input)?;
        let (input, hours) = Hours::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (input, _) = tag(".")(input)?;
        let (input, tenths_minutes) = Tenths::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Ascension {
            hours,
            minutes,
            tenths_minutes,
        })
    }
}

impl Display for Ascension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "roms"))]
        write!(
            f,
            "{}:{}.{}#",
            self.hours, self.minutes, self.tenths_minutes
        )?;
        #[cfg(feature = "roms")]
        write!(f, "{}:{}:{}#", self.hours, self.minutes, self.seconds)?;
        Ok(())
    }
}
