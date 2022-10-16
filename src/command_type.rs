use crate::my_error::IllegalArgumentError;
use crate::ArithmeticType;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
}

impl CommandType {
    pub fn from(command: &str) -> Result<CommandType, IllegalArgumentError> {
        match command {
            "push" => Ok(CommandType::Push),
            "pop" => Ok(CommandType::Pop),
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
    fn return_error() {
        assert!(CommandType::from("subb").is_err())
    }
}
