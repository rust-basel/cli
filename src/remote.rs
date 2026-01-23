use std::{fs, io::Read};

use xshell::cmd;

use crate::meetups::Meetups;

pub fn get_config_dir() -> std::path::PathBuf {
    dirs::config_dir().unwrap()
}

pub fn get_files() -> (Meetups, Vec<(String, String)>) {
    let config_dir = get_config_dir();
    let sh = shell();
    sh.change_dir(config_dir.clone());
    let _res = sh.current_dir();
    let cli_dir = config_dir.join("cli");
    fs::remove_dir_all(cli_dir).unwrap();

    let git_result = cmd!(sh, "git clone https://github.com/rust-basel/cli.git")
        .read()
        .unwrap();
    println!("{}", git_result);

    // read files from repo

    let public = config_dir.join("cli").join("public");
    let meetups_dir = public.join("meetups");
    let mut toml_str = String::new();

    let _res = fs::OpenOptions::new()
        .read(true)
        .open(public.join("meetups.toml"))
        .unwrap()
        .read_to_string(&mut toml_str);

    println!("{}", toml_str);

    let meetups: Meetups = toml::from_str(&toml_str).unwrap();

    let meetup_markdowns = std::fs::read_dir(meetups_dir).unwrap();
    let mut mtps: Vec<(String, String)> = vec![];
    for entry in meetup_markdowns {
        let path = entry.unwrap().path();
        if path.clone().extension().unwrap() == "md" {
            let content = std::fs::read_to_string(path.clone()).unwrap();
            mtps.push((
                path.file_name().unwrap().to_string_lossy().to_string(),
                content,
            ));
        }
    }

    (meetups, mtps)
}

fn shell() -> xshell::Shell {
    xshell::Shell::new().unwrap()
}
