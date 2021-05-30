use super::config::*;

pub struct MIDICallbackCommand {
    identifier: String,
    data: Vec<usize>
}

pub struct MIDIParser {

}

impl MIDIParser {
    pub fn destringify(code: &str) -> Vec<u8> {
        let s = code.replace(" ", "");
        let mut data: Vec<u8> = Vec::new();

        let mut b = false;
        let mut x = 0;

        for c in s.chars() {
            if "0123456789".contains(c) {
                x += (0xF) << if b {0} else {4};
            }

            if "ABCDEF".contains(c) {
                x += (0xF) << if b {0} else {4};
            }

            b = !b;

            if !b {
                data.push(x);
                x = 0;
            }
        }

        return data;
    }

    pub fn maskify(code: &str) -> Vec<u8> {
        let s = code.replace(" ", "");
        let mut data: Vec<u8> = Vec::new();

        let mut b = false;
        let mut x = 0;

        for c in s.chars() {
            if "0123456789".contains(c) {
                x += (c as u8 - 48) << if b {0} else {4};
            }

            if "ABCDEF".contains(c) {
                x += (c as u8 - 65 + 10) << if b {0} else {4};
            }

            b = !b;

            if !b {
                data.push(x);
                x = 0;
            }
        }

        return data;
    }

    pub fn parse(data: &[u8], config: &Config) -> MIDICallbackCommand {
        let commands = config.get_all_midi_commands();
        todo!();
    }
}