use anyhow::{bail, Result};

use crate::illegal_argument_error::IllegalArgumentError;

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticType {
    Add,
    Sub,
    Neg,
    EQ,
    GT,
    LT,
    And,
    OR,
    Not,
}

impl ArithmeticType {
    pub fn from(command: &str) -> Result<ArithmeticType> {
        match command {
            "add" => Ok(ArithmeticType::Add),
            "sub" => Ok(ArithmeticType::Sub),
            "neg" => Ok(ArithmeticType::Neg),
            "eq" => Ok(ArithmeticType::EQ),
            "gt" => Ok(ArithmeticType::GT),
            "lt" => Ok(ArithmeticType::LT),
            "and" => Ok(ArithmeticType::And),
            "or" => Ok(ArithmeticType::OR),
            "not" => Ok(ArithmeticType::Not),
            &_ => bail!(IllegalArgumentError),
        }
    }

    pub fn exists(command: &str) -> bool {
        ArithmeticType::from(command).is_ok()
    }

    pub fn is_comparison_type(&self) -> bool {
        matches!(
            self,
            ArithmeticType::EQ | ArithmeticType::GT | ArithmeticType::LT
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::arithmetic_type::ArithmeticType;
    use crate::arithmetic_type::ArithmeticType::*;

    #[test]
    fn return_add_type() {
        assert_eq!(Add, ArithmeticType::from("add").unwrap())
    }

    #[test]
    fn return_sub_type() {
        assert_eq!(Sub, ArithmeticType::from("sub").unwrap())
    }

    #[test]
    fn return_neg_type() {
        assert_eq!(Neg, ArithmeticType::from("neg").unwrap())
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
        assert_eq!(And, ArithmeticType::from("and").unwrap())
    }

    #[test]
    fn return_or_type() {
        assert_eq!(OR, ArithmeticType::from("or").unwrap())
    }

    #[test]
    fn return_not_type() {
        assert_eq!(Not, ArithmeticType::from("not").unwrap())
    }

    #[test]
    fn return_error() {
        assert!(ArithmeticType::from("hoge").is_err())
    }

    #[test]
    fn true_if_command_exists() {
        assert!(ArithmeticType::exists("add"))
    }
}
