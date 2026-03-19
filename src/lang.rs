use std::path::Path;

#[derive(Debug, Clone)]
pub struct LangConfig {
    pub name: &'static str,
    pub extensions: &'static [&'static str],
    pub line_comments: &'static [&'static str],
    pub block_comment_start: Option<&'static str>,
    pub block_comment_end: Option<&'static str>,
}

pub static LANGUAGES: &[LangConfig] = &[
    LangConfig {
        name: "Rust",
        extensions: &["rs"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "JavaScript",
        extensions: &["js", "mjs", "cjs"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "TypeScript",
        extensions: &["ts", "mts", "cts"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "TSX",
        extensions: &["tsx"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "JSX",
        extensions: &["jsx"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Python",
        extensions: &["py", "pyi"],
        line_comments: &["#"],
        block_comment_start: Some("\"\"\""),
        block_comment_end: Some("\"\"\""),
    },
    LangConfig {
        name: "Java",
        extensions: &["java"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "C",
        extensions: &["c", "h"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "C++",
        extensions: &["cpp", "cxx", "cc", "hpp", "hxx"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "C#",
        extensions: &["cs"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Go",
        extensions: &["go"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Ruby",
        extensions: &["rb"],
        line_comments: &["#"],
        block_comment_start: Some("=begin"),
        block_comment_end: Some("=end"),
    },
    LangConfig {
        name: "PHP",
        extensions: &["php"],
        line_comments: &["//", "#"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Swift",
        extensions: &["swift"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Kotlin",
        extensions: &["kt", "kts"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "CSS",
        extensions: &["css"],
        line_comments: &[],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "SCSS",
        extensions: &["scss", "sass"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "HTML",
        extensions: &["html", "htm"],
        line_comments: &[],
        block_comment_start: Some("<!--"),
        block_comment_end: Some("-->"),
    },
    LangConfig {
        name: "XML",
        extensions: &["xml", "xsl", "xsd"],
        line_comments: &[],
        block_comment_start: Some("<!--"),
        block_comment_end: Some("-->"),
    },
    LangConfig {
        name: "Vue",
        extensions: &["vue"],
        line_comments: &["//"],
        block_comment_start: Some("<!--"),
        block_comment_end: Some("-->"),
    },
    LangConfig {
        name: "Svelte",
        extensions: &["svelte"],
        line_comments: &["//"],
        block_comment_start: Some("<!--"),
        block_comment_end: Some("-->"),
    },
    LangConfig {
        name: "Shell",
        extensions: &["sh", "bash", "zsh"],
        line_comments: &["#"],
        block_comment_start: None,
        block_comment_end: None,
    },
    LangConfig {
        name: "YAML",
        extensions: &["yml", "yaml"],
        line_comments: &["#"],
        block_comment_start: None,
        block_comment_end: None,
    },
    LangConfig {
        name: "TOML",
        extensions: &["toml"],
        line_comments: &["#"],
        block_comment_start: None,
        block_comment_end: None,
    },
    LangConfig {
        name: "JSON",
        extensions: &["json", "jsonc"],
        line_comments: &[],
        block_comment_start: None,
        block_comment_end: None,
    },
    LangConfig {
        name: "Markdown",
        extensions: &["md", "mdx"],
        line_comments: &[],
        block_comment_start: None,
        block_comment_end: None,
    },
    LangConfig {
        name: "SQL",
        extensions: &["sql"],
        line_comments: &["--"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Lua",
        extensions: &["lua"],
        line_comments: &["--"],
        block_comment_start: Some("--[["),
        block_comment_end: Some("]]"),
    },
    LangConfig {
        name: "Dart",
        extensions: &["dart"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Scala",
        extensions: &["scala"],
        line_comments: &["//"],
        block_comment_start: Some("/*"),
        block_comment_end: Some("*/"),
    },
    LangConfig {
        name: "Elixir",
        extensions: &["ex", "exs"],
        line_comments: &["#"],
        block_comment_start: None,
        block_comment_end: None,
    },
    LangConfig {
        name: "Haskell",
        extensions: &["hs"],
        line_comments: &["--"],
        block_comment_start: Some("{-"),
        block_comment_end: Some("-}"),
    },
    LangConfig {
        name: "Zig",
        extensions: &["zig"],
        line_comments: &["//"],
        block_comment_start: None,
        block_comment_end: None,
    },
];

pub fn detect_language(path: &Path) -> Option<&'static LangConfig> {
    let ext = path.extension()?.to_str()?;
    LANGUAGES.iter().find(|lang| lang.extensions.contains(&ext))
}

pub fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
}
