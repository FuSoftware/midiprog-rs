use regex::Regex;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;

pub struct Command {
    pub name: String,
    pub parameters: std::collections::HashMap<String, String>,
    pub values: Vec<String>,
}

impl Command {
    pub fn new(name: String) -> Command {
        Command {
            name: name,
            parameters: std::collections::HashMap::new(),
            values: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, key: String, value: String) {
        self.values.push(value.clone());
        self.parameters.insert(key, value);
    }

    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        return self.parameters.get(key);
    }

    pub fn get_parameter_from_index(&self, index: usize) -> Option<&String> {
        return self.values.get(index);
    }

    pub fn has_parameter(&self, key: &str) -> bool {
        return self.parameters.contains_key(key);
    }

    pub fn has_numbered_parameter(&self, parameter: &str, index: usize) -> bool {
        return self.has_parameter(format!("{}_{}", parameter, index).as_str());
    }

    pub fn get_numbered_parameter(&self, parameter: &str, index: usize) -> Option<&String> {
        let s: String = format!("{}_{}", parameter, index);
        return self.get_parameter(s.as_str());
    }
}

impl fmt::Display for Command {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        writeln!(f, "{}", self.name);
        for (k, v) in &self.parameters {
            writeln!(f, "  - {} {}", k, v);
        }
        write!(f, "{}", "")
    }
}

pub struct CommandParser {}

impl CommandParser {
    pub fn get_commands_file_name(file: String) -> Result<Vec<String>, std::io::Error> {
        return Ok(CommandParser::get_commands_file(File::open(file)?));
    }

    pub fn get_commands_file(mut file: File) -> Vec<String> {
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        return CommandParser::get_commands(contents);
    }

    pub fn get_commands(content: String) -> Vec<String> {
        let mut current_command: String = String::new();
        let mut commands: Vec<String> = Vec::new();
        let lines = content.split("\n");

        for l in lines {
            let line = l.trim();

            if line.starts_with("-") {
                current_command.push_str(line);
            } else if current_command == "" {
                current_command = String::from(line);
            } else {
                commands.push(current_command.clone());
                current_command = String::from(line);
            }
        }

        if current_command != "" {
            commands.push(current_command);
        }

        return commands;
    }

    pub fn parse_commands_file_name(file: &str) -> Result<Vec<Command>, std::io::Error> {
        let commands = CommandParser::get_commands_file(File::open(file)?);
        let r = CommandParser::parse_commands(commands.as_slice());
        return Ok(r);
    }

    pub fn parse_commands_file(file: File) -> Vec<Command> {
        let commands = CommandParser::get_commands_file(file);
        let r = CommandParser::parse_commands(&commands);
        return r;
    }

    pub fn parse_commands<T: AsRef<str>>(content: &[T]) -> Vec<Command> {
        let mut v: Vec<Command> = Vec::new();
        for c in content {
            match CommandParser::parse_command(c.as_ref().to_string()) {
                Some(x) => v.push(x),
                None => println!("Found invalid command : {}", c.as_ref().to_string()),
            }
        }
        return v;
    }

    pub fn parse_command(content: String) -> Option<Command> {
        lazy_static! {
            static ref RE_PARAMS: Regex = Regex::new("-(\\S+) \"([^\"]+)\"").expect("Failed to create Regex for command params");
            static ref RE_COMM: Regex = Regex::new("([^\"\\s]+)(\\s?+)-").expect("Failed to create Regex for command name");
        }

        let mut command: Command;

        if RE_COMM.is_match(content.as_str()) {
            command = Command::new(String::from(
                RE_COMM
                    .captures(content.as_str())
                    .expect("Match not found")
                    .get(1)
                    .expect("Command name not found")
                    .as_str(),
            ));
        } else {
            return None;
        }

        let mut count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for cap in RE_PARAMS.captures_iter(content.as_str()) {
            let is_numbered: bool = cap.get(1).unwrap().as_str().starts_with("@");
            let mut p = String::from(cap.get(1).unwrap().as_str());

            if is_numbered {
                if !count.contains_key(&p) {
                    count.insert(p.clone(), 0);
                }

                let c: &mut usize = count.get_mut(&p).expect("Expected parameter count");
                //println!("{} {}", p, *c);
                p.remove(0);
                p.push('_');
                p.push_str(format!("{}", *c).as_str());
                *c = *c + 1;
            }

            command.add_parameter(p, String::from(cap.get(2).unwrap().as_str()));
        }

        return Some(command);
    }
}
