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

    JPEGError,
    NoMetadata,
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

impl From<jpeg_decoder::Error> for QError {
    fn from(_: jpeg_decoder::Error) -> Self {
        QError::JPEGError
    }
}
