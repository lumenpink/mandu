//! Layout dos 3 modos e renderização TUI.

use crate::app::TuiApp;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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
}
