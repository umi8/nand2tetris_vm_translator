use anyhow::Result;
use std::fs::File;
use std::io::Write;

use crate::arithmetic_type::ArithmeticType;
use crate::segment::Segment;
use crate::{arithmetic_writer, push_pop_writer};
use crate::{return_writer, CommandType};

pub struct CodeWriter {
    file: File,
    comparison_counter: i32,
    return_address_counter: i32,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self> {
        let out = File::create(file_path)?;
        Ok(CodeWriter {
            file: out,
            comparison_counter: 0,
            return_address_counter: 0,
        })
    }

    pub fn write_init(&mut self) -> Result<()> {
        // SP = 256
        writeln!(&mut self.file, "@256")?;
        writeln!(&mut self.file, "D=A")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=D")?;

        // call Sys.init
        self.write_call("Sys.init", 0)?;
        Ok(())
    }

    pub fn write_arithmetic(&mut self, arithmetic_command: ArithmeticType) -> Result<()> {
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
    ) -> Result<()> {
        write!(
            &mut self.file,
            "{}",
            push_pop_writer::write(command, segment, index)?
        )?;
        Ok(())
    }

    pub fn write_label(&mut self, label: &str) -> Result<()> {
        writeln!(&mut self.file, "({})", label)?;
        Ok(())
    }

    pub fn write_goto(&mut self, label: &str) -> Result<()> {
        writeln!(&mut self.file, "@{}", label)?;
        writeln!(&mut self.file, "0;JMP")?;
        Ok(())
    }

    pub fn write_if(&mut self, label: &str) -> Result<()> {
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

    pub fn write_call(&mut self, function_name: &str, num_args: i32) -> Result<()> {
        // push return-address
        writeln!(
            &mut self.file,
            "@return-address{}",
            self.return_address_counter
        )?;
        writeln!(&mut self.file, "D=A")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        writeln!(&mut self.file, "M=D")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;

        // push LCL
        writeln!(&mut self.file, "@LCL")?;
        writeln!(&mut self.file, "D=M")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        writeln!(&mut self.file, "M=D")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;

        // push ARG
        writeln!(&mut self.file, "@ARG")?;
        writeln!(&mut self.file, "D=M")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        writeln!(&mut self.file, "M=D")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;

        // push THIS
        writeln!(&mut self.file, "@THIS")?;
        writeln!(&mut self.file, "D=M")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        writeln!(&mut self.file, "M=D")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;

        // push THAT
        writeln!(&mut self.file, "@THAT")?;
        writeln!(&mut self.file, "D=M")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        writeln!(&mut self.file, "M=D")?;
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;

        // ARG = SP-n-5
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "D=M")?;
        writeln!(&mut self.file, "@{}", num_args)?;
        writeln!(&mut self.file, "D=D-A")?;
        writeln!(&mut self.file, "@5")?;
        writeln!(&mut self.file, "D=D-A")?;
        writeln!(&mut self.file, "@ARG")?;
        writeln!(&mut self.file, "M=D")?;

        // LCL = SP
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "D=M")?;
        writeln!(&mut self.file, "@LCL")?;
        writeln!(&mut self.file, "M=D")?;

        // goto f
        writeln!(&mut self.file, "@{}", function_name)?;
        writeln!(&mut self.file, "0;JMP")?;

        // declare label for return-address
        writeln!(
            &mut self.file,
            "(return-address{})",
            self.return_address_counter
        )?;
        self.return_address_counter += 1;
        Ok(())
    }

    pub fn write_return(&mut self) -> Result<()> {
        write!(&mut self.file, "{}", return_writer::write()?)?;
        Ok(())
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: i32) -> Result<()> {
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
