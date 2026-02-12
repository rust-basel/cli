use std::io;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
pub fn connect(_chat_command: usize) {
    // connect to server => panic if fail
    // run server connection
    // create ui => link to server connection

    color_eyre::install().unwrap();
    ratatui::run(|terminal| App::default().run(terminal)).unwrap();
}

#[derive(Debug, Default)]
pub struct App {
    history: Vec<String>,
    message: String,
    exit: bool,
}

impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Enter => self.enter(),
            KeyCode::Char(c) => self.write(c),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn enter(&mut self) {
        self.history.push(self.message.clone());
        self.message = String::new();
    }

    fn write(&mut self, c: char) {
        self.message.push(c)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Basel Rust Chat Room ".bold());
        let instructions = Line::from(vec![
            " Send ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        let chat_history: Vec<Line> = self
            .history
            .iter()
            .rev()
            .take(4)
            .rev()
            .map(|l| Line::from(l.as_str()))
            .collect();

        Paragraph::new(Text::from(chat_history))
            .centered()
            .block(block)
            .render(chunks[0], buf);

        let input_block = Block::bordered().title(" Message ");
        Paragraph::new(self.message.as_str())
            .block(input_block)
            .render(chunks[1], buf);
    }
}

