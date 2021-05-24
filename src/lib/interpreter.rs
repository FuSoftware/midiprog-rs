use super::config::*;
use super::midi_command::*;
use super::midi_interface::*;
use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::fs::File;
use std::io::prelude::*;

use derive_more::*;
#[derive(Debug, From)]
pub enum InterpreterError {
    SimpleError(String),
    MidiSendError(midir::SendError),
    InterfaceError(MidiInterfaceError)
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        return match self {
            InterpreterError::SimpleError(e) => {
                write!(f, "{}", e)
            }

            InterpreterError::MidiSendError(e) => {
                write!(f, "{}", e)
            }

            InterpreterError::InterfaceError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

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

#[derive(Default)]
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

    pub fn run_commands_str(&mut self, commands: &[String]) -> Result<(), InterpreterError> {
        for command in commands {
            if let Some(c) = self.parse_command(String::from(command.as_str())) {
                self.run_command(c)?;
            }
        }
        Ok(())
    }

    pub fn run_command_str(&mut self, command: &str) -> Result<(), InterpreterError> {
        let c = self.parse_command(String::from(command))
            .ok_or(InterpreterError::SimpleError(format!("Failed to parse command")))?;
        self.run_command(c)?;
        Ok(())
    }

    pub fn run_command(&mut self, command: InterpreterCommand) -> Result<(), InterpreterError> {
        match command {
            InterpreterCommand::Interactive => {
                self.repl();
                Ok(())
            }

            InterpreterCommand::Source(file) => {
                match File::open(file.clone()) {
                    Ok(mut file) => {
                        let mut contents: String = String::new();
                        file.read_to_string(&mut contents);
                        let commands = contents.split('\n');
                        for command in commands {
                            self.run_command_str(command.trim());
                        }
                    }

                    Err(e) => {
                        return Err(InterpreterError::SimpleError(format!("Error sourcing file {} : {}", file.as_str(), e.to_string())));
                    }
                }
                self.run_file(file);
                Ok(())
            }

            InterpreterCommand::Config(file) => {
                match self.config.run_file(file.as_str()) {
                    Ok(_a) => {

                    }

                    Err(_e) => {
                        return Err(InterpreterError::SimpleError(format!("Error loading config file")));
                    }
                };
                Ok(())
            }

            InterpreterCommand::Synth(id) => {
                if self.config.has_synth(id.as_str()) {
                    let sysex = self.config.load_synth(id.as_str()).ok_or(InterpreterError::SimpleError(format!("Failed to load synth {} sysex configuration", id)))?; 
                    self.sysex = sysex;
                    Ok(())
                } else {
                    Err(InterpreterError::SimpleError(format!("Synth {} was not found", id)))
                }
            }

            InterpreterCommand::MidiConfig(file) => {
                self.config.run_file(file.as_str());
                if self.config.has_synth("midi") {
                    let midi = self.config.load_synth("midi").ok_or(InterpreterError::SimpleError(format!("Failed to load MIDI standard")))?; 
                    self.midi = midi;
                    Ok(())
                } else {
                    Err(InterpreterError::SimpleError(format!("MIDI standard configuration was not found")))
                }
            }

            InterpreterCommand::Port(midi_in, midi_out) => {
                self.interface.set_input_port(midi_in, |_stamp, message|{
                    println!("{:?}", message);
                })?;

                if let Some(o) = midi_out {
                    self.interface.set_output_port(o)?;
                }
                Ok(())
            }

            InterpreterCommand::PortList => {
                println!("{:?}", MidiInterface::list_input_ports().unwrap_or("Error listing the MIDI input ports".to_owned()));
                println!("{:?}", MidiInterface::list_output_ports().unwrap_or("Error listing the MIDI output ports".to_owned()));
                Ok(())
            }

            InterpreterCommand::Channel(channel) => {
                self.channel = channel;
                Ok(())
            }

            InterpreterCommand::Receive(timeout) => {
                Ok(())
            }

            InterpreterCommand::Send(bytes) => {
                self.interface.send_midi(&bytes)?;
                Ok(())
            }

            InterpreterCommand::Sysex(command, mut data) => {
                if self.channel >= 0 {
                    data.insert(0, self.channel as usize);
                }

                if let Some(sysex) = self.sysex.get(command.as_str()) {
                    let data = sysex.generate_bytes(&data);
                    self.interface.send_midi(&data)?;
                    println!("Send SYSEX {} with data {:?}", command, data);
                    Ok(())
                } else {
                    Err(InterpreterError::SimpleError(format!("SYSEX command {} not found", command)))
                }
            }

            InterpreterCommand::Midi(command, mut data) => {
                if self.channel >= 0 {
                    data.insert(0, self.channel as usize);
                }

                if let Some(midi) = self.midi.get(command.as_str()) {
                    let data = midi.generate_bytes(&data);
                    self.interface.send_midi(&data)?;
                    println!("Send MIDI {} with data {:?}", command, data);
                    Ok(())
                } else {
                    Err(InterpreterError::SimpleError(format!("MIDI command {} not found", command)))
                }
            }

            InterpreterCommand::MidiList => {
                if !self.midi.is_empty() {
                    for (alias, command) in &self.midi {
                        println!("{} : {}", alias, command.name);
                    }
                    Ok(())
                } else {
                    Err(InterpreterError::SimpleError(format!("No MIDI configuration loaded")))
                }
            }

            InterpreterCommand::SysexList => {
                if !self.sysex.is_empty() {
                    for (alias, command) in &self.sysex {
                        println!("{} : {}", alias, command.name);
                    }
                    Ok(())
                } else {
                    Err(InterpreterError::SimpleError(format!("No SYSEX configuration loaded")))
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
                                self.run_command_str(command);
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
