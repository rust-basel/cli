use std::io;
use std::time::{Duration, Instant};

use ratatui::{DefaultTerminal, text::Text};

use crate::markdown::{
    commands::Command,
    events::{self, Action, CommandAction},
    navigation::Navigation,
    search::Search,
    ui::{self, Mode},
};

/// Main application state and logic
pub struct App {
    text: Text<'static>,
    original_text: Text<'static>,
    navigation: Navigation,
    search: Search,
    mode: Mode,
    command_input: String,
    status_message: Option<String>,
    highlight_time: Option<Instant>,
    exit: bool,
}

impl App {
    pub fn new(text: Text<'static>) -> Self {
        Self {
            original_text: text.clone(),
            text,
            navigation: Navigation::new(),
            search: Search::new(),
            mode: Mode::Normal,
            command_input: String::new(),
            status_message: None,
            highlight_time: None,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                ui::draw(
                    frame,
                    &self.text,
                    self.navigation.scroll,
                    &self.mode,
                    &self.command_input,
                    &self.status_message,
                )
            })?;

            // Check if highlight time has expired
            if let Some(highlight_time) = self.highlight_time
                && highlight_time.elapsed() >= Duration::from_secs(2)
            {
                self.clear_highlights();
                self.highlight_time = None;
            }

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // Use a timeout to check for highlight expiration
        if let Some(key_event) = events::poll_events(Duration::from_millis(100))? {
            match self.mode {
                Mode::Normal => self.handle_normal_mode(key_event),
                Mode::Command => self.handle_command_mode(key_event),
            }
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, key_event: crossterm::event::KeyEvent) {
        match events::handle_normal_mode(key_event) {
            Action::Quit => self.exit = true,
            Action::EnterCommandMode => {
                self.mode = Mode::Command;
                self.command_input.clear();
                self.status_message = None;
            }
            Action::ScrollDown(amount) => self.navigation.scroll_down(amount, &self.text),
            Action::ScrollUp(amount) => self.navigation.scroll_up(amount),
            Action::NextMatch => self.next_search_result(),
            Action::PrevMatch => self.prev_search_result(),
            Action::None => {}
        }
    }

    fn handle_command_mode(&mut self, key_event: crossterm::event::KeyEvent) {
        match events::handle_command_mode(key_event) {
            CommandAction::Exit => {
                self.mode = Mode::Normal;
                self.command_input.clear();
            }
            CommandAction::Execute => {
                self.execute_command();
                self.mode = Mode::Normal;
            }
            CommandAction::AppendChar(c) => {
                self.command_input.push(c);
            }
            CommandAction::Backspace => {
                self.command_input.pop();
            }
            CommandAction::None => {}
        }
    }

    fn execute_command(&mut self) {
        let command = Command::parse(&self.command_input);

        match command {
            Command::Quit => {
                self.exit = true;
            }
            Command::Search(query) => {
                self.perform_search(&query);
            }
            Command::Jump(line_num) => match self.navigation.jump_to_line(line_num, &self.text) {
                Ok(()) => {
                    self.status_message = Some(format!("Jumped to line {}", line_num));
                }
                Err(msg) => {
                    self.status_message = Some(msg);
                }
            },
            Command::Help => {
                self.status_message = Some(Command::help_text());
            }
            Command::Unknown(msg) => {
                self.status_message = Some(msg);
            }
        }

        self.command_input.clear();
    }

    fn perform_search(&mut self, query: &str) {
        if query.is_empty() {
            self.status_message = Some("Empty search query".to_string());
            return;
        }

        let count = self.search.perform_search(query, &self.original_text);

        if count > 0 {
            // Apply highlighting and start timer
            self.text = self.search.highlight_matches(&self.original_text);
            self.highlight_time = Some(Instant::now());

            // Jump to first result
            if let Some(line) = self.search.current_match() {
                self.navigation.scroll_to_line(line as u16);
            }

            self.status_message = Some(format!("Found {} matches for '{}'", count, query));
        } else {
            self.status_message = Some(format!("No matches found for '{}'", query));
        }
    }

    fn next_search_result(&mut self) {
        if !self.search.has_results() {
            self.status_message = Some("No search results. Use :s <query> to search".to_string());
            return;
        }

        if let Some(line) = self.search.next_match() {
            // Re-highlight to update the current match indicator and reset timer
            self.text = self.search.highlight_matches(&self.original_text);
            self.highlight_time = Some(Instant::now());

            self.navigation.scroll_to_line(line as u16);

            self.status_message = Some(format!(
                "Match {}/{}",
                self.search.current_index + 1,
                self.search.results.len()
            ));
        }
    }

    fn prev_search_result(&mut self) {
        if !self.search.has_results() {
            self.status_message = Some("No search results. Use :s <query> to search".to_string());
            return;
        }

        if let Some(line) = self.search.prev_match() {
            // Re-highlight to update the current match indicator and reset timer
            self.text = self.search.highlight_matches(&self.original_text);
            self.highlight_time = Some(Instant::now());

            self.navigation.scroll_to_line(line as u16);

            self.status_message = Some(format!(
                "Match {}/{}",
                self.search.current_index + 1,
                self.search.results.len()
            ));
        }
    }

    fn clear_highlights(&mut self) {
        // Reset to original text without highlights
        self.text = self.original_text.clone();
        self.search.clear();
    }
}
