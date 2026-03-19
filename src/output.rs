use crate::counter::FileResult;
use crate::scanner::ScanSummary;
use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;
use std::io::Write;

pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

// ─── ANSI (only for the hand-drawn overview card) ────────────────────────────

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const GRAY: &str = "\x1b[90m";

// ─── Public API ──────────────────────────────────────────────────────────────

pub fn print_summary(summary: &ScanSummary, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => print_table_summary(summary),
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(summary)?);
            Ok(())
        }
        OutputFormat::Csv => print_csv_summary(summary),
    }
}

pub fn print_per_file(results: &[FileResult], format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => print_table_per_file(results),
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(results)?);
            Ok(())
        }
        OutputFormat::Csv => print_csv_per_file(results),
    }
}

pub fn print_top_files(results: &[FileResult], count: usize) {
    let mut sorted: Vec<&FileResult> = results.iter().collect();
    sorted.sort_by(|a, b| b.counts.total().cmp(&a.counts.total()));

    let tw = term_width();

    let mut table = new_table(tw);
    table.set_header(vec![
        Cell::new("File"),
        Cell::new("Total").set_alignment(CellAlignment::Right),
        Cell::new("Code").set_alignment(CellAlignment::Right),
        Cell::new("Comment").set_alignment(CellAlignment::Right),
        Cell::new("Blank").set_alignment(CellAlignment::Right),
    ]);

    // File (flex) | Total | Code | Comment | Blank
    let total_w = 7u16;
    let code_w = 7u16;
    let comment_w = 9u16;
    let blank_w = 7u16;
    let fixed = total_w + code_w + comment_w + blank_w;
    let file_w = flexible_width(tw, fixed, 5);

    table.set_constraints(vec![
        ColumnConstraint::Absolute(Width::Fixed(file_w)),
        ColumnConstraint::Absolute(Width::Fixed(total_w)),
        ColumnConstraint::Absolute(Width::Fixed(code_w)),
        ColumnConstraint::Absolute(Width::Fixed(comment_w)),
        ColumnConstraint::Absolute(Width::Fixed(blank_w)),
    ]);

    for file in sorted.iter().take(count) {
        table.add_row(vec![
            Cell::new(&file.path),
            Cell::new(file.counts.total()).set_alignment(CellAlignment::Right),
            cell_code(file.counts.code),
            cell_comment(file.counts.comments),
            cell_blank(file.counts.blank),
        ]);
    }

    println!();
    println!(
        "  {BOLD}{CYAN}\u{25b8} Top {} Largest Files{RESET}",
        count.min(sorted.len())
    );
    println!();
    println!("{table}");
}

// ─── Summary table ───────────────────────────────────────────────────────────

fn print_table_summary(summary: &ScanSummary) -> Result<()> {
    let tw = term_width();
    let total_lines = summary.total_counts.total();

    println!();
    print_overview_card(summary, tw);
    println!();

    // ── Language table ──
    let mut table = new_table(tw);
    table.set_header(vec![
        Cell::new("Language"),
        Cell::new("Files").set_alignment(CellAlignment::Right),
        Cell::new("Total").set_alignment(CellAlignment::Right),
        Cell::new("Code").set_alignment(CellAlignment::Right),
        Cell::new("Comment").set_alignment(CellAlignment::Right),
        Cell::new("Blank").set_alignment(CellAlignment::Right),
        Cell::new("%").set_alignment(CellAlignment::Right),
    ]);

    // Language (flex) | Files | Total | Code | Comment | Blank | %
    let files_w = 7u16;
    let total_w = 7u16;
    let code_w = 7u16;
    let comment_w = 9u16;
    let blank_w = 7u16;
    let pct_w = 7u16;
    let fixed = files_w + total_w + code_w + comment_w + blank_w + pct_w;
    let lang_w = flexible_width(tw, fixed, 7);

    table.set_constraints(vec![
        ColumnConstraint::Absolute(Width::Fixed(lang_w)),
        ColumnConstraint::Absolute(Width::Fixed(files_w)),
        ColumnConstraint::Absolute(Width::Fixed(total_w)),
        ColumnConstraint::Absolute(Width::Fixed(code_w)),
        ColumnConstraint::Absolute(Width::Fixed(comment_w)),
        ColumnConstraint::Absolute(Width::Fixed(blank_w)),
        ColumnConstraint::Absolute(Width::Fixed(pct_w)),
    ]);

    for lang in &summary.by_language {
        let pct = if total_lines > 0 {
            lang.counts.total() as f64 / total_lines as f64 * 100.0
        } else {
            0.0
        };

        table.add_row(vec![
            Cell::new(&lang.language),
            Cell::new(lang.files).set_alignment(CellAlignment::Right),
            Cell::new(lang.counts.total())
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Right),
            cell_code(lang.counts.code),
            cell_comment(lang.counts.comments),
            cell_blank(lang.counts.blank),
            Cell::new(format!("{:.1}%", pct))
                .fg(Color::Cyan)
                .set_alignment(CellAlignment::Right),
        ]);
    }

    // Totals row
    table.add_row(vec![
        Cell::new("TOTAL").add_attribute(Attribute::Bold),
        Cell::new(summary.total_files)
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new(summary.total_counts.total())
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new(summary.total_counts.code)
            .fg(Color::Green)
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new(summary.total_counts.comments)
            .fg(Color::Yellow)
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new(summary.total_counts.blank)
            .fg(Color::Grey)
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
        Cell::new("100%")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Right),
    ]);

    println!("  {BOLD}{CYAN}\u{25b8} Languages{RESET}");
    println!();
    println!("{table}");
    println!();

    Ok(())
}

