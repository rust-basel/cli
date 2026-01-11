use std::{fs, io::Write};

pub fn init() {
    let meetups = super::Meetups::default();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("public_files/meetups.toml")
        .expect("Failed to create meetups.json file");

    let toml_string =
        toml::to_string_pretty(&meetups).expect("Failed to serialize meetups to TOML");
    file.write_all(toml_string.as_bytes())
        .expect("Failed to write to file");
}
