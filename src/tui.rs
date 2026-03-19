use crate::counter::FileResult;
use crate::scanner::{LanguageSummary, ScanSummary};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table, TableState, Tabs},
    Frame, Terminal,
};
use std::io::stdout;

enum Tab {
    Summary,
    ByLanguage,
    Files,
}

struct App {
    summary: ScanSummary,
    files: Vec<FileResult>,
    current_tab: Tab,
    lang_table_state: TableState,
    file_table_state: TableState,
    file_filter: Option<String>,
    filtered_files: Vec<usize>,
    should_quit: bool,
}

impl App {
    fn new(summary: ScanSummary, files: Vec<FileResult>) -> Self {
        let filtered_files: Vec<usize> = (0..files.len()).collect();
        let mut app = App {
            summary,
            files,
            current_tab: Tab::Summary,
            lang_table_state: TableState::default(),
            file_table_state: TableState::default(),
            file_filter: None,
            filtered_files,
            should_quit: false,
        };
        if !app.summary.by_language.is_empty() {
            app.lang_table_state.select(Some(0));
        }
        if !app.files.is_empty() {
            app.file_table_state.select(Some(0));
        }
        app
    }

    fn apply_filter(&mut self) {
        self.filtered_files = match &self.file_filter {
            Some(filter) => self
                .files
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    f.language.to_lowercase().contains(&filter.to_lowercase())
                        || f.extension.to_lowercase().contains(&filter.to_lowercase())
                        || f.path.to_lowercase().contains(&filter.to_lowercase())
                })
                .map(|(i, _)| i)
                .collect(),
            None => (0..self.files.len()).collect(),
        };
        self.file_table_state.select(if self.filtered_files.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Summary => Tab::ByLanguage,
            Tab::ByLanguage => Tab::Files,
            Tab::Files => Tab::Summary,
        };
    }

    fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Summary => Tab::Files,
            Tab::ByLanguage => Tab::Summary,
            Tab::Files => Tab::ByLanguage,
        };
    }

    fn scroll_up(&mut self) {
        match self.current_tab {
            Tab::ByLanguage => {
                if let Some(sel) = self.lang_table_state.selected() {
                    if sel > 0 {
                        self.lang_table_state.select(Some(sel - 1));
                    }
                }
            }
            Tab::Files => {
                if let Some(sel) = self.file_table_state.selected() {
                    if sel > 0 {
                        self.file_table_state.select(Some(sel - 1));
                    }
                }
            }
            _ => {}
        }
    }

    fn scroll_down(&mut self) {
        match self.current_tab {
            Tab::ByLanguage => {
                if let Some(sel) = self.lang_table_state.selected() {
                    if sel + 1 < self.summary.by_language.len() {
                        self.lang_table_state.select(Some(sel + 1));
                    }
                }
            }
            Tab::Files => {
                if let Some(sel) = self.file_table_state.selected() {
                    if sel + 1 < self.filtered_files.len() {
                        self.file_table_state.select(Some(sel + 1));
                    }
                }
            }
            _ => {}
        }
    }

    fn filter_by_selected_language(&mut self) {
        if !matches!(self.current_tab, Tab::ByLanguage) {
            return;
        }
        if let Some(sel) = self.lang_table_state.selected() {
            let lang = self.summary.by_language[sel].language.clone();
            self.file_filter = Some(lang);
            self.apply_filter();
            self.current_tab = Tab::Files;
        }
    }
}

