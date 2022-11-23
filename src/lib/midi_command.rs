use super::utils::*;

#[derive(Debug, Clone)]
pub struct MidiParameter {
    key: String,
    size: usize,
    pub name: String,
}

impl MidiParameter {
    pub fn new(key: String, size: usize, name: String) -> MidiParameter {
        MidiParameter { key, size, name }
    }

    pub fn new_str(key: &str, size: usize, name: &str) -> MidiParameter {
        MidiParameter {
            key: key.to_owned(),
            size: size,
            name: name.to_owned(),
        }
    }

    pub fn new_parse(data: &str) -> MidiParameter {
        let tokens: Vec<&str> = data.split(':').collect();
        let trimmed = tokens.iter().map(|&x| x.trim()).collect::<Vec<_>>();
        MidiParameter::new_str(
            trimmed[0],
            trimmed[1]
                .parse::<usize>()
                .expect("Failed to parse parameter size"),
            trimmed[2],
        )
    }

    pub fn characters(&self) -> usize {
        self.size
    }

    pub fn midi(&self, value: usize) -> String {
        format!(
            "{number:0>width$x}",
            number = value,
            width = self.characters()
        )
    }
}

#[derive(Debug, Clone)]
pub struct MidiCommand {
    pub name: String,
    pub midi: String,
    pub parameters: std::collections::HashMap<String, MidiParameter>,
    pub parameter_names: Vec<String>,
    pub aliases: Vec<String>,
    pub mask: Vec<u8>,
    pub masked_val: Vec<u8>
}

impl MidiCommand {
    pub fn new(name: String) -> MidiCommand {
        MidiCommand {
            name: name,
            midi: String::from(""),
            parameters: std::collections::HashMap::<String, MidiParameter>::new(),
            parameter_names: Vec::new(),
            aliases: Vec::new(),
            mask: Vec::new(),
            masked_val: Vec::new(),
        }
    }

    pub fn from_json(val: &json::JsonValue) -> MidiCommand {
        let mut c = MidiCommand::new(val["name"].as_str().unwrap().to_owned());
        c.midi = val["midi"].as_str().unwrap().to_owned();
        c.add_aliases(val["alias"].as_str().unwrap().to_owned());

        for param_val in val["parameters"].members() {
            c.add_parameter(MidiParameter::new_parse(
                param_val.as_str().unwrap()
            ));
        }

        c.mask = MidiCommand::maskify(&c.midi);
        c.masked_val = MidiCommand::destringify(&c.midi);

        return c;
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

    pub fn extract_values(&self, data: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for i in 0..data.len() {
            let m = self.mask[i];

            if m != 0xFF {
                let dat = data[i] & !m; 
                result.push(dat);
            }
        }

        return result;
    }

    pub fn destringify_byte(s: &str) -> u8 {
        if s.len() == 1 {
            let c = s.chars().next().unwrap();
            if "0123456789".contains(c) {
                return c as u8 - 48;
            } else if "ABCDEF".contains(c) {
                return c as u8 - 65 + 10;
            }else {
                return 0x00;
            }
        } else if s.len() == 2 {
            let mut c = s.chars();

            let mut x = 0x00;
            let mut b = false;

            for _i in 0..2 {
                let cc = c.next().unwrap();

                if "0123456789".contains(cc) {
                    x += (cc as u8 - 48) << if b {0} else {4};
                } else if "ABCDEF".contains(cc) {
                    x += (cc as u8 - 65 + 10) << if b {0} else {4};
                }

                b = !b;
            }
            
            return x;
        } else {
            return 0x00;
        }
    }

    pub fn destringify(code: &str) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for b in code.split(" ") {
            data.push(MidiCommand::destringify_byte(b));
        }

        return data;
    }

    pub fn maskify_byte(s: &str) -> u8 {
        if s.len() == 1 {
            if "0123456789ABCDEF".contains(s.chars().next().unwrap()) {
                return 0xFF;
            } else {
                return 0x00;
            }
        } else if s.len() == 2 {
            let mut c = s.chars();

            let mut x = 0x00;

            if "0123456789ABCDEF".contains(c.next().unwrap()) {
                x += 0xF0;
            }

            if "0123456789ABCDEF".contains(c.next().unwrap()) {
                x += 0x0F;
            }

            return x;
        } else {
            return 0x00;
        }
    }

    pub fn maskify(code: &str) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for b in code.split(" ") {
            data.push(MidiCommand::maskify_byte(b));
        }

        return data;
    }

    pub fn matches(&self, data: &[u8]) -> bool {
        if data.len() != self.masked_val.len() {
            return false;
        }

        for i in 0..data.len() {
            if (data[i] & self.mask[i]) != self.masked_val[i] {
                return false;
            }
        }

        return true;
    }
}