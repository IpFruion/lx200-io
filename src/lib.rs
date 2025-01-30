use std::ops::{Deref, DerefMut};

use error::Error;
use formats::hrs::SignedHours;
use requests::{
    alt::{Altitude, GET_ALTITUDE, GET_ALTITUDE_SIZE},
    asc::{
        Ascension, GET_OBJECT_RIGHT_ASCENSION, GET_OBJECT_RIGHT_ASCENSION_SIZE,
        GET_RIGHT_ASCENSION, GET_RIGHT_ASCENSION_SIZE,
    },
    azim::{Azimuth, GET_AZMITH, GET_AZMITH_SIZE},
    cal::{Date, GET_DATE, GET_DATE_SIZE},
    decl::{Declination, GET_DECLANATION, GET_DECLANATION_SIZE, GET_OBJECT_DECLANATION},
    latlng::{
        Latitude, Longitude, GET_LATITUDE, GET_LATITUDE_SIZE, GET_LONGITUDE, GET_LONGITUDE_SIZE,
    },
    site::Site,
    time::{
        Time, GET_GREENWICH_MEAN_TIME, GET_GREENWICH_MEAN_TIME_SIZE, GET_LOCAL_12_HOUR_TIME,
        GET_LOCAL_24_HOUR_TIME, GET_SIDREAL_TIME, GET_TIME_SIZE,
    },
    AlignmentStatus, Direction, MotionRate, SlewStatus, GET_ALIGNMENT_STATUS,
    GET_ALIGNMENT_STATUS_SIZE, SYNC,
};

pub mod error;
pub mod formats;
pub mod requests;

#[cfg(not(feature = "tokio"))]
pub trait Stream: std::io::Read + std::io::Write {}
#[cfg(not(feature = "tokio"))]
impl<T: std::io::Read + std::io::Write> Stream for T {}

#[cfg(feature = "tokio")]
pub trait Stream: tokio::io::AsyncReadExt + tokio::io::AsyncWriteExt + std::marker::Unpin {}
#[cfg(feature = "tokio")]
impl<T: tokio::io::AsyncReadExt + tokio::io::AsyncWriteExt + std::marker::Unpin> Stream for T {}

/// Command set derived from [LX200 Spec](https://www.skymtn.com/mapug-astronomy/ragreiner/LX200Commands.html)
pub struct Client<T> {
    stream: T,
}

impl<T: Stream> Client<T> {
    pub fn new(stream: T) -> Self {
        Client { stream }
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.stream
    }
}

impl<T: Stream + 'static> Client<T> {
    pub fn into_boxed(self) -> BoxedClient {
        BoxedClient(Client {
            stream: Box::new(self.stream),
        })
    }
}

