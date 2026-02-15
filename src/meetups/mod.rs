pub mod website;

use crate::{markdown, remote};

pub fn meetup_ui() {
    let (_css, markdown) = remote::get_files();
    markdown::render(markdown);
}
