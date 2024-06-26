use eframe::egui::RichText;

use crate::parse::ParsedLine;

pub fn parse_line(line: &str) -> Option<ParsedLine> {
    Some(ParsedLine::new(
        line.split_whitespace()
            .map(|s| RichText::new(s.to_string()))
            .collect(),
    ))
}
