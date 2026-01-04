use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum QError {
    // Unimplemented,
    NotEnoughArguments,
    InvalidImageType,

    IOError,

    MinifbError,

    ParseIntError,

    FailedToLoadImage,
}

impl From<io::Error> for QError {
    fn from(_: io::Error) -> Self {
        QError::IOError
    }
}

impl From<minifb::Error> for QError {
    fn from(_: minifb::Error) -> Self {
        QError::MinifbError
    }
}

impl From<ParseIntError> for QError {
    fn from(_: ParseIntError) -> Self {
        QError::ParseIntError
    }
}

