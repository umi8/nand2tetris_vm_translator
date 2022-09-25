use std::fs::File;
use std::io::BufReader;

pub struct CodeWriter {
    file: File,
    stack_pointer: i32,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self, &'static str> {
        let out = File::open(file_path);

        match out {
            Ok(file) => {
                Ok(CodeWriter {
                    file,
                    stack_pointer: 256,
                })
            }
            Err(_) => Err("file error"),
        }
    }

    pub fn write_arithmetic(command: &str) {}

    pub fn write_push_pop(command: &str, segment: &str, index: i32) {}

}
