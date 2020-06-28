pub struct MidiInterface {
    in_conn: Option<midir::MidiInputConnection<()>>,
    out_conn: Option<midir::MidiOutputConnection>,

    in_port: usize,
    out_port: usize,
}

impl MidiInterface {
    pub fn new() -> MidiInterface {
        MidiInterface {
            in_conn: None,
            out_conn: None,
            in_port: 0,
            out_port: 0,
        }
    }

    pub fn set_input_port(&mut self, midi_in: usize) {
        let in_m = midir::MidiInput::new("midi-prog").unwrap();
        let in_ports = in_m.ports();
        self.in_conn = Some(
            in_m.connect(
                in_ports.get(midi_in).unwrap(),
                "midi-in",
                |stamp, message, _| {
                    //println!("{}: {:?} (len = {})", stamp, message, message.len());
                },
                (),
            )
            .expect("Connection to input port failed"),
        );
    }

    pub fn set_output_port(&mut self, midi_out: usize) {
        let out_m = midir::MidiOutput::new("midi-prog").unwrap();
        let out_ports = out_m.ports();
        self.out_conn = Some(
            out_m
                .connect(out_ports.get(midi_out).unwrap(), "midi-out")
                .expect("Connection to output port failed"),
        );
    }

    pub fn set_ports(&mut self, midi_in: usize, midi_out: usize) {
        self.set_input_port(midi_in);
        self.set_output_port(midi_out);
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
