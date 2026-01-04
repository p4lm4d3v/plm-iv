use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum QError {
    IncorrectUsageOfProgram,
    WrongImageType,
    IO(io::Error),
    Minifb(minifb::Error),
    ParseIntError(ParseIntError),
}

impl From<io::Error> for QError {
    fn from(e: io::Error) -> Self {
        QError::IO(e)
    }
}

impl From<minifb::Error> for QError {
    fn from(e: minifb::Error) -> Self {
        QError::Minifb(e)
    }
}

impl From<ParseIntError> for QError {
    fn from(e: ParseIntError) -> Self {
        QError::ParseIntError(e)
    }
}