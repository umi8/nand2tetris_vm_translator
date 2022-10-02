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

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> std::io::Result<()> {
        self.push(index)?;
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

    fn push(&mut self, index: &i32) -> std::io::Result<()> {
        writeln!(&mut self.file, "@{}", index)?;
        writeln!(&mut self.file, "D=A")?;
        self.set_memory_address_to_stack_pointer()?;
        writeln!(&mut self.file, "M=D")?;
        self.increment_stack_pointer()?;
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
