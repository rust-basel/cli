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
    Admin(AdminCommand),
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
#[argh(subcommand, name = "admin")]
struct AdminCommand {}

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
        Command::Admin(_admin_command) => inquire_admin(),
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

fn inquire_admin() {
    let ans: Result<&str, InquireError> =
        Select::new("admin commands", vec!["init", "website"]).prompt();

    let Ok(ans) = ans else {
        println!("No selection made");
        return;
    };

    match ans {
        "init" => meetups::init::init(),
        "website" => meetups::website::build(),
        _ => println!("Unknown admin command"),
    }
}
