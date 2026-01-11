mod meetups;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// The Rust-Basel cli.
struct Basel {
    #[argh(subcommand)]
    nested: Commands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Commands {
    One(JobCommand),
    Two(MeetupCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "job")]
struct JobCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "meetup")]
struct MeetupCommand {}

fn main() {
    let up: Basel = argh::from_env();
}
