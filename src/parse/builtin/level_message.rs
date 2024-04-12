use eframe::egui::{Color32, RichText};

use crate::parse::ParsedLine;

pub fn parse_line(line: &str) -> Option<ParsedLine> {
    // Split on the first colon if it exists
    let level;
    let message;
    if line.contains(':') {
        let mut parts = line.splitn(2, ':');
        level = parts.next()?;
        message = parts.next()?.trim();
    } else {
        level = "";
        message = line;
    }

    // Determine an appropriate color for the level
    let color = match level.to_uppercase().as_str() {
        "ERROR" | "ERR" => Some(Color32::RED),
        "WARN" | "WARNING" => Some(Color32::YELLOW),
        "INFO" => Some(Color32::GREEN),
        "DEBUG" | "DBG" => Some(Color32::LIGHT_BLUE),
        "TRACE" => Some(Color32::GRAY),
        _ => None,
    };

    // Format the level
    let level = match color {
        Some(color) => RichText::new(level).color(color),
        None => RichText::new(level),
    };

    // Generate the line
    Some(ParsedLine::new(vec![
        level,
        RichText::new(message.to_string()),
    ]))
}
