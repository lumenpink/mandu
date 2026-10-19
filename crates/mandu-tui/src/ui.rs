//! Layout dos 3 modos e renderização TUI.

use crate::app::TuiApp;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Source,
    Split,
    Preview,
}

impl ViewMode {
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Source => Self::Split,
            Self::Split => Self::Preview,
            Self::Preview => Self::Source,
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Source => "Modo: Escrita",
            Self::Split => "Modo: Split",
            Self::Preview => "Modo: Preview",
        }
    }
}

pub fn render_app(app: &TuiApp, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(frame.area());

    let main_area = chunks[0];
    let status_area = chunks[1];

    match app.mode {
        ViewMode::Source => {
            frame.render_widget(&app.editor.textarea, main_area);
        }
        ViewMode::Split => {
            let split_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_area);

            frame.render_widget(&app.editor.textarea, split_chunks[0]);

            let text = app.editor.get_text();
            let preview_widget =
                render_preview_pane(&text, "Visualização Joaju", split_chunks[1].width);
            frame.render_widget(preview_widget, split_chunks[1]);
        }
        ViewMode::Preview => {
            let text = app.editor.get_text();
            let preview_widget =
                render_preview_pane(&text, "Visualização Joaju (Modo Leitura)", main_area.width);
            frame.render_widget(preview_widget, main_area);
        }
    }

    // Barra de status de 1 linha
    let status_bar = Paragraph::new(Line::from(vec![Span::styled(
        app.status_text(),
        Style::default()
            .bg(Color::Rgb(30, 30, 35))
            .fg(Color::Rgb(220, 220, 225))
            .add_modifier(Modifier::BOLD),
    )]));
    frame.render_widget(status_bar, status_area);

    // Modal de busca overlay (Ctrl+O)
    if app.is_search_open {
        render_search_modal(app, frame);
    }

    // Modal de Peek overlay (Ctrl+P)
    if app.is_peek_open {
        render_peek_modal(app, frame);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_search_modal(app: &TuiApp, frame: &mut Frame) {
    let area = centered_rect(70, 60, frame.area());
    frame.render_widget(Clear, area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .title(" [ 🔍 Buscar / Abrir Nota (Ctrl+O) ] ")
        .style(Style::default().bg(Color::Rgb(20, 20, 24)));
    frame.render_widget(modal_block.clone(), area);

    let inner = modal_block.inner(area);
    let modal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(inner);

    // Linha de consulta
    let query_line = Line::from(vec![
        Span::styled(
            " > ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(&app.search.query),
        Span::styled("_", Style::default().add_modifier(Modifier::RAPID_BLINK)),
    ]);
    frame.render_widget(Paragraph::new(query_line), modal_chunks[0]);

    // Lista de resultados
    let mut lines = Vec::new();
    if app.search.results.is_empty() {
        lines.push(Line::from(Span::styled(
            "  Nenhuma nota encontrada.",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (i, item) in app.search.results.iter().enumerate().take(10) {
            let is_selected = i == app.search.selected_index;
            let file_stem = item
                .path
                .file_name()
                .map(|f| f.to_string_lossy())
                .unwrap_or_default();

            let line_style = if is_selected {
                Style::default()
                    .bg(Color::Rgb(50, 50, 70))
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let prefix = if is_selected { "▶ " } else { "  " };
            lines.push(Line::from(vec![
                Span::styled(prefix, line_style),
                Span::styled(format!("{} ", item.title), line_style),
                Span::styled(
                    format!("({file_stem})"),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }
    }
    frame.render_widget(Paragraph::new(lines), modal_chunks[1]);

    // Rodapé de instruções
    let footer = Line::from(vec![Span::styled(
        " ↑/↓ Navegar  |  Enter Abrir  |  Esc Cancelar",
        Style::default().fg(Color::DarkGray),
    )]);
    frame.render_widget(Paragraph::new(footer), modal_chunks[2]);
}

fn render_peek_modal(app: &TuiApp, frame: &mut Frame) {
    let area = centered_rect(65, 60, frame.area());
    frame.render_widget(Clear, area);

    if let Some(ref peek) = app.peek_content {
        let title = format!(
            " 🔍 Espiar (Peek): {} [Enter: Abrir | Esc: Fechar] ",
            peek.title
        );
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(title);
        let paragraph = Paragraph::new(peek.content.as_str())
            .block(block)
            .style(Style::default().fg(Color::White));
        frame.render_widget(paragraph, area);
    }
}

fn parse_inline_spans(line: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut rest = line;

    while !rest.is_empty() {
        if let Some(start) = rest.find("[[") {
            if let Some(end) = rest[start..].find("]]") {
                if start > 0 {
                    spans.push(Span::styled(
                        rest[..start].to_string(),
                        Style::default().fg(Color::Rgb(220, 220, 225)),
                    ));
                }
                let link_text = &rest[start..start + end + 2];
                let display = if let Some(pipe) = link_text.find('|') {
                    format!("🔗 {}", link_text[pipe + 1..link_text.len() - 2].trim())
                } else {
                    format!("🔗 {}", link_text[2..link_text.len() - 2].trim())
                };
                spans.push(Span::styled(
                    display,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::UNDERLINED),
                ));
                rest = &rest[start + end + 2..];
                continue;
            }
        }

        if let Some(start) = rest.find('@') {
            if start == 0
                || rest[..start]
                    .ends_with(|c: char| c.is_whitespace() || c == '(' || c == '[' || c == '{')
            {
                if start > 0 {
                    spans.push(Span::styled(
                        rest[..start].to_string(),
                        Style::default().fg(Color::Rgb(220, 220, 225)),
                    ));
                }
                let after = &rest[start..];
                let end = after
                    .find(|c: char| {
                        c.is_whitespace()
                            || c == ','
                            || c == '.'
                            || c == ')'
                            || c == ']'
                            || c == ';'
                    })
                    .unwrap_or(after.len());
                let cite = &after[..end];
                spans.push(Span::styled(
                    cite.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
                rest = &after[end..];
                continue;
            }
        }

        spans.push(Span::styled(
            rest.to_string(),
            Style::default().fg(Color::Rgb(220, 220, 225)),
        ));
        break;
    }

    if spans.is_empty() {
        spans.push(Span::styled(
            line.to_string(),
            Style::default().fg(Color::Rgb(220, 220, 225)),
        ));
    }

    spans
}

fn render_preview_pane<'a>(text: &'a str, title: &'a str, pane_width: u16) -> Paragraph<'a> {
    let mut lines = Vec::new();
    let (meta_opt, raw_body) = mandu_core::vault::Vault::extract_metadata(text);

    let number_headings = meta_opt.as_ref().is_some_and(|m| m.number_headings);

    if let Some(ref meta) = meta_opt {
        let mut meta_spans = vec![Span::styled(" [ 📑 ", Style::default().fg(Color::Cyan))];
        if let Some(ref t) = meta.title {
            meta_spans.push(Span::styled(
                t.clone(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        if !meta.tags.is_empty() {
            meta_spans.push(Span::styled(" | 🏷 ", Style::default().fg(Color::Yellow)));
            meta_spans.push(Span::styled(
                meta.tags.join(", "),
                Style::default().fg(Color::LightYellow),
            ));
        }
        if number_headings {
            meta_spans.push(Span::styled(
                " | 🔢 1.1 ",
                Style::default().fg(Color::Green),
            ));
        }
        meta_spans.push(Span::styled(" ]", Style::default().fg(Color::Cyan)));
        lines.push(Line::from(meta_spans));
        lines.push(Line::from(""));
    }

    let processed_body = if number_headings {
        mandu_core::vault::apply_heading_numbering(raw_body)
    } else {
        raw_body.to_string()
    };

    let card_width = (pane_width as usize).saturating_sub(4).clamp(24, 76);

    let mut body_lines = processed_body.lines().peekable();
    while let Some(line) = body_lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            let engine = trimmed.trim_start_matches('`').trim();
            let mut code_lines = Vec::new();
            while let Some(next_line) = body_lines.peek() {
                if next_line.trim_start().starts_with("```") {
                    body_lines.next();
                    break;
                }
                code_lines.push(body_lines.next().unwrap());
            }
            let code = code_lines.join("\n");
            match engine {
                "unfurl" | "link" => {
                    let mut parts = code.split('|').map(str::trim);
                    let url = parts.next().unwrap_or("https://mandu.2lp.in");
                    let title = parts.next().unwrap_or(url);
                    let desc = parts.next();
                    lines.extend(crate::blocks::render_link_card(
                        title, url, desc, card_width,
                    ));
                }
                "table" | "csv" => {
                    lines.extend(crate::blocks::render_table_block(&code, card_width));
                }
                "pedigree" | "heredograma" | "kinship2" | "pedtools" => {
                    lines.extend(crate::blocks::render_pedigree_summary(&code, card_width));
                }
                _ => {
                    let label = if engine.is_empty() { "código" } else { engine };
                    lines.extend(crate::blocks::render_code_box(label, &code, card_width));
                }
            }
            continue;
        }

        if (trimmed.starts_with("http://") || trimmed.starts_with("https://"))
            && !trimmed.contains(' ')
        {
            lines.extend(crate::blocks::render_link_card(
                trimmed, trimmed, None, card_width,
            ));
            continue;
        }

        if let Some(stripped) = trimmed.strip_prefix("# ") {
            lines.push(Line::from(vec![
                Span::styled(
                    "▌ ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = trimmed.strip_prefix("## ") {
            lines.push(Line::from(vec![
                Span::styled(
                    "▍ ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = trimmed.strip_prefix("### ") {
            lines.push(Line::from(vec![
                Span::styled(
                    "▎ ",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = trimmed.strip_prefix('>') {
            let mut quote_spans = vec![Span::styled("▍ ", Style::default().fg(Color::LightYellow))];
            quote_spans.push(Span::styled(
                stripped.to_string(),
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ));
            lines.push(Line::from(quote_spans));
        } else if let Some(stripped) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            let mut list_spans = vec![Span::styled("  • ", Style::default().fg(Color::Cyan))];
            list_spans.extend(parse_inline_spans(stripped));
            lines.push(Line::from(list_spans));
        } else if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            lines.push(Line::from(vec![Span::styled(
                "──────────────────────────────────────────",
                Style::default().fg(Color::DarkGray),
            )]));
        } else {
            lines.push(Line::from(parse_inline_spans(line)));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {title} "));

    Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false })
}
