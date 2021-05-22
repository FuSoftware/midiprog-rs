use derive_more::*;
use midir::{InitError, PortInfoError};
use std::fmt;

#[derive(Debug, From)]
pub enum MidiInterfaceError {
    ConnectErrorMidiInput(midir::ConnectError<midir::MidiInput>),
    ConnectErrorMidiOutput(midir::ConnectError<midir::MidiOutput>),
    PortDoesNotExist(String),
    PortInfoError(PortInfoError),
    PortInitError(InitError)
}

impl std::fmt::Display for MidiInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        return match self {
            MidiInterfaceError::ConnectErrorMidiInput(e) => {
                write!(f, "{}", e)
            }

            MidiInterfaceError::ConnectErrorMidiOutput(e) => {
                write!(f, "{}", e)
            }

            MidiInterfaceError::PortDoesNotExist(e) => {
                write!(f, "{}", e)
            }

            MidiInterfaceError::PortInfoError(e) => {
                write!(f, "{}", e)
            }

            MidiInterfaceError::PortInitError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

#[derive(Default)]
pub struct MidiInterface {
    in_conn: Option<midir::MidiInputConnection<()>>,
    out_conn: Option<midir::MidiOutputConnection>,
}

impl MidiInterface {
    pub fn new() -> MidiInterface {
        MidiInterface {
            in_conn: None,
            out_conn: None,
        }
    }

    pub fn set_input_port(&mut self, midi_in: usize, callback: fn(u64, &[u8]) ) -> Result<(), MidiInterfaceError>  {
        let in_m = midir::MidiInput::new("midi-prog")?;
        let in_ports = in_m.ports();
        if let Some(p) = in_ports.get(midi_in) {
            self.in_conn = Some(
                in_m.connect(
                    p,
                    "midi-in",
                    move |stamp, message, _| {
                        callback(stamp, message);
                    },
                    (),
                )?
            );
            Ok(())
        } else {
            Err(MidiInterfaceError::PortDoesNotExist(format!("MIDI input port {} doesn't exist", midi_in)))
        }
    }

    pub fn set_output_port(&mut self, midi_out: usize) -> Result<(), MidiInterfaceError> {
        let out_m = midir::MidiOutput::new("midi-prog")?;
        let out_ports = out_m.ports();
        if let Some(p) = out_ports.get(midi_out) {
            self.out_conn = Some(
                out_m
                    .connect(p, "midi-out")?
            );
            Ok(())
        } else {
            Err(MidiInterfaceError::PortDoesNotExist(format!("MIDI output port {} doesn't exist", midi_out)))
        }
        
    }

    pub fn set_ports(&mut self, midi_in: usize, midi_out: usize, callback: fn(u64, &[u8])) -> Result<(), MidiInterfaceError> {
        self.set_input_port(midi_in, callback)?;
        self.set_output_port(midi_out)?;
        Ok(())
    }

    pub fn list_input_ports() -> Result<String, MidiInterfaceError> {
        let midi_in: midir::MidiInput = midir::MidiInput::new("midi-prog")?;

        let mut s: String = String::new();
        s.push_str("Available input ports:\n");

        for (i, p) in midi_in.ports().iter().enumerate() {
            s.push_str(&format!("{}: {}\n", i, midi_in.port_name(p)?));
        }

        Ok(s)
    }

    pub fn list_output_ports() -> Result<String, MidiInterfaceError> {
        let midi_out: midir::MidiOutput = midir::MidiOutput::new("midi-prog")?;

        let mut s: String = String::new();
        s.push_str("Available output ports:\n");
        for (i, p) in midi_out.ports().iter().enumerate() {
            s.push_str(&format!("{}: {}\n", i, midi_out.port_name(p)?));
        }
        Ok(s)
    }

    pub fn send_midi(&mut self, data: &[u8]) -> Result<(), midir::SendError> {
        self.out_conn.as_mut().unwrap().send(data)
    }
}
