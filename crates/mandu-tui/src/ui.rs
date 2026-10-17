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

            let preview_block = Block::default()
                .borders(Borders::LEFT)
                .title(" Visualização Joaju ");
            let preview_text = Paragraph::new("Preview estruturado do Joaju (Typst / Documentos)")
                .block(preview_block);
            frame.render_widget(preview_text, split_chunks[1]);
        }
        ViewMode::Preview => {
            let preview_block = Block::default()
                .borders(Borders::ALL)
                .title(" Visualização Joaju (Modo Leitura) ");
            let preview_text =
                Paragraph::new("100% da tela dedicada à leitura diagramada pelo Joaju.")
                    .block(preview_block);
            frame.render_widget(preview_text, main_area);
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
