#[derive(Debug, PartialEq)]
pub enum ArithmeticType {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

impl ArithmeticType {
    pub fn from(command: &str) -> Result<ArithmeticType, &'static str> {
        return match command {
            "add" => Ok(ArithmeticType::ADD),
            "sub" => Ok(ArithmeticType::SUB),
            "neg" => Ok(ArithmeticType::NEG),
            "eq" => Ok(ArithmeticType::EQ),
            "gt" => Ok(ArithmeticType::GT),
            "lt" => Ok(ArithmeticType::LT),
            "and" => Ok(ArithmeticType::AND),
            "or" => Ok(ArithmeticType::OR),
            "not" => Ok(ArithmeticType::NOT),
            &_ => Err("Not type")
        };
    }

    pub fn is_comparison_type(&self) -> bool {
        return match self {
            ArithmeticType::EQ | ArithmeticType::GT | ArithmeticType::LT => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::arithmetic_type::ArithmeticType;
    use crate::arithmetic_type::ArithmeticType::*;

    #[test]
    fn return_add_type() {
        assert_eq!(ADD, ArithmeticType::from("add").unwrap())
    }

    #[test]
    fn return_sub_type() {
        assert_eq!(SUB, ArithmeticType::from("sub").unwrap())
    }

    #[test]
    fn return_neg_type() {
        assert_eq!(NEG, ArithmeticType::from("neg").unwrap())
    }

    #[test]
    fn return_eq_type() {
        assert_eq!(EQ, ArithmeticType::from("eq").unwrap())
    }

    #[test]
    fn return_gt_type() {
        assert_eq!(GT, ArithmeticType::from("gt").unwrap())
    }

    #[test]
    fn return_lt_type() {
        assert_eq!(LT, ArithmeticType::from("lt").unwrap())
    }

    #[test]
    fn return_and_type() {
        assert_eq!(AND, ArithmeticType::from("and").unwrap())
    }

    #[test]
    fn return_or_type() {
        assert_eq!(OR, ArithmeticType::from("or").unwrap())
    }

    #[test]
    fn return_not_type() {
        assert_eq!(NOT, ArithmeticType::from("not").unwrap())
    }

    #[test]
    fn return_error() {
        assert!(ArithmeticType::from("hoge").is_err())
    }
}
