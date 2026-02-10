use super::Address;
use super::Sponsor;
use std::process::exit;
use std::{fs, io::Write};

use maud::{Markup, html};
mod md;

use crate::remote;

impl Address {
    pub fn html(&self) -> Markup {
        maud::html! {
            div {
                p { @if let Some(description) = &self.description {
                (description)
                } " | " (self.street) " | "  (self.city) " | " (self.postal_code) }
            }
        }
    }
}

impl Sponsor {
    pub fn html(&self) -> Markup {
        maud::html! {
                div {
                    p { (self.name) }
                    @if let Some(website) = &self.website {
                         a href=(website) { (website) }
                    }
                    @if let Some(content) = &self.content {
                        p { (content) }
                    }
            }
        }
    }
}

pub fn build() {
    let (css, markdown) = remote::get_files();

    let html = create_main_page(md::single_markdown_to_html(markdown), css);

    write_html_file(html.into_string().as_bytes());
}

fn create_main_page(meetups_html: Markup, css: String) -> maud::Markup {
    html! {
        head{
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            style {
                (maud::PreEscaped(css))
            }
        }

        title { "Rust Basel Meetups" }


        a href="https://rust-basel.ch" {
            h1 { "Rust Basel | Meetups | Workshops" }
        }

        (meetups_html)
    }
}

fn write_html_file(html: &[u8]) {
    fs::create_dir_all("page").unwrap();
    let mut html_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("page/index.html")
        .unwrap();
    html_file.write_all(html).unwrap();
}
