use eframe::egui::{Color32, RichText};

use crate::parse::ParsedLine;

#[derive(Debug, serde::Deserialize)]
pub struct JournalEntry {
    #[serde(rename = "__REALTIME_TIMESTAMP")]
    pub timestamp: String,
    #[serde(rename = "_SYSTEMD_UNIT")]
    pub unit: Option<String>,
    #[serde(rename = "_PID")]
    pub pid: Option<String>,
    #[serde(rename = "SYSLOG_IDENTIFIER")]
    pub identifier: Option<String>,
    #[serde(rename = "PRIORITY", default = "String::new")]
    pub priority: String,
    #[serde(rename = "MESSAGE")]
    pub message: String,
}

impl JournalEntry {
    pub fn timestamp(&self) -> Option<i64> {
        self.timestamp.parse().ok()
    }

    pub fn pid(&self) -> Option<i32> {
        self.pid.clone()?.parse().ok()
    }

    pub fn priority(&self) -> Option<i32> {
        self.priority.parse().ok()
    }
}

pub fn parse_line(line: &str) -> Option<ParsedLine> {
    // Attempt to parse the line
    let entry: JournalEntry = match serde_json::from_str(line) {
        Ok(entry) => entry,
        Err(_) => {
            log::warn!("Failed to parse JSON line: {}", line);
            return None;
        }
    };

    // Parse encoded integers
    let timestamp = entry.timestamp()?;
    let pid = entry.pid()?;
    let priority = entry.priority()?;

    // Convert the timestamp to a human-readable format
    let timestamp = chrono::DateTime::from_timestamp(timestamp / 1_000_000, 0)?
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    Some(ParsedLine::new(vec![
        RichText::new(timestamp.to_string()),
        match priority {
            0 => RichText::new("EMERG").color(Color32::RED),
            1 => RichText::new("ALERT").color(Color32::RED),
            2 => RichText::new("CRIT").color(Color32::RED),
            3 => RichText::new("ERR").color(Color32::RED),
            4 => RichText::new("WARNING").color(Color32::YELLOW),
            5 => RichText::new("NOTICE").color(Color32::GREEN),
            6 => RichText::new("INFO").color(Color32::GREEN),
            7 => RichText::new("DEBUG").color(Color32::LIGHT_BLUE),
            _ => RichText::new("UNKNOWN").color(Color32::GRAY),
        },
        RichText::new(entry.unit.unwrap_or_default()).color(Color32::DARK_GREEN),
        RichText::new(entry.identifier.unwrap_or_default()).color(Color32::LIGHT_GREEN),
        RichText::new(pid.to_string()).color(Color32::LIGHT_BLUE),
        RichText::new(entry.message).color(Color32::WHITE).weak(),
    ]))
}
