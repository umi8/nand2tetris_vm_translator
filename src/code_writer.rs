use std::fs::File;
use std::io::Write;

use crate::arithmetic_writer;
use crate::CommandType;
use crate::my_error::MyError;

pub struct CodeWriter {
    file: File,
    comparison_counter: i32,
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self, MyError> {
        let out = File::create(file_path);

        match out {
            Ok(file) => {
                Ok(CodeWriter {
                    file,
                    comparison_counter: 0,
                })
            }
            Err(e) => Err(MyError::Io(e))
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) -> Result<(), MyError> {
        let (operation, counter) = arithmetic_writer::write(command, self.comparison_counter)?;
        self.comparison_counter = counter;
        write!(&mut self.file, "{}", operation)?;
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        if command == CommandType::PUSH {
            self.push(segment, index)?;
        } else if command == CommandType::POP {
            self.pop(segment, index)?;
        }
        Ok(())
    }

    fn add(&mut self) -> std::io::Result<()> {
        self.binary_operation("+")?;
        Ok(())
    }

    fn sub(&mut self) -> std::io::Result<()> {
        self.binary_operation("-")?;
        Ok(())
    }

    fn neg(&mut self) -> std::io::Result<()> {
        self.unary_operation("-")?;
        Ok(())
    }

    fn eq(&mut self) -> std::io::Result<()> {
        self.comparison("JEQ")?;
        Ok(())
    }

    fn gt(&mut self) -> std::io::Result<()> {
        self.comparison("JGT")?;
        Ok(())
    }

    fn lt(&mut self) -> std::io::Result<()> {
        self.comparison("JLT")?;
        Ok(())
    }

    fn and(&mut self) -> std::io::Result<()> {
        self.binary_operation("&")?;
        Ok(())
    }

    fn or(&mut self) -> std::io::Result<()> {
        self.binary_operation("|")?;
        Ok(())
    }

    fn not(&mut self) -> std::io::Result<()> {
        self.unary_operation("!")?;
        Ok(())
    }

    fn push(&mut self, segment: &str, index: &i32) -> std::io::Result<()> {
        if segment.eq("constant") {
            writeln!(&mut self.file, "@{}", index)?;
            writeln!(&mut self.file, "D=A")?;
            self.set_memory_address_to_stack_pointer()?;
            writeln!(&mut self.file, "M=D")?;
            self.increment_stack_pointer()?;
        } else if segment.eq("pointer") {
            if *index == 0 {
                writeln!(&mut self.file, "@THIS")?;
            } else if *index == 1 {
                writeln!(&mut self.file, "@THAT")?;
            }
            writeln!(&mut self.file, "D=M")?;
            self.set_memory_address_to_stack_pointer()?;
            writeln!(&mut self.file, "M=D")?;
            self.increment_stack_pointer()?;
        } else {
            self.store_address_into_d_register(segment, index)?;
            writeln!(&mut self.file, "A=D")?;
            writeln!(&mut self.file, "D=M")?;
            self.set_memory_address_to_stack_pointer()?;
            writeln!(&mut self.file, "M=D")?;
            self.increment_stack_pointer()?;
        }
        Ok(())
    }

    fn pop(&mut self, segment: &str, index: &i32) -> std::io::Result<()> {
        if segment.eq("pointer") {
            if *index == 0 {
                writeln!(&mut self.file, "@THIS")?;
            } else if *index == 1 {
                writeln!(&mut self.file, "@THAT")?;
            }
            writeln!(&mut self.file, "D=A")?;
            writeln!(&mut self.file, "@R13")?;
            writeln!(&mut self.file, "M=D")?;

            self.peek_value_into_d_register()?;

            writeln!(&mut self.file, "@R13")?;
            writeln!(&mut self.file, "A=M")?;

            writeln!(&mut self.file, "M=D")?;
        } else {
            self.store_address_into_d_register(segment, index)?;

            writeln!(&mut self.file, "@R13")?;
            writeln!(&mut self.file, "M=D")?;

            self.peek_value_into_d_register()?;

            writeln!(&mut self.file, "@R13")?;
            writeln!(&mut self.file, "A=M")?;

            writeln!(&mut self.file, "M=D")?;
        }
        Ok(())
    }

    fn store_address_into_d_register(&mut self, segment: &str, index: &i32) -> std::io::Result<()> {
        if segment.eq("local") {
            writeln!(&mut self.file, "@LCL")?;
            writeln!(&mut self.file, "D=M")?;
        } else if segment.eq("argument") {
            writeln!(&mut self.file, "@ARG")?;
            writeln!(&mut self.file, "D=M")?;
        } else if segment.eq("this") {
            writeln!(&mut self.file, "@THIS")?;
            writeln!(&mut self.file, "D=M")?;
        } else if segment.eq("that") {
            writeln!(&mut self.file, "@THAT")?;
            writeln!(&mut self.file, "D=M")?;
        } else if segment.eq("temp") {
            writeln!(&mut self.file, "@5")?;
            writeln!(&mut self.file, "D=A")?;
        }

        writeln!(&mut self.file, "@{}", index)?;
        writeln!(&mut self.file, "D=D+A")?;
        Ok(())
    }

    fn binary_operation(&mut self, operator: &str) -> std::io::Result<()> {
        self.peek_value_into_d_register()?;
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=M{}D", operator)?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn unary_operation(&mut self, operator: &str) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M={}M", operator)?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn comparison(&mut self, jump_mnemonic: &str) -> std::io::Result<()> {
        self.peek_value_into_d_register()?;
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // D=x-y
        writeln!(&mut self.file, "D=M-D")?;
        // set the destination address if true in A register
        writeln!(&mut self.file, "@COMP{}", self.comparison_counter)?;
        // jump operation
        writeln!(&mut self.file, "D;{}", jump_mnemonic)?;
        // set false
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=0")?;
        // set the destination address of finally process in A register and jump
        writeln!(&mut self.file, "@ENDCOMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "0;JMP")?;
        // define a label if true
        writeln!(&mut self.file, "(COMP{})", self.comparison_counter)?;
        // set true
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=-1")?;
        // define a label for finally process
        writeln!(&mut self.file, "(ENDCOMP{})", self.comparison_counter)?;
        self.increment_stack_pointer()?;
        self.comparison_counter += 1;
        Ok(())
    }

    fn peek_value_into_d_register(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;
        Ok(())
    }

    fn set_memory_address_to_stack_pointer(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "A=M")?;
        Ok(())
    }

    fn increment_stack_pointer(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M+1")?;
        Ok(())
    }

    fn decrement_stack_pointer(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.file, "@SP")?;
        writeln!(&mut self.file, "M=M-1")?;
        Ok(())
    }
}
