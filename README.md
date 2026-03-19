# lc - Line Counter

A fast, parallel line counter CLI tool with TUI support. Counts code, comments, and blank lines across 33 languages with automatic project detection.

Built with Rust for maximum performance using parallel file processing.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
lc [OPTIONS] [PATH] [COMMAND]
```

By default, `lc` scans the current directory, auto-detects the project type, and displays a summary table.

```bash
# Scan current directory
lc

# Scan a specific path
lc ./src

# Launch interactive TUI
lc --tui

# Show per-file breakdown
lc --per-file

# Output as JSON
lc --format json

# Show top 10 largest files
lc --top 10
```

## Flags & Options

| Flag | Description |
|------|-------------|
| `-i, --include <EXTS>` | File extensions to include (e.g., `rs,js,ts`) |
| `-e, --exclude <PATTERNS>` | Patterns to exclude (e.g., `vendor,dist`) |
| `--no-detect` | Disable auto-detection of project type |
| `--all` | Count all file types, not just known languages |
| `--summary-only` | Show only the summary, not per-file details |
| `--per-file` | Show per-file breakdown |
| `--format <FORMAT>` | Output format: `table`, `json`, `csv` (default: `table`) |
| `--top <N>` | Show top N largest files |
| `--export-json <PATH>` | Export results to a JSON file |
| `--export-csv <PATH>` | Export results to a CSV file |
| `--progress` | Show progress bar during scan |
| `--tui` | Launch interactive TUI |
| `--save` | Save scan to history |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## Subcommands

| Command | Description |
|---------|-------------|
| `history [PATH]` | Show scan history for a directory |

Use `--save` to record scans, then `lc history` to view past results with deltas.

## Supported Languages

| Language | Extensions |
|----------|------------|
| Rust | `.rs` |
| JavaScript | `.js`, `.mjs`, `.cjs` |
| TypeScript | `.ts`, `.mts`, `.cts` |
| TSX | `.tsx` |
| JSX | `.jsx` |
| Python | `.py`, `.pyi` |
| Java | `.java` |
| C | `.c`, `.h` |
| C++ | `.cpp`, `.cxx`, `.cc`, `.hpp`, `.hxx` |
| C# | `.cs` |
| Go | `.go` |
| Ruby | `.rb` |
| PHP | `.php` |
| Swift | `.swift` |
| Kotlin | `.kt`, `.kts` |
| CSS | `.css` |
| SCSS | `.scss`, `.sass` |
| HTML | `.html`, `.htm` |
| XML | `.xml`, `.xsl`, `.xsd` |
| Vue | `.vue` |
| Svelte | `.svelte` |
| Shell | `.sh`, `.bash`, `.zsh` |
| YAML | `.yml`, `.yaml` |
| TOML | `.toml` |
| JSON | `.json`, `.jsonc` |
| Markdown | `.md`, `.mdx` |
| SQL | `.sql` |
| Lua | `.lua` |
| Dart | `.dart` |
| Scala | `.scala` |
| Elixir | `.ex`, `.exs` |
| Haskell | `.hs` |
| Zig | `.zig` |

Each language has configured line and block comment markers for accurate counting.

## Project Auto-Detection

`lc` automatically detects project types by looking for marker files and applies smart defaults for file inclusion and exclusion:

| Project | Marker Files | Auto-Excluded Dirs |
|---------|-------------|-------------------|
| Rust | `Cargo.toml` | `target` |
| Node.js | `package.json` | `node_modules`, `dist`, `.next`, `build`, `coverage` |
| Python | `pyproject.toml`, `setup.py`, `requirements.txt` | `__pycache__`, `.venv`, `venv`, `dist`, `build` |
| Go | `go.mod` | `vendor` |
| Java (Maven) | `pom.xml` | `target` |
| Java (Gradle) | `build.gradle` | `build` |
| .NET | `.sln`, `.csproj` | `bin`, `obj`, `.vs` |
| Ruby | `Gemfile` | `vendor`, `.bundle`, `tmp` |
| PHP | `composer.json` | `vendor`, `node_modules`, `storage` |
| Swift | `Package.swift` | `.build`, `Pods`, `DerivedData` |
| Flutter | `pubspec.yaml` | `.dart_tool`, `build` |
| Elixir | `mix.exs` | `_build`, `deps` |
| Zig | `build.zig` | `zig-cache`, `zig-out` |
| Haskell | `stack.yaml` | `.stack-work`, `dist-newstyle` |
| Scala | `build.sbt` | `target`, `.bsp`, `.metals` |
| C/C++ (CMake) | `CMakeLists.txt` | `build`, `cmake-build-*` |
| C/C++ (Make) | `Makefile` | `build` |

Monorepos with multiple project types are supported — profiles are merged automatically.

Use `--no-detect` to disable auto-detection.

## Interactive TUI

Launch with `lc --tui` for an interactive terminal interface with three tabs:

- **Summary** — Overview card, code/comment/blank gauges, top 10 languages bar chart
- **By Language** — Navigable table of all detected languages
- **Files** — Full file list with filtering by language

**Keyboard shortcuts:**

| Key | Action |
|-----|--------|
| `Tab` / `→` | Next tab |
| `Shift+Tab` / `←` | Previous tab |
| `↑` / `↓` or `j` / `k` | Navigate rows |
| `Enter` | Filter files by selected language |
| `c` | Clear filter |
| `q` / `Esc` | Quit |

## Output Formats

**Table** (default) — Colored terminal table with summary card and language breakdown.

**JSON** — Structured output for programmatic use:
```bash
lc --format json
```

**CSV** — For spreadsheets and data pipelines:
```bash
lc --format csv
```

**Export to file:**
```bash
lc --export-json results.json
lc --export-csv results.csv
```

## Examples

```bash
# Scan only Rust and TOML files
lc -i rs,toml

# Exclude test directories
lc -e tests,fixtures

# Full scan with progress bar, save to history
lc --progress --save

# View scan history
lc history

# JSON export of a specific directory
lc ./src --export-json report.json

# Count all files including unknown types
lc --all
```

## License

MIT
