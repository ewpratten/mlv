#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod parse;

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
    let parsed_file = Arc::new(Mutex::new(parse::ParsedPartialFile::new()));

    // Start a thread that reads from the file and updates the parsed file in memory
    let parsed_file_clone = parsed_file.clone();
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

            // Get a lock on the parsed file
            let mut parsed_file = parsed_file_clone.lock().unwrap();

            // Parse the line
            let parsed_line = args.parser.parse_line(&line);
            parsed_file.add_line(parsed_line);
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
                    let parsed_file = parsed_file.clone();
                    let parsed_file = parsed_file.lock().unwrap();

                    // If the row count is still `None` we don't need to render anything yet because there is no data
                    if parsed_file.len() == 0 {
                        return;
                    }

                    // Set up a fake UI for us to render to for sizing
                    let mut fake_ui = Ui::new(
                        ctx.clone(),
                        LayerId::background(),
                        Id::new("fake_ui"),
                        ui.max_rect(),
                        ui.clip_rect(),
                    );

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

                    // Get the maximum width of each column
                    let column_widths = parsed_file.column_max_widths(&mut fake_ui);

                    // Auto-create columns
                    for col_width in column_widths.iter() {
                        table = table.column(Column::exact(*col_width));
                    }

                    // Fill in the table body
                    table.body(|body| {
                        // Render each row
                        body.rows(30.0, parsed_file.len(), |mut row| {
                            let row_index = row.index();
                            let log_row = parsed_file.get_line(row_index).unwrap();
                            for cell_index in 0..parsed_file.column_count() {
                                // If a cell doesn't exist, just render an empty cell
                                let cell = log_row
                                    .cells()
                                    .get(cell_index)
                                    .unwrap_or(&RichText::new(String::new()))
                                    .clone();
                                row.col(|ui| {
                                    let label = Label::new(cell).wrap(false);
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
