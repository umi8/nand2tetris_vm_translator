use std::{error, fmt};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Fmt(std::fmt::Error),
    IllegalArgument(IllegalArgumentError)
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Fmt(cause) => write!(f, "Format Error : {}", cause),
            MyError::IllegalArgument(cause) => write!(f, "Illegal Argument Error : {}", cause),
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

impl error::Error for MyError {}

#[derive(Debug, Clone)]
pub struct IllegalArgumentError;

impl fmt::Display for IllegalArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Illegal Argument")
    }
}

impl error::Error for IllegalArgumentError {}
