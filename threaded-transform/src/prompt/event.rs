use std::str::FromStr;

pub struct Event {
    pub e_type: EventType,
    pub command: Command,
    pub payload: String,
}

#[derive(Debug)]
pub struct CommandParseErr;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Command {
    Lower,
    Upper,
    Slugify,
    Nospace,
    Title,
    Pascal,
    Passwordify,
    Csv,
    Help,
    Exit,
}

#[derive(PartialEq)]
pub enum EventType {
    CommandInput,
    CommandOutput,
    CommandError,
}

impl FromStr for Command {
    type Err = CommandParseErr;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lower" => Ok(Command::Lower),
            "upper" => Ok(Command::Upper),
            "nospace" => Ok(Command::Nospace),
            "slugify" => Ok(Command::Slugify),
            "title" => Ok(Command::Title),
            "pascal" => Ok(Command::Pascal),
            "password" => Ok(Command::Passwordify),
            "csv" => Ok(Command::Csv),
            "help" => Ok(Command::Help),
            "exit" => Ok(Command::Exit),
            _ => Err(CommandParseErr),
        }
    }

}