mod builtin;

use clap::ValueEnum;
use eframe::egui::{FontFamily, Label, RichText};

/// Represents each type of parser available
#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum FileParsers {
    /// Space-separated values
    Spaces,
    /// Comma-separated values
    Csv,
    /// Simple "LEVEL: MESSAGE" log format
    LevelMessage,
    /// Journalctl JSON output
    JournalJson,
}

impl FileParsers {
    /// Parse a line using the selected parser
    pub fn parse_line(&self, line: &str) -> Option<ParsedLine> {
        match self {
            FileParsers::Spaces => builtin::spaces::parse_line(line),
            FileParsers::Csv => builtin::csv::parse_line(line),
            FileParsers::LevelMessage => builtin::level_message::parse_line(line),
            FileParsers::JournalJson => builtin::journal_json::parse_line(line),
        }
    }
}

/// Represents a parsed line
#[derive(Clone)]
pub struct ParsedLine {
    cells: Vec<RichText>,
}

impl ParsedLine {
    /// Construct a new ParsedLine
    pub fn new(cells: Vec<RichText>) -> Self {
        Self {
            cells: cells
                .iter()
                .map(|cell| cell.clone().family(FontFamily::Monospace))
                .collect(),
        }
    }

    /// Get the cells in the line
    pub fn cells(&self) -> &[RichText] {
        &self.cells
    }

    /// Get the number of cells in the line
    pub fn len(&self) -> usize {
        self.cells.len()
    }
}

/// Represents a partially parsed file
pub struct ParsedPartialFile {
    lines: Vec<ParsedLine>,
    cached_column_max_char_counts: Vec<usize>,
}

impl ParsedPartialFile {
    /// Construct a new ParsedPartialFile
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            cached_column_max_char_counts: Vec::new(),
        }
    }

    /// Get nth line
    pub fn get_line(&self, index: usize) -> Option<&ParsedLine> {
        self.lines.get(index)
    }

    /// Get the number of lines
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Add a line
    pub fn add_line(&mut self, line: ParsedLine) {
        // Figure out how many cells are in this line
        let cell_count = line.len();

        // If we don't have enough space in the cached column max char counts, extend it
        if self.cached_column_max_char_counts.len() < cell_count {
            self.cached_column_max_char_counts.extend(
                std::iter::repeat(0).take(cell_count - self.cached_column_max_char_counts.len()),
            );
        }

        // Update the column char counts
        for (index, cell) in line.cells().iter().enumerate() {
            if let Some(current_max) = self.cached_column_max_char_counts.get_mut(index) {
                *current_max = *current_max.max(&mut cell.text().len());
            }
        }

        // Store the line
        self.lines.push(line);
    }

    /// Get the maximum width of each column
    pub fn column_max_widths(&self, ui: &mut eframe::egui::Ui) -> Vec<f32> {
        self.cached_column_max_char_counts
            .iter()
            .map(|char_count| {
                // Generate a fake label to fill with text
                let label = Label::new(RichText::new("A".repeat(*char_count))).wrap(false);

                // Lay it out in order to figure out its size
                let (_, layout, _) = label.layout_in_ui(ui);

                // Return the width of the label
                layout.size().x
            })
            .collect::<Vec<f32>>()
    }

    // Get the expected font height
    pub fn font_height(&self, ui: &mut eframe::egui::Ui) -> f32 {
        let label = Label::new(RichText::new("A")).wrap(false);
        let (_, layout, _) = label.layout_in_ui(ui);
        layout.size().y
    }

    /// Get the number of columns
    pub fn column_count(&self) -> usize {
        self.cached_column_max_char_counts.len()
    }
}