/// General Telescope Information
impl<T: Stream> Client<T> {
    /// Gets alignment status.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn alignment_status(&mut self) -> Result<AlignmentStatus, Error> {
        self.stream.write_all(GET_ALIGNMENT_STATUS).await?;
        let mut response = [0u8; GET_ALIGNMENT_STATUS_SIZE];
        self.stream.read_exact(&mut response).await?;
        AlignmentStatus::parse(&response)
    }

    /// Gets the current Right Ascension.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_right_ascension(&mut self) -> Result<Ascension, Error> {
        self.stream.write_all(GET_RIGHT_ASCENSION).await?;
        let mut response = [0u8; GET_RIGHT_ASCENSION_SIZE];
        self.stream.read_exact(&mut response).await?;
        Ascension::parse(&response)
    }

    /// Gets the current declination.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_declination(&mut self) -> Result<Declination, Error> {
        self.stream.write_all(GET_DECLANATION).await?;
        let mut response = [0u8; GET_DECLANATION_SIZE];
        self.stream.read_exact(&mut response).await?;
        Declination::parse(&response)
    }

    /// Gets the current altitude.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_altitude(&mut self) -> Result<Altitude, Error> {
        self.stream.write_all(GET_ALTITUDE).await?;
        let mut response = [0u8; GET_ALTITUDE_SIZE];
        self.stream.read_exact(&mut response).await?;
        Altitude::parse(&response)
    }

    /// Gets the current azimuth.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_azmith(&mut self) -> Result<Azimuth, Error> {
        self.stream.write_all(GET_AZMITH).await?;
        let mut response = [0u8; GET_AZMITH_SIZE];
        self.stream.read_exact(&mut response).await?;
        Azimuth::parse(&response)
    }

    /// Gets the current sidereal time.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_sidreal_time(&mut self) -> Result<Time, Error> {
        self.stream.write_all(GET_SIDREAL_TIME).await?;
        let mut response = [0u8; GET_TIME_SIZE];
        self.stream.read_exact(&mut response).await?;
        Time::parse(&response)
    }

    /// Gets the local time in 24 hour.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_local_24_hour_time(&mut self) -> Result<Time, Error> {
        self.stream.write_all(GET_LOCAL_24_HOUR_TIME).await?;
        let mut response = [0u8; GET_TIME_SIZE];
        self.stream.read_exact(&mut response).await?;
        Time::parse(&response)
    }

    /// Gets the local time in 12 hour.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_local_12_hour_time(&mut self) -> Result<Time, Error> {
        self.stream.write_all(GET_LOCAL_12_HOUR_TIME).await?;
        let mut response = [0u8; GET_TIME_SIZE];
        self.stream.read_exact(&mut response).await?;
        Time::parse(&response)
    }

    /// Gets the calendar date.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_calendar_date(&mut self) -> Result<Date, Error> {
        self.stream.write_all(GET_DATE).await?;
        let mut response = [0u8; GET_DATE_SIZE];
        self.stream.read_exact(&mut response).await?;
        Date::parse(&response)
    }

    /// Gets the latitude of the currently selected site.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_latitude(&mut self) -> Result<Latitude, Error> {
        self.stream.write_all(GET_LATITUDE).await?;
        let mut response = [0u8; GET_LATITUDE_SIZE];
        self.stream.read_exact(&mut response).await?;
        Latitude::parse(&response)
    }

    /// Gets the longitude of the currently selected site.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_longitude(&mut self) -> Result<Longitude, Error> {
        self.stream.write_all(GET_LONGITUDE).await?;
        let mut response = [0u8; GET_LONGITUDE_SIZE];
        self.stream.read_exact(&mut response).await?;
        Longitude::parse(&response)
    }

    /// Gets the offset from Greenwich Mean Time.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_gmt(&mut self) -> Result<SignedHours, Error> {
        self.stream.write_all(GET_GREENWICH_MEAN_TIME).await?;
        let mut response = [0u8; GET_GREENWICH_MEAN_TIME_SIZE];
        self.stream.read_exact(&mut response).await?;
        let (_, hrs) = SignedHours::from_bytes(&response)?;
        Ok(hrs)
    }

    /// Set the sidereal time.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_sidreal_time(&mut self, time: &Time) -> Result<(), Error> {
        self.stream
            .write_all(format!(":SS {}#", time).as_bytes())
            .await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    /// Sets the local time.
    ///
    /// **NOTE**: The parameter should always be in 24 hour format.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_local_time(&mut self, time: &Time) -> Result<(), Error> {
        self.stream
            .write_all(format!(":SL {}#", time).as_bytes())
            .await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    /// Sets the calendar date.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_calendar_date(&mut self, date: &Date) -> Result<(), Error> {
        self.stream
            .write_all(format!(":SC {}#", date).as_bytes())
            .await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)?;

        //NOTE: After the Ok, if the date is valid, two strings will be sent. The first will contain the message
        // "Updating planetary data," the second (sent after the planetary calculations) will contain only blanks.
        // Both strings will be terminated by the (*) symbol.
        let mut buf = [0u8; 8];
        let mut count = 0;
        loop {
            let read = self.stream.read(&mut buf).await?;
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

    /// Sets the latitude of the currently selected site.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_latitude(&mut self, latitude: &Latitude) -> Result<(), Error> {
        self.stream.write_all(&latitude.set_request()).await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    /// Sets the longitude of the currently selected site
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_longitude(&mut self, longitude: &Longitude) -> Result<(), Error> {
        self.stream.write_all(&longitude.set_request()).await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    ///  Sets the offset from Greenwich Mean Time.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_gmt_offset(&mut self, offset: &SignedHours) -> Result<(), Error> {
        self.stream
            .write_all(format!(":SG {}#", offset).as_bytes())
            .await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    /// Sets the current site number.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_site(&mut self, site: &Site) -> Result<(), Error> {
        self.stream
            .write_all(format!(":{}#", site).as_bytes())
            .await?;
        Ok(())
    }
}

