use crate::counter::FileResult;
use crate::scanner::ScanSummary;
use anyhow::Result;
use std::io::Write;

pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

pub fn print_summary(summary: &ScanSummary, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => print_table_summary(summary),
        OutputFormat::Json => print_json(summary),
        OutputFormat::Csv => print_csv_summary(summary),
    }
}

pub fn print_per_file(results: &[FileResult], format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => print_table_per_file(results),
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(results)?;
            println!("{json}");
            Ok(())
        }
        OutputFormat::Csv => print_csv_per_file(results),
    }
}

pub fn print_top_files(results: &[FileResult], count: usize) {
    let mut sorted: Vec<&FileResult> = results.iter().collect();
    sorted.sort_by(|a, b| b.counts.total().cmp(&a.counts.total()));

    let tw = term_width();
    let num_w = 8;
    let nums_total = num_w * 4 + 3; // 4 number columns + 3 separators between them
    let file_w = (tw - 4 - nums_total - 4).max(20); // 4 for borders+pad, 4 for separators

    println!();
    print_header_box(&format!(
        " Top {} Largest Files ",
        count.min(sorted.len())
    ));

    // Header
    print!(
        "  {}{:<file_w$} {} {:>w$} {} {:>w$} {} {:>w$} {} {:>w$}{}",
        "\x1b[1;36m",
        "File",
        "\x1b[0m\x1b[90m|\x1b[0m",
        "Total",
        "\x1b[90m|\x1b[0m",
        "Code",
        "\x1b[90m|\x1b[0m",
        "Comment",
        "\x1b[90m|\x1b[0m",
        "Blank",
        "\x1b[0m",
        file_w = file_w,
        w = num_w,
    );
    println!();
    println!(
        "  \x1b[90m{}\x1b[0m",
        "\u{2500}".repeat(tw.saturating_sub(4))
    );

    for file in sorted.iter().take(count) {
        let path_str = truncate_path(&file.path, file_w);
        print!(
            "  {:<file_w$} \x1b[90m|\x1b[0m {:>w$} \x1b[90m|\x1b[0m \x1b[32m{:>w$}\x1b[0m \x1b[90m|\x1b[0m \x1b[33m{:>w$}\x1b[0m \x1b[90m|\x1b[0m \x1b[90m{:>w$}\x1b[0m",
            path_str,
            file.counts.total(),
            file.counts.code,
            file.counts.comments,
            file.counts.blank,
            file_w = file_w,
            w = num_w,
        );
        println!();
    }
    println!();
}

// ─── Table summary ───────────────────────────────────────────────────────────

