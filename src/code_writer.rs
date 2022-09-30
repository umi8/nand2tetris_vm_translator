use std::fs::File;
use std::io::Write;

use crate::arithmetic_type::ArithmeticType;
use crate::CommandType;

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
        };
        Ok(())
    }

    fn add(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer();
        self.set_memory_address_to_stack_pointer();
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer();
        self.set_memory_address_to_stack_pointer();
        // store the result of add calc in A register
        writeln!(&mut self.file, "M=M+D")?;
        self.increment_stack_pointer();
        Ok(())
    }

    fn sub(&mut self) -> std::io::Result<()> {
        self.decrement_stack_pointer();
        self.set_memory_address_to_stack_pointer();
        // store top of stack value in D register
        writeln!(&mut self.file, "D=M")?;

        self.decrement_stack_pointer();
        self.set_memory_address_to_stack_pointer();
        // store the result of sub calc in A register
        writeln!(&mut self.file, "M=M-D")?;
        self.increment_stack_pointer();
        Ok(())
    }

    fn neg(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn eq(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn gt(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn lt(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn and(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn or(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn not(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        self.push(index);
        Ok(())
    }

    fn push(&mut self, index: &i32) -> std::io::Result<()> {
        // store index in D register
        writeln!(&mut self.file, "@{}", index)?;
        writeln!(&mut self.file, "D=A")?;
        self.set_memory_address_to_stack_pointer();
        // store index in stack[sp]
        writeln!(&mut self.file, "M=D")?;
        self.increment_stack_pointer();
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
