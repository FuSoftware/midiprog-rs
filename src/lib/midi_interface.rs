use derive_more::{From};
#[derive(From)]
pub enum MidiInterfaceError {
    ConnectErrorMidiInput(midir::ConnectError<midir::MidiInput>),
    ConnectErrorMidiOutput(midir::ConnectError<midir::MidiOutput>),
    PortDoesNotExist(String)
}

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

    pub fn set_input_port(&mut self, midi_in: usize) -> Result<(), MidiInterfaceError> {
        let in_m = midir::MidiInput::new("midi-prog").unwrap();
        let in_ports = in_m.ports();
        if let Some(p) = in_ports.get(midi_in) {
            self.in_conn = Some(
                in_m.connect(
                    p,
                    "midi-in",
                    |_stamp, _message, _| {
                        //println!("{}: {:?} (len = {})", stamp, message, message.len());
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
        let out_m = midir::MidiOutput::new("midi-prog").unwrap();
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

    pub fn set_ports(&mut self, midi_in: usize, midi_out: usize) -> Result<(), MidiInterfaceError> {
        self.set_input_port(midi_in)?;
        self.set_output_port(midi_out)?;
        Ok(())
    }

    pub fn list_input_ports() {
        let midi_in: midir::MidiInput = midir::MidiInput::new("midi-prog").unwrap();

        println!("Available input ports:");
        for (i, p) in midi_in.ports().iter().enumerate() {
            println!("{}: {}", i, midi_in.port_name(p).unwrap());
        }
    }

    pub fn list_output_ports() {
        let midi_out: midir::MidiOutput = midir::MidiOutput::new("midi-prog").unwrap();

        println!("Available output ports:");
        for (i, p) in midi_out.ports().iter().enumerate() {
            println!("{}: {}", i, midi_out.port_name(p).unwrap());
        }
    }

    pub fn send_midi(&mut self, data: &[u8]) -> Result<(), midir::SendError> {
        self.out_conn.as_mut().unwrap().send(data)
    }

    pub fn receive_midi(&self) -> Vec<u8> {
        let data: Vec<u8> = Vec::new();
        return data;
    }
}
