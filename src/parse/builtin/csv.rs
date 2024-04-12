use eframe::egui::RichText;

use crate::parse::ParsedLine;

pub fn parse_line(line: &str) -> Option<ParsedLine> {
   // Read the line
   let mut reader = csv::ReaderBuilder::new()
       .has_headers(false)
       .comment(Some(b'#'))
       .trim(csv::Trim::All)
       .from_reader(line.as_bytes());

    // Parse the line
    let mut cells = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        for field in record.iter() {
            cells.push(RichText::new(field.to_string()));
        }
    }

    // If there are no cells, return None
    if cells.is_empty() || cells.iter().all(|c| c.text().is_empty()) {
        return None;
    }

    Some(ParsedLine::new(cells))
}
