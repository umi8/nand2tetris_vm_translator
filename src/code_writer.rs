use std::fs::File;
use std::io::Write;

use crate::{arithmetic_writer, push_pop_writer};
use crate::arithmetic_type::ArithmeticType;
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
        write!(&mut self.file, "{}", arithmetic_writer::write(command, self.comparison_counter)?)?;
        if ArithmeticType::from(command).unwrap().is_comparison_type() {
            self.comparison_counter += 1;
        }
        Ok(())
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: &i32) -> Result<(), MyError> {
        write!(&mut self.file, "{}", push_pop_writer::write(command, segment, index)?)?;
        Ok(())
    }

}
