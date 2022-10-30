use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{bail, Context, Result};
use regex::Regex;

use crate::command_type::CommandType;
use crate::illegal_argument_error::IllegalArgumentError;

pub struct Parser {
    reader: BufReader<File>,
    command: String,
}

impl Parser {
    pub fn new(file_path: &str) -> Result<Self> {
        let vm_file = File::open(file_path)?;
        let reader = BufReader::new(vm_file);
        Ok(Parser {
            reader,
            command: String::new(),
        })
    }

    pub fn has_more_commands(&mut self) -> Result<bool> {
        loop {
            let mut buf = String::new();
            return match &self.reader.read_line(&mut buf) {
                Ok(0) => Ok(false),
                Ok(_) => {
                    let re = Regex::new(r"//.*").context("Regular expression is invalid")?;
                    let comments_removed = re.replace_all(&buf, "");
                    let command = &comments_removed.trim();
                    if command.is_empty() {
                        continue;
                    }
                    self.command = command.parse().context("Failed to parse command")?;
                    Ok(true)
                }
                Err(_) => Ok(false),
            };
        }
    }

    pub fn command_type(&self) -> Result<CommandType> {
        let commands: Vec<&str> = self.command.split_whitespace().collect();
        CommandType::from(commands[0])
    }

    pub fn arg1(&self) -> Result<&str> {
        return match self.command_type()? {
            CommandType::Arithmetic => Ok(&self.command),
            CommandType::Push
            | CommandType::Pop
            | CommandType::Label
            | CommandType::IfGoto
            | CommandType::Goto
            | CommandType::Function
            | CommandType::Call => {
                let commands: Vec<&str> = self.command.split_whitespace().collect();
                Ok(commands[1])
            }
            CommandType::Return => bail!(IllegalArgumentError),
        };
    }

    pub fn arg2(&self) -> Result<i32> {
        let commands: Vec<&str> = self.command.split_whitespace().collect();
        Ok(commands[2].parse::<i32>()?)
    }
}
