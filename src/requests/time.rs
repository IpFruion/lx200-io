use std::fmt::Display;

use nom::bytes::complete::tag;

use crate::{
    error::Error,
    formats::{hrs::Hours, mins::Minutes, secs::Seconds},
};

pub const GET_SIDREAL_TIME: &[u8] = b":GS#";
pub const GET_LOCAL_24_HOUR_TIME: &[u8] = b":GL#";
pub const GET_LOCAL_12_HOUR_TIME: &[u8] = b":Ga#";
pub const GET_TIME_SIZE: usize = b"HH:MM:SS#".len();
pub const GET_GREENWICH_MEAN_TIME: &[u8] = b":GG#";
pub const GET_GREENWICH_MEAN_TIME_SIZE: usize = b"sHH#".len();

pub struct Time {
    pub hours: Hours,
    pub minutes: Minutes,
    pub seconds: Seconds,
}

impl Time {
    pub fn parse(input: &[u8]) -> Result<Time, Error> {
        let (input, hours) = Hours::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, minutes) = Minutes::from_bytes(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, seconds) = Seconds::from_bytes(input)?;
        let (_, _) = tag("#")(input)?;
        Ok(Time {
            hours,
            minutes,
            seconds,
        })
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hours, self.minutes, self.seconds)
    }
}
