use crate::theme::Theme;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use ratatui::{
    style::Style,
    text::{Line, Span},
};

fn render_table(rows: &[Vec<String>], lines: &mut Vec<Line<'static>>, theme: &Theme) {
    if rows.is_empty() {
        return;
    }

    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut col_widths = vec![0usize; num_cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }

    let border_style = theme.table_border;
    let header_style = theme.table_header;
    let cell_style = theme.table_cell;

    let build_separator = |left: &str, mid: &str, right: &str, fill: &str| -> Line<'static> {
        let mut spans = vec![Span::styled(left.to_string(), border_style)];
        for (i, &w) in col_widths.iter().enumerate() {
            spans.push(Span::styled(fill.repeat(w + 2), border_style));
            if i < num_cols - 1 {
                spans.push(Span::styled(mid.to_string(), border_style));
            }
        }
        spans.push(Span::styled(right.to_string(), border_style));
        Line::from(spans)
    };

    lines.push(build_separator("┌", "┬", "┐", "─"));

    for (row_idx, row) in rows.iter().enumerate() {
        let style = if row_idx == 0 {
            header_style
        } else {
            cell_style
        };
        let mut spans = vec![Span::styled("│", border_style)];
        for (i, &w) in col_widths.iter().enumerate() {
            let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
            spans.push(Span::styled(
                format!(" {:<width$} ", cell, width = w),
                style,
            ));
            spans.push(Span::styled("│", border_style));
        }
        lines.push(Line::from(spans));

        if row_idx == 0 {
            lines.push(build_separator("├", "┼", "┤", "─"));
        }
    }

    lines.push(build_separator("└", "┴", "┘", "─"));
    lines.push(Line::default());
}

fn render_code_block(
    lang: &str,
    code_lines: &[String],
    lines: &mut Vec<Line<'static>>,
    theme: &Theme,
) {
    let code_style = theme.code_text;
    let line_nr_style = theme.code_line_nr;
    let border_style = theme.border;

    // Strip trailing empty line from parser
    let code_lines: Vec<&String> = code_lines
        .iter()
        .rev()
        .skip_while(|l| l.is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let nr_width = code_lines.len().to_string().len();

    // Language label
    if !lang.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  {lang}"),
            theme.code_label,
        )));
    }

    for (i, line) in code_lines.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:>width$}", i + 1, width = nr_width),
                line_nr_style,
            ),
            Span::styled(" │ ", border_style),
            Span::styled(line.to_string(), code_style),
        ]));
    }

    lines.push(Line::default());
}

pub struct Heading {
    pub text: String,
    pub level: u8,
    pub line_index: usize,
}

