use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

use crate::command_type::CommandType;

pub struct Parser {
    reader: BufReader<File>,
    command: String,
}

impl Parser {
    pub fn new(file_path: &str) -> Result<Self, &'static str> {
        let asm = File::open(file_path);

        match asm {
            Ok(file) => {
                let reader = BufReader::new(file);
                Ok(Parser {
                    reader,
                    command: "".parse().unwrap(),
                })
            }
            Err(_) => Err("file error"),
        }
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

    pub fn command_type(&self) -> CommandType {
        let commands: Vec<&str> = self.command.split_whitespace().collect();
        CommandType::from(commands[0])
    }

    pub fn arg1(&self) -> &str {
        return match self.command_type() {
            CommandType::ARITHMETIC => { &self.command }
            CommandType::PUSH => {
                let commands: Vec<&str> = self.command.split_whitespace().collect();
                commands[1]
            }
        }
    }

    pub fn arg2(&self) -> &str {
        let commands: Vec<&str> = self.command.split_whitespace().collect();
        commands[2]
    }
}
