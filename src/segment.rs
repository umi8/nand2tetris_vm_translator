#[derive(Debug, PartialEq)]
pub enum Segment {
    CONSTANT,
    LOCAL,
    ARGUMENT,
    THIS,
    THAT,
    TEMP,
    POINTER,
}

impl Segment {
    pub fn from(segment: &str) -> Result<Self, &'static str> {
        return match segment {
            "constant" => Ok(Segment::CONSTANT),
            "local" => Ok(Segment::LOCAL),
            "argument" => Ok(Segment::ARGUMENT),
            "this" => Ok(Segment::THIS),
            "that" => Ok(Segment::THAT),
            "temp" => Ok(Segment::TEMP),
            "pointer" => Ok(Segment::POINTER),
            &_ => Err("Illegal Argument")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::segment::Segment;
    use crate::segment::Segment::*;

    #[test]
    fn return_constant() {
        assert_eq!(CONSTANT, Segment::from("constant").unwrap())
    }

    #[test]
    fn return_local() {
        assert_eq!(LOCAL, Segment::from("local").unwrap())
    }

    #[test]
    fn return_argument() {
        assert_eq!(ARGUMENT, Segment::from("argument").unwrap())
    }

    #[test]
    fn return_this() {
        assert_eq!(THIS, Segment::from("this").unwrap())
    }

    #[test]
    fn return_that() {
        assert_eq!(THAT, Segment::from("that").unwrap())
    }

    #[test]
    fn return_temp() {
        assert_eq!(TEMP, Segment::from("temp").unwrap())
    }

    #[test]
    fn return_pointer() {
        assert_eq!(POINTER, Segment::from("pointer").unwrap())
    }

    #[test]
    fn error_if_invalid_argument() {
        assert!(Segment::from("hoge").is_err())
    }
}
