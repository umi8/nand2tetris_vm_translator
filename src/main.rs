use crate::arithmetic_type::ArithmeticType;
use crate::code_writer::CodeWriter;
use crate::command_type::CommandType;
use crate::my_error::MyError;
use crate::parser::Parser;
use crate::segment::Segment;

mod command_type;
mod code_writer;
mod parser;
mod arithmetic_type;
mod arithmetic_writer;
mod my_error;
mod push_pop_writer;
mod segment;

fn main() -> Result<(), MyError> {
    let mut parser = Parser::new("File.vsm")?;

    let mut code_writer = CodeWriter::new("File.asm")?;

    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::ARITHMETIC => {
                code_writer.write_arithmetic(
                    ArithmeticType::from(&parser.arg1()).unwrap()
                )?
            }
            CommandType::PUSH | CommandType::POP => {
                code_writer.write_push_pop(
                    parser.command_type(),
                    Segment::from(&parser.arg1()).unwrap(),
                    &parser.arg2().parse::<i32>().unwrap(),
                )?
            }
        }
    }

    println!("File translation succeeded: File.asm");
    Ok(())
}
