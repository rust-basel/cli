mod chat;
mod docs;
mod meetups;
mod remote;

use argh::FromArgs;
use inquire::{InquireError, Select};

#[derive(FromArgs, PartialEq, Debug)]
/// The Rust-Basel cli.
struct Basel {
    #[argh(subcommand)]
    commands: Option<Command>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Job(JobCommand),
    Mtp(MeetupCommand),
    Doc(DocCommand),
    Init(InitCommand),
    Website(WebsiteCommand),
    Chat(ChatCommand),
}

impl From<String> for Command {
    fn from(s: String) -> Self {
        match s.as_str() {
            "job" => Command::Job(JobCommand {}),
            "meetup" => Command::Mtp(MeetupCommand {}),
            "doc" => Command::Doc(DocCommand {}),
            _ => panic!("Unknown command"),
        }
    }
}

fn command_as_vec() -> Vec<&'static str> {
    vec!["job", "meetup", "doc"]
}

#[derive(FromArgs, PartialEq, Debug)]
/// Find the featured jobs.
#[argh(subcommand, name = "job")]
struct JobCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Go to the documentation for this application.
#[argh(subcommand, name = "doc")]
struct DocCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Explore meetups
#[argh(subcommand, name = "meetup")]
struct MeetupCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Inits the repository
#[argh(subcommand, name = "init")]
struct InitCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Builds the website
#[argh(subcommand, name = "website")]
struct WebsiteCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Runs the chat
#[argh(subcommand, name = "chat")]
struct ChatCommand {
    /// the login code
    #[argh(option, short = 'c', long = "code")]
    code: usize,
}

fn main() {
    let basel: Basel = argh::from_env();
    match basel.commands {
        Some(c) => match_command(c),
        None => inquire(),
    }
}

fn match_command(c: Command) {
    match c {
        Command::Job(_jobs) => {
            println!("Help needed to implement a nice job ui and systemy");
        }
        Command::Mtp(_mtp) => meetups::meetup_ui(),

        Command::Doc(_doc) => docs::docs_ui(),
        Command::Init(_init_command) => meetups::init::init(),
        Command::Website(_website_command) => meetups::website::build(),

        Command::Chat(_chat_command) => chat::connect(_chat_command.code),
    }
}

fn inquire() {
    let ans: Result<&str, InquireError> = Select::new("commands", command_as_vec()).prompt();

    let Ok(ans) = ans else {
        println!("No selection made");
        return;
    };

    match_command(ans.to_owned().into());
}
