use std::result;
use std::io::Error as IoError;
use std::convert::From;
use std::string::FromUtf8Error;
use core::num::ParseIntError;
use mmap::MapError;

use self::Error::{
    UnexpectedError,
    Io,
    Map
};

#[derive(Debug)]
pub enum Error {
    RootRequired,
    UnexpectedError,
    UnsupportedHardware,
    UnconnectedPin,
    Map(MapError),
    Io(IoError),
}

pub type Result<T> = result::Result<T, Error>;

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Io(err)
    }
}

impl From<MapError> for Error {
    fn from(err: MapError) -> Error {
        Map(err)
    }
}


impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        UnexpectedError
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        UnexpectedError
    }
}
