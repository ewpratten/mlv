use eframe::egui::RichText;

use crate::parse::ParsedLine;

pub fn parse_line(line: &str) -> Option<ParsedLine> {
    Some(ParsedLine::new(vec![RichText::new(line.to_string())]))
}
