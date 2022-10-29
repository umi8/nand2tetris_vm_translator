use std::fmt::Formatter;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct IllegalArgumentError;

impl fmt::Display for IllegalArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Illegal Argument")
    }
}

impl error::Error for IllegalArgumentError {}
