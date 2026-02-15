/// Command execution and parsing for the markdown viewer
pub enum Command {
    Quit,
    Search(String),
    Jump(usize),
    Help,
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return Command::Unknown(String::new());
        }

        match parts[0] {
            "q" | "quit" => Command::Quit,
            "s" | "search" => {
                if parts.len() > 1 {
                    let query = parts[1..].join(" ");
                    Command::Search(query)
                } else {
                    Command::Unknown("Usage: :s <query>".to_string())
                }
            }
            "jump" => {
                if parts.len() > 1 {
                    if let Ok(line_num) = parts[1].parse::<usize>() {
                        Command::Jump(line_num)
                    } else {
                        Command::Unknown("Usage: :jump <line_number>".to_string())
                    }
                } else {
                    Command::Unknown("Usage: :jump <line_number>".to_string())
                }
            }
            "help" => Command::Help,
            _ => Command::Unknown(format!(
                "Unknown command: {}. Type :help for help",
                parts[0]
            )),
        }
    }

    pub fn help_text() -> String {
        "Commands: :q (quit) | :s <query> (search) | :jump <line> | :help | j/k (scroll) | n/N (next/prev match)".to_string()
    }
}
