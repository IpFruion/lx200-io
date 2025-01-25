pub mod days;
pub mod degs;
pub mod hrs;
pub mod mins;
pub mod months;
pub mod secs;
pub mod yrs;

use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
enum FormatError {
    Utf8(#[from] Utf8Error),
    Num(#[from] Box<dyn std::error::Error>),
    #[error("Boundary reached")]
    BoundaryReached,
}
