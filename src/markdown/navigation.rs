use ratatui::text::Text;

pub struct Navigation {
    pub scroll: u16,
}

impl Navigation {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }

    pub fn scroll_down(&mut self, amount: u16, text: &Text) {
        let max = self.max_scroll(text);
        self.scroll = (self.scroll + amount).min(max);
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    pub fn jump_to_line(&mut self, line: usize, text: &Text) -> Result<(), String> {
        let total_lines = text.lines.len();
        if line > 0 && line <= total_lines {
            self.scroll = (line - 1) as u16;
            Ok(())
        } else {
            Err(format!("Line {} out of range (1-{})", line, total_lines))
        }
    }

    pub fn scroll_to_line(&mut self, line: u16) {
        // Center the target line in the viewport with a 3-line offset
        self.scroll = line.saturating_sub(3);
    }

    fn max_scroll(&self, text: &Text) -> u16 {
        text.lines.len().saturating_sub(1) as u16
    }
}
