use std::fmt::Formatter;
use std::num::ParseIntError;
use std::{error, fmt};

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Fmt(std::fmt::Error),
    IllegalArgument(IllegalArgumentError),
    ParseInt(ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Fmt(cause) => write!(f, "Format Error : {}", cause),
            MyError::IllegalArgument(cause) => write!(f, "Illegal Argument Error : {}", cause),
            MyError::ParseInt(cause) => {
                write!(f, "Parse Int Error: {}", cause)
            }
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

impl From<IllegalArgumentError> for MyError {
    fn from(e: IllegalArgumentError) -> Self {
        MyError::IllegalArgument(e)
    }
}

impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::ParseInt(e)
    }
}

impl error::Error for MyError {}

#[derive(Debug, Clone)]
pub struct IllegalArgumentError;

impl fmt::Display for IllegalArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Illegal Argument")
    }
}

impl error::Error for IllegalArgumentError {}
