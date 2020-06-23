use super::midicommand::*;
use super::synth::Synth;
use super::command_parser::*;

pub struct Config {
    synths: std::collections::HashMap<String, Synth>,
    aliases: std::collections::HashMap<String, MidiCommand>,
    current_folder: String,
    current_synth: String
}

impl Config {
    pub fn new() -> Config {
        Config {
            synths: std::collections::HashMap::new(),
            aliases: std::collections::HashMap::new(),
            current_folder: String::new(),
            current_synth: String::new()
        }
    }

    pub fn run_file(&self, path: &str) {

    }

    pub fn run_commands_str<T: AsRef<str>>(&mut self, content: &[T]) {
        let c = CommandParser::parse_commands(content);

        self.run_commands(&c);
    }

    pub fn run_command_str(&mut self, content: &str) {
        match CommandParser::parse_command(content.to_owned()) {
            Some(c) => self.run_command(&c),
            None => println!("Failed to parse command")
        }
        
    }

    pub fn run_commands(&mut self, commands: &[Command]) {
        for c in commands {
            self.run_command(c);
        }
    }

    pub fn run_command(&mut self, command: &Command) {
        match command.name.as_str() {
            "synth" => {
                let id = command.get_parameter("id").expect("Expected synth ID");
                let mut synth = Synth::from_id(id.to_string());

                if let Some(name) = command.get_parameter("name") {
                   synth.name = name.to_string();
                }

                if let Some(manufacturer) = command.get_parameter("manufacturer") {
                    synth.manufacturer = manufacturer.to_string();
                }

                self.current_synth = id.to_string();
                self.synths.insert(id.to_string(), synth);
            },

            "command" => {
                if self.current_synth != "" {
                    let name = command.get_parameter("name").expect("Expected command name").to_string();
                    let mut midi_command = MidiCommand::new(name);

                    let aliases = command.get_parameter("alias").expect("Expected command name").to_string();
                    midi_command.add_aliases(aliases);

                    let midi = command.get_parameter("midi").expect("Expected command name").to_string();
                    midi_command.midi = midi;

                    // Parameters
                    let mut i: usize = 0;
                    while command.has_numbered_parameter("parameter", i) {
                        let param = command.get_numbered_parameter("parameter", i).expect("Expected numbered parameter").as_str();
                        i += 1;
                        midi_command.add_parameter(MidiParameter::new_parse(param));
                    }
                    let s: &mut Synth = self.get_current_synth_mut().expect("");
                    s.commands.push(midi_command);
                }
            }

            "source" => {

            }

            "folder" => {

            }

            _ => {

            }
        }
    }

    pub fn get_current_synth(&self) -> Option<&Synth> {
        self.synths.get(&self.current_synth)
    }

    pub fn get_current_synth_mut(&mut self) -> Option<&mut Synth> {
        self.synths.get_mut(&self.current_synth)
    }
}

/*
void Config::run_file(std::string path)
{
    this->curr_folder = get_folder(path);
    std::vector<Command> commands = CommandParser::parse_commands_file(path);
    this->run(commands);
}

void Config::run(std::vector<Command> commands)
{
    for(auto c: commands)
    {
        this->run(c);
    }
}

void Config::run(Command c)
{
    else if(c.getName() == "command")
    {
        if(this->curr_synth != nullptr)
        {
            MIDICommand comm(c.getParameter("name"));
            comm.addAliases(c.getParameter("alias"));
            comm.setMidi(c.getParameter("midi"));

            //Parameters
            size_t i = 0;
            std::string key = "parameter";
            while(c.hasNumberedParameter("parameter", i))
            {
                std::string p = c.getNumberedParameter("parameter",i);
                i++;
                comm.addParameter(p);
            }
            this->curr_synth->getCommands()->push_back(comm);
        }
    }
    else if(c.getName() == "source")
    {
        std::string path = (this->folder != "" ? this->folder + "/" : "") + c.getParameter(0);
        this->run_file(path);
    }
    else if(c.getName() == "folder")
    {
        if(c.hasParameter("type"))
        {
            std::string type = c.getParameter("type");

            if(type == "relative")
            {
                this->folder = this->curr_folder;
            }
            else if (type == "absolute")
            {
                this->folder = "";
            }
        }
    }
    else
    {
        std::cout << "Unrecognized command " << c.getName() << std::endl;
    }
}

Synth* Config::get_synth(std::string id)
{
    return map_has_key(this->synths, id) ? this->synths[id] : nullptr;
}

void Config::report_parameter_number_error(std::string command, size_t number, size_t found)
{
    std::cout << "Expected '" << number << "' parameters for the '" << command << "' command. Found '" << found << "'" << std::endl;
}

std::vector<std::string> Config::get_synth_list()
{
    std::vector<std::string> keys;
    for(auto it = this->synths.begin(); it != this->synths.end(); ++it) {
        keys.push_back(it->first);
    }
    return keys;
}

std::vector<Synth*> Config::get_synth_values()
{
    std::vector<Synth*> values;
    for(auto it = this->synths.begin(); it != this->synths.end(); ++it) {
        values.push_back(it->second);
    }
    return values;
}
*/