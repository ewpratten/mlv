#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;

use std::{
    io::BufRead,
    sync::{Arc, Mutex},
};

use clap::Parser;
use cli::Args;
use eframe::egui::{self, Id, Label, LayerId, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder};

pub fn main() {
    // Parse CLI args
    let args = Args::parse();

    // Set up the logger
    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("{}: {}", record.level(), message)))
        .level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    // Configure the egui window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        vsync: args.vsync,
        follow_system_theme: args.theme == cli::Themes::System,
        default_theme: match args.theme {
            cli::Themes::Light => eframe::Theme::Light,
            _ => eframe::Theme::Dark,
        },
        // For some reason, turning this on *doesn't* center the window
        centered: false,
        ..Default::default()
    };

    // Allocate a shared 2D array to store log rows
    let mut log_rows: Arc<Mutex<Vec<Vec<String>>>> = Arc::new(Mutex::new(Vec::new()));
    let mut col_max_widths: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    // Start a thread that reads from the file and updates the log_rows
    let log_rows_clone = log_rows.clone();
    let col_max_widths_clone = col_max_widths.clone();
    std::thread::spawn(move || {
        // Either read from a file or stdin
        let file_reader = if let Some(file) = args.file {
            log::debug!("Reading from file: {:?}", file);

            // If the file doesn't exist, we can't read from it
            if !file.exists() {
                log::error!("File does not exist: {:?}", file);
                std::process::exit(1);
            }

            // Set up reader
            Box::new(std::fs::File::open(file).unwrap()) as Box<dyn std::io::Read>
        } else {
            log::debug!("Reading from stdin");
            Box::new(std::io::stdin()) as Box<dyn std::io::Read>
        };

        let reader = std::io::BufReader::new(file_reader);
        for line in reader.lines() {
            let line = line.unwrap();

            // If this line is blank, skip
            if line.trim().is_empty() {
                continue;
            }

            // TEMP: Split the line on spaces
            let row_cells: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

            // Update the column widths
            let mut col_max_widths = col_max_widths_clone.lock().unwrap();
            if col_max_widths.is_empty() {
                col_max_widths.extend(std::iter::repeat(0).take(row_cells.len()));
            }
            let col_count = col_max_widths.len();
            if col_count < row_cells.len() {
                col_max_widths.extend(std::iter::repeat(0).take(row_cells.len() - col_count));
            }
            for (i, cell) in row_cells.iter().enumerate() {
                col_max_widths[i] = col_max_widths[i].max(cell.len());
            }

            let mut log_rows = log_rows_clone.lock().unwrap();
            log_rows.push(row_cells);
            log::debug!("Line");
        }
        log::debug!("File closed");
    });

    // Start the render task
    eframe::run_simple_native("Mini Log Viewer", options, move |ctx, _frame| {

        // Top bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Close").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal()
                .drag_to_scroll(false)
                .auto_shrink(false)
                .show(ui, |ui| {
                    // To auto-generate the table, we need access to the log
                    let log_rows = log_rows.clone();

                    // If the row count is still `None` we don't need to render anything yet because there is no data
                    let col_max_widths = col_max_widths.lock().unwrap();
                    let col_count = col_max_widths.len();
                    if col_count == 0 {
                        return;
                    }
                    log::debug!("Col count: {:?}", col_count);

                    // We need to solve the widths of the columns before we can render the table
                    let mut fake_ui = Ui::new(ctx.clone(), LayerId::background(), Id::new("fake_ui"), ui.max_rect(), ui.clip_rect());
                    let column_widths = col_max_widths
                        .iter()
                        .map(|char_count| {
                            // Generate a fake label to fill with text
                            let label =
                                Label::new(RichText::new("A".repeat(*char_count))).wrap(false);

                            // Lay it out in order to figure out its size
                            let (position, layout, _) = label.layout_in_ui(&mut fake_ui);

                            // Return the width of the label
                            layout.size().x
                        })
                        .collect::<Vec<f32>>();

                    // Begin basic configuration for a table
                    let window_height = ui.max_rect().height();
                    let mut table = TableBuilder::new(ui)
                        .cell_layout(Layout::default().with_main_wrap(false))
                        .vscroll(true)
                        .striped(true)
                        .stick_to_bottom(true)
                        .drag_to_scroll(false)
                        .max_scroll_height(window_height)
                        .auto_shrink(false);

                    // Auto-create columns
                    for col_width in column_widths.iter() {
                        table = table.column(Column::exact(*col_width));
                    }

                    // Fill in the table body
                    table.body(|mut body| {
                        // Render each row
                        let log_rows = log_rows.lock().unwrap();
                        body.rows(30.0, log_rows.len(), |mut row| {
                            let row_index = row.index();
                            let log_row = &log_rows[row_index];
                            for cell_index in 0..col_count {
                                // If a cell doesn't exist, just render an empty cell
                                let cell = log_row
                                    .get(cell_index)
                                    .map(|s| s.to_string())
                                    .unwrap_or(String::new());
                                row.col(|ui| {
                                    let label = Label::new(RichText::new(cell)).wrap(false);
                                    ui.add(label);
                                });
                            }
                        });
                    });
                });
        });
    })
    .unwrap();
}
