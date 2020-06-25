use super::utils::*;

pub struct MidiParameter {
    key: String,
    size: f32,
    pub name: String,
}

impl MidiParameter {
    pub fn new(key: String, size: f32, name: String) -> MidiParameter {
        MidiParameter { key, size, name }
    }

    pub fn new_str(key: &str, size: f32, name: &str) -> MidiParameter {
        MidiParameter { key: key.to_owned(), size: size, name: name.to_owned() }
    }

    pub fn new_parse(data: &str) -> MidiParameter {
        let tokens: Vec<&str> = data.split(':').collect();
        let trimmed = tokens.iter().map(|&x| x.trim()).collect::<Vec<_>>();
        MidiParameter::new_str(trimmed[0], trimmed[1].parse::<f32>().expect("Failed to parse parameter size"), trimmed[2])
    }

    pub fn characters(&self) -> usize {
        (self.size * 2.0).round() as usize
    }

    pub fn midi(&self, value: usize) -> String {
        format!(
            "{number:0>width$x}",
            number = value,
            width = self.characters()
        )
    }
}

pub struct MidiCommand {
    pub name: String,
    pub midi: String,
    pub parameters: std::collections::HashMap<String, MidiParameter>,
    pub parameter_names: Vec<String>,
    pub aliases: Vec<String>,
}

impl MidiCommand {
    pub fn new(name: String) -> MidiCommand {
        MidiCommand {
            name: name,
            midi: String::from(""),
            parameters: std::collections::HashMap::<String, MidiParameter>::new(),
            parameter_names: Vec::new(),
            aliases: Vec::new(),
        }
    }

    pub fn generate(&self, values: &[usize]) -> String {
        if values.len() != self.parameters.len() {
            println!(
                "Command {} expected {} parameters, got {}",
                self.name,
                self.parameters.len(),
                values.len()
            );
            return String::from("");
        } else {
            let mut val = std::collections::HashMap::<String, String>::new();

            for i in 0..values.len() {
                let s = self.parameters[&self.parameter_names[i]].midi(values[i]);
                val.insert(self.parameter_names[i].clone(), s);
            }

            return self.generate_map(val);
        }
    }

    pub fn generate_map(&self, values: std::collections::HashMap<String, String>) -> String {
        let mut midi: String = self.midi.clone();

        for (k, v) in values {
            midi = midi.replace(k.as_str(), v.as_str());
        }
        return midi;
    }

    pub fn generate_bytes(&self, values: &[usize]) -> Vec<u8> {
        stob(self.generate(values).as_str())
    }

    pub fn generate_bytes_map(&self, values: std::collections::HashMap<String, String>) -> Vec<u8> {
        stob(self.generate_map(values).as_str())
    }

    pub fn add_parameter(&mut self, p: MidiParameter) {
        self.parameter_names.push(p.key.clone());
        self.parameters.insert(p.key.clone(), p);
    }

    pub fn add_aliases(&mut self, values: String) {
        for s in values.split(" ") {
            self.aliases.push(s.trim().to_owned())
        }
    }

    pub fn add_aliases_list(&mut self, values: &[String]) {
        for s in values {
            self.aliases.push(s.trim().to_owned())
        }
    }
}
