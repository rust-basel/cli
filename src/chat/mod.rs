use std::io;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
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
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('s') => self.insert_dummy_text(),
            _ => {}
        }
    }

    fn insert_dummy_text(&mut self) {
        self.history.push("Hi you, pls answer :(".to_string());
        self.message = "Writing a message ...".to_string();
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Basel Rust Chat Room ".bold());
        let instructions = Line::from(vec![
            " Send ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut lines: Vec<Line> = vec![];

        self.history.iter().for_each(|l|{

            let new_line = Line::from(vec![
               l.into()
            ]);

            lines.push(new_line);
        });

        let counter_text = Text::from(lines);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

