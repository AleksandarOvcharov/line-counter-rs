use crate::counter::{self, FileResult, LineCounts};
use crate::lang;
use anyhow::Result;
use ignore::WalkBuilder;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

pub struct ScanConfig {
    pub root: PathBuf,
    pub include_exts: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub count_all: bool,
}

pub fn collect_files(config: &ScanConfig) -> Vec<PathBuf> {
    let walker = WalkBuilder::new(&config.root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    walker
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .map(|entry| entry.into_path())
        .filter(|path| {
            for pattern in &config.exclude_patterns {
                let path_str = path.to_string_lossy();
                if path_str.contains(pattern.as_str()) {
                    return false;
                }
                if let Ok(glob) = glob::Pattern::new(pattern) {
                    if glob.matches(&path_str) {
                        return false;
                    }
                }
            }
            true
        })
        .filter(|path| {
            if config.count_all || config.include_exts.is_empty() {
                return true;
            }
            lang::get_extension(path)
                .is_some_and(|ext| config.include_exts.iter().any(|inc| inc == &ext))
        })
        .collect()
}

pub fn scan_files(files: &[PathBuf], root: &Path, show_progress: bool) -> Result<Vec<FileResult>> {
    let style = ProgressStyle::default_bar()
        .template("  {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
        .unwrap()
        .progress_chars("\u{2588}\u{2592}\u{2591}");

    let iter = files.par_iter();

    let results: Vec<FileResult> = if show_progress {
        iter.progress_with_style(style)
            .filter_map(|path| scan_one(path, root))
            .collect()
    } else {
        iter.filter_map(|path| scan_one(path, root)).collect()
    };

    Ok(results)
}

fn scan_one(path: &Path, root: &Path) -> Option<FileResult> {
    let lang_cfg = lang::detect_language(path);
    let ext = lang::get_extension(path).unwrap_or_default();
    let language = lang_cfg.map(|l| l.name).unwrap_or("Other");

    match counter::count_lines(path, lang_cfg) {
        Ok(counts) => {
            let rel_path = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");

            Some(FileResult {
                path: rel_path,
                language: language.to_string(),
                extension: ext,
                counts,
            })
        }
        Err(_) => None,
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LanguageSummary {
    pub language: String,
    pub files: usize,
    pub counts: LineCounts,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanSummary {
    pub total_files: usize,
    pub total_counts: LineCounts,
    pub by_language: Vec<LanguageSummary>,
}

pub fn summarize(results: &[FileResult]) -> ScanSummary {
    let mut total_counts = LineCounts::default();
    let mut lang_map: std::collections::HashMap<String, (usize, LineCounts)> =
        std::collections::HashMap::new();

    for r in results {
        total_counts.merge(&r.counts);
        let entry = lang_map
            .entry(r.language.clone())
            .or_insert((0, LineCounts::default()));
        entry.0 += 1;
        entry.1.merge(&r.counts);
    }

    let mut by_language: Vec<LanguageSummary> = lang_map
        .into_iter()
        .map(|(language, (files, counts))| LanguageSummary {
            language,
            files,
            counts,
        })
        .collect();

    by_language.sort_by(|a, b| b.counts.total().cmp(&a.counts.total()));

    ScanSummary {
        total_files: results.len(),
        total_counts,
        by_language,
    }
}