/// Telescope Motion
impl<T: Stream> Client<T> {
    /// Starts motion in the specified direction at the current rate.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn start_motion(&mut self, direction: &Direction) -> Result<(), Error> {
        self.stream
            .write_all(format!(":M{}#", direction).as_bytes())
            .await?;
        Ok(())
    }

    /// Slews telescope to current object coordinates and yields it's status
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn slew(&mut self) -> Result<SlewStatus, Error> {
        self.stream.write_all(b":MS#").await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        //TODO: If 1 or 2 is returned, a string containing an appropriate message is also returned.
        //Handle this message???
        SlewStatus::parse(&response)
    }
    /// Stops motion in the specified direction. Also stops the telescope if a slew to object is in progress.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn stop_motion(&mut self, direction: &Direction) -> Result<(), Error> {
        self.stream
            .write_all(format!(":Q{}#", direction).as_bytes())
            .await?;
        Ok(())
    }

    /// Stops a slew to an object.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn stop_slew(&mut self) -> Result<(), Error> {
        self.stream.write_all(b":Q#").await?;
        Ok(())
    }

    /// Sets the motion rate.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_motion_rate(&mut self, motion_rate: &MotionRate) -> Result<(), Error> {
        self.stream
            .write_all(format!(":{}#", motion_rate).as_bytes())
            .await?;
        Ok(())
    }
}

/// Library / Objects
impl<T: Stream> Client<T> {
    /// Gets object Right Ascension.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_object_right_ascension(&mut self) -> Result<Ascension, Error> {
        self.stream.write_all(GET_OBJECT_RIGHT_ASCENSION).await?;
        let mut response = [0u8; GET_OBJECT_RIGHT_ASCENSION_SIZE];
        self.stream.read_exact(&mut response).await?;
        Ascension::parse(&response)
    }

    /// Gets object declination.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn get_object_declination(&mut self) -> Result<Declination, Error> {
        self.stream.write_all(GET_OBJECT_DECLANATION).await?;
        let mut response = [0u8; GET_DECLANATION_SIZE];
        self.stream.read_exact(&mut response).await?;
        Declination::parse(&response)
    }

    /// Sets object Right Ascension.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_object_right_ascension(
        &mut self,
        right_ascension: &Ascension,
    ) -> Result<(), Error> {
        self.stream
            .write_all(format!(":Sr {}#", right_ascension).as_bytes())
            .await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    /// Sets object declination.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn set_object_declination(&mut self, declination: &Declination) -> Result<(), Error> {
        self.stream.write_all(&declination.set_request()).await?;
        let mut response = [0u8; 1];
        self.stream.read_exact(&mut response).await?;
        Error::parse(&response)
    }

    /// Sync. Matches current telescope coordinates to the object coordinates.
    #[cfg_attr(not(feature = "tokio"), maybe_async::must_be_sync)]
    pub async fn sync(&mut self) -> Result<(), Error> {
        self.stream.write_all(SYNC).await?;
        // TODO: sends a string indicating which object's coordinates were used.
        Ok(())
    }
}

#[cfg(feature = "io")]
impl Client<Box<dyn serialport::SerialPort>> {
    pub fn open(port: &str) -> serialport::Result<Self> {
        serialport::new(port, 9600)
            .open()
            .map(|stream| Client { stream })
    }

    /// Uses `/dev/ttyUSB0` as the default port
    pub fn open_usb0() -> serialport::Result<Self> {
        Self::open("/dev/ttyUSB0")
    }
}

#[cfg(feature = "io")]
impl Client<std::net::TcpStream> {
    pub fn connect(addr: std::net::Ipv4Addr) -> std::io::Result<Self> {
        std::net::TcpStream::connect((addr, 9998)).map(|stream| Client { stream })
    }
}

pub struct BoxedClient(Client<Box<dyn Stream>>);

impl Deref for BoxedClient {
    type Target = Client<Box<dyn Stream>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(all(test, not(feature = "tokio")))]
mod tests {
    use std::io::Cursor;

    use crate::Client;

    #[test]
    fn test_init() {
        let cursor = Cursor::new(Vec::<u8>::new());
        let _ = Client::new(cursor);
    }
}
