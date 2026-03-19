use crate::scanner::ScanSummary;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub root: String,
    pub summary: ScanSummary,
}

fn history_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("line-counter")
        .join("history.json")
}

pub fn save_entry(root: &str, summary: &ScanSummary) -> Result<()> {
    let path = history_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut entries = load_history()?;
    entries.push(HistoryEntry {
        timestamp: Utc::now(),
        root: root.to_string(),
        summary: summary.clone(),
    });

    // Keep last 100 entries
    if entries.len() > 100 {
        entries.drain(..entries.len() - 100);
    }

    let json = serde_json::to_string_pretty(&entries)?;
    std::fs::write(&path, json)?;
    Ok(())
}

pub fn load_history() -> Result<Vec<HistoryEntry>> {
    let path = history_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(&path)?;
    let entries: Vec<HistoryEntry> = serde_json::from_str(&data)?;
    Ok(entries)
}

pub fn show_history(root: &str) -> Result<()> {
    let entries = load_history()?;
    let matching: Vec<&HistoryEntry> = entries.iter().filter(|e| e.root == root).collect();

    if matching.is_empty() {
        println!("  No history found for {root}");
        return Ok(());
    }

    let tw = crossterm::terminal::size()
        .map(|(w, _)| w as usize)
        .unwrap_or(80);

    println!();
    let line = "\u{2550}".repeat(tw.saturating_sub(4));
    println!("  \x1b[36m{}\x1b[0m", line);
    println!();

    println!("  \x1b[1mScan history for\x1b[0m {root}");
    println!();

    println!(
        "  \x1b[1;36m{:<24} {:>8} \x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m {:>10} \x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m {:>10} \x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m {:>10} \x1b[0m\x1b[90m|\x1b[0m\x1b[1;36m {:>10}\x1b[0m",
        "Timestamp", "Files", "Total", "Code", "Comment", "Blank"
    );
    println!(
        "  \x1b[90m{}\x1b[0m",
        "\u{2500}".repeat(tw.saturating_sub(4))
    );

    for entry in &matching {
        let s = &entry.summary;
        println!(
            "  {:<24} {:>8} \x1b[90m|\x1b[0m {:>10} \x1b[90m|\x1b[0m \x1b[32m{:>10}\x1b[0m \x1b[90m|\x1b[0m \x1b[33m{:>10}\x1b[0m \x1b[90m|\x1b[0m \x1b[90m{:>10}\x1b[0m",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            s.total_files,
            s.total_counts.total(),
            s.total_counts.code,
            s.total_counts.comments,
            s.total_counts.blank,
        );
    }

    if matching.len() >= 2 {
        let prev = &matching[matching.len() - 2].summary.total_counts;
        let curr = &matching[matching.len() - 1].summary.total_counts;

        let delta_total = curr.total() as i64 - prev.total() as i64;
        let delta_code = curr.code as i64 - prev.code as i64;

        let sign = |v: i64| if v >= 0 { "+" } else { "" };
        let color_total = if delta_total >= 0 { "32" } else { "31" };
        let color_code = if delta_code >= 0 { "32" } else { "31" };
        println!(
            "\n  Change: \x1b[{color_total}m{}{} total\x1b[0m, \x1b[{color_code}m{}{} code\x1b[0m",
            sign(delta_total), delta_total,
            sign(delta_code), delta_code,
        );
    }

    println!();
    Ok(())
}
