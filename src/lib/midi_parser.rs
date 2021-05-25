use super::config::*;

pub struct MIDICallbackCommand {
    identifier: String,
    data: Vec<usize>
}

pub struct MIDIParser {

}

impl MIDIParser {
    pub fn parse(data: &[u8], config: &Config) -> MIDICallbackCommand {
        let commands = config.get_all_midi_commands();
        todo!();
    }
}