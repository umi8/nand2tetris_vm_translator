use anyhow::{bail, Result};

use crate::illegal_argument_error::IllegalArgumentError;

#[derive(Debug, PartialEq, Eq)]
pub enum Segment {
    Constant,
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static,
}

impl Segment {
    pub fn from(segment: &str) -> Result<Segment> {
        match segment {
            "constant" => Ok(Segment::Constant),
            "local" => Ok(Segment::Local),
            "argument" => Ok(Segment::Argument),
            "this" => Ok(Segment::This),
            "that" => Ok(Segment::That),
            "temp" => Ok(Segment::Temp),
            "pointer" => Ok(Segment::Pointer),
            "static" => Ok(Segment::Static),
            &_ => bail!(IllegalArgumentError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::Segment;
    use crate::segment::Segment::*;

    #[test]
    fn return_constant() {
        assert_eq!(Constant, Segment::from("constant").unwrap())
    }

    #[test]
    fn return_local() {
        assert_eq!(Local, Segment::from("local").unwrap())
    }

    #[test]
    fn return_argument() {
        assert_eq!(Argument, Segment::from("argument").unwrap())
    }

    #[test]
    fn return_this() {
        assert_eq!(This, Segment::from("this").unwrap())
    }

    #[test]
    fn return_that() {
        assert_eq!(That, Segment::from("that").unwrap())
    }

    #[test]
    fn return_temp() {
        assert_eq!(Temp, Segment::from("temp").unwrap())
    }

    #[test]
    fn return_pointer() {
        assert_eq!(Pointer, Segment::from("pointer").unwrap())
    }

    #[test]
    fn return_static() {
        assert_eq!(Static, Segment::from("static").unwrap())
    }

    #[test]
    fn error_if_invalid_argument() {
        assert!(Segment::from("hoge").is_err())
    }
}
