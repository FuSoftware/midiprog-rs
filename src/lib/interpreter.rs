use super::config::*;
use super::midi_command::*;
use super::midi_interface::*;
use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::fs::File;
use std::io::prelude::*;

pub enum InterpreterCommand {
    Interactive,
    Config(String),
    Synth(String),
    MidiConfig(String),
    Port(usize, Option<usize>),
    PortList,
    Channel(i8),
    Receive(u32),
    Send(Vec<u8>),
    Sysex(String, Vec<usize>),
    Midi(String, Vec<usize>),
    MidiList,
    SysexList,
    Source(String),
}

pub struct Interpreter {
    channel: i8,
    config: Config,
    interface: MidiInterface,
    midi: HashMap<String, MidiCommand>,
    sysex: HashMap<String, MidiCommand>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            channel: -1,
            config: Config::new(),
            interface: MidiInterface::new(),
            midi: HashMap::new(),
            sysex: HashMap::new(),
        }
    }

    pub fn run_file(&mut self, contents: String) {}

    pub fn parse_command(&mut self, command: String) -> Option<InterpreterCommand> {
        let mut tokens = command.split_whitespace();

        match tokens.next() {
            Some(command) => match command {
                "synth" => match tokens.next() {
                    Some(synth) => {
                        return Some(InterpreterCommand::Synth(String::from(synth)));
                    }
                    None => {
                        println!("Missing 'synth id' argument");
                        return None;
                    }
                },

                "source" => match tokens.next() {
                    Some(source) => {
                        return Some(InterpreterCommand::Source(String::from(source)));
                    }
                    None => {
                        println!("Missing 'source' argument");
                        return None;
                    }
                },

                "interactive" => {
                    return Some(InterpreterCommand::Interactive);
                }

                "config" => match tokens.next() {
                    Some(file) => {
                        return Some(InterpreterCommand::Config(String::from(file)));
                    }
                    None => {
                        println!("Missing 'config file' argument");
                        return None;
                    }
                },

                "midiconfig" => match tokens.next() {
                    Some(file) => {
                        return Some(InterpreterCommand::MidiConfig(String::from(file)));
                    }
                    None => {
                        println!("Missing 'config file' argument");
                        return None;
                    }
                },

                "port" => match tokens.next() {
                    Some(in_port) => {
                        if let Ok(ip) = in_port.parse::<usize>() {
                            match tokens.next() {
                                Some(in_port) => {
                                    if let Ok(op) = in_port.parse::<usize>() {
                                        return Some(InterpreterCommand::Port(ip, Some(op)));
                                    } else {
                                        println!("Failed to parse the output port");
                                        return None;
                                    }
                                }
                                None => {
                                    return Some(InterpreterCommand::Port(ip, None));
                                }
                            }
                        } else {
                            println!("Failed to parse the input port");
                            return None;
                        }
                    }
                    None => {
                        println!("Missing 'input port' argument");
                        return None;
                    }
                },

                "lsport" => {
                    return Some(InterpreterCommand::PortList);
                }

                "lsmidi" => {
                    return Some(InterpreterCommand::MidiList);
                }

                "lssysex" => {
                    return Some(InterpreterCommand::SysexList);
                }

                "channel" => match tokens.next() {
                    Some(channel) => {
                        if let Ok(c) = channel.parse::<i8>() {
                            return Some(InterpreterCommand::Channel(c));
                        } else {
                            println!("Failed to parse the channel");
                            return None;
                        }
                    }
                    None => {
                        println!("Missing 'channel' argument");
                        return None;
                    }
                },

                "receive" => match tokens.next() {
                    Some(timeout) => {
                        if let Ok(t) = timeout.parse::<u32>() {
                            return Some(InterpreterCommand::Receive(t));
                        } else {
                            println!("Failed to parse the timeout");
                            return None;
                        }
                    }
                    None => {
                        return Some(InterpreterCommand::Receive(0));
                    }
                },

                "sysex" => match tokens.next() {
                    Some(alias) => {
                        let mut data: Vec<usize> = Vec::new();
                        while let Some(t) = tokens.next() {
                            if let Ok(a) = t.parse::<usize>() {
                                data.push(a);
                            } else {
                                println!("Failed to parse argument");
                                return None;
                            }
                        }

                        return Some(InterpreterCommand::Sysex(String::from(alias), data));
                    }
                    None => {
                        println!("Missing 'alias' argument");
                        return None;
                    }
                },

                "midi" => match tokens.next() {
                    Some(alias) => {
                        let mut data: Vec<usize> = Vec::new();
                        while let Some(t) = tokens.next() {
                            if let Ok(a) = t.parse::<usize>() {
                                data.push(a);
                            } else {
                                println!("Failed to parse argument");
                                return None;
                            }
                        }

                        return Some(InterpreterCommand::Midi(String::from(alias), data));
                    }
                    None => {
                        println!("Missing 'alias' argument");
                        return None;
                    }
                },

                _ => {
                    println!("Command {} does not exist", command);
                }
            },

            None => {
                println!("Expected command name, got empty command");
            }
        }

        None
    }

    pub fn run_commands_str(&mut self, commands: &[String]) {
        for command in commands {
            if let Some(c) = self.parse_command(String::from(command.as_str())) {
                self.run_command(c);
            }
        }
    }

    pub fn run_command_str(&mut self, command: String) {
        if let Some(c) = self.parse_command(String::from(command.as_str())) {
            self.run_command(c);
        }
    }

    pub fn run_command(&mut self, command: InterpreterCommand) {
        match command {
            InterpreterCommand::Interactive => {
                self.repl();
            }

            InterpreterCommand::Source(file) => {
                match File::open(file.clone()) {
                    Ok(mut file) => {
                        let mut contents: String = String::new();
                        file.read_to_string(&mut contents);
                        let commands = contents.split('\n');
                        for command in commands {
                            self.run_command_str(command.trim().to_owned());
                        }
                    }

                    Err(E) => {
                        println!("Error sourcing file {} : {}", file.as_str(), E.to_string());
                    }
                }
                self.run_file(file)
            }

            InterpreterCommand::Config(file) => {
                self.config.run_file(file.as_str());
            }

            InterpreterCommand::Synth(id) => {
                if self.config.has_synth(id.as_str()) {
                    if let Some(sysex) = self.config.load_synth(id.as_str()) {
                        self.sysex = sysex;
                    }
                } else {
                    println!("Failed to load synth {} configuration", id);
                }
            }

            InterpreterCommand::MidiConfig(file) => {
                self.config.run_file(file.as_str());

                if self.config.has_synth("midi") {
                    if let Some(midi) = self.config.load_synth("midi") {
                        self.midi = midi;
                    }
                } else {
                    println!("Failed to load MIDI configuration");
                }
            }

            InterpreterCommand::Port(midi_in, midi_out) => {
                self.interface.set_input_port(midi_in);

                if !midi_out.is_none() {
                    self.interface.set_output_port(midi_out.unwrap());
                }
            }

            InterpreterCommand::PortList => {
                MidiInterface::list_input_ports();
                MidiInterface::list_output_ports();
            }

            InterpreterCommand::Channel(channel) => {
                self.channel = channel;
            }

            InterpreterCommand::Receive(timeout) => {}

            InterpreterCommand::Send(bytes) => {
                self.interface.send_midi(&bytes);
            }

            InterpreterCommand::Sysex(command, mut data) => {
                if self.channel >= 0 {
                    data.insert(0, self.channel as usize);
                }

                if self.sysex.contains_key(command.as_str()) {
                    let data = self
                        .sysex
                        .get(command.as_str())
                        .unwrap()
                        .generate_bytes(&data);
                    self.interface.send_midi(&data);
                    println!("Send SYSEX {} with data {:?}", command, data);
                } else {
                    println!("SYSEX command '{}' not found", command);
                }
            }

            InterpreterCommand::Midi(command, mut data) => {
                if self.channel >= 0 {
                    data.insert(0, self.channel as usize);
                }

                if self.midi.contains_key(command.as_str()) {
                    let data = self
                        .midi
                        .get(command.as_str())
                        .unwrap()
                        .generate_bytes(&data);
                    self.interface.send_midi(&data);
                    println!("Sent MIDI '{}' with data {:?}", command, data);
                } else {
                    println!("MIDI command {} not found", command);
                }
            }

            InterpreterCommand::MidiList => {
                if !self.midi.is_empty() {
                    for (alias, command) in &self.midi {
                        println!("{} : {}", alias, command.name);
                    }
                } else {
                    println!("No MIDI configuration loaded.")
                }
            }

            InterpreterCommand::SysexList => {
                if !self.sysex.is_empty() {
                    for (alias, command) in &self.sysex {
                        println!("{} : {}", alias, command.name);
                    }
                } else {
                    println!("No SYSEX configuration loaded.")
                }
            }
        }
    }

    pub fn repl(&mut self) {
        let mut rl = Editor::<()>::new();

        loop {
            let readline = rl.readline(">> ");

            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    match line.as_str() {
                        "exit" => {
                            break;
                        }

                        "" => {}

                        _ => {
                            let commands = line.split("|");
                            for command in commands {
                                self.run_command_str(String::from(command));
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}
