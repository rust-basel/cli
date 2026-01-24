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
    let (meetups, markdowns) = remote::get_files();

    let mut meetups_htmls: Vec<Markup> = Vec::new();

    markdowns.iter().for_each(|(name, content)| {
        let meetup = meetups
            .meetups
            .iter()
            .find(|meetup| meetup.markdown_name == *name);

        let Some(meetup) = meetup else {
            eprintln!("could not find {name}, as a meetup");
            exit(0);
        };

        meetups_htmls.push(md::single_markdown_to_html(meetup.clone(), content.clone()))
    });

    let html = create_main_page(meetups_htmls);

    write_html_file(html.into_string().as_bytes());
}

fn create_main_page(meetups_htmls: Vec<Markup>) -> maud::Markup {
    html! {
        head{
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            style {
                (maud::PreEscaped(include_str!("../../../public/page.css")))
            }
        }

        title { "Rust Basel Meetups" }


        a href="https://rust-basel.ch" {
            h1 { "Rust Basel | Meetups | Workshops" }
        }

        @for h in &meetups_htmls {
                  (h.clone())
        }
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