fn print_overview_card(summary: &ScanSummary, tw: usize) {
    let total_lines = summary.total_counts.total();
    let inner = tw.saturating_sub(4);

    println!(
        "  {GRAY}\u{256d}{}\u{256e}{RESET}",
        "\u{2500}".repeat(inner)
    );

    let stats_vis = format!(
        " Files  {}   Lines  {}   Languages  {}",
        summary.total_files,
        format_num(total_lines),
        summary.by_language.len(),
    );
    let pad = inner.saturating_sub(stats_vis.len() + 1);
    println!(
        "  {GRAY}\u{2502}{RESET} {BOLD}Files{RESET}  {}   {BOLD}Lines{RESET}  {}   {BOLD}Languages{RESET}  {}{}{GRAY}\u{2502}{RESET}",
        summary.total_files,
        format_num(total_lines),
        summary.by_language.len(),
        " ".repeat(pad),
    );

    if total_lines > 0 {
        let bar_w = inner.saturating_sub(4);
        let code_pct = summary.total_counts.code as f64 / total_lines as f64;
        let comment_pct = summary.total_counts.comments as f64 / total_lines as f64;
        let blank_pct = 1.0 - code_pct - comment_pct;
        let code_len = (code_pct * bar_w as f64).round() as usize;
        let comment_len = (comment_pct * bar_w as f64).round() as usize;
        let blank_len = bar_w.saturating_sub(code_len + comment_len);

        print!("  {GRAY}\u{2502}{RESET} ");
        print!("\x1b[48;5;22m{}\x1b[0m", " ".repeat(code_len));
        print!("\x1b[48;5;136m{}\x1b[0m", " ".repeat(comment_len));
        print!("\x1b[48;5;238m{}\x1b[0m", " ".repeat(blank_len));
        let bar_pad = inner.saturating_sub(bar_w + 3);
        println!("{}{GRAY}\u{2502}{RESET}", " ".repeat(bar_pad));

        let legend_vis = format!(
            " \u{25cf} Code {:.1}%   \u{25cf} Comments {:.1}%   \u{25cf} Blank {:.1}%",
            code_pct * 100.0,
            comment_pct * 100.0,
            blank_pct * 100.0,
        );
        let legend_pad = inner.saturating_sub(legend_vis.len() + 1);
        println!(
            "  {GRAY}\u{2502}{RESET} {GREEN}\u{25cf}{RESET} Code {:.1}%   {YELLOW}\u{25cf}{RESET} Comments {:.1}%   {GRAY}\u{25cf}{RESET} Blank {:.1}%{}{GRAY}\u{2502}{RESET}",
            code_pct * 100.0,
            comment_pct * 100.0,
            blank_pct * 100.0,
            " ".repeat(legend_pad),
        );
    }

    println!(
        "  {GRAY}\u{2570}{}\u{256f}{RESET}",
        "\u{2500}".repeat(inner)
    );
}

// ─── Per-file table ──────────────────────────────────────────────────────────

