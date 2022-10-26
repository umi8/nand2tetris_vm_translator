use std::path::Path;

use walkdir::{DirEntry, WalkDir};

use crate::arithmetic_type::ArithmeticType;
use crate::code_writer::CodeWriter;
use crate::command_type::CommandType;
use crate::my_error::{IllegalArgumentError, MyError};
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
    let arg = "vm";
    let path = Path::new(arg);
    let files: Vec<DirEntry> = extract_files_from(path);

    let mut code_writer = CodeWriter::new("File.asm")?;
    code_writer.write_init()?;
    for file in files {
        parse(&mut code_writer, file.path().to_str().unwrap())?;
    }

    println!("File translation succeeded: File.asm");
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

fn create_output_file_name(path: &Path) -> Result<String, IllegalArgumentError> {
    if path.is_file() && path.extension().unwrap() == "vm" {
        let file_name = path.file_stem().unwrap().to_string_lossy();
        return Ok(format!("{}.asm", file_name));
    }

    if path.is_dir() {
        let dir_name = path.file_name().unwrap().to_string_lossy();
        return Ok(format!("{}/{}.asm", dir_name, dir_name));
    }

    Err(IllegalArgumentError)
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::create_output_file_name;

    #[test]
    fn if_path_is_vm_file() {
        assert_eq!(
            "file_name_test.asm",
            create_output_file_name(Path::new("tests/resources/file_name_test.vm")).unwrap()
        );
    }

    #[test]
    fn if_path_is_dir() {
        assert_eq!(
            "dir_test/dir_test.asm",
            create_output_file_name(Path::new("tests/resources/dir_test")).unwrap()
        );
    }

    #[test]
    fn if_path_is_not_allowed_file() {
        assert!(create_output_file_name(Path::new("tests/resources/file_name_test.hoge")).is_err());
    }
}
