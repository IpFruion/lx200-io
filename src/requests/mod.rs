use std::fmt::{Display, Write};

use nom::{branch::alt, bytes::complete::tag, combinator::value};

use crate::error::Error;

pub mod alt;
pub mod asc;
pub mod azim;
pub mod cal;
pub mod decl;
pub mod latlng;
pub mod site;
pub mod time;

pub const GET_ALIGNMENT_STATUS: &[u8] = &[6];
pub const GET_ALIGNMENT_STATUS_SIZE: usize = 1;

pub const SYNC: &[u8] = b":GM#";

#[derive(Clone)]
pub enum AlignmentStatus {
    AltitudeAzmuth,
    Land,
    Polar,
    GermanPolar,
}

impl AlignmentStatus {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, status) = alt((
            value(AlignmentStatus::AltitudeAzmuth, tag(b"A")),
            value(AlignmentStatus::Land, tag(b"L")),
            value(AlignmentStatus::Polar, tag(b"P")),
            value(AlignmentStatus::GermanPolar, tag(b"G")),
        ))(input)?;

        Ok(status)
    }
}

pub enum Direction {
    North,
    Easth,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => f.write_char('n'),
            Direction::Easth => f.write_char('e'),
            Direction::South => f.write_char('s'),
            Direction::West => f.write_char('w'),
        }
    }
}

#[derive(Clone)]
pub enum SlewStatus {
    /// 0 is returned if the telescope can complete the slew,
    CanComplete,
    /// 1 is returned if the object is below the horizon,
    BelowHorizon,
    /// 2 is returned if the object is below the 'higher' limit.
    BelowLimit,
}

impl SlewStatus {
    pub fn parse(input: &[u8]) -> Result<Self, Error> {
        let (_, status) = alt((
            value(SlewStatus::CanComplete, tag(b"0")),
            value(SlewStatus::BelowHorizon, tag(b"1")),
            value(SlewStatus::BelowLimit, tag(b"2")),
        ))(input)?;

        Ok(status)
    }
}

pub enum MotionRate {
    Guide,
    Center,
    Find,
    Slew,
}

impl Display for MotionRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MotionRate::Guide => write!(f, "RG"),
            MotionRate::Center => write!(f, "RC"),
            MotionRate::Find => write!(f, "RM"),
            MotionRate::Slew => write!(f, "RS"),
        }
    }
}
