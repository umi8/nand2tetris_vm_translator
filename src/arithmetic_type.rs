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
    pub fn from(command: &str) -> Option<ArithmeticType> {
        return match command {
            "add" => Some(ArithmeticType::ADD),
            "sub" => Some(ArithmeticType::SUB),
            "neg" => Some(ArithmeticType::NEG),
            "eq" => Some(ArithmeticType::EQ),
            "gt" => Some(ArithmeticType::GT),
            "lt" => Some(ArithmeticType::LT),
            "and" => Some(ArithmeticType::AND),
            "or" => Some(ArithmeticType::OR),
            "not" => Some(ArithmeticType::NOT),
            &_ => None
        };
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
    fn return_none() {
        assert_eq!(None, ArithmeticType::from("hoge"))
    }
}
