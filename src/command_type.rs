use crate::my_error::IllegalArgumentError;
use crate::ArithmeticType;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Ifgoto,
}

impl CommandType {
    pub fn from(command: &str) -> Result<CommandType, IllegalArgumentError> {
        match command {
            "push" => Ok(CommandType::Push),
            "pop" => Ok(CommandType::Pop),
            "label" => Ok(CommandType::Label),
            "if-goto" => Ok(CommandType::Ifgoto),
            &_ => {
                if ArithmeticType::exists(command) {
                    Ok(CommandType::Arithmetic)
                } else {
                    Err(IllegalArgumentError)
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
    fn return_type_ifgoto() {
        assert_eq!(CommandType::Ifgoto, CommandType::from("if-goto").unwrap())
    }

    #[test]
    fn return_error() {
        assert!(CommandType::from("subb").is_err())
    }
}
