use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "line-counter")]
#[command(about = "A fast, parallel line counter with TUI support")]
#[command(version)]
pub struct Cli {
    /// Root directory to scan (defaults to current directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// File extensions to include (e.g., rs,js,ts)
    #[arg(short, long, value_delimiter = ',')]
    pub include: Vec<String>,

    /// Patterns to exclude (e.g., vendor,dist)
    #[arg(short, long, value_delimiter = ',')]
    pub exclude: Vec<String>,

    /// Disable auto-detection of project type
    #[arg(long)]
    pub no_detect: bool,

    /// Count all file types, not just known languages
    #[arg(long)]
    pub all: bool,

    /// Show only the summary, not per-file details
    #[arg(long)]
    pub summary_only: bool,

    /// Show per-file breakdown
    #[arg(long)]
    pub per_file: bool,

    /// Output format: table, json, csv
    #[arg(long, default_value = "table")]
    pub format: String,

    /// Show top N largest files
    #[arg(long)]
    pub top: Option<usize>,

    /// Export results to a JSON file
    #[arg(long)]
    pub export_json: Option<String>,

    /// Export results to a CSV file
    #[arg(long)]
    pub export_csv: Option<String>,

    /// Show progress bar during scan
    #[arg(long)]
    pub progress: bool,

    /// Launch interactive TUI
    #[arg(long)]
    pub tui: bool,

    /// Save scan to history
    #[arg(long)]
    pub save: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show scan history for a directory
    History {
        /// Directory to show history for
        #[arg(default_value = ".")]
        path: String,
    },
}