fn print_table_summary(summary: &ScanSummary) -> Result<()> {
    let tw = term_width();
    let total_lines = summary.total_counts.total();

    println!();
    print_header_box(" Line Counter ");

    // Overview stats
    println!(
        "  \x1b[1m{}\x1b[0m {} \x1b[90m|\x1b[0m \x1b[1m{}\x1b[0m {} \x1b[90m|\x1b[0m \x1b[1m{}\x1b[0m {}",
        "Files:", summary.total_files,
        "Lines:", total_lines,
        "Languages:", summary.by_language.len(),
    );
    println!();

    // Bar chart overview
    if total_lines > 0 {
        let bar_width = tw.saturating_sub(30).max(10);
        let code_pct = summary.total_counts.code as f64 / total_lines as f64;
        let comment_pct = summary.total_counts.comments as f64 / total_lines as f64;
        let blank_pct = summary.total_counts.blank as f64 / total_lines as f64;

        let code_len = (code_pct * bar_width as f64).round() as usize;
        let comment_len = (comment_pct * bar_width as f64).round() as usize;
        let blank_len = bar_width.saturating_sub(code_len + comment_len);

        print!("  ");
        print!("\x1b[42;30m{}\x1b[0m", " ".repeat(code_len));
        print!("\x1b[43;30m{}\x1b[0m", " ".repeat(comment_len));
        print!("\x1b[100m{}\x1b[0m", " ".repeat(blank_len));
        println!();

        println!(
            "  \x1b[32m\u{25cf} Code {:.1}%\x1b[0m  \x1b[33m\u{25cf} Comments {:.1}%\x1b[0m  \x1b[90m\u{25cf} Blank {:.1}%\x1b[0m",
            code_pct * 100.0,
            comment_pct * 100.0,
            blank_pct * 100.0,
        );
        println!();
    }

    // Language table
    let flag_w = 4;
    let lang_w = 14;
    let num_w = 8;
    let bar_col_w = tw.saturating_sub(4 + flag_w + lang_w + num_w * 5 + 6).max(6); // 6 separators

    // Header
    print!(
        "  {}{:<fw$}{:<lw$} {:>nw$} {} {:>nw$} {} {:>nw$} {} {:>nw$} {} {:>nw$} {} {:<bw$}{}",
        "\x1b[1;36m",
        "",
        "Language",
        "Files",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Total",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Code",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Comment",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Blank",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "",
        "\x1b[0m",
        fw = flag_w,
        lw = lang_w,
        nw = num_w,
        bw = bar_col_w,
    );
    println!();
    println!(
        "  \x1b[90m{}\x1b[0m",
        "\u{2500}".repeat(tw.saturating_sub(4))
    );

    for lang in &summary.by_language {
        let pct = if total_lines > 0 {
            lang.counts.total() as f64 / total_lines as f64
        } else {
            0.0
        };
        let bar_len = (pct * bar_col_w as f64).round() as usize;
        let bar: String = "\u{2588}".repeat(bar_len);
        let flag = language_flag(&lang.language);

        print!(
            "  {:<fw$}{:<lw$} {:>nw$} \x1b[90m|\x1b[0m {:>nw$} \x1b[90m|\x1b[0m \x1b[32m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[33m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[90m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[36m{}\x1b[0m \x1b[90m{:.0}%\x1b[0m",
            flag,
            lang.language,
            lang.files,
            lang.counts.total(),
            lang.counts.code,
            lang.counts.comments,
            lang.counts.blank,
            bar,
            pct * 100.0,
            fw = flag_w,
            lw = lang_w,
            nw = num_w,
        );
        println!();
    }

    // Totals row
    println!(
        "  \x1b[90m{}\x1b[0m",
        "\u{2500}".repeat(tw.saturating_sub(4))
    );
    print!(
        "  \x1b[1m{:<fw$}{:<lw$} {:>nw$} \x1b[90m|\x1b[0m\x1b[1m {:>nw$} \x1b[90m|\x1b[0m \x1b[1;32m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[1;33m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[1;90m{:>nw$}\x1b[0m",
        "",
        "Total",
        summary.total_files,
        summary.total_counts.total(),
        summary.total_counts.code,
        summary.total_counts.comments,
        summary.total_counts.blank,
        fw = flag_w,
        lw = lang_w,
        nw = num_w,
    );
    println!("\x1b[0m");
    println!();

    Ok(())
}

fn print_json(summary: &ScanSummary) -> Result<()> {
    let json = serde_json::to_string_pretty(summary)?;
    println!("{json}");
    Ok(())
}

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

fn print_table_per_file(results: &[FileResult]) -> Result<()> {
    let mut sorted: Vec<&FileResult> = results.iter().collect();
    sorted.sort_by(|a, b| b.counts.total().cmp(&a.counts.total()));

    let tw = term_width();
    let flag_w = 4;
    let lang_w = 12;
    let num_w = 8;
    let nums_total = num_w * 4 + 3;
    let file_w = (tw - 4 - flag_w - lang_w - nums_total - 5).max(16);

    println!();
    print_header_box(" Files Breakdown ");

    // Header
    print!(
        "  {}{:<fw$}{:<file_w$} {:<lw$}{} {:>nw$} {} {:>nw$} {} {:>nw$} {} {:>nw$}{}",
        "\x1b[1;36m",
        "",
        "File",
        "Language",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Total",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Code",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Comment",
        "\x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m",
        "Blank",
        "\x1b[0m",
        fw = flag_w,
        file_w = file_w,
        lw = lang_w,
        nw = num_w,
    );
    println!();
    println!(
        "  \x1b[90m{}\x1b[0m",
        "\u{2500}".repeat(tw.saturating_sub(4))
    );

    for file in &sorted {
        let path_str = truncate_path(&file.path, file_w);
        let flag = language_flag(&file.language);

        print!(
            "  {:<fw$}{:<file_w$} {:<lw$}\x1b[90m|\x1b[0m {:>nw$} \x1b[90m|\x1b[0m \x1b[32m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[33m{:>nw$}\x1b[0m \x1b[90m|\x1b[0m \x1b[90m{:>nw$}\x1b[0m",
            flag,
            path_str,
            file.language,
            file.counts.total(),
            file.counts.code,
            file.counts.comments,
            file.counts.blank,
            fw = flag_w,
            file_w = file_w,
            lw = lang_w,
            nw = num_w,
        );
        println!();
    }
    println!();
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

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn term_width() -> usize {
    crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80)
}

