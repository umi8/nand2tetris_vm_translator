use std::{error, fmt};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Fmt(std::fmt::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Fmt(cause) => write!(f, "Format Error : {}", cause),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(e: std::io::Error) -> Self {
        MyError::Io(e)
    }
}

impl From<std::fmt::Error> for MyError {
    fn from(e: std::fmt::Error) -> Self {
        MyError::Fmt(e)
    }
}

impl error::Error for MyError {}
