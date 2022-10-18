use std::fs::File;
use std::io::{Error, Write};

use crate::arithmetic_type::ArithmeticType;
use crate::my_error::MyError;
use crate::segment::Segment;
use crate::CommandType;
use crate::{arithmetic_writer, push_pop_writer};

pub struct CodeWriter {
    file: File,
    comparison_counter: i32,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self, Error> {
        let out = File::create(file_path)?;
        Ok(CodeWriter {
            file: out,
            comparison_counter: 0,
        })
    }

    pub fn write_arithmetic(&mut self, arithmetic_command: ArithmeticType) -> Result<(), MyError> {
        write!(
            &mut self.file,
            "{}",
            arithmetic_writer::write(&arithmetic_command, self.comparison_counter)?
        )?;
        if arithmetic_command.is_comparison_type() {
            self.comparison_counter += 1;
        }
        Ok(())
    }

    pub fn write_push_pop(
        &mut self,
        command: CommandType,
        segment: Segment,
        index: &i32,
    ) -> Result<(), MyError> {
        write!(
            &mut self.file,
            "{}",
            push_pop_writer::write(command, segment, index)?
        )?;
        Ok(())
    }

    pub fn write_label(&mut self, label: &str) -> Result<(), MyError> {
        writeln!(&mut self.file, "({})", label)?;
        Ok(())
    }

    pub fn write_if(&mut self, label: &str) -> Result<(), MyError> {
        // decrement stack pointer
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M-1")?;
        // set memory address to stack pointer
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;
        // if D != 0 goto label
        writeln!(&mut self.file, "@{}", label)?;
        writeln!(&mut self.file, "D;JNE")?;
        Ok(())
    }

    pub fn write_goto(&mut self, label: &str) -> Result<(), MyError> {
        writeln!(&mut self.file, "@{}", label)?;
        writeln!(&mut self.file, "0;JMP")?;
        Ok(())
    }

    pub fn write_return(&mut self) -> Result<(), MyError> {
        Ok(())
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: i32) -> Result<(), MyError> {
        // declare function label
        writeln!(&mut self.file, "({})", function_name)?;
        // initialize with 0 for the number of local variables
        for _n in 0..num_locals {
            writeln!(&mut self.file, "@0")?;
            writeln!(&mut self.file, "D=A")?;
            writeln!(&mut self.file, "@SP")?;
            writeln!(&mut self.file, "A=M")?;
            writeln!(&mut self.file, "M=D")?;
            writeln!(&mut self.file, "@SP")?;
            writeln!(&mut self.file, "M=M+1")?;
        }
        Ok(())
    }
}
