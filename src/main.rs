use std::io::{self, Write};

use crate::buttler::respond;

pub mod buttler {
    use regex::Regex;

    enum Command {
        Open(String),
        Close(String),
        Play(String),
        Fetch(String),
        Switch(bool, String),
    }

    fn text_to_command(text: &str) -> Option<Command> {
        let text = text.trim().to_lowercase();

        println!("{}", text);

        let open = Regex::new(r"open (.+).?").unwrap();
        let close = Regex::new(r"close (.+).?").unwrap();
        let play = Regex::new(r"play (.+).?").unwrap();
        let fetch = Regex::new(r"fetch (.+).?").unwrap();
        let switch = Regex::new(r"(switch (on|off) (.+))").unwrap();

        if let Some(group) = open.captures(&text) {
            let item = group.get(1).unwrap().as_str();
            return Some(Command::Open(item.to_string()));
        }

        if let Some(group) = close.captures(&text) {
            let item = group.get(1).unwrap().as_str();
            return Some(Command::Close(item.to_string()));
        }

        if let Some(group) = play.captures(&text) {
            let item = group.get(1).unwrap().as_str();
            return Some(Command::Play(item.to_string()));
        }

        if let Some(group) = fetch.captures(&text) {
            let item = group.get(1).unwrap().as_str();
            return Some(Command::Fetch(item.to_string()));
        }

        if let Some(group) = switch.captures(&text) {
            let toggle = group.get(2).unwrap().as_str();
            let item = group.get(3).unwrap().as_str();

            return Some(Command::Switch(toggle == "on", item.to_string()));
        }

        return None;
    }

    pub fn respond(text: &str) -> String {
        let greeting = Regex::new(r"^(hello|hi|hey|howdy)[!.]?$").unwrap();

        if let Some(greeting) = greeting.captures(&text) {
            let g = greeting.get(1).unwrap().as_str();
            return format!("{}! You can do it!!!", g);
        }

        match text_to_command(&text) {
            Some(Command::Open(item)) => format!("Opening {}...", item),
            Some(Command::Close(item)) => format!("Closing {}...", item),
            Some(Command::Play(item)) => format!("Playing {}...", item),
            Some(Command::Fetch(item)) => format!("Fetching {}...", item),
            Some(Command::Switch(toggle, item)) => {
                let next_state = if toggle { "on" } else { "off" };
                format!("Switching {} {}...", item, next_state)
            },
            _ => "Please ask me to open, close, play, fetch, or switch something on/off for you..."
                .to_string(),
        }
    }
}

fn main() {
    loop {
        print!("command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        println!("{}", respond(&input));
    }
}
