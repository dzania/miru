mod markdown;
mod theme;

use std::collections::HashSet;
use std::{
    fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};
use theme::Theme;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

#[derive(Parser)]
#[command(name = "miru", about = "A terminal markdown reader")]
struct Cli {
    /// Markdown file(s) to open. If omitted, lists .md files in current dir.
    files: Vec<PathBuf>,
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn relative_time(modified: SystemTime) -> String {
    let elapsed = modified.elapsed().unwrap_or_default();
    let days = elapsed.as_secs() / 86400;
    if days == 0 {
        "today".to_string()
    } else if days == 1 {
        "yesterday".to_string()
    } else if days < 14 {
        format!("{days} days ago")
    } else if days < 60 {
        format!("{} weeks ago", days / 7)
    } else {
        format!("{} months ago", days / 30)
    }
}

fn file_meta(path: &Path) -> (String, String) {
    match fs::metadata(path) {
        Ok(m) => {
            let size = format_size(m.len());
            let time = m
                .modified()
                .map(relative_time)
                .unwrap_or_else(|_| "unknown".to_string());
            (size, time)
        }
        Err(_) => ("? B".to_string(), "unknown".to_string()),
    }
}

// ── Picker ───────────────────────────────────────────────────────────────

fn styled_filename_with_matches(
    name: &str,
    matched_indices: &HashSet<usize>,
    base_style: Style,
    match_style: Style,
) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut buf = String::new();
    for (i, ch) in name.chars().enumerate() {
        if matched_indices.contains(&i) {
            if !buf.is_empty() {
                spans.push(Span::styled(buf.clone(), base_style));
                buf.clear();
            }
            spans.push(Span::styled(ch.to_string(), match_style));
        } else {
            buf.push(ch);
        }
    }
    if !buf.is_empty() {
        spans.push(Span::styled(buf, base_style));
    }
    spans
}

enum PickerAction {
    Open(PathBuf),
    Quit,
}

struct PickerState<'a> {
    files: &'a [PathBuf],
    file_metas: &'a [(String, String)],
    visible: &'a [(usize, HashSet<usize>)],
    selected: usize,
    filter_mode: bool,
    filter_text: &'a str,
}

fn draw_picker(frame: &mut ratatui::Frame, state: &PickerState, theme: &Theme) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    // ── Title / filter bar ──
    let title_line = if state.filter_mode {
        Line::from(vec![
            Span::styled("Filter: ", theme.picker_filter_prompt),
            Span::styled(state.filter_text.to_string(), theme.picker_filter_input),
        ])
    } else {
        Line::from(Span::styled("miru", theme.picker_title))
    };
    frame.render_widget(Paragraph::new(Text::from(title_line)), chunks[0]);

    // ── File list ──
    let list_height = chunks[1].height as usize;
    let visible_files = list_height / 2;
    let scroll_offset = if state.selected >= visible_files {
        state.selected - visible_files + 1
    } else {
        0
    };

    let mut list_lines: Vec<Line<'static>> = Vec::new();
    for (vi, (file_idx, matched_indices)) in state.visible.iter().enumerate() {
        if vi < scroll_offset {
            continue;
        }
        if list_lines.len() >= list_height {
            break;
        }
        let path = &state.files[*file_idx];
        let name = path.display().to_string();
        let is_selected = vi == state.selected;

        // Split into directory prefix and filename
        let (dir_prefix, filename) = if let Some(parent) = path.parent() {
            let parent_str = parent.display().to_string();
            if parent_str == "." {
                (String::new(), name.clone())
            } else {
                let prefix = format!("{}/", parent_str.strip_prefix("./").unwrap_or(&parent_str));
                let fname = path
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or(name.clone());
                (prefix, fname)
            }
        } else {
            (String::new(), name.clone())
        };

        let gutter = if is_selected { "▌ " } else { "  " };
        let base_style = if is_selected {
            theme.picker_selected
        } else {
            theme.picker_file
        };

        let mut spans = vec![Span::styled(
            gutter.to_string(),
            if is_selected {
                theme.picker_gutter
            } else {
                Style::default()
            },
        )];

        if !state.filter_text.is_empty() && !matched_indices.is_empty() {
            if !dir_prefix.is_empty() {
                spans.extend(styled_filename_with_matches(
                    &dir_prefix,
                    matched_indices,
                    theme.picker_dir,
                    theme.picker_match,
                ));
                let shifted: HashSet<usize> = matched_indices
                    .iter()
                    .filter_map(|&i| i.checked_sub(dir_prefix.len()))
                    .collect();
                spans.extend(styled_filename_with_matches(
                    &filename,
                    &shifted,
                    base_style,
                    theme.picker_match,
                ));
            } else {
                spans.extend(styled_filename_with_matches(
                    &filename,
                    matched_indices,
                    base_style,
                    theme.picker_match,
                ));
            }
        } else {
            if !dir_prefix.is_empty() {
                spans.push(Span::styled(dir_prefix, theme.picker_dir));
            }
            spans.push(Span::styled(filename, base_style));
        }
        list_lines.push(Line::from(spans));

        if list_lines.len() < list_height {
            let (size, mtime) = &state.file_metas[*file_idx];
            list_lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("{size}  {mtime}"), theme.picker_meta),
            ]));
        }
    }

    frame.render_widget(Paragraph::new(Text::from(list_lines)), chunks[1]);

    let help = Line::from(Span::styled(
        "j/k navigate · / filter · enter open · q quit",
        theme.picker_help,
    ));
    frame.render_widget(Paragraph::new(Text::from(help)), chunks[2]);
}

