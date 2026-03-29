use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub h1: Style,
    pub h2: Style,
    pub h3: Style,
    pub h_other: Style,
    pub emphasis: Style,
    pub strong: Style,
    pub blockquote: Style,
    pub link: Style,
    pub inline_code: Style,
    pub list_bullet: Style,
    pub rule: Style,
    pub border: Style,
    pub code_label: Style,
    pub code_line_nr: Style,
    pub code_text: Style,
    pub table_header: Style,
    pub table_cell: Style,
    pub table_border: Style,
    // Picker
    pub picker_title: Style,
    pub picker_selected: Style,
    pub picker_gutter: Style,
    pub picker_file: Style,
    pub picker_meta: Style,
    pub picker_help: Style,
    pub picker_filter_prompt: Style,
    pub picker_filter_input: Style,
    pub picker_match: Style,
    pub picker_dir: Style,
    // Reader search
    pub search_highlight: Style,
    pub search_active: Style,
    pub search_prompt: Style,
    // Tabs
    pub tab_active: Style,
    pub tab_inactive: Style,
    pub tab_bar_bg: Style,
    // Help bar
    pub help_bar: Style,
    pub help_key: Style,
    // Index (headings list)
    pub index_title: Style,
    pub index_selected: Style,
    pub index_item: Style,
    pub index_gutter: Style,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            h1: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            h2: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            h3: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            h_other: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            emphasis: Style::default().add_modifier(Modifier::ITALIC),
            strong: Style::default().add_modifier(Modifier::BOLD),
            blockquote: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
            link: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
            inline_code: Style::default().fg(Color::Green).bg(Color::Rgb(40, 40, 40)),
            list_bullet: Style::default(),
            rule: Style::default().fg(Color::DarkGray),
            border: Style::default().fg(Color::DarkGray),
            code_label: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            code_line_nr: Style::default().fg(Color::DarkGray),
            code_text: Style::default(),
            table_header: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            table_cell: Style::default(),
            table_border: Style::default().fg(Color::DarkGray),
            picker_title: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            picker_selected: Style::default().fg(Color::Magenta),
            picker_gutter: Style::default().fg(Color::Magenta),
            picker_file: Style::default().fg(Color::Rgb(221, 221, 221)),
            picker_meta: Style::default().fg(Color::DarkGray),
            picker_help: Style::default().fg(Color::DarkGray),
            picker_filter_prompt: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            picker_filter_input: Style::default().fg(Color::White),
            picker_match: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::UNDERLINED),
            picker_dir: Style::default().fg(Color::DarkGray),
            search_highlight: Style::default().fg(Color::Black).bg(Color::Yellow),
            search_active: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(255, 150, 50)),
            search_prompt: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            tab_active: Style::default()
                .fg(Color::Black)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            tab_inactive: Style::default()
                .fg(Color::Rgb(180, 180, 180))
                .bg(Color::Rgb(30, 30, 30)),
            tab_bar_bg: Style::default().bg(Color::Rgb(30, 30, 30)),
            help_bar: Style::default().fg(Color::DarkGray),
            help_key: Style::default()
                .fg(Color::Rgb(200, 200, 200))
                .add_modifier(Modifier::BOLD),
            index_title: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            index_selected: Style::default().fg(Color::Magenta),
            index_item: Style::default().fg(Color::Rgb(180, 180, 180)),
            index_gutter: Style::default().fg(Color::Magenta),
        }
    }

    pub fn light() -> Self {
        Self {
            h1: Style::default()
                .fg(Color::Rgb(170, 0, 170))
                .add_modifier(Modifier::BOLD),
            h2: Style::default()
                .fg(Color::Rgb(0, 130, 155))
                .add_modifier(Modifier::BOLD),
            h3: Style::default()
                .fg(Color::Rgb(0, 130, 0))
                .add_modifier(Modifier::BOLD),
            h_other: Style::default()
                .fg(Color::Rgb(160, 120, 0))
                .add_modifier(Modifier::BOLD),
            emphasis: Style::default().add_modifier(Modifier::ITALIC),
            strong: Style::default().add_modifier(Modifier::BOLD),
            blockquote: Style::default()
                .fg(Color::Rgb(120, 120, 120))
                .add_modifier(Modifier::ITALIC),
            link: Style::default()
                .fg(Color::Rgb(0, 50, 200))
                .add_modifier(Modifier::UNDERLINED),
            inline_code: Style::default()
                .fg(Color::Rgb(0, 120, 0))
                .bg(Color::Rgb(230, 230, 230)),
            list_bullet: Style::default(),
            rule: Style::default().fg(Color::Rgb(180, 180, 180)),
            border: Style::default().fg(Color::Rgb(180, 180, 180)),
            code_label: Style::default()
                .fg(Color::Rgb(0, 130, 155))
                .add_modifier(Modifier::BOLD),
            code_line_nr: Style::default().fg(Color::Rgb(160, 160, 160)),
            code_text: Style::default(),
            table_header: Style::default()
                .fg(Color::Rgb(0, 130, 155))
                .add_modifier(Modifier::BOLD),
            table_cell: Style::default(),
            table_border: Style::default().fg(Color::Rgb(180, 180, 180)),
            picker_title: Style::default()
                .fg(Color::Rgb(170, 0, 170))
                .add_modifier(Modifier::BOLD),
            picker_selected: Style::default().fg(Color::Rgb(170, 0, 170)),
            picker_gutter: Style::default().fg(Color::Rgb(170, 0, 170)),
            picker_file: Style::default().fg(Color::Rgb(30, 30, 30)),
            picker_meta: Style::default().fg(Color::Rgb(140, 140, 140)),
            picker_help: Style::default().fg(Color::Rgb(140, 140, 140)),
            picker_filter_prompt: Style::default()
                .fg(Color::Rgb(0, 130, 155))
                .add_modifier(Modifier::BOLD),
            picker_filter_input: Style::default().fg(Color::Rgb(30, 30, 30)),
            picker_match: Style::default()
                .fg(Color::Rgb(170, 0, 170))
                .add_modifier(Modifier::UNDERLINED),
            picker_dir: Style::default().fg(Color::Rgb(140, 140, 140)),
            search_highlight: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(255, 230, 100)),
            search_active: Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(255, 170, 50)),
            search_prompt: Style::default()
                .fg(Color::Rgb(0, 130, 155))
                .add_modifier(Modifier::BOLD),
            tab_active: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(170, 0, 170))
                .add_modifier(Modifier::BOLD),
            tab_inactive: Style::default()
                .fg(Color::Rgb(80, 80, 80))
                .bg(Color::Rgb(230, 230, 230)),
            tab_bar_bg: Style::default().bg(Color::Rgb(230, 230, 230)),
            help_bar: Style::default().fg(Color::Rgb(140, 140, 140)),
            help_key: Style::default()
                .fg(Color::Rgb(60, 60, 60))
                .add_modifier(Modifier::BOLD),
            index_title: Style::default()
                .fg(Color::Rgb(170, 0, 170))
                .add_modifier(Modifier::BOLD),
            index_selected: Style::default().fg(Color::Rgb(170, 0, 170)),
            index_item: Style::default().fg(Color::Rgb(80, 80, 80)),
            index_gutter: Style::default().fg(Color::Rgb(170, 0, 170)),
        }
    }

    pub fn for_mode(mode: Mode) -> Self {
        match mode {
            Mode::Dark => Self::dark(),
            Mode::Light => Self::light(),
        }
    }
}

/// Detect whether the terminal has a dark or light background.
/// Uses `terminal-light` which handles OSC 11 and `COLORFGBG` fallback.
/// Defaults to dark if detection fails.
pub fn detect_mode() -> Mode {
    terminal_light::luma()
        .map(|luma| if luma > 0.6 { Mode::Light } else { Mode::Dark })
        .unwrap_or(Mode::Dark)
}
