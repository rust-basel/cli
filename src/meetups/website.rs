use std::{fs, io::Write};

use maud::{Markup, html};

use crate::{meetups::Meetup, remote};

pub fn build() {
    let (meetups, markdowns) = remote::get_files();

    let mut meetups_htmls: Vec<Markup> = Vec::new();
    meetups.meetups.iter().for_each(|meetup| {
        let markdown = markdowns
            .iter()
            .find(|markdown| markdown.0 == meetup.markdown_name)
            .unwrap();
        meetups_htmls.push(single_markdown(meetup.clone(), markdown.clone()))
    });

    let html = html! {
        @for h in &meetups_htmls {
                  (h.clone())
        }
    };

    fs::create_dir_all("page").unwrap();

    let mut html_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("page/index.html")
        .unwrap();
    html_file.write_all(html.into_string().as_bytes()).unwrap();
}

fn single_markdown(meetup: Meetup, markdown: (String, String)) -> maud::Markup {
    let m_as_html = markdown::to_html_with_options(&markdown.1, &markdown::Options::gfm())
        .unwrap()
        .to_string();
    let address_html = meetup.address.html();
    let sponsor_htmls = meetup
        .sponsors
        .iter()
        .map(|sponsor| sponsor.html())
        .collect::<Vec<Markup>>();

    let sponsor_html = html! {
        @for sponsor in &sponsor_htmls {
            (sponsor)
        }
    };

    html! {
        h2 { (meetup.title) }
        p { (meetup.description) }
        (address_html)
        (sponsor_html)

        div{
            (maud::PreEscaped(m_as_html))
        }
    }
}