pub fn run_tui(summary: ScanSummary, files: Vec<FileResult>) -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(summary, files);

    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Tab | KeyCode::Right => app.next_tab(),
                    KeyCode::BackTab | KeyCode::Left => app.prev_tab(),
                    KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                    KeyCode::Enter => app.filter_by_selected_language(),
                    KeyCode::Char('c') => {
                        app.file_filter = None;
                        app.apply_filter();
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_tabs(f, app, chunks[0]);

    match app.current_tab {
        Tab::Summary => draw_summary(f, app, chunks[1]),
        Tab::ByLanguage => draw_languages(f, app, chunks[1]),
        Tab::Files => draw_files(f, app, chunks[1]),
    }

    draw_help(f, app, chunks[2]);
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let tab_index = match app.current_tab {
        Tab::Summary => 0,
        Tab::ByLanguage => 1,
        Tab::Files => 2,
    };

    let titles: Vec<Line> = ["Summary", "By Language", "Files"]
        .iter()
        .map(|t| Line::from(*t))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Line Counter "))
        .select(tab_index)
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn draw_summary(f: &mut Frame, app: &App, area: Rect) {
    let s = &app.summary;
    let total = s.total_counts.total();

    let code_pct = if total > 0 {
        s.total_counts.code as f64 / total as f64
    } else {
        0.0
    };
    let comment_pct = if total > 0 {
        s.total_counts.comments as f64 / total as f64
    } else {
        0.0
    };
    let blank_pct = if total > 0 {
        s.total_counts.blank as f64 / total as f64
    } else {
        0.0
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .split(area);

    let stats_text = format!(
        "  Total Files: {}    Total Lines: {}    Languages: {}",
        s.total_files,
        total,
        s.by_language.len()
    );
    let stats = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Overview "),
    );
    f.render_widget(stats, chunks[0]);

    let code_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Code: {} ({:.1}%) ",
            s.total_counts.code,
            code_pct * 100.0
        )))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(code_pct);
    f.render_widget(code_gauge, chunks[1]);

    let comment_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Comments: {} ({:.1}%) ",
            s.total_counts.comments,
            comment_pct * 100.0
        )))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(comment_pct);
    f.render_widget(comment_gauge, chunks[2]);

    let blank_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Blank: {} ({:.1}%) ",
            s.total_counts.blank,
            blank_pct * 100.0
        )))
        .gauge_style(Style::default().fg(Color::DarkGray))
        .ratio(blank_pct);
    f.render_widget(blank_gauge, chunks[3]);

    // Top languages bar
    let top: Vec<Line> = s
        .by_language
        .iter()
        .take(10)
        .map(|lang| {
            let pct = if total > 0 {
                lang.counts.total() as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            let bar_len = (pct * 0.4) as usize;
            let bar: String = "█".repeat(bar_len);
            Line::from(vec![
                Span::styled(
                    format!("  {:<14}", lang.language),
                    Style::default().fg(Color::White),
                ),
                Span::styled(bar, Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(" {:.1}%", pct),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        })
        .collect();

    let top_langs = Paragraph::new(top).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Top Languages "),
    );
    f.render_widget(top_langs, chunks[4]);
}

fn draw_languages(f: &mut Frame, app: &mut App, area: Rect) {
    let header = Row::new(vec!["Language", "Files", "Total", "Code", "Comment", "Blank"])
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let rows: Vec<Row> = app
        .summary
        .by_language
        .iter()
        .map(|lang| make_lang_row(lang, &app.summary))
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(16),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Languages (Enter to filter files) "),
    )
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(table, area, &mut app.lang_table_state);
}

fn make_lang_row<'a>(lang: &LanguageSummary, summary: &ScanSummary) -> Row<'a> {
    let total = summary.total_counts.total();
    let pct = if total > 0 {
        lang.counts.total() as f64 / total as f64 * 100.0
    } else {
        0.0
    };

    Row::new(vec![
        Cell::from(lang.language.clone()),
        Cell::from(lang.files.to_string()),
        Cell::from(format!("{} ({:.0}%)", lang.counts.total(), pct)),
        Cell::from(lang.counts.code.to_string()).style(Style::default().fg(Color::Green)),
        Cell::from(lang.counts.comments.to_string()).style(Style::default().fg(Color::Yellow)),
        Cell::from(lang.counts.blank.to_string()).style(Style::default().fg(Color::DarkGray)),
    ])
}

fn draw_files(f: &mut Frame, app: &mut App, area: Rect) {
    let title = match &app.file_filter {
        Some(filter) => format!(" Files [filter: {filter}] (c to clear) "),
        None => " Files ".to_string(),
    };

    let header = Row::new(vec!["File", "Language", "Total", "Code", "Comment", "Blank"]).style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .filtered_files
        .iter()
        .map(|&i| {
            let file = &app.files[i];
            Row::new(vec![
                Cell::from(file.path.clone()),
                Cell::from(file.language.clone()),
                Cell::from(file.counts.total().to_string()),
                Cell::from(file.counts.code.to_string())
                    .style(Style::default().fg(Color::Green)),
                Cell::from(file.counts.comments.to_string())
                    .style(Style::default().fg(Color::Yellow)),
                Cell::from(file.counts.blank.to_string())
                    .style(Style::default().fg(Color::DarkGray)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Min(40),
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(title))
    .row_highlight_style(Style::default().bg(Color::DarkGray));

    f.render_stateful_widget(table, area, &mut app.file_table_state);
}

fn draw_help(f: &mut Frame, app: &App, area: Rect) {
    let help = match app.current_tab {
        Tab::Summary => "Tab/→: Next tab  |  q: Quit",
        Tab::ByLanguage => "↑↓/jk: Navigate  |  Enter: Filter files  |  Tab/→: Next tab  |  q: Quit",
        Tab::Files => "↑↓/jk: Navigate  |  c: Clear filter  |  Tab/→: Next tab  |  q: Quit",
    };

    let help_widget = Paragraph::new(format!("  {help}"))
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(help_widget, area);
}
