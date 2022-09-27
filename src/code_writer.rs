use std::fs::File;

use crate::CommandType;

pub struct CodeWriter {
    file: File,
    stack_pointer: i32,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self, &'static str> {
        let out = File::create(file_path);

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

    pub fn write_arithmetic(&self, command: &str) {
        println!("{}", command)
    }

    pub fn write_push_pop(&self, command: CommandType, segment: &str, index: &i32) {
        println!("{},{}", segment, index);
    }
}
