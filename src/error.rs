#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error during encoding or decoding {0}")]
    Format(String),
}

impl<I> From<nom::Err<nom::error::Error<I>>> for Error {
    fn from(value: nom::Err<nom::error::Error<I>>) -> Self {
        match value {
            nom::Err::Incomplete(needed) => Error::Format(format!(
                "needs {}",
                match needed {
                    nom::Needed::Unknown => -1,
                    nom::Needed::Size(non_zero) => non_zero.get() as isize,
                }
            )),
            nom::Err::Error(err) => Error::Format(format!("{:?}", err.code)),
            nom::Err::Failure(err) => Error::Format(format!("{:?}", err.code)),
        }
    }
}
