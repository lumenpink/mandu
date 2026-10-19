//! Renderização de blocos ricos Joaju e cartões alinhados geometricamente para o terminal.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Trunca uma string garantindo que sua largura visual não ultrapasse `max_width`.
/// Se truncada, adiciona `…` (largura 1).
#[must_use]
pub fn truncate_to_width(s: &str, max_width: usize) -> (String, usize) {
    let current_width = UnicodeWidthStr::width(s);
    if current_width <= max_width {
        return (s.to_string(), current_width);
    }

    if max_width == 0 {
        return (String::new(), 0);
    }

    if max_width == 1 {
        return ("…".to_string(), 1);
    }

    let budget = max_width - 1; // reserva 1 coluna para '…'
    let mut acc_width = 0;
    let mut truncated = String::new();

    for c in s.chars() {
        let cw = UnicodeWidthChar::width(c).unwrap_or(0);
        if acc_width + cw > budget {
            break;
        }
        acc_width += cw;
        truncated.push(c);
    }

    truncated.push('…');
    acc_width += 1;
    (truncated, acc_width)
}

/// Cria uma linha de caixa perfeitamente ajustada à largura `total_width`.
fn build_box_line(
    left_border: &'static str,
    spans: Vec<Span<'static>>,
    content_visual_width: usize,
    right_border: &'static str,
    total_width: usize,
    border_style: Style,
) -> Line<'static> {
    let left_w = UnicodeWidthStr::width(left_border);
    let right_w = UnicodeWidthStr::width(right_border);
    let border_w = left_w + right_w;

    let target_inner = total_width.saturating_sub(border_w);
    let pad_len = target_inner.saturating_sub(content_visual_width);

    let mut line_spans = Vec::with_capacity(spans.len() + 3);
    line_spans.push(Span::styled(left_border, border_style));
    line_spans.extend(spans);
    if pad_len > 0 {
        line_spans.push(Span::raw(" ".repeat(pad_len)));
    }
    line_spans.push(Span::styled(right_border, border_style));

    Line::from(line_spans)
}

/// Cria borda horizontal com título embutido (ex: `╭─ 🔗 Título ─────╮` ou `┌── Bloco ──┐`).
fn build_top_border(
    corner_left: &'static str,
    title_spans: Vec<Span<'static>>,
    corner_right: &'static str,
    total_width: usize,
    border_style: Style,
) -> Line<'static> {
    let cl_w = UnicodeWidthStr::width(corner_left);
    let cr_w = UnicodeWidthStr::width(corner_right);
    let available = total_width.saturating_sub(cl_w + cr_w);

    let title_visual_width: usize = title_spans
        .iter()
        .map(|s| UnicodeWidthStr::width(s.content.as_ref()))
        .sum();

    if title_visual_width >= available {
        let (trunc, tw) = truncate_to_width(
            &title_spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>(),
            available.saturating_sub(2),
        );
        let rem = available.saturating_sub(tw);
        return Line::from(vec![
            Span::styled(corner_left, border_style),
            Span::styled(trunc, border_style),
            Span::styled("─".repeat(rem), border_style),
            Span::styled(corner_right, border_style),
        ]);
    }

    let remaining = available.saturating_sub(title_visual_width);
    let mut spans = Vec::new();
    spans.push(Span::styled(corner_left, border_style));
    spans.extend(title_spans);
    spans.push(Span::styled("─".repeat(remaining), border_style));
    spans.push(Span::styled(corner_right, border_style));

    Line::from(spans)
}