fn run_picker(
    terminal: &mut DefaultTerminal,
    files: &[PathBuf],
    file_metas: &[(String, String)],
    theme: &Theme,
) -> io::Result<PickerAction> {
    let mut selected: usize = 0;
    let mut filter_mode = false;
    let mut filter_text = String::new();
    let matcher = SkimMatcherV2::default();

    loop {
        let visible: Vec<(usize, HashSet<usize>)> = if filter_text.is_empty() {
            files
                .iter()
                .enumerate()
                .map(|(i, _)| (i, HashSet::new()))
                .collect()
        } else {
            files
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    let name = p.display().to_string();
                    let display_name = name.strip_prefix("./").unwrap_or(&name);
                    matcher
                        .fuzzy_indices(display_name, &filter_text)
                        .map(|(_, indices)| (i, indices.into_iter().collect()))
                })
                .collect()
        };

        if selected >= visible.len() {
            selected = visible.len().saturating_sub(1);
        }

        terminal.draw(|frame| {
            draw_picker(
                frame,
                &PickerState {
                    files,
                    file_metas,
                    visible: &visible,
                    selected,
                    filter_mode,
                    filter_text: &filter_text,
                },
                theme,
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if filter_mode {
                match key.code {
                    KeyCode::Esc => {
                        filter_mode = false;
                        filter_text.clear();
                    }
                    KeyCode::Backspace => {
                        filter_text.pop();
                        if filter_text.is_empty() {
                            filter_mode = false;
                        }
                    }
                    KeyCode::Enter => {
                        if !visible.is_empty() {
                            return Ok(PickerAction::Open(files[visible[selected].0].clone()));
                        }
                    }
                    KeyCode::Down => {
                        if !visible.is_empty() {
                            selected = (selected + 1).min(visible.len().saturating_sub(1));
                        }
                    }
                    KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Char(c) => {
                        filter_text.push(c);
                        selected = 0;
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(PickerAction::Quit),
                    KeyCode::Char('j') | KeyCode::Down => {
                        if !visible.is_empty() {
                            selected = (selected + 1).min(visible.len().saturating_sub(1));
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Char('/') => {
                        filter_mode = true;
                        filter_text.clear();
                    }
                    KeyCode::Enter => {
                        if !visible.is_empty() {
                            return Ok(PickerAction::Open(files[visible[selected].0].clone()));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// ── Reader ───────────────────────────────────────────────────────────────

struct Tab {
    name: String,
    path: PathBuf,
    lines: Vec<Line<'static>>,
    headings: Vec<markdown::Heading>,
    scroll: usize,
    last_viewport_height: usize,
    search_mode: bool,
    search_query: String,
    match_set: HashSet<usize>,
    matches: Vec<usize>,
    current_match: usize,
}

impl Tab {
    fn new(path: PathBuf, theme: &Theme, width: u16) -> io::Result<Self> {
        let content = fs::read_to_string(&path)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let (lines, headings) = markdown::parse(&content, theme, width);
        Ok(Self {
            name,
            path,
            lines,
            headings,
            scroll: 0,
            last_viewport_height: 24,
            search_mode: false,
            search_query: String::new(),
            match_set: HashSet::new(),
            matches: Vec::new(),
            current_match: 0,
        })
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
    }

    fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    fn clamp_scroll(&mut self, viewport_height: usize) {
        self.last_viewport_height = viewport_height;
        let max = self.lines.len().saturating_sub(viewport_height);
        self.scroll = self.scroll.min(max);
    }

    fn update_matches(&mut self) {
        let query = self.search_query.to_lowercase();
        self.matches = self
            .lines
            .iter()
            .enumerate()
            .filter(|(_, line)| {
                let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
                text.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect();
        self.match_set = self.matches.iter().copied().collect();
        self.current_match = 0;
    }

    fn jump_to_current_match(&mut self) {
        if let Some(&line_idx) = self.matches.get(self.current_match) {
            self.scroll = line_idx;
        }
    }

    fn next_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.matches.len();
            self.jump_to_current_match();
        }
    }

    fn prev_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = if self.current_match == 0 {
                self.matches.len() - 1
            } else {
                self.current_match - 1
            };
            self.jump_to_current_match();
        }
    }
}

struct Reader {
    tabs: Vec<Tab>,
    active: usize,
    show_help: bool,
    index_mode: bool,
    index_selected: usize,
}

impl Reader {
    fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active: 0,
            show_help: false,
            index_mode: false,
            index_selected: 0,
        }
    }

    fn open(&mut self, path: PathBuf, theme: &Theme, width: u16) -> io::Result<()> {
        // If already open, switch to it
        if let Some(idx) = self.tabs.iter().position(|t| t.path == path) {
            self.active = idx;
            return Ok(());
        }
        let tab = Tab::new(path, theme, width)?;
        self.tabs.push(tab);
        self.active = self.tabs.len() - 1;
        Ok(())
    }

    fn close_active(&mut self) {
        if !self.tabs.is_empty() {
            self.tabs.remove(self.active);
            if self.active >= self.tabs.len() && self.active > 0 {
                self.active -= 1;
            }
        }
    }

    fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active = (self.active + 1) % self.tabs.len();
        }
    }

    fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active = if self.active == 0 {
                self.tabs.len() - 1
            } else {
                self.active - 1
            };
        }
    }

    fn tab(&mut self) -> &mut Tab {
        &mut self.tabs[self.active]
    }
}

fn highlight_line(line: &Line<'static>, query: &str, highlight_style: Style) -> Line<'static> {
    let query_lower = query.to_lowercase();
    let query_lower_len = query_lower.len();
    let mut new_spans: Vec<Span<'static>> = Vec::new();

    for span in &line.spans {
        let text = span.content.as_ref();
        let text_lower = text.to_lowercase();
        let base_style = span.style;

        // Walk the lowercased text to find matches, but map byte offsets back
        // to the original text via char boundaries so we slice the correct
        // number of *original* bytes (which may differ from query_lower.len()
        // for case-folding edge cases like ß → ss).
        let mut last_lower = 0;
        let mut last_orig = 0;
        let mut start_lower = 0;
        while start_lower < text_lower.len() {
            if let Some(pos) = text_lower[start_lower..].find(&query_lower) {
                let match_start_lower = start_lower + pos;
                let match_end_lower = match_start_lower + query_lower_len;

                // Map lowercased byte offsets back to original string offsets
                // by walking chars in parallel.
                let orig_start = map_lower_offset_to_orig(
                    text,
                    &text_lower,
                    last_orig,
                    last_lower,
                    match_start_lower,
                );
                let orig_end = map_lower_offset_to_orig(
                    text,
                    &text_lower,
                    orig_start,
                    match_start_lower,
                    match_end_lower,
                );

                if orig_start > last_orig {
                    new_spans.push(Span::styled(
                        text[last_orig..orig_start].to_string(),
                        base_style,
                    ));
                }
                new_spans.push(Span::styled(
                    text[orig_start..orig_end].to_string(),
                    highlight_style,
                ));
                last_lower = match_end_lower;
                last_orig = orig_end;
                start_lower = match_end_lower;
            } else {
                break;
            }
        }
        if last_orig < text.len() {
            new_spans.push(Span::styled(text[last_orig..].to_string(), base_style));
        }
    }

    Line::from(new_spans)
}

/// Map a byte offset in the lowercased string back to the corresponding byte
/// offset in the original string, starting from known aligned positions.
fn map_lower_offset_to_orig(
    orig: &str,
    lower: &str,
    mut orig_pos: usize,
    mut lower_pos: usize,
    target_lower_pos: usize,
) -> usize {
    let mut orig_chars = orig[orig_pos..].chars();
    let mut lower_chars = lower[lower_pos..].chars();
    while lower_pos < target_lower_pos {
        let oc = orig_chars.next().unwrap();
        // A single original char may lowercase to multiple chars (e.g. İ → i̇).
        // Consume all lowercased chars produced by this one original char.
        let mut consumed_lower = 0;
        for lc in oc.to_lowercase() {
            let expected = lower_chars.next().unwrap();
            debug_assert_eq!(lc, expected);
            consumed_lower += expected.len_utf8();
        }
        orig_pos += oc.len_utf8();
        lower_pos += consumed_lower;
    }
    orig_pos
}

fn render_tab_bar(reader: &Reader, theme: &Theme, width: u16) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut used = 0usize;

    for (i, tab) in reader.tabs.iter().enumerate() {
        let style = if i == reader.active {
            theme.tab_active
        } else {
            theme.tab_inactive
        };
        let label = format!(" {} ", tab.name);
        used += label.len();
        spans.push(Span::styled(label, style));

        if i < reader.tabs.len() - 1 {
            spans.push(Span::styled(" ", theme.tab_bar_bg));
            used += 1;
        }
    }

    // Fill remaining width with background
    let remaining = (width as usize).saturating_sub(used);
    if remaining > 0 {
        spans.push(Span::styled(" ".repeat(remaining), theme.tab_bar_bg));
    }

    Line::from(spans)
}

fn render_help_bar(theme: &Theme) -> Line<'static> {
    let binds: &[(&str, &str)] = &[
        ("j/k", "scroll"),
        ("d/u", "page"),
        ("/", "search"),
        ("h/l", "tabs"),
        ("tab", "index"),
        ("^T", "open"),
        ("w", "close"),
        ("q", "quit"),
    ];

    let mut spans: Vec<Span<'static>> = Vec::new();
    for (i, (key, desc)) in binds.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", theme.help_bar));
        }
        spans.push(Span::styled(key.to_string(), theme.help_key));
        spans.push(Span::styled(format!(" {desc}"), theme.help_bar));
    }
    Line::from(spans)
}

