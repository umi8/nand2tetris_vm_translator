use anyhow::{bail, Result};

use crate::illegal_argument_error::IllegalArgumentError;
use crate::ArithmeticType;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    IfGoto,
    Goto,
    Function,
    Return,
    Call,
}

impl CommandType {
    pub fn from(command: &str) -> Result<CommandType> {
        match command {
            "push" => Ok(CommandType::Push),
            "pop" => Ok(CommandType::Pop),
            "label" => Ok(CommandType::Label),
            "if-goto" => Ok(CommandType::IfGoto),
            "goto" => Ok(CommandType::Goto),
            "function" => Ok(CommandType::Function),
            "return" => Ok(CommandType::Return),
            "call" => Ok(CommandType::Call),
            &_ => {
                if ArithmeticType::exists(command) {
                    Ok(CommandType::Arithmetic)
                } else {
                    bail!(IllegalArgumentError)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command_type::CommandType;

    #[test]
    fn return_type_arithmetic() {
        assert_eq!(CommandType::Arithmetic, CommandType::from("add").unwrap())
    }

    #[test]
    fn return_type_push() {
        assert_eq!(CommandType::Push, CommandType::from("push").unwrap())
    }

    #[test]
    fn return_type_pop() {
        assert_eq!(CommandType::Pop, CommandType::from("pop").unwrap())
    }

    #[test]
    fn return_type_label() {
        assert_eq!(CommandType::Label, CommandType::from("label").unwrap())
    }

    #[test]
    fn return_type_if_goto() {
        assert_eq!(CommandType::IfGoto, CommandType::from("if-goto").unwrap())
    }

    #[test]
    fn return_type_goto() {
        assert_eq!(CommandType::Goto, CommandType::from("goto").unwrap())
    }

    #[test]
    fn return_type_function() {
        assert_eq!(
            CommandType::Function,
            CommandType::from("function").unwrap()
        )
    }

    #[test]
    fn return_type_return() {
        assert_eq!(CommandType::Return, CommandType::from("return").unwrap())
    }

    #[test]
    fn return_type_call() {
        assert_eq!(CommandType::Call, CommandType::from("call").unwrap())
    }

    #[test]
    fn return_error() {
        assert!(CommandType::from("subb").is_err())
    }
}
