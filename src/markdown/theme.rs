use ratatui::style::{Color, Modifier, Style};
use tui_markdown::StyleSheet;

#[derive(Debug, Clone)]
pub struct RustyTheme;

impl StyleSheet for RustyTheme {
    fn heading(&self, level: u8) -> Style {
        let color = match level {
            1 => Color::Rgb(255, 69, 0),
            2 => Color::Rgb(255, 99, 0),
            _ => Color::Rgb(206, 92, 0),
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }

    fn code(&self) -> Style {
        Style::default()
            .fg(Color::Rgb(255, 220, 180))
            .bg(Color::Rgb(64, 32, 16))
    }

    fn link(&self) -> Style {
        Style::default()
            .fg(Color::Rgb(255, 191, 0))
            .add_modifier(Modifier::UNDERLINED)
    }

    fn blockquote(&self) -> Style {
        Style::default()
            .fg(Color::Rgb(180, 82, 45))
            .add_modifier(Modifier::ITALIC)
    }

    fn heading_meta(&self) -> Style {
        Style::default()
            .fg(Color::Rgb(139, 69, 19))
            .add_modifier(Modifier::DIM)
    }

    fn metadata_block(&self) -> Style {
        Style::default().fg(Color::Rgb(210, 105, 30))
    }
}
