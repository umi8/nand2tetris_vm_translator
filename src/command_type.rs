#[derive(Debug, PartialEq)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
}

impl CommandType {
    pub fn from(command: &str) -> CommandType {
        return if command.eq("push") {
            CommandType::PUSH
        } else {
            CommandType::ARITHMETIC
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command_type::CommandType;

    #[test]
    fn return_type_arithmetic() {
        assert_eq!(CommandType::ARITHMETIC, CommandType::from("add"))
    }

    #[test]
    fn return_type_push() {
        assert_eq!(CommandType::PUSH, CommandType::from("push"))
    }
}
