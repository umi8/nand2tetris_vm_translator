use std::fs::File;
use std::io::Write;

use crate::arithmetic_type::ArithmeticType;
use crate::CommandType;

pub struct CodeWriter {
    file: File,
    comparison_counter: i32
}

impl CodeWriter {
    pub fn new(file_path: &str) -> Result<Self, &'static str> {
        let out = File::create(file_path);

        match out {
            Ok(file) => {
                Ok(CodeWriter { file, comparison_counter: 0 })
            }
            Err(_) => Err("file error"),
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) -> std::io::Result<()> {
        match ArithmeticType::from(command).unwrap() {
            ArithmeticType::ADD => self.add(),
            ArithmeticType::SUB => self.sub(),
            ArithmeticType::NEG => self.neg(),
            ArithmeticType::EQ => self.eq(),
            ArithmeticType::GT => self.gt(),
            ArithmeticType::LT => self.lt(),
            ArithmeticType::AND => self.and(),
            ArithmeticType::OR => self.or(),
            ArithmeticType::NOT => self.not()
        }?;
        Ok(())
    }

    fn add(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store the result of add calc in A register
        writeln!(&mut self.file, "M=M+D")?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn sub(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store the result of sub calc in A register
        writeln!(&mut self.file, "M=M-D")?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn neg(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // reverse sign
        writeln!(&mut self.file, "M=-M")?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn eq(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;

        writeln!(&mut self.file, "D=M-D")?;
        writeln!(&mut self.file, "@COMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "D;JEQ")?;
        // set false
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=0")?;

        writeln!(&mut self.file, "@ENDCOMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "0;JMP")?;

        writeln!(&mut self.file, "(COMP{})", self.comparison_counter)?;
        self.set_memory_address_to_stack_pointer()?;
        // set true
        writeln!(&mut self.file, "M=-1")?;

        writeln!(&mut self.file, "(ENDCOMP{})", self.comparison_counter)?;
        self.increment_stack_pointer()?;
        self.comparison_counter += 1;
        Ok(())
    }

    fn gt(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;

        // D = x - y
        writeln!(&mut self.file, "D=M-D")?;
        writeln!(&mut self.file, "@COMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "D;JGT")?;
        // set false
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=0")?;

        writeln!(&mut self.file, "@ENDCOMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "0;JMP")?;

        writeln!(&mut self.file, "(COMP{})", self.comparison_counter)?;
        self.set_memory_address_to_stack_pointer()?;
        // set true
        writeln!(&mut self.file, "M=-1")?;

        writeln!(&mut self.file, "(ENDCOMP{})", self.comparison_counter)?;
        self.increment_stack_pointer()?;
        self.comparison_counter += 1;
        Ok(())
    }

    fn lt(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;

        // D = x - y
        writeln!(&mut self.file, "D=M-D")?;
        writeln!(&mut self.file, "@COMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "D;JLT")?;
        // set false
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=0")?;

        writeln!(&mut self.file, "@ENDCOMP{}", self.comparison_counter)?;
        writeln!(&mut self.file, "0;JMP")?;

        writeln!(&mut self.file, "(COMP{})", self.comparison_counter)?;
        self.set_memory_address_to_stack_pointer()?;
        // set true
        writeln!(&mut self.file, "M=-1")?;

        writeln!(&mut self.file, "(ENDCOMP{})", self.comparison_counter)?;
        self.increment_stack_pointer()?;
        self.comparison_counter += 1;
        Ok(())
    }

    fn and(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store the result of and operation in A register
        writeln!(&mut self.file, "M=D&M")?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn or(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store the result of or operation in A register
        writeln!(&mut self.file, "M=D|M")?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    fn not(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer()?;
        self.set_memory_address_to_stack_pointer()?;
        // store the result of not operation in A register
        writeln!(&mut self.file, "M=!M")?;
        self.increment_stack_pointer()?;
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        self.push(index)?;
        Ok(())
    }

    fn push(&mut self, index: &i32) -> std::io::Result<()> {
        // store index in D register
        writeln!(&mut self.file, "@{}", index)?;
        writeln!(&mut self.file, "D=A")?;
        self.set_memory_address_to_stack_pointer()?;
        // store index in stack[sp]
        writeln!(&mut self.file, "M=D")?;
        self.increment_stack_pointer()?;
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