/// Renderiza um cartão de link/unfurl com cálculo visual rigoroso e bordas retas.
#[must_use]
pub fn render_link_card(
    title: &str,
    url: &str,
    desc: Option<&str>,
    width: usize,
) -> Vec<Line<'static>> {
    let width = width.max(16);
    let inner_width = width.saturating_sub(4); // "│ " (2) + " │" (2)
    let border_style = Style::default().fg(Color::Cyan);

    let mut lines = Vec::new();

    // Borda superior
    let header_prefix = "─ 🔗 ";
    let header_title = "Link / Prévia ";
    lines.push(build_top_border(
        "╭",
        vec![
            Span::styled(header_prefix, border_style),
            Span::styled(
                header_title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ],
        "╮",
        width,
        border_style,
    ));

    // Linha de Título
    let title_prefix = "📄 ";
    let prefix_w = UnicodeWidthStr::width(title_prefix);
    let max_title_w = inner_width.saturating_sub(prefix_w);
    let (clean_title, title_w) = truncate_to_width(title, max_title_w);
    lines.push(build_box_line(
        "│ ",
        vec![
            Span::styled(title_prefix, Style::default().fg(Color::Yellow)),
            Span::styled(
                clean_title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ],
        prefix_w + title_w,
        " │",
        width,
        border_style,
    ));

    // Linha de URL / Domínio
    let url_prefix = "🌐 ";
    let url_prefix_w = UnicodeWidthStr::width(url_prefix);
    let max_url_w = inner_width.saturating_sub(url_prefix_w);
    let (clean_url, url_w) = truncate_to_width(url, max_url_w);
    lines.push(build_box_line(
        "│ ",
        vec![
            Span::styled(url_prefix, Style::default().fg(Color::Blue)),
            Span::styled(
                clean_url,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ],
        url_prefix_w + url_w,
        " │",
        width,
        border_style,
    ));

    // Linha de Descrição opcional
    if let Some(description) = desc {
        if !description.trim().is_empty() {
            let desc_prefix = "💬 ";
            let desc_prefix_w = UnicodeWidthStr::width(desc_prefix);
            let max_desc_w = inner_width.saturating_sub(desc_prefix_w);
            let (clean_desc, desc_w) = truncate_to_width(description.trim(), max_desc_w);
            lines.push(build_box_line(
                "│ ",
                vec![
                    Span::styled(desc_prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        clean_desc,
                        Style::default()
                            .fg(Color::Gray)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ],
                desc_prefix_w + desc_w,
                " │",
                width,
                border_style,
            ));
        }
    }

    // Borda inferior
    let bottom_rem = width.saturating_sub(2);
    lines.push(Line::from(vec![
        Span::styled("╰", border_style),
        Span::styled("─".repeat(bottom_rem), border_style),
        Span::styled("╯", border_style),
    ]));

    lines
}

/// Renderiza uma tabela formatada para a TUI com colunas alinhadas e bordas fechadas.
#[must_use]
pub fn render_table_block(raw_body: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(20);
    let border_style = Style::default().fg(Color::Green);
    let mut lines = Vec::new();

    let table_lines: Vec<&str> = raw_body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("%%"))
        .collect();

    if table_lines.is_empty() {
        return render_code_box("tabela", "(tabela vazia)", width);
    }

    // Processa cabeçalho e linhas tabulares
    let rows: Vec<Vec<&str>> = table_lines
        .iter()
        .map(|l| {
            if l.contains('\t') {
                l.split('\t').map(str::trim).collect()
            } else if l.contains(',') {
                l.split(',').map(str::trim).collect()
            } else if l.contains('|') {
                l.split('|')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                vec![*l]
            }
        })
        .collect();

    let num_cols = rows.iter().map(Vec::len).max().unwrap_or(1).max(1);

    // Borda superior da tabela
    lines.push(build_top_border(
        "┌",
        vec![
            Span::styled("─ 📊 ", border_style),
            Span::styled(
                "Tabela ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ],
        "┐",
        width,
        border_style,
    ));

    // Distribuição de largura entre as colunas
    let inner_avail = width.saturating_sub(4 + (num_cols.saturating_sub(1) * 3));
    let col_w = (inner_avail / num_cols).max(4);

    for (row_idx, row) in rows.iter().enumerate() {
        let mut row_spans = Vec::new();
        let mut visual_total = 0;

        for col_idx in 0..num_cols {
            let cell = row.get(col_idx).copied().unwrap_or("");
            let (truncated_cell, cell_w) = truncate_to_width(cell, col_w);
            let pad_len = col_w.saturating_sub(cell_w);

            let style = if row_idx == 0 {
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(220, 220, 225))
            };

            row_spans.push(Span::styled(truncated_cell, style));
            if pad_len > 0 {
                row_spans.push(Span::raw(" ".repeat(pad_len)));
            }
            visual_total += col_w;

            if col_idx < num_cols - 1 {
                row_spans.push(Span::styled(" │ ", border_style));
                visual_total += 3;
            }
        }

        lines.push(build_box_line(
            "│ ",
            row_spans,
            visual_total,
            " │",
            width,
            border_style,
        ));

        // Divisor após cabeçalho
        if row_idx == 0 && rows.len() > 1 {
            let sep_len = width.saturating_sub(2);
            lines.push(Line::from(vec![
                Span::styled("├", border_style),
                Span::styled("─".repeat(sep_len), border_style),
                Span::styled("┤", border_style),
            ]));
        }
    }

    // Borda inferior
    let bottom_rem = width.saturating_sub(2);
    lines.push(Line::from(vec![
        Span::styled("└", border_style),
        Span::styled("─".repeat(bottom_rem), border_style),
        Span::styled("┘", border_style),
    ]));

    lines
}

/// Renderiza resumo clínico do heredograma para a TUI com cálculo de largura uniforme.
#[must_use]
pub fn render_pedigree_summary(raw_body: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(20);
    let border_style = Style::default().fg(Color::Magenta);
    let mut lines = Vec::new();

    let ind_lines: Vec<&str> = raw_body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();

    let count = ind_lines.len();

    // Borda superior
    lines.push(build_top_border(
        "╭",
        vec![
            Span::styled("─ 🧬 ", border_style),
            Span::styled(
                "Heredograma Clínico ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ],
        "╮",
        width,
        border_style,
    ));

    let info_text = format!("Total de Indivíduos: {count}");
    let (trunc_info, info_w) = truncate_to_width(&info_text, width.saturating_sub(4));
    lines.push(build_box_line(
        "│ ",
        vec![Span::styled(
            trunc_info,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )],
        info_w,
        " │",
        width,
        border_style,
    ));

    let sample_text = if count > 0 {
        format!("Amostra: {}", ind_lines[0])
    } else {
        "Nenhum registro clínico encontrado".to_string()
    };
    let (trunc_sample, sample_w) = truncate_to_width(&sample_text, width.saturating_sub(4));
    lines.push(build_box_line(
        "│ ",
        vec![Span::styled(
            trunc_sample,
            Style::default().fg(Color::DarkGray),
        )],
        sample_w,
        " │",
        width,
        border_style,
    ));

    // Borda inferior
    let bottom_rem = width.saturating_sub(2);
    lines.push(Line::from(vec![
        Span::styled("╰", border_style),
        Span::styled("─".repeat(bottom_rem), border_style),
        Span::styled("╯", border_style),
    ]));

    lines
}

/// Renderiza caixa fechada de código ou bloco arbitrário garantindo largura constante.
#[must_use]
pub fn render_code_box(engine: &str, code: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(16);
    let border_style = Style::default().fg(Color::Cyan);
    let mut lines = Vec::new();

    let title = format!(" Bloco [{engine}] ");
    lines.push(build_top_border(
        "┌",
        vec![
            Span::styled("──", border_style),
            Span::styled(
                title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ],
        "┐",
        width,
        border_style,
    ));

    let inner_width = width.saturating_sub(4);
    for code_line in code.lines() {
        let (trunc, w) = truncate_to_width(code_line, inner_width);
        lines.push(build_box_line(
            "│ ",
            vec![Span::styled(trunc, Style::default().fg(Color::Yellow))],
            w,
            " │",
            width,
            border_style,
        ));
    }

    let bottom_rem = width.saturating_sub(2);
    lines.push(Line::from(vec![
        Span::styled("└", border_style),
        Span::styled("─".repeat(bottom_rem), border_style),
        Span::styled("┘", border_style),
    ]));

    lines
}

/// Renderiza prévia tipográfica de blocos Typst/Joaju na TUI.
#[must_use]
pub fn render_typst_preview(_code: &str, _width: usize) -> Vec<Line<'static>> {
    Vec::new()
}
