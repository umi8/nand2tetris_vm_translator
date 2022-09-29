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
        // decrement sp
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M-1")?;
        // set sp address in A register
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        // decrement sp
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M-1")?;
        // set sp address in A register
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        // store the result of add calc in A register
        writeln!(&mut self.file, "M=M+D")?;
        // increment sp
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        // store index in D register
        writeln!(&mut self.file, "@{}", index)?;
        writeln!(&mut self.file, "D=A")?;
        // set sp address in A register
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        // store index in stack[sp]
        writeln!(&mut self.file, "M=D")?;
        // increment sp
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;
        Ok(())
    }
}