fn print_table_per_file(results: &[FileResult]) -> Result<()> {
    let mut sorted: Vec<&FileResult> = results.iter().collect();
    sorted.sort_by(|a, b| b.counts.total().cmp(&a.counts.total()));

    let tw = term_width();

    let mut table = new_table(tw);
    table.set_header(vec![
        Cell::new("File"),
        Cell::new("Language"),
        Cell::new("Total").set_alignment(CellAlignment::Right),
        Cell::new("Code").set_alignment(CellAlignment::Right),
        Cell::new("Comment").set_alignment(CellAlignment::Right),
        Cell::new("Blank").set_alignment(CellAlignment::Right),
    ]);

    // File (flex) | Language | Total | Code | Comment | Blank
    let lang_w = 12u16;
    let total_w = 7u16;
    let code_w = 7u16;
    let comment_w = 9u16;
    let blank_w = 7u16;
    let fixed = lang_w + total_w + code_w + comment_w + blank_w;
    let file_w = flexible_width(tw, fixed, 6);

    table.set_constraints(vec![
        ColumnConstraint::Absolute(Width::Fixed(file_w)),
        ColumnConstraint::Absolute(Width::Fixed(lang_w)),
        ColumnConstraint::Absolute(Width::Fixed(total_w)),
        ColumnConstraint::Absolute(Width::Fixed(code_w)),
        ColumnConstraint::Absolute(Width::Fixed(comment_w)),
        ColumnConstraint::Absolute(Width::Fixed(blank_w)),
    ]);

    for file in &sorted {
        table.add_row(vec![
            Cell::new(&file.path),
            Cell::new(&file.language),
            Cell::new(file.counts.total()).set_alignment(CellAlignment::Right),
            cell_code(file.counts.code),
            cell_comment(file.counts.comments),
            cell_blank(file.counts.blank),
        ]);
    }

    println!();
    println!("  {BOLD}{CYAN}\u{25b8} Files{RESET}");
    println!();
    println!("{table}");
    println!();

    Ok(())
}

// ─── CSV output ──────────────────────────────────────────────────────────────

fn print_csv_summary(summary: &ScanSummary) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(["Language", "Files", "Total", "Code", "Comment", "Blank"])?;
    for lang in &summary.by_language {
        wtr.write_record([
            &lang.language,
            &lang.files.to_string(),
            &lang.counts.total().to_string(),
            &lang.counts.code.to_string(),
            &lang.counts.comments.to_string(),
            &lang.counts.blank.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

fn print_csv_per_file(results: &[FileResult]) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(["File", "Language", "Extension", "Total", "Code", "Comment", "Blank"])?;
    for file in results {
        wtr.write_record([
            &file.path,
            &file.language,
            &file.extension,
            &file.counts.total().to_string(),
            &file.counts.code.to_string(),
            &file.counts.comments.to_string(),
            &file.counts.blank.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

// ─── Export ──────────────────────────────────────────────────────────────────

pub fn export_json(summary: &ScanSummary, path: &str) -> Result<()> {
    let mut file = std::fs::File::create(path)?;
    let json = serde_json::to_string_pretty(summary)?;
    file.write_all(json.as_bytes())?;
    println!("  Exported JSON to {path}");
    Ok(())
}

pub fn export_csv(summary: &ScanSummary, path: &str) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(["Language", "Files", "Total", "Code", "Comment", "Blank"])?;
    for lang in &summary.by_language {
        wtr.write_record([
            &lang.language,
            &lang.files.to_string(),
            &lang.counts.total().to_string(),
            &lang.counts.code.to_string(),
            &lang.counts.comments.to_string(),
            &lang.counts.blank.to_string(),
        ])?;
    }
    wtr.flush()?;
    println!("  Exported CSV to {path}");
    Ok(())
}

// ─── Table helpers ───────────────────────────────────────────────────────────

fn new_table(_tw: usize) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table
}

/// Calculate width for the flexible (expanding) column.
/// `fixed_total` = sum of all fixed column widths.
/// `num_cols` = total number of columns (for border/padding overhead).
fn flexible_width(tw: usize, fixed_total: u16, num_cols: u16) -> u16 {
    // Width::Fixed already includes padding; only add border chars (num_cols+1)
    let overhead = num_cols + 1;
    let available = (tw as u16).saturating_sub(fixed_total + overhead);
    available.clamp(8, 20)
}

fn cell_code(n: usize) -> Cell {
    Cell::new(n)
        .fg(Color::Green)
        .set_alignment(CellAlignment::Right)
}

fn cell_comment(n: usize) -> Cell {
    Cell::new(n)
        .fg(Color::Yellow)
        .set_alignment(CellAlignment::Right)
}

fn cell_blank(n: usize) -> Cell {
    Cell::new(n)
        .fg(Color::Grey)
        .set_alignment(CellAlignment::Right)
}

fn term_width() -> usize {
    crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80)
        .max(60)
}

fn format_num(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

