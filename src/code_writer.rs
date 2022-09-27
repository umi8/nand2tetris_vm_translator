use std::fs::File;
use std::io::Write;

use crate::{arithmetic_writer, CommandType};

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

    pub fn write_arithmetic(&mut self, command: &str) -> std::io::Result<()> {
        writeln!(&mut self.file, "{}", arithmetic_writer::add(self.stack_pointer))?;
        self.stack_pointer -= 1;
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        writeln!(&mut self.file, "@{}\nD=A\n@{}\nM=D", index, self.stack_pointer)?;
        self.stack_pointer += 1;
        Ok(())
    }
}
