#[macro_use]
extern crate lazy_static;
extern crate midir;
extern crate regex;

mod lib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stob() {
        assert_eq!(lib::utils::stob("F0 C1 A2 F7"), [0xF0, 0xC1, 0xA2, 0xF7]);
    }

    #[test]
    fn test_param_half_byte() {
        let p: lib::midicommand::MidiParameter = lib::midicommand::MidiParameter::new_str("n", 0.5, "Channel");
        assert_eq!(p.characters(), 1);
        assert_eq!(p.midi(10), "a");
    }

    #[test]
    fn test_param_full_byte() {
        let p: lib::midicommand::MidiParameter = lib::midicommand::MidiParameter::new_str("n", 1.0, "Channel");
        assert_eq!(p.characters(), 2);
        assert_eq!(p.midi(10), "0a");
        assert_eq!(p.midi(254), "fe");
    }

    #[test]
    fn test_command() {
        let n: lib::midicommand::MidiParameter = lib::midicommand::MidiParameter::new_str("n", 0.5, "Channel");
        let p: lib::midicommand::MidiParameter =
            lib::midicommand::MidiParameter::new_str("p", 1.0, "Parameter");
        let v: lib::midicommand::MidiParameter = lib::midicommand::MidiParameter::new_str("v", 1.0, "Value");
        let mut m: lib::midicommand::MidiCommand =
            lib::midicommand::MidiCommand::new("Parameter Change".to_owned());
        m.midi = String::from("F0 0n p v F7");
        m.add_parameter(n);
        m.add_parameter(p);
        m.add_parameter(v);

        let mut b = std::collections::HashMap::<String, String>::new();
        b.insert(String::from("n"), String::from("1"));
        b.insert(String::from("p"), String::from("02"));
        b.insert(String::from("v"), String::from("03"));

        assert_eq!(m.generate_map(b), "F0 01 02 03 F7");
        assert_eq!(m.generate(&[1, 2, 3]), "F0 01 02 03 F7");
    }

    #[test]
    fn test_parser() {
        let c = lib::command_parser::CommandParser::parse_command(String::from("command -name \"Program Parameter Request\" -midi \"F0 42 3n 0B 10 p F7\" -@parameter \"n : 0.5 : Channel\" -@parameter \"p : 1 : Parameter\" -alias \"pr\"")).unwrap();
        assert_eq!(c.name, "command");
        assert!(c.has_parameter("name"));
        assert!(c.has_parameter("midi"));
        assert!(c.has_parameter("parameter_0"));
        assert!(c.has_parameter("parameter_1"));
        assert!(c.has_parameter("alias"));
        assert_eq!(c.get_parameter("alias"), Some(&String::from("pr")));
    }

    #[test]
    fn test_config() {
        let cs = lib::command_parser::CommandParser::parse_command(String::from("synth -id \"ju-2\" -name \"Juno-2\" -manufacturer \"Roland\"")).unwrap();
        let cc = lib::command_parser::CommandParser::parse_command(String::from("command -name \"Program Parameter Request\" -midi \"F0 42 3n 0B 10 p F7\" -@parameter \"n : 0.5 : Channel\" -@parameter \"p : 1 : Parameter\" -alias \"pr\"")).unwrap();
        let cp = lib::command_parser::CommandParser::parse_command(String::from("command -name \"Program Parameter Change\" -midi \"F0 41 3n 0B 10 p v F7\" -@parameter \"n : 0.5 : Channel\" -@parameter \"p : 1 : Parameter\" -@parameter \"v : 1 : Value\" -alias \"pc param-change\"")).unwrap();
        let mut conf = lib::config::Config::new();

        conf.run_command(&cs);
        conf.run_command(&cc);
        conf.run_command(&cp);

        let s: &lib::synth::Synth = conf.get_current_synth().expect("No synth loaded");
        assert_eq!(s.name, "Juno-2");
        assert_eq!(s.id, "ju-2");
        assert_eq!(s.manufacturer, "Roland");
        assert!(s.has_command("pr"));
        assert!(s.has_command("Program Parameter Request"));
        assert!(s.has_command("pc"));
        assert!(s.has_command("param-change"));
        assert!(s.has_command("Program Parameter Change"));
    }

    #[test]
    fn test_config_command_parser() {
        let mut conf = lib::config::Config::new();

        conf.run_commands_str(&vec![
            "synth -id \"ju-2\" -name \"Juno-2\" -manufacturer \"Roland\"",
            "command -name \"Program Parameter Request\" -midi \"F0 42 3n 0B 10 p F7\" -@parameter \"n : 0.5 : Channel\" -@parameter \"p : 1 : Parameter\" -alias \"pr\"",
            "command -name \"Program Parameter Change\" -midi \"F0 41 3n 0B 10 p v F7\" -@parameter \"n : 0.5 : Channel\" -@parameter \"p : 1 : Parameter\" -@parameter \"v : 1 : Value\" -alias \"pc param-change\""
        ]);

        let s: &lib::synth::Synth = conf.get_current_synth().expect("No synth loaded");
        assert_eq!(s.name, "Juno-2");
        assert_eq!(s.id, "ju-2");
        assert_eq!(s.manufacturer, "Roland");
        assert!(s.has_command("pr"));
        assert!(s.has_command("Program Parameter Request"));
        assert!(s.has_command("pc"));
        assert!(s.has_command("param-change"));
        assert!(s.has_command("Program Parameter Change"));
    }
}

fn main() {
    println!("Hello, world!");
}
