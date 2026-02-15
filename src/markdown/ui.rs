use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

pub fn draw(
    frame: &mut Frame,
    text: &Text,
    scroll: u16,
    mode: &Mode,
    command_input: &str,
    status_message: &Option<String>,
) {
    let area = frame.area();

    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

    let outer = Block::default()
        .title("Markdown Viewer")
        .borders(Borders::ALL);

    let inner = outer.inner(chunks[0]);
    let docs = Paragraph::new(text.clone())
        .scroll((scroll, 0))
        .block(outer);

    frame.render_widget(docs, chunks[0]);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(text.lines.len()).position(scroll as usize);
    frame.render_stateful_widget(scrollbar, inner, &mut scrollbar_state);

    let bottom_text = match mode {
        Mode::Command => format!(":{}", command_input),
        Mode::Normal => {
            if let Some(msg) = status_message {
                msg.clone()
            } else {
                String::new()
            }
        }
    };

    let bottom_line = Paragraph::new(Line::from(bottom_text));
    frame.render_widget(bottom_line, chunks[1]);
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Command,
}
