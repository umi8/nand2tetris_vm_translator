use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use regex::Regex;

use crate::command_type::CommandType;
use crate::my_error::IllegalArgumentError;

pub struct Parser {
    reader: BufReader<File>,
    command: String,
}

impl Parser {
    pub fn new(file_path: &str) -> Result<Self, Error> {
        let vm_file = File::open(file_path)?;
        let reader = BufReader::new(vm_file);
        Ok(Parser {
            reader,
            command: String::new(),
        })
    }

    pub fn has_more_commands(&mut self) -> bool {
        loop {
            let mut buf = String::new();
            return match &self.reader.read_line(&mut buf) {
                Ok(0) => false,
                Ok(_) => {
                    let re = Regex::new(r"//.*").unwrap();
                    let comments_removed = re.replace_all(&buf, "");
                    let command = &comments_removed.trim();
                    if command.is_empty() {
                        continue;
                    }
                    self.command = command.parse().unwrap();
                    true
                }
                Err(_) => false
            };
        }
    }

    pub fn command_type(&self) -> Result<CommandType, IllegalArgumentError> {
        let commands: Vec<&str> = self.command.split_whitespace().collect();
        CommandType::from(commands[0])
    }

    pub fn arg1(&self) -> Result<&str, IllegalArgumentError> {
        return match self.command_type()? {
            CommandType::ARITHMETIC => { Ok(&self.command) }
            CommandType::PUSH | CommandType::POP => {
                let commands: Vec<&str> = self.command.split_whitespace().collect();
                Ok(commands[1])
            }
        }
    }

    pub fn arg2(&self) -> &str {
        let commands: Vec<&str> = self.command.split_whitespace().collect();
        commands[2]
    }
}
