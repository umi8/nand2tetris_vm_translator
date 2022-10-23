use std::fs::read_dir;

use crate::arithmetic_type::ArithmeticType;
use crate::code_writer::CodeWriter;
use crate::command_type::CommandType;
use crate::my_error::MyError;
use crate::parser::Parser;
use crate::segment::Segment;

mod arithmetic_type;
mod arithmetic_writer;
mod code_writer;
mod command_type;
mod my_error;
mod parser;
mod push_pop_writer;
mod return_writer;
mod segment;

fn main() -> Result<(), MyError> {
    let mut code_writer = CodeWriter::new("File.asm")?;
    code_writer.write_init()?;
    for entry in read_dir("vm")? {
        parse(&mut code_writer, &entry?.path().to_string_lossy())?;
    }
    println!("File translation succeeded: File.asm");
    Ok(())
}

fn parse(code_writer: &mut CodeWriter, file_path: &str) -> Result<(), MyError> {
    let mut parser = Parser::new(file_path)?;
    while parser.has_more_commands() {
        match parser.command_type()? {
            CommandType::Arithmetic => {
                code_writer.write_arithmetic(ArithmeticType::from(parser.arg1()?)?)?
            }
            CommandType::Push | CommandType::Pop => code_writer.write_push_pop(
                parser.command_type()?,
                Segment::from(parser.arg1()?)?,
                &parser.arg2()?,
            )?,
            CommandType::Label => code_writer.write_label(parser.arg1()?)?,
            CommandType::IfGoto => code_writer.write_if(parser.arg1()?)?,
            CommandType::Goto => code_writer.write_goto(parser.arg1()?)?,
            CommandType::Function => code_writer.write_function(parser.arg1()?, parser.arg2()?)?,
            CommandType::Return => code_writer.write_return()?,
            CommandType::Call => code_writer.write_call(parser.arg1()?, parser.arg2()?)?,
        }
    }
    Ok(())
}
