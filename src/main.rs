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
mod segment;

fn main() -> Result<(), MyError> {
    let mut parser = Parser::new("File.vsm")?;

    let mut code_writer = CodeWriter::new("File.asm")?;

    while parser.has_more_commands() {
        match parser.command_type()? {
            CommandType::Arithmetic => {
                code_writer.write_arithmetic(ArithmeticType::from(parser.arg1()?)?)?
            }
            CommandType::Push | CommandType::Pop => code_writer.write_push_pop(
                parser.command_type()?,
                Segment::from(parser.arg1()?)?,
                &parser.arg2().parse::<i32>()?,
            )?,
        }
    }

    println!("File translation succeeded: File.asm");
    Ok(())
}
