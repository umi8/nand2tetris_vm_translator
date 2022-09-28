use std::fs::File;
use std::io::Write;

use crate::{arithmetic_writer, CommandType};

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self, &'static str> {
        let out = File::create(file_path);

        match out {
            Ok(file) => {
                Ok(CodeWriter { file })
            }
            Err(_) => Err("file error"),
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) -> std::io::Result<()> {
        writeln!(&mut self.file, "{}", arithmetic_writer::add())?;
        self.write_stack_pointer()?;
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        writeln!(&mut self.file, "@{}\nD=A\n@SP\nM=D", index)?;
        self.write_stack_pointer()?;
        Ok(())
    }

    fn write_stack_pointer(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.file, "@SP\nD=A\n@0\nM=D")?;
        Ok(())
    }
}
