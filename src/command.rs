use std::str::FromStr;

pub enum SupportedCommand {
    Add,
    List,
    Update,
    Delete,
    Help,
    Exit
}

impl ToString for SupportedCommand {
    fn to_string(&self) -> String {
        match self {
            SupportedCommand::Add => String::from("add"),
            SupportedCommand::List => String::from("list"),
            SupportedCommand::Update => String::from("update"),
            SupportedCommand::Delete => String::from("delete"),
            SupportedCommand::Help => String::from("help"),
            SupportedCommand::Exit => String::from("exit"),
        }
    }
}

impl FromStr for SupportedCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<SupportedCommand, Self::Err> {
        match input {
            "add" => Ok(SupportedCommand::Add),
            "list" => Ok(SupportedCommand::List),
            "update" => Ok(SupportedCommand::Update),
            "delete" => Ok(SupportedCommand::Delete),
            "help" => Ok(SupportedCommand::Help),
            "exit" => Ok(SupportedCommand::Exit),
            _ => Err(()),
        }
    }
}