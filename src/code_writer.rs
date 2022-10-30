use std::fs::File;
use std::io::Write;

use anyhow::Result;

use crate::arithmetic_type::ArithmeticType;
use crate::segment::Segment;
use crate::{arithmetic_writer, push_pop_writer};
use crate::{return_writer, CommandType};

pub struct CodeWriter {
    file: File,
    label_counter: i32,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self> {
        let out = File::create(file_path)?;
        Ok(CodeWriter {
            file: out,
            label_counter: 0,
        })
    }

    pub fn write_init(&mut self) -> Result<()> {
        // SP = 256
        self.write("@256")?;
        self.write("D=A")?;
        self.write("@SP")?;
        self.write("M=D")?;

        // call Sys.init
        self.write_call("Sys.init", 0)?;
        Ok(())
    }

    pub fn write_arithmetic(&mut self, arithmetic_command: ArithmeticType) -> Result<()> {
        write!(
            &mut self.file,
            "{}",
            arithmetic_writer::write(&arithmetic_command, self.label_counter)?
        )?;
        if arithmetic_command.is_comparison_type() {
            self.label_counter += 1;
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
        self.write(format!("({})", label).as_str())?;
        Ok(())
    }

    pub fn write_goto(&mut self, label: &str) -> Result<()> {
        self.write(format!("@{}", label).as_str())?;
        self.write("0;JMP")?;
        Ok(())
    }

    pub fn write_if(&mut self, label: &str) -> Result<()> {
        // decrement stack pointer
        self.write("@SP")?;
        self.write("M=M-1")?;
        // set memory address to stack pointer
        self.write("@SP")?;
        self.write("A=M")?;
        // store top of stack value in D register
        self.write("D=M")?;
        // if D != 0 goto label
        self.write(format!("@{}", label).as_str())?;
        self.write("D;JNE")?;
        Ok(())
    }

    pub fn write_call(&mut self, function_name: &str, num_args: i32) -> Result<()> {
        // push return-address
        writeln!(&mut self.file, "@return-address{}", self.label_counter)?;
        self.write("D=A")?;
        self.write("@SP")?;
        self.write("A=M")?;
        self.write("M=D")?;
        self.write("@SP")?;
        self.write("M=M+1")?;

        // push LCL
        self.write("@LCL")?;
        self.write("D=M")?;
        self.write("@SP")?;
        self.write("A=M")?;
        self.write("M=D")?;
        self.write("@SP")?;
        self.write("M=M+1")?;

        // push ARG
        self.write("@ARG")?;
        self.write("D=M")?;
        self.write("@SP")?;
        self.write("A=M")?;
        self.write("M=D")?;
        self.write("@SP")?;
        self.write("M=M+1")?;

        // push THIS
        self.write("@THIS")?;
        self.write("D=M")?;
        self.write("@SP")?;
        self.write("A=M")?;
        self.write("M=D")?;
        self.write("@SP")?;
        self.write("M=M+1")?;

        // push THAT
        self.write("@THAT")?;
        self.write("D=M")?;
        self.write("@SP")?;
        self.write("A=M")?;
        self.write("M=D")?;
        self.write("@SP")?;
        self.write("M=M+1")?;

        // ARG = SP-n-5
        self.write("@SP")?;
        self.write("D=M")?;
        self.write(format!("@{}", num_args).as_str())?;
        self.write("D=D-A")?;
        self.write("@5")?;
        self.write("D=D-A")?;
        self.write("@ARG")?;
        self.write("M=D")?;

        // LCL = SP
        self.write("@SP")?;
        self.write("D=M")?;
        self.write("@LCL")?;
        self.write("M=D")?;

        // goto f
        self.write_goto(function_name)?;

        // declare label for return-address
        self.write_label(format!("return-address{}", self.label_counter).as_str())?;
        self.label_counter += 1;
        Ok(())
    }

    pub fn write_return(&mut self) -> Result<()> {
        write!(&mut self.file, "{}", return_writer::write()?)?;
        Ok(())
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: i32) -> Result<()> {
        // declare function label
        self.write_label(function_name)?;
        // initialize with 0 for the number of local variables
        for _n in 0..num_locals {
            self.write("@0")?;
            self.write("D=A")?;
            self.write("@SP")?;
            self.write("A=M")?;
            self.write("M=D")?;
            self.write("@SP")?;
            self.write("M=M+1")?;
        }
        Ok(())
    }

    fn write(&mut self, text: &str) -> Result<()> {
        writeln!(&mut self.file, "{}", text)?;
        Ok(())
    }
}
