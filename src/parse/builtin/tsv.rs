use eframe::egui::RichText;

use crate::parse::ParsedLine;

pub fn parse_line(line: &str) -> Option<ParsedLine> {
    Some(ParsedLine::new(
        line.split("\t")
            .map(|s| RichText::new(s.to_string()))
            .collect(),
    ))
}
