use std::{fs, io::Read};

use xshell::cmd;

pub fn get_config_dir() -> std::path::PathBuf {
    dirs::config_dir().unwrap()
}

pub fn get_files() -> (String, String) {
    let config_dir = get_config_dir();
    let sh = shell();
    sh.change_dir(config_dir.clone());
    let _res = sh.current_dir();
    let cli_dir = config_dir.join("meetups");
    let _res = fs::remove_dir_all(cli_dir);

    let git_result = cmd!(sh, "git clone https://github.com/rust-basel/meetups.git")
        .read()
        .unwrap();

    println!("{}", git_result);

    // read files from repo

    let meetups_dir = config_dir.join("meetups");
    let mut css_str = String::new();

    let _res = fs::OpenOptions::new()
        .read(true)
        .open(meetups_dir.join("page.css"))
        .unwrap()
        .read_to_string(&mut css_str);

    let mut meetup_markdown = String::new();

    let _res = fs::OpenOptions::new()
        .read(true)
        .open(meetups_dir.join("meetups.md"))
        .unwrap()
        .read_to_string(&mut meetup_markdown);

    (css_str, meetup_markdown)
}

fn shell() -> xshell::Shell {
    xshell::Shell::new().unwrap()
}
