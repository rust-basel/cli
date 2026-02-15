mod app;
mod commands;
mod events;
mod navigation;
mod search;
mod theme;
mod ui;

use tui_markdown::{Options, from_str_with_options};

use app::App;
use theme::RustyTheme;

pub fn render(markdown: String) {
    color_eyre::install().unwrap();

    let leaked: &'static str = Box::leak(markdown.into_boxed_str());
    let options = Options::new(RustyTheme);
    let text = from_str_with_options(leaked, &options);

    ratatui::run(|terminal| App::new(text).run(terminal)).unwrap();
}
