use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub enum Action {
    Quit,
    EnterCommandMode,
    ScrollDown(u16),
    ScrollUp(u16),
    NextMatch,
    PrevMatch,
    None,
}

pub fn poll_events(timeout: Duration) -> io::Result<Option<KeyEvent>> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Ok(Some(key_event)),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn handle_normal_mode(key_event: KeyEvent) -> Action {
    match key_event.code {
        KeyCode::Char('c')
            if key_event
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL) =>
        {
            Action::Quit
        }
        KeyCode::Esc => Action::Quit,
        KeyCode::Char(':') => Action::EnterCommandMode,
        KeyCode::Char('j') | KeyCode::Down => Action::ScrollDown(1),
        KeyCode::Char('k') | KeyCode::Up => Action::ScrollUp(1),
        KeyCode::Char('n') => Action::NextMatch,
        KeyCode::Char('N') => Action::PrevMatch,
        _ => Action::None,
    }
}

pub enum CommandAction {
    Exit,
    Execute,
    AppendChar(char),
    Backspace,
    None,
}

pub fn handle_command_mode(key_event: KeyEvent) -> CommandAction {
    match key_event.code {
        KeyCode::Esc => CommandAction::Exit,
        KeyCode::Enter => CommandAction::Execute,
        KeyCode::Char(c) => CommandAction::AppendChar(c),
        KeyCode::Backspace => CommandAction::Backspace,
        _ => CommandAction::None,
    }
}
