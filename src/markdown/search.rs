use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

pub struct Search {
    pub results: Vec<usize>,
    pub current_index: usize,
    pub query: String,
}

impl Search {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            current_index: 0,
            query: String::new(),
        }
    }

    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    pub fn perform_search(&mut self, query: &str, text: &Text) -> usize {
        if query.is_empty() {
            return 0;
        }

        self.results.clear();
        self.query = query.to_string();
        self.current_index = 0;

        let query_lower = query.to_lowercase();

        for (idx, line) in text.lines.iter().enumerate() {
            let line_text = line.to_string().to_lowercase();
            if fuzzy_match(&line_text, &query_lower) {
                self.results.push(idx);
            }
        }

        self.results.len()
    }

    pub fn next_match(&mut self) -> Option<usize> {
        if self.results.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.results.len();
        Some(self.results[self.current_index])
    }

    pub fn prev_match(&mut self) -> Option<usize> {
        if self.results.is_empty() {
            return None;
        }

        if self.current_index == 0 {
            self.current_index = self.results.len() - 1;
        } else {
            self.current_index -= 1;
        }

        Some(self.results[self.current_index])
    }

    pub fn current_match(&self) -> Option<usize> {
        if self.results.is_empty() {
            None
        } else {
            Some(self.results[self.current_index])
        }
    }

    pub fn highlight_matches<'a>(&self, original_text: &Text<'a>) -> Text<'a> {
        if self.results.is_empty() {
            return original_text.clone();
        }

        let query_lower = self.query.to_lowercase();

        let highlight_style = Style::default()
            .bg(Color::Rgb(206, 92, 0))
            .fg(Color::Rgb(255, 255, 255))
            .add_modifier(Modifier::BOLD);

        let current_highlight_style = Style::default()
            .bg(Color::Rgb(255, 69, 0))
            .fg(Color::Rgb(255, 255, 255))
            .add_modifier(Modifier::BOLD);

        let mut new_lines: Vec<Line> = Vec::new();
        let current_match_line = if !self.results.is_empty() {
            Some(self.results[self.current_index])
        } else {
            None
        };

        for (idx, line) in original_text.lines.iter().enumerate() {
            if self.results.contains(&idx) {
                let is_current = Some(idx) == current_match_line;
                let style = if is_current {
                    current_highlight_style
                } else {
                    highlight_style
                };

                let line_text = line.to_string();
                let line_text_lower = line_text.to_lowercase();

                let mut match_positions = Vec::new();
                let mut pattern_chars = query_lower.chars();
                let mut current_pattern_char = pattern_chars.next();

                for (pos, ch) in line_text_lower.chars().enumerate() {
                    if Some(ch) == current_pattern_char {
                        match_positions.push(pos);
                        current_pattern_char = pattern_chars.next();
                    }
                }

                let mut spans = Vec::new();
                let mut last_pos = 0;
                let chars: Vec<char> = line_text.chars().collect();

                if is_current {
                    spans.push(Span::styled("→ ", current_highlight_style));
                }

                for &match_pos in &match_positions {
                    if match_pos > last_pos {
                        let before: String = chars[last_pos..match_pos].iter().collect();
                        spans.push(Span::raw(before));
                    }

                    if match_pos < chars.len() {
                        spans.push(Span::styled(chars[match_pos].to_string(), style));
                    }

                    last_pos = match_pos + 1;
                }

                if last_pos < chars.len() {
                    let after: String = chars[last_pos..].iter().collect();
                    spans.push(Span::raw(after));
                }

                new_lines.push(Line::from(spans));
            } else {
                new_lines.push(line.clone());
            }
        }

        Text::from(new_lines)
    }

    pub fn clear(&mut self) {
        self.results.clear();
        self.current_index = 0;
        self.query.clear();
    }
}

fn fuzzy_match(text: &str, pattern: &str) -> bool {
    let mut pattern_chars = pattern.chars();
    let mut current_pattern_char = pattern_chars.next();

    if current_pattern_char.is_none() {
        return true;
    }

    for text_char in text.chars() {
        if Some(text_char) == current_pattern_char {
            current_pattern_char = pattern_chars.next();
            if current_pattern_char.is_none() {
                return true;
            }
        }
    }

    false
}