enum ReaderAction {
    BackToPicker,
    OpenNewTab,
    Quit,
}

fn run_reader(
    terminal: &mut DefaultTerminal,
    reader: &mut Reader,
    theme: &Theme,
) -> io::Result<ReaderAction> {
    loop {
        let index_mode = reader.index_mode;

        terminal.draw(|frame| {
            let area = frame.area();

            if index_mode {
                // ── Index (headings list) ──
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ])
                    .split(area);

                let title = Line::from(Span::styled("Index", theme.index_title));
                frame.render_widget(Paragraph::new(Text::from(title)), chunks[0]);

                let headings = &reader.tabs[reader.active].headings;
                let list_height = chunks[1].height as usize;
                let selected = reader.index_selected;
                let scroll_offset = if selected >= list_height {
                    selected - list_height + 1
                } else {
                    0
                };

                let mut list_lines: Vec<Line<'static>> = Vec::new();
                for (i, heading) in headings.iter().enumerate().skip(scroll_offset) {
                    if list_lines.len() >= list_height {
                        break;
                    }
                    let is_selected = i == selected;
                    let indent = "  ".repeat((heading.level as usize).saturating_sub(1));
                    let gutter = if is_selected { "▌ " } else { "  " };
                    let style = if is_selected {
                        theme.index_selected
                    } else {
                        theme.index_item
                    };

                    let mut spans = vec![Span::styled(
                        gutter.to_string(),
                        if is_selected {
                            theme.index_gutter
                        } else {
                            Style::default()
                        },
                    )];
                    spans.push(Span::styled(format!("{}{}", indent, heading.text), style));
                    list_lines.push(Line::from(spans));
                }

                frame.render_widget(Paragraph::new(Text::from(list_lines)), chunks[1]);

                let help = Line::from(Span::styled(
                    "j/k navigate · enter jump · tab/esc close",
                    theme.help_bar,
                ));
                frame.render_widget(Paragraph::new(Text::from(help)), chunks[2]);
            } else {
                // ── Normal reader view ──
                let search = reader.tabs[reader.active].search_mode;
                let show_help = reader.show_help;

                let mut constraints = vec![
                    Constraint::Length(1), // tab bar
                    Constraint::Min(1),    // content
                ];
                if search {
                    constraints.push(Constraint::Length(1));
                }
                constraints.push(Constraint::Length(1));

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(area);

                let mut chunk_idx = 0;

                let tab_line = render_tab_bar(reader, theme, area.width);
                frame.render_widget(Paragraph::new(Text::from(tab_line)), chunks[chunk_idx]);
                chunk_idx += 1;

                let content_area = chunks[chunk_idx];
                chunk_idx += 1;

                let tab = &mut reader.tabs[reader.active];
                let viewport_height = content_area.height as usize;
                tab.clamp_scroll(viewport_height);

                let has_search = !tab.search_query.is_empty() && !tab.match_set.is_empty();
                let active_match_line = tab.matches.get(tab.current_match).copied();
                let search_info = if has_search {
                    Some(format!(
                        "{}/{}",
                        if tab.matches.is_empty() {
                            0
                        } else {
                            tab.current_match + 1
                        },
                        tab.matches.len(),
                    ))
                } else {
                    None
                };

                let display_lines: Vec<Line<'static>> = tab
                    .lines
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        if has_search && tab.match_set.contains(&i) {
                            let style = if Some(i) == active_match_line {
                                theme.search_active
                            } else {
                                theme.search_highlight
                            };
                            highlight_line(line, &tab.search_query, style)
                        } else {
                            line.clone()
                        }
                    })
                    .collect();

                let paragraph =
                    Paragraph::new(Text::from(display_lines)).scroll((tab.scroll as u16, 0));
                frame.render_widget(paragraph, content_area);

                let mut scrollbar_state = ScrollbarState::new(tab.lines.len()).position(tab.scroll);
                frame.render_stateful_widget(
                    Scrollbar::new(ScrollbarOrientation::VerticalRight),
                    content_area,
                    &mut scrollbar_state,
                );

                if search {
                    let mut search_spans = vec![
                        Span::styled("/ ", theme.search_prompt),
                        Span::raw(tab.search_query.clone()),
                    ];
                    if let Some(ref info) = search_info {
                        search_spans.push(Span::styled(format!("  [{info}]"), theme.help_bar));
                    }
                    let search_line = Line::from(search_spans);
                    frame.render_widget(Paragraph::new(Text::from(search_line)), chunks[chunk_idx]);
                    chunk_idx += 1;
                }

                let bottom_line = if show_help {
                    render_help_bar(theme)
                } else {
                    Line::from(Span::styled("? help", theme.help_bar))
                };
                frame.render_widget(Paragraph::new(Text::from(bottom_line)), chunks[chunk_idx]);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if reader.index_mode {
                let heading_count = reader.tabs[reader.active].headings.len();
                match key.code {
                    KeyCode::Esc | KeyCode::Tab => {
                        reader.index_mode = false;
                    }
                    KeyCode::Char('q') => return Ok(ReaderAction::Quit),
                    KeyCode::Char('j') | KeyCode::Down => {
                        if heading_count > 0 {
                            reader.index_selected =
                                (reader.index_selected + 1).min(heading_count - 1);
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        reader.index_selected = reader.index_selected.saturating_sub(1);
                    }
                    KeyCode::Char('g') => reader.index_selected = 0,
                    KeyCode::Char('G') => {
                        reader.index_selected = heading_count.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        if let Some(heading) = reader.tabs[reader.active]
                            .headings
                            .get(reader.index_selected)
                        {
                            reader.tabs[reader.active].scroll = heading.line_index;
                        }
                        reader.index_mode = false;
                    }
                    _ => {}
                }
            } else if reader.tabs[reader.active].search_mode {
                let tab = reader.tab();
                match key.code {
                    KeyCode::Esc => {
                        tab.search_mode = false;
                        tab.search_query.clear();
                        tab.matches.clear();
                        tab.match_set.clear();
                        tab.current_match = 0;
                    }
                    KeyCode::Enter => {
                        tab.search_mode = false;
                        if !tab.matches.is_empty() {
                            tab.jump_to_current_match();
                        }
                    }
                    KeyCode::Backspace => {
                        tab.search_query.pop();
                        if tab.search_query.is_empty() {
                            tab.matches.clear();
                            tab.match_set.clear();
                        } else {
                            tab.update_matches();
                        }
                    }
                    KeyCode::Char(c) => {
                        tab.search_query.push(c);
                        tab.update_matches();
                    }
                    _ => {}
                }
            } else if key.code == KeyCode::Char('t')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                return Ok(ReaderAction::OpenNewTab);
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(ReaderAction::Quit),
                    KeyCode::Backspace => return Ok(ReaderAction::BackToPicker),
                    KeyCode::Char('?') => reader.show_help = !reader.show_help,
                    KeyCode::Char('h') => reader.prev_tab(),
                    KeyCode::Char('l') => reader.next_tab(),
                    KeyCode::Tab => {
                        reader.index_mode = true;
                        reader.index_selected = 0;
                    }
                    KeyCode::Char('w') => {
                        reader.close_active();
                        if reader.tabs.is_empty() {
                            return Ok(ReaderAction::BackToPicker);
                        }
                    }
                    _ => {
                        let tab = reader.tab();
                        match key.code {
                            KeyCode::Char('j') | KeyCode::Down => tab.scroll_down(1),
                            KeyCode::Char('k') | KeyCode::Up => tab.scroll_up(1),
                            KeyCode::Char('d') => tab.scroll_down(tab.last_viewport_height / 2),
                            KeyCode::Char('u') => tab.scroll_up(tab.last_viewport_height / 2),
                            KeyCode::Char('g') => tab.scroll = 0,
                            KeyCode::Char('G') => tab.scroll = tab.lines.len(),
                            KeyCode::Char('/') => {
                                tab.search_mode = true;
                                tab.search_query.clear();
                                tab.matches.clear();
                                tab.match_set.clear();
                                tab.current_match = 0;
                            }
                            KeyCode::Char('n') => tab.next_match(),
                            KeyCode::Char('N') => tab.prev_match(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

// ── File discovery ───────────────────────────────────────────────────────

fn find_markdown_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs = vec![PathBuf::from(".")];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path
                .extension()
                .map(|ext| ext == "md" || ext == "markdown")
                .unwrap_or(false)
            {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

// ── Main ─────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mode = theme::detect_mode();
    let theme = Theme::for_mode(mode);

    let mut terminal = ratatui::init();
    let term_width = crossterm::terminal::size().map(|(w, _)| w).unwrap_or(80);
    let mut reader = Reader::new();

    // If files passed on CLI, open them directly as tabs
    if !cli.files.is_empty() {
        for f in &cli.files {
            reader.open(f.clone(), &theme, term_width)?;
        }
    }

    let md_files = find_markdown_files();
    let file_metas: Vec<(String, String)> = md_files.iter().map(|p| file_meta(p)).collect();

    // If no CLI files and no local .md files, bail
    if reader.tabs.is_empty() && md_files.is_empty() {
        ratatui::restore();
        eprintln!("No markdown files found in current directory or subdirectories.");
        std::process::exit(1);
    }

    // Main loop: picker ↔ reader
    loop {
        // If no tabs open, show the picker first
        if reader.tabs.is_empty() {
            if md_files.is_empty() {
                break;
            }
            match run_picker(&mut terminal, &md_files, &file_metas, &theme)? {
                PickerAction::Quit => break,
                PickerAction::Open(path) => {
                    reader.open(path, &theme, term_width)?;
                }
            }
        }

        match run_reader(&mut terminal, &mut reader, &theme)? {
            ReaderAction::Quit => break,
            ReaderAction::BackToPicker => {
                if md_files.is_empty() {
                    break;
                }
                continue;
            }
            ReaderAction::OpenNewTab => {
                if md_files.is_empty() {
                    continue;
                }
                match run_picker(&mut terminal, &md_files, &file_metas, &theme)? {
                    PickerAction::Quit => break,
                    PickerAction::Open(path) => {
                        reader.open(path, &theme, term_width)?;
                    }
                }
            }
        }
    }

    ratatui::restore();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn find_markdown_files_discovers_nested() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path();

        // Create files at root
        fs::write(base.join("root.md"), "# Root").unwrap();
        fs::write(base.join("ignore.txt"), "not md").unwrap();

        // Create nested dirs
        fs::create_dir_all(base.join("sub")).unwrap();
        fs::write(base.join("sub/nested.md"), "# Nested").unwrap();

        fs::create_dir_all(base.join("sub/deep")).unwrap();
        fs::write(base.join("sub/deep/deep.markdown"), "# Deep").unwrap();

        // Run from the temp dir
        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(base).unwrap();
        let files = find_markdown_files();
        std::env::set_current_dir(original).unwrap();

        let names: Vec<String> = files.iter().map(|p| p.display().to_string()).collect();
        assert!(names.iter().any(|n| n.contains("root.md")));
        assert!(names.iter().any(|n| n.contains("nested.md")));
        assert!(names.iter().any(|n| n.contains("deep.markdown")));
        assert!(!names.iter().any(|n| n.contains("ignore.txt")));
    }

    #[test]
    fn find_markdown_files_returns_sorted() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path();

        fs::write(base.join("z.md"), "").unwrap();
        fs::write(base.join("a.md"), "").unwrap();
        fs::create_dir_all(base.join("m")).unwrap();
        fs::write(base.join("m/b.md"), "").unwrap();

        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(base).unwrap();
        let files = find_markdown_files();
        std::env::set_current_dir(original).unwrap();

        let names: Vec<String> = files.iter().map(|p| p.display().to_string()).collect();
        assert_eq!(names, {
            let mut sorted = names.clone();
            sorted.sort();
            sorted
        });
    }

    #[test]
    fn styled_filename_no_matches() {
        let spans = styled_filename_with_matches(
            "readme.md",
            &HashSet::new(),
            Style::default(),
            Style::default(),
        );
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content.as_ref(), "readme.md");
    }

    #[test]
    fn styled_filename_with_some_matches() {
        let matched: HashSet<usize> = [0, 4].into_iter().collect();
        let spans =
            styled_filename_with_matches("readme.md", &matched, Style::default(), Style::default());
        // 'r' matched, 'ead' normal, 'm' matched, 'e.md' normal
        let text: String = spans.iter().map(|s| s.content.to_string()).collect();
        assert_eq!(text, "readme.md");
        assert!(spans.len() > 1);
    }
}
