extern crate byteorder;

mod util;

pub mod format;
pub mod mixer;
pub mod module;

use std::error;
use std::fmt;
use std::io;

pub const MAX_RATE: i32 = 96000;
pub const MIN_RATE: i32 = 4000;
pub const MIN_BPM : i32 = 20;
// frame rate = (50 * bpm / 125) Hz
// frame size = (sampling rate * channels * size) / frame rate
pub const MAX_FRAMESIZE: usize = (5 * MAX_RATE * 2 / MIN_BPM) as usize;
pub const MAX_KEYS: usize = 128;


#[derive(Debug)]
pub enum Error {
    Format(&'static str),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Format(descr) => write!(f, "{}", descr),
            Error::Io(ref err)   => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Format(_)   => "Unsupported module format",
            Error::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            _                  => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

