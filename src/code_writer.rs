pub struct CodeWriter {
    stack_pointer: i32,
}

impl CodeWriter {
    pub fn new(&mut self) {
        self.stack_pointer = 0;
    }

    pub fn write_arithmetic(command: &str) {}

    pub fn write_push_pop(command: &str, segment: &str, index: i32) {}

    fn add(&mut self) -> String {
        let first = self.stack_pointer - 2;
        let second = self.stack_pointer - 1;
        self.stack_pointer -= 1;
        format!("@{}\nD=M\n@{}\nM=M+D", second, first)
    }

    fn sub(&mut self) -> String {
        let first = self.stack_pointer - 2;
        let second = self.stack_pointer - 1;
        self.stack_pointer -= 1;
        format!("@{}\nD=M\n@{}\nM=M-D", second, first)
    }
}

#[cfg(test)]
mod tests {
    use crate::code_writer::CodeWriter;

    #[test]
    fn can_add() {
        let mut code_writer = CodeWriter { stack_pointer: 258 };
        assert_eq!(code_writer.add(), "@257\nD=M\n@256\nM=M+D");
        assert_eq!(code_writer.stack_pointer, 257)
    }

    #[test]
    fn can_sub() {
        let mut code_writer = CodeWriter { stack_pointer: 258 };
        assert_eq!(code_writer.sub(), "@257\nD=M\n@256\nM=M-D");
        assert_eq!(code_writer.stack_pointer, 257)
    }
}
