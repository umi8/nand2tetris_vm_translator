use crate::my_error::IllegalArgumentError;
use crate::ArithmeticType;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
}

impl CommandType {
    pub fn from(command: &str) -> Result<CommandType, IllegalArgumentError> {
        return match command {
            "push" => Ok(CommandType::PUSH),
            "pop" => Ok(CommandType::POP),
            &_ => {
                if ArithmeticType::exists(command) {
                    Ok(CommandType::ARITHMETIC)
                } else {
                    Err(IllegalArgumentError)
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::command_type::CommandType;

    #[test]
    fn return_type_arithmetic() {
        assert_eq!(CommandType::ARITHMETIC, CommandType::from("add").unwrap())
    }

    #[test]
    fn return_type_push() {
        assert_eq!(CommandType::PUSH, CommandType::from("push").unwrap())
    }

    #[test]
    fn return_type_pop() {
        assert_eq!(CommandType::POP, CommandType::from("pop").unwrap())
    }

    #[test]
    fn return_error() {
        assert!(CommandType::from("subb").is_err())
    }
}
