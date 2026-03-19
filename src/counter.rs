use crate::lang::LangConfig;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineCounts {
    pub code: usize,
    pub comments: usize,
    pub blank: usize,
}

impl LineCounts {
    pub fn total(&self) -> usize {
        self.code + self.comments + self.blank
    }

    pub fn merge(&mut self, other: &LineCounts) {
        self.code += other.code;
        self.comments += other.comments;
        self.blank += other.blank;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    pub path: String,
    pub language: String,
    pub extension: String,
    pub counts: LineCounts,
}

pub fn count_lines(path: &Path, lang: Option<&LangConfig>) -> std::io::Result<LineCounts> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::with_capacity(64 * 1024, file);

    let mut counts = LineCounts::default();
    let mut in_block_comment = false;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            counts.blank += 1;
            continue;
        }

        match lang {
            Some(lang_cfg) => {
                if in_block_comment {
                    counts.comments += 1;
                    if let Some(end) = lang_cfg.block_comment_end {
                        if trimmed.contains(end) {
                            in_block_comment = false;
                        }
                    }
                    continue;
                }

                if let Some(start) = lang_cfg.block_comment_start {
                    if trimmed.starts_with(start) {
                        counts.comments += 1;
                        if let Some(end) = lang_cfg.block_comment_end {
                            if start != end && !trimmed[start.len()..].contains(end) {
                                in_block_comment = true;
                            } else if start == end
                                && !trimmed[start.len()..].trim_end().ends_with(end)
                            {
                                in_block_comment = true;
                            }
                        }
                        continue;
                    }
                }

                let is_line_comment = lang_cfg
                    .line_comments
                    .iter()
                    .any(|prefix| trimmed.starts_with(prefix));

                if is_line_comment {
                    counts.comments += 1;
                } else {
                    counts.code += 1;
                }
            }
            None => {
                counts.code += 1;
            }
        }
    }

    Ok(counts)
}
