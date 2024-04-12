use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::parse::FileParsers;

/// Simple Log Viewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to read (default: stdin)
    #[arg()]
    pub file: Option<PathBuf>,

    /// File parsing format
    #[arg(short, long, required = true)]
    pub parser: FileParsers,

    /// Enable V-sync
    #[arg(long, default_value = "true")]
    pub vsync: bool,

    /// Window theme
    #[arg(short, long, default_value = "dark")]
    pub theme: Themes,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Themes {
    /// Light Theme
    Light,
    /// Dark Theme
    #[default]
    Dark,
    /// System Theme
    System,
}
