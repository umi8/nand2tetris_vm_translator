use std::path::Path;

use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

use crate::arithmetic_type::ArithmeticType;
use crate::code_writer::CodeWriter;
use crate::command_type::CommandType;
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

fn main() -> Result<()> {
    let arg = "vm";
    let path = Path::new(arg);
    let files: Vec<DirEntry> = extract_files_from(path);

    if files.is_empty() {
        println!("The translation target doesn't exist.");
        return Ok(());
    }

    let output_file_name = create_output_file_name(path);

    let mut code_writer = CodeWriter::new(&output_file_name)?;
    code_writer.write_init()?;
    for file in files {
        parse(&mut code_writer, file.path().to_str().unwrap())?;
    }

    println!("File translation succeeded: {}", output_file_name);
    Ok(())
}

fn extract_files_from(path: &Path) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(is_vm_file)
        .collect()
}

fn is_vm_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".vm"))
        .unwrap_or(false)
}

fn create_output_file_name(path: &Path) -> String {
    if path.is_file() && path.extension().unwrap() == "vm" {
        return String::from(path.with_extension("asm").to_string_lossy());
    }

    let dir = path.to_string_lossy();
    let dir_name = path.file_name().unwrap().to_string_lossy();
    format!("{}/{}.asm", dir, dir_name)
}

fn parse(code_writer: &mut CodeWriter, file_path: &str) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::create_output_file_name;

    #[test]
    fn if_path_is_vm_file() {
        assert_eq!(
            "tests/resources/file_name_test.asm",
            create_output_file_name(Path::new("tests/resources/file_name_test.vm"))
        );
    }

    #[test]
    fn if_path_is_dir() {
        assert_eq!(
            "tests/resources/dir_test/dir_test.asm",
            create_output_file_name(Path::new("tests/resources/dir_test"))
        );
    }
}