pub fn parse(content: &str, theme: &Theme, width: u16) -> (Vec<Line<'static>>, Vec<Heading>) {
    let parser = Parser::new_ext(content, Options::all());

    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut headings: Vec<Heading> = Vec::new();
    let mut current_spans: Vec<Span<'static>> = Vec::new();
    let mut style_stack: Vec<Style> = vec![Style::default()];

    let mut in_table = false;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();

    let mut in_code_block = false;
    let mut code_block_lines: Vec<String> = Vec::new();
    let mut code_block_lang = String::new();

    let mut list_depth: usize = 0;
    let mut link_url = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                let style = match &tag {
                    Tag::Heading { level, .. } => match level {
                        pulldown_cmark::HeadingLevel::H1 => theme.h1,
                        pulldown_cmark::HeadingLevel::H2 => theme.h2,
                        pulldown_cmark::HeadingLevel::H3 => theme.h3,
                        _ => theme.h_other,
                    },
                    Tag::Emphasis => theme.emphasis,
                    Tag::Strong => theme.strong,
                    Tag::BlockQuote(_) => theme.blockquote,
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        code_block_lines.clear();
                        code_block_lang = match kind {
                            pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                            _ => String::new(),
                        };
                        *style_stack.last().unwrap()
                    }
                    Tag::Link { dest_url, .. } => {
                        link_url = dest_url.to_string();
                        theme.link
                    }
                    Tag::List(_) => {
                        list_depth += 1;
                        *style_stack.last().unwrap()
                    }
                    Tag::Item => {
                        let indent = "  ".repeat(list_depth);
                        current_spans.push(Span::styled(format!("{indent}• "), theme.list_bullet));
                        *style_stack.last().unwrap()
                    }
                    Tag::Table(_) => {
                        in_table = true;
                        table_rows.clear();
                        *style_stack.last().unwrap()
                    }
                    Tag::TableHead => {
                        current_row.clear();
                        *style_stack.last().unwrap()
                    }
                    Tag::TableRow => {
                        current_row.clear();
                        *style_stack.last().unwrap()
                    }
                    Tag::TableCell => {
                        current_cell.clear();
                        *style_stack.last().unwrap()
                    }
                    _ => *style_stack.last().unwrap(),
                };
                style_stack.push(style);
            }
            Event::End(tag_end) => {
                style_stack.pop();
                match tag_end {
                    TagEnd::Heading(level) => {
                        let text: String =
                            current_spans.iter().map(|s| s.content.as_ref()).collect();
                        let lvl = match level {
                            pulldown_cmark::HeadingLevel::H1 => 1,
                            pulldown_cmark::HeadingLevel::H2 => 2,
                            pulldown_cmark::HeadingLevel::H3 => 3,
                            pulldown_cmark::HeadingLevel::H4 => 4,
                            pulldown_cmark::HeadingLevel::H5 => 5,
                            pulldown_cmark::HeadingLevel::H6 => 6,
                        };
                        let style = match level {
                            pulldown_cmark::HeadingLevel::H1 => theme.h1,
                            pulldown_cmark::HeadingLevel::H2 => theme.h2,
                            pulldown_cmark::HeadingLevel::H3 => theme.h3,
                            _ => theme.h_other,
                        };
                        current_spans.clear();
                        let text_len = text.len();
                        let line_index = lines.len();
                        match level {
                            pulldown_cmark::HeadingLevel::H1 => {
                                let upper = text.to_uppercase();
                                headings.push(Heading {
                                    text,
                                    level: lvl,
                                    line_index,
                                });
                                lines.push(Line::from(Span::styled(upper, style)));
                                lines.push(Line::from(Span::styled(
                                    "═".repeat(text_len),
                                    theme.border,
                                )));
                            }
                            pulldown_cmark::HeadingLevel::H2 => {
                                headings.push(Heading {
                                    text: text.clone(),
                                    level: lvl,
                                    line_index,
                                });
                                lines.push(Line::from(Span::styled(text, style)));
                                lines.push(Line::from(Span::styled(
                                    "─".repeat(text_len),
                                    theme.border,
                                )));
                            }
                            _ => {
                                headings.push(Heading {
                                    text: text.clone(),
                                    level: lvl,
                                    line_index,
                                });
                                lines.push(Line::from(Span::styled(text, style)));
                            }
                        }
                        lines.push(Line::default());
                    }
                    TagEnd::Paragraph | TagEnd::BlockQuote(_) => {
                        lines.push(Line::from(std::mem::take(&mut current_spans)));
                        lines.push(Line::default());
                    }
                    TagEnd::Item => {
                        lines.push(Line::from(std::mem::take(&mut current_spans)));
                    }
                    TagEnd::List(_) => {
                        list_depth = list_depth.saturating_sub(1);
                        if list_depth == 0 {
                            lines.push(Line::default());
                        }
                    }
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        render_code_block(&code_block_lang, &code_block_lines, &mut lines, theme);
                        code_block_lines.clear();
                    }
                    TagEnd::TableCell => {
                        current_row.push(current_cell.clone());
                        current_cell.clear();
                    }
                    TagEnd::TableHead | TagEnd::TableRow => {
                        table_rows.push(current_row.clone());
                        current_row.clear();
                    }
                    TagEnd::Table => {
                        in_table = false;
                        render_table(&table_rows, &mut lines, theme);
                        table_rows.clear();
                    }
                    TagEnd::Link => {
                        if !link_url.is_empty() {
                            current_spans.push(Span::styled(format!(" ({link_url})"), theme.link));
                            link_url.clear();
                        }
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    for line in text.split('\n') {
                        code_block_lines.push(line.to_string());
                    }
                } else if in_table {
                    current_cell.push_str(&text);
                } else {
                    let style = *style_stack.last().unwrap();
                    for (i, line) in text.lines().enumerate() {
                        if i > 0 {
                            lines.push(Line::from(std::mem::take(&mut current_spans)));
                        }
                        current_spans.push(Span::styled(line.to_string(), style));
                    }
                }
            }
            Event::Code(code) => {
                if in_table {
                    current_cell.push_str(&format!("`{code}`"));
                } else {
                    current_spans.push(Span::styled(format!(" {code} "), theme.inline_code));
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                lines.push(Line::from(std::mem::take(&mut current_spans)));
            }
            Event::Rule => {
                if !current_spans.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current_spans)));
                }
                lines.push(Line::from(Span::styled(
                    "─".repeat(width as usize),
                    theme.rule,
                )));
                lines.push(Line::default());
            }
            _ => {}
        }
    }

    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }

    (lines, headings)
}
