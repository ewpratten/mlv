use eframe::egui::RichText;

use crate::parse::ParsedLine;

pub fn parse_line(line: &str) -> ParsedLine {
    ParsedLine::new(
        line.split_whitespace()
            .map(|s| RichText::new(s.to_string()))
            .collect(),
    )
}
