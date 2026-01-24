use maud::{Markup, html};

use crate::meetups::Meetup;

pub fn single_markdown_to_html(meetup: Meetup, markdown: String) -> maud::Markup {
    let m_as_html = markdown::to_html_with_options(&markdown, &markdown::Options::gfm())
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
