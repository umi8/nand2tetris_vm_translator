pub struct CodeWriter {
    stack_pointer: i32,
}

impl CodeWriter {
    pub fn new(&mut self) {
        self.stack_pointer = 0;
    }

    pub fn write_arithmetic(command: &str) {}

    pub fn write_push_pop(command: &str, segment: &str, index: i32) {}

}
