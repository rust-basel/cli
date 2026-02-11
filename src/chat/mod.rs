use ratatui::{DefaultTerminal, Frame};

pub fn connect(_chat_command: usize) {
    println!("Chatting with remote users {}", _chat_command);
    // connect to server => panic if fail
    // run server connection
    // create ui => link to server connection

    color_eyre::install().unwrap();
    ratatui::run(app).unwrap();
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
