use std::fmt::Display;

use nom::bytes::complete::tag;

use crate::{
    error::Error,
    formats::{days::Days, months::Months, yrs::Years},
};

pub const GET_DATE: &[u8] = b":GC#";
pub const GET_DATE_SIZE: usize = b"MM/DD/YY#".len();

/// Range: 01/01/00 to 12/31/99
///
/// Month, day, and year. The two digit year indicates the following: 92 through 99 = 1992 through 1999
/// and 00 through 91 = 2000 through 2091
pub struct Date {
    pub months: Months,
    pub days: Days,
    pub years: Years,
}

impl Date {
    pub fn set_request(&self) -> Vec<u8> {
        format!(":SC {}#", self).into_bytes()
    }

    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (input, months) = Months::from_bytes(input)?;
        let (input, _) = tag(b"/")(input)?;
        let (input, days) = Days::from_bytes(input)?;
        let (input, _) = tag(b"/")(input)?;
        let (input, years) = Years::from_bytes(input)?;
        let (_, _) = tag(b"#")(input)?;
        Ok(Date {
            months,
            days,
            years,
        })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.months, self.days, self.years)
    }
}
