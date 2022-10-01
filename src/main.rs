use crate::code_writer::CodeWriter;
use crate::command_type::CommandType;
use crate::parser::Parser;

mod command_type;
mod code_writer;
mod parser;
mod arithmetic_type;

fn main() -> std::io::Result<()> {
    let mut parser = match Parser::new("File.vm") {
        Ok(parser) => parser,
        Err(why) => panic!("couldn't parse: {}", why)
    };

    let mut code_writer = match CodeWriter::new("File.asm") {
        Ok(writer) => writer,
        Err(why) => panic!("Couldn't create file: {}", why)
    };

    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::ARITHMETIC => {
                code_writer.write_arithmetic(&parser.arg1())?
            }
            CommandType::PUSH => {
                code_writer.write_push_pop(
                    CommandType::PUSH,
                    &parser.arg1(),
                    &parser.arg2().parse::<i32>().unwrap(),
                )?
            }
        }
    }

    println!("File compilation succeeded: Prog.hack");
    Ok(())
}
