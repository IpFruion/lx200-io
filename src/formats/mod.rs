pub mod degs;
pub mod hrs;
pub mod mins;
pub mod secs;

use std::str::Utf8Error;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
enum FormatError {
    Utf8(#[from] Utf8Error),
    Num(#[from] Box<dyn std::error::Error>),
    #[error("Boundary reached")]
    BoundaryReached,
}
