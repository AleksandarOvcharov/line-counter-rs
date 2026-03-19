mod cli;
mod counter;
mod detect;
mod history;
mod lang;
mod output;
mod scanner;
mod tui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use output::OutputFormat;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(Commands::History { path }) = &args.command {
        let root = std::fs::canonicalize(path)?;
        return history::show_history(&root.to_string_lossy());
    }

    let root = std::fs::canonicalize(&args.path)?;
    let root_display = root.to_string_lossy().to_string();

    // Auto-detect project type unless disabled or manual includes given
    let profile = if !args.no_detect && args.include.is_empty() && !args.all {
        detect::detect_project(&root)
    } else {
        None
    };

    if let Some(ref p) = profile {
        println!("  \x1b[1;36m\u{25b6}\x1b[0m Detected project: \x1b[1m{}\x1b[0m", p.name);
    }

    let mut exclude_patterns = vec![".git".to_string()];
    if let Some(ref p) = profile {
        exclude_patterns.extend(p.excludes.clone());
    } else {
        // Fallback defaults when no project detected
        exclude_patterns.extend([
            "node_modules".to_string(),
            ".next".to_string(),
            "target".to_string(),
            "dist".to_string(),
            "build".to_string(),
            "__pycache__".to_string(),
            ".venv".to_string(),
            "vendor".to_string(),
        ]);
    }
    exclude_patterns.extend(args.exclude.clone());

    let include_exts: Vec<String> = if !args.include.is_empty() {
        // Manual includes take priority
        args.include
            .iter()
            .map(|e| e.trim_start_matches('.').to_lowercase())
            .collect()
    } else if let Some(ref p) = profile {
        p.extensions.clone()
    } else {
        Vec::new() // empty = all known languages
    };

    let config = scanner::ScanConfig {
        root: root.clone(),
        include_exts,
        exclude_patterns,
        count_all: args.all,
    };

    let files = scanner::collect_files(&config);

    if files.is_empty() {
        println!("  No files found matching the given criteria.");
        return Ok(());
    }

    let results = scanner::scan_files(&files, &root, args.progress)?;
    let summary = scanner::summarize(&results);

    if args.tui {
        return tui::run_tui(summary, results);
    }

    let format = match args.format.as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        _ => OutputFormat::Table,
    };

    output::print_summary(&summary, &format)?;

    if args.per_file {
        output::print_per_file(&results, &format)?;
    }

    if let Some(top_n) = args.top {
        output::print_top_files(&results, top_n);
    }

    if let Some(path) = &args.export_json {
        output::export_json(&summary, path)?;
    }

    if let Some(path) = &args.export_csv {
        output::export_csv(&summary, path)?;
    }

    if args.save {
        history::save_entry(&root_display, &summary)?;
        println!("  Scan saved to history.");
    }

    Ok(())
}
