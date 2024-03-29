use super::command_parser::*;
use super::midi_command::*;
use super::synth::Synth;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use derive_more::*;

#[derive(Debug, From)]
pub enum ConfigError {
    IOError(std::io::Error),
    JsonError(json::Error),
    CommonError(String)
}

#[derive(Default)]
pub struct Config {
    synths: HashMap<String, Synth>,
    aliases: HashMap<String, MidiCommand>,
    current_folder: String,
    current_synth: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            synths: HashMap::new(),
            aliases: HashMap::new(),
            current_folder: String::new(),
            current_synth: String::new(),
        }
    }

    pub fn run_file(&mut self, path: &str) -> Result<(), ConfigError> {
        let mut file = File::open(path)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        self.run_json(contents)?;
        
        /*
        let commands = CommandParser::parse_commands_file(file);
        self.run_commands(&commands);
        */
        Ok(())
    }

    pub fn run_json(&mut self, contents: String) -> Result<(), ConfigError> {
        let parsed = json::parse(&contents)?;
        self.create_synth_json(parsed)?;
        Ok(())
    }

    pub fn create_synth_json(&mut self, val: json::JsonValue) -> Result<(), ConfigError> {
        if let Some(id) = val["id"].as_str() {
            let mut synth = Synth::from_id(id.to_string());

            if let Some(name) = val["name"].as_str() {
                synth.name = name.to_string();
            }

            if let Some(manufacturer) = val["manufacturer"].as_str() {
                synth.manufacturer = manufacturer.to_string();
            }

            // Commands
            for c_val in val["commands"].members() {
                let c = MidiCommand::from_json(c_val);
                synth.commands.push(c);
            }

            self.current_synth = id.to_string();
            self.synths.insert(id.to_string(), synth);

            return Ok(());
        } else {
            return Err(ConfigError::CommonError("Failed to load synth id".to_owned()));
        }
        
    }

    pub fn run_commands_str<T: AsRef<str>>(&mut self, content: &[T]) {
        let c = CommandParser::parse_commands(content);

        self.run_commands(&c);
    }

    pub fn run_command_str(&mut self, content: &str) {
        match CommandParser::parse_command(content.to_owned()) {
            Some(c) => self.run_command(&c),
            None => println!("Failed to parse command"),
        }
    }

    pub fn run_commands(&mut self, commands: &[Command]) {
        for c in commands {
            self.run_command(c);
        }
    }

    pub fn run_command(&mut self, command: &Command) {
        match command.name.as_str() {
            "synth" => {
                let id = command.get_parameter("id").expect("Expected synth ID");
                let mut synth = Synth::from_id(id.to_string());

                if let Some(name) = command.get_parameter("name") {
                    synth.name = name.to_string();
                }

                if let Some(manufacturer) = command.get_parameter("manufacturer") {
                    synth.manufacturer = manufacturer.to_string();
                }

                self.current_synth = id.to_string();
                self.synths.insert(id.to_string(), synth);
            }

            "command" => {
                if self.current_synth != "" {
                    let name = command
                        .get_parameter("name")
                        .expect("Expected command name")
                        .to_string();
                    let mut midi_command = MidiCommand::new(name);

                    let aliases = command
                        .get_parameter("alias")
                        .expect("Expected command name")
                        .to_string();
                    midi_command.add_aliases(aliases);

                    let midi = command
                        .get_parameter("midi")
                        .expect("Expected command name")
                        .to_string();
                    midi_command.midi = midi;

                    // Parameters
                    let mut i: usize = 0;
                    while command.has_numbered_parameter("parameter", i) {
                        let param = command
                            .get_numbered_parameter("parameter", i)
                            .expect("Expected numbered parameter")
                            .as_str();
                        i += 1;
                        midi_command.add_parameter(MidiParameter::new_parse(param));
                    }
                    let s: &mut Synth = self.get_current_synth_mut().expect("");
                    s.commands.push(midi_command);
                }
            }

            "source" => {
                let mut path: String = if self.current_folder.is_empty() {
                    String::from("")
                } else {
                    self.current_folder.clone()
                };
                path.push_str(
                    command
                        .get_parameter_from_index(0)
                        .expect("Expected file name"),
                );
                self.run_file(path.as_str());
            }

            "folder" => match command.get_parameter("type") {
                Some(t) => match t.as_str() {
                    "relative" => {
                        self.current_folder = "".to_owned();
                    }

                    "absolute" => {
                        self.current_folder = "".to_owned();
                    }

                    _ => {
                        println!(
                            "Wrong folder type, expected 'absolute' or 'relative', got {}",
                            t
                        );
                    }
                },
                None => {
                    println!("Expected folder type");
                }
            },

            _ => {}
        }
    }

    pub fn load_synth(&self, synth: &str) -> Option<HashMap<String, MidiCommand>> {
        if let Some(s) = self.synths.get(synth) {
            let mut m: HashMap<String, MidiCommand> = HashMap::new();

            for command in &s.commands {
                for alias in &command.aliases {
                    m.insert(alias.clone(), command.clone());
                }
            }

            return Some(m);
        } else {
            None
        }
    }

    pub fn get_all_midi_commands(&self) -> Vec<MidiCommand> {
        let mut v : Vec<MidiCommand> = Vec::new();

        for (id, synth) in &self.synths {
            for c in &synth.commands {
                v.push(c.clone());
            }
        }

        return v;
    }

    pub fn has_synth(&self, synth: &str) -> bool {
        self.synths.contains_key(synth)
    }

    pub fn get_current_synth(&self) -> Option<&Synth> {
        self.synths.get(&self.current_synth)
    }

    pub fn get_current_synth_mut(&mut self) -> Option<&mut Synth> {
        self.synths.get_mut(&self.current_synth)
    }

    pub fn get_synth_list(&self) -> Vec<&str> {
        let mut v: Vec<&str> = Vec::new();
        for (k, _s) in &self.synths {
            v.push(k.as_str());
        }
        return v;
    }
}