fn truncate_path(path: &str, max_len: usize) -> String {
    if path.len() <= max_len {
        path.to_string()
    } else {
        format!("...{}", &path[path.len() - (max_len - 3)..])
    }
}

fn print_header_box(title: &str) {
    let tw = term_width();
    let line_len = tw.saturating_sub(4);
    let title_len = title.len();
    let pad_left = (line_len.saturating_sub(title_len)) / 2;
    let pad_right = line_len.saturating_sub(title_len + pad_left);

    println!(
        "  \x1b[36m{}\x1b[1m{}\x1b[0m\x1b[36m{}\x1b[0m",
        "\u{2550}".repeat(pad_left),
        title,
        "\u{2550}".repeat(pad_right),
    );
    println!();
}

fn language_flag(language: &str) -> &'static str {
    match language {
        "Rust"       => "\u{1f980} ",  // 🦀
        "Python"     => "\u{1f40d} ",  // 🐍
        "JavaScript" => "\u{26a1} ",   // ⚡ (JS lightning)
        "TypeScript" => "\u{1f535} ",  // 🔵
        "TSX"        => "\u{1f535} ",  // 🔵
        "JSX"        => "\u{26a1} ",   // ⚡
        "Java"       => "\u{2615} ",   // ☕
        "Go"         => "\u{1f439} ",  // 🐹 (Go gopher)
        "C"          => "\u{2699}\u{fe0f} ", // ⚙️
        "C++"        => "\u{2699}\u{fe0f} ", // ⚙️
        "C#"         => "\u{1f3b5} ",  // 🎵
        "Ruby"       => "\u{1f48e} ",  // 💎
        "PHP"        => "\u{1f418} ",  // 🐘
        "Swift"      => "\u{1f426} ",  // 🐦
        "Kotlin"     => "\u{1f4a0} ",  // 💠
        "Dart"       => "\u{1f3af} ",  // 🎯
        "Elixir"     => "\u{1f52e} ",  // 🔮
        "Haskell"    => "\u{03bb} ",   // λ
        "Scala"      => "\u{1f525} ",  // 🔥
        "Lua"        => "\u{1f319} ",  // 🌙
        "Shell"      => "\u{1f41a} ",  // 🐚
        "HTML"       => "\u{1f310} ",  // 🌐
        "CSS"        => "\u{1f3a8} ",  // 🎨
        "SCSS"       => "\u{1f3a8} ",  // 🎨
        "Vue"        => "\u{1f343} ",  // 🍃
        "Svelte"     => "\u{1f525} ",  // 🔥
        "YAML"       => "\u{1f4cb} ",  // 📋
        "TOML"       => "\u{1f4cb} ",  // 📋
        "JSON"       => "\u{1f4e6} ",  // 📦
        "Markdown"   => "\u{1f4dd} ",  // 📝
        "SQL"        => "\u{1f5c4}\u{fe0f} ", // 🗄️
        "XML"        => "\u{1f4c4} ",  // 📄
        "Zig"        => "\u{26a1} ",   // ⚡
        _            => "\u{1f4c1} ",  // 📁
    }
}
