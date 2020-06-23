use super::midicommand::MidiCommand;

pub struct Synth {
    pub name: String,
    pub id: String,
    pub manufacturer: String,
    pub commands: Vec<MidiCommand>,
}

impl Synth {
    pub fn new(name: String, id: String, manufacturer: String) -> Synth {
        Synth {
            name,
            id,
            manufacturer,
            commands: Vec::new(),
        }
    }

    pub fn from_id(id: String) -> Synth {
        Synth {
            name: String::from(""),
            id,
            manufacturer: String::from(""),
            commands: Vec::new(),
        }
    }

    pub fn has_command(&self, id: &str) -> bool {
        let owned_id = id.to_owned();
        for comm in &self.commands {
            if comm.name == id || comm.aliases.contains(&owned_id) {
                return true;
            }
        }
        return false;
    }
}
