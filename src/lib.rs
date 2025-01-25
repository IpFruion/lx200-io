use std::io::{Read, Write};

use error::Error;
use requests::{
    alt::{Altitude, GET_ALTITUDE, GET_ALTITUDE_SIZE},
    asc::{Ascension, GET_RIGHT_ASCENSION, GET_RIGHT_ASCENSION_SIZE},
    azim::{Azimuth, GET_AZMITH, GET_AZMITH_SIZE},
    cal::{Date, GET_DATE, GET_DATE_SIZE},
    decl::{Declination, GET_DECLANATION, GET_DECLANATION_SIZE},
    time::{Time, GET_LOCAL_12_HOUR_TIME, GET_LOCAL_24_HOUR_TIME, GET_SIDREAL_TIME, GET_TIME_SIZE},
};

pub mod error;
pub mod formats;
pub mod requests;

pub struct Protocol<T> {
    reader_writer: T,
}

impl<T: Read + Write> Protocol<T> {
    pub fn new(reader_writer: T) -> Self {
        Protocol { reader_writer }
    }

    // pub fn get_status(&mut self) -> Result<Status, Error> {
    //
    // }

    /// Gets the current Right Ascension.
    pub fn get_right_ascension(&mut self) -> Result<Ascension, Error> {
        self.reader_writer.write_all(GET_RIGHT_ASCENSION)?;
        let mut response = [0u8; GET_RIGHT_ASCENSION_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Ascension::parse(&response)
    }

    /// Gets the current declination.
    pub fn get_declination(&mut self) -> Result<Declination, Error> {
        self.reader_writer.write_all(GET_DECLANATION)?;
        let mut response = [0u8; GET_DECLANATION_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Declination::parse(&response)
    }

    /// Gets the current altitude.
    pub fn get_altitude(&mut self) -> Result<Altitude, Error> {
        self.reader_writer.write_all(GET_ALTITUDE)?;
        let mut response = [0u8; GET_ALTITUDE_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Altitude::parse(&response)
    }

    /// Gets the current azimuth.
    pub fn get_azmith(&mut self) -> Result<Azimuth, Error> {
        self.reader_writer.write_all(GET_AZMITH)?;
        let mut response = [0u8; GET_AZMITH_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Azimuth::parse(&response)
    }

    /// Gets the current sidereal time.
    pub fn get_sidreal_time(&mut self) -> Result<Time, Error> {
        self.reader_writer.write_all(GET_SIDREAL_TIME)?;
        let mut response = [0u8; GET_TIME_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Time::parse(&response)
    }

    /// Gets the local time in 24 hour.
    pub fn get_local_24_hour_time(&mut self) -> Result<Time, Error> {
        self.reader_writer.write_all(GET_LOCAL_24_HOUR_TIME)?;
        let mut response = [0u8; GET_TIME_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Time::parse(&response)
    }

    /// Gets the local time in 12 hour.
    pub fn get_local_12_hour_time(&mut self) -> Result<Time, Error> {
        self.reader_writer.write_all(GET_LOCAL_12_HOUR_TIME)?;
        let mut response = [0u8; GET_TIME_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Time::parse(&response)
    }

    /// Gets the calendar date.
    pub fn get_calendar_date(&mut self) -> Result<Date, Error> {
        self.reader_writer.write_all(GET_DATE)?;
        let mut response = [0u8; GET_DATE_SIZE];
        self.reader_writer.read_exact(&mut response)?;
        Date::parse(&response)
    }

    /// Set the sidereal time.
    pub fn set_sidreal_time(&mut self, time: &Time) -> Result<(), Error> {
        self.reader_writer.write_all(&time.set_sidreal_request())?;
        let mut response = [0u8; 1];
        self.reader_writer.read_exact(&mut response)?;
        Error::parse(&response)
    }

    /// Sets the local time.
    ///
    /// **NOTE**: The parameter should always be in 24 hour format.
    pub fn set_local_time(&mut self, time: &Time) -> Result<(), Error> {
        self.reader_writer.write_all(&time.set_local_request())?;
        let mut response = [0u8; 1];
        self.reader_writer.read_exact(&mut response)?;
        Error::parse(&response)
    }

    /// Sets the calendar date.
    pub fn set_calendar_date(&mut self, date: &Date) -> Result<(), Error> {
        self.reader_writer.write_all(&date.set_request())?;
        let mut response = [0u8; 1];
        self.reader_writer.read_exact(&mut response)?;
        Error::parse(&response)?;

        //NOTE: After the Ok, if the date is valid, two strings will be sent. The first will contain the message
        // "Updating planetary data," the second (sent after the planetary calculations) will contain only blanks.
        // Both strings will be terminated by the (*) symbol.
        let mut buf = [0u8; 8];
        let mut count = 0;
        loop {
            let read = self.reader_writer.read(&mut buf)?;
            if read == 0 {
                continue;
            }
            for b in buf.iter_mut().take(read) {
                if *b == b'\xDF' {
                    count += 1;
                }
                *b = 0;
            }
            if count == 2 {
                break;
            }
        }
        Ok(())
    }
}
