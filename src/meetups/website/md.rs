use maud::{Markup, html};


pub fn single_markdown_to_html(markdown: String) -> maud::Markup {
    let m_as_html = markdown::to_html_with_options(&markdown, &markdown::Options::gfm())
        .unwrap()
        .to_string();

    html! {
        div{
            (maud::PreEscaped(m_as_html))
        }
    }
}
