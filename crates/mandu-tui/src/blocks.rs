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

/// Embeleza fórmulas matemáticas para apresentação clara no terminal.
#[must_use]
pub fn format_math_symbols(formula: &str) -> String {
    formula
        .replace("log_2", "log₂")
        .replace("log_10", "log₁₀")
        .replace("sqrt", "√")
        .replace("alpha", "α")
        .replace("beta", "β")
        .replace("gamma", "γ")
        .replace("delta", "δ")
        .replace("sigma", "σ")
        .replace("omega", "ω")
        .replace("pi", "π")
        .replace("!=", "≠")
        .replace("<=", "≤")
        .replace(">=", "≥")
        .replace("->", "→")
        .replace("=>", "⇒")
        .replace("times", "×")
}

/// Extrai células e dimensões de uma chamada `#table(...)` do Typst.
fn parse_typst_table(code: &str, width: usize) -> Option<Vec<Line<'static>>> {
    if !code.contains("#table(") {
        return None;
    }

    // Identificar a quantidade de colunas na assinatura de columns:
    let cols = if let Some(col_idx) = code.find("columns:") {
        let rest = code[col_idx + 8..].trim_start();
        if let Some(open_paren) = rest.find('(') {
            if let Some(close_paren) = rest[open_paren..].find(')') {
                let inside = &rest[open_paren + 1..open_paren + close_paren];
                let count = inside.split(',').filter(|c| !c.trim().is_empty()).count();
                count.max(1)
            } else {
                3
            }
        } else {
            let num_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            num_str.parse::<usize>().unwrap_or(3).max(1)
        }
    } else {
        3
    };

    // Extrair células delimitadas por [...]
    let mut cells = Vec::new();
    let mut chars = code.char_indices().peekable();

    while let Some((idx, c)) = chars.next() {
        if c == '[' {
            let mut depth = 1;
            let start = idx + 1;
            let mut end = start;
            for (sub_idx, sub_c) in chars.by_ref() {
                if sub_c == '[' {
                    depth += 1;
                } else if sub_c == ']' {
                    depth -= 1;
                    if depth == 0 {
                        end = sub_idx;
                        break;
                    }
                }
            }
            if end >= start {
                let cell_raw = &code[start..end];
                let cell_clean = cell_raw.trim().trim_matches('*').trim_matches('_').trim();
                cells.push(cell_clean.to_string());
            }
        }
    }

    if cells.is_empty() {
        return None;
    }

    let rows: Vec<String> = cells.chunks(cols).map(|chunk| chunk.join("\t")).collect();

    let tsv = rows.join("\n");
    Some(render_table_block(&tsv, width))
}

/// Formata expressões inline do Typst (negrito `*...*`, itálico `_..._`, matemática `$x$`).
fn parse_typst_inline(text: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            if !chars.clone().any(|mc| mc == '$') {
                current.push('$');
                continue;
            }
            if !current.is_empty() {
                spans.push(Span::styled(
                    current.clone(),
                    Style::default().fg(Color::Gray),
                ));
                current.clear();
            }
            let mut math_content = String::new();
            while let Some(&mc) = chars.peek() {
                chars.next();
                if mc == '$' {
                    break;
                }
                math_content.push(mc);
            }
            let pretty_math = format_math_symbols(&math_content);
            spans.push(Span::styled(
                pretty_math,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            ));
        } else if c == '*' {
            if !chars.clone().any(|bc| bc == '*') {
                current.push('*');
                continue;
            }
            if !current.is_empty() {
                spans.push(Span::styled(
                    current.clone(),
                    Style::default().fg(Color::Gray),
                ));
                current.clear();
            }
            let mut bold_content = String::new();
            while let Some(&bc) = chars.peek() {
                chars.next();
                if bc == '*' {
                    break;
                }
                bold_content.push(bc);
            }
            spans.push(Span::styled(
                bold_content,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        spans.push(Span::styled(current, Style::default().fg(Color::Gray)));
    }

    spans
}

/// Renderiza prévia tipográfica de blocos Typst/Joaju na TUI.
#[must_use]
pub fn render_typst_preview(code: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(20);
    let trimmed = code.trim();

    if trimmed.is_empty() {
        return Vec::new();
    }

    // Se o bloco consistir inteiramente de uma tabela #table(...)
    if trimmed.starts_with("#table(") {
        if let Some(table_lines) = parse_typst_table(trimmed, width) {
            return table_lines;
        }
    }

    let mut lines = Vec::new();
    let mut in_multiline_math = false;
    let mut multiline_math_buf = String::new();
    let mut in_table = false;
    let mut table_buf = String::new();

    // Cabeçalho estilizado discreto do Joaju
    lines.push(Line::from(vec![Span::styled(
        "  [ 🖋 Joaju / Typst ]",
        Style::default().fg(Color::Cyan),
    )]));
    lines.push(Line::from(""));

    for raw_line in code.lines() {
        let line = raw_line.trim();

        // Tratamento de tabela multi-linhas interna
        if in_table {
            table_buf.push('\n');
            table_buf.push_str(raw_line);
            if line.contains(')') {
                in_table = false;
                if let Some(tbl) = parse_typst_table(&table_buf, width) {
                    lines.extend(tbl);
                    lines.push(Line::from(""));
                }
                table_buf.clear();
            }
            continue;
        } else if line.starts_with("#table(") {
            if line.contains(')') && line.matches('(').count() <= line.matches(')').count() {
                if let Some(tbl) = parse_typst_table(line, width) {
                    lines.extend(tbl);
                    lines.push(Line::from(""));
                }
            } else {
                in_table = true;
                table_buf.clear();
                table_buf.push_str(raw_line);
            }
            continue;
        }

        // 1. Filtrar diretivas de compilação Typst (#set, #show, #import)
        if line.starts_with("#set ") || line.starts_with("#show ") || line.starts_with("#import ") {
            continue;
        }

        // 2. Fórmulas matemáticas em bloco delimitadas por $
        if in_multiline_math {
            if line.ends_with('$') {
                multiline_math_buf.push(' ');
                multiline_math_buf.push_str(line.trim_end_matches('$').trim());
                in_multiline_math = false;
                let math_display = format_math_symbols(&multiline_math_buf);
                lines.push(Line::from(vec![
                    Span::styled(
                        "   ∑  ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        math_display,
                        Style::default()
                            .fg(Color::LightYellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(""));
                multiline_math_buf.clear();
            } else {
                multiline_math_buf.push(' ');
                multiline_math_buf.push_str(line);
            }
            continue;
        } else if line.starts_with('$') && line.ends_with('$') && line.len() > 1 {
            let formula = line.trim_start_matches('$').trim_end_matches('$').trim();
            let math_display = format_math_symbols(formula);
            lines.push(Line::from(vec![
                Span::styled(
                    "   ∑  ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    math_display,
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(""));
            continue;
        } else if line.starts_with('$') && !line.ends_with('$') {
            in_multiline_math = true;
            multiline_math_buf = line.trim_start_matches('$').trim().to_string();
            continue;
        }

        // 3. Títulos Typst (=, ==, ===, ====)
        if let Some(stripped) = line.strip_prefix("==== ") {
            lines.push(Line::from(vec![
                Span::styled(
                    "▪ ",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = line.strip_prefix("=== ") {
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
        } else if let Some(stripped) = line.strip_prefix("== ") {
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
        } else if let Some(stripped) = line.strip_prefix("= ") {
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
        } else if let Some(stripped) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            // 4. Listas com marcadores
            let mut list_spans = vec![Span::styled("  • ", Style::default().fg(Color::Cyan))];
            list_spans.extend(parse_typst_inline(stripped));
            lines.push(Line::from(list_spans));
        } else if line.is_empty() {
            lines.push(Line::from(""));
        } else {
            // 5. Parágrafo / Texto comum com spans inline
            lines.push(Line::from(parse_typst_inline(line)));
        }
    }

    if in_multiline_math && !multiline_math_buf.is_empty() {
        let math_display = format_math_symbols(&multiline_math_buf);
        lines.push(Line::from(vec![
            Span::styled(
                "   ∑  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                math_display,
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    if in_table && !table_buf.is_empty() {
        if let Some(tbl) = parse_typst_table(&table_buf, width) {
            lines.extend(tbl);
        }
    }

    lines
}

/// Analisa spans inline em Markdown reconhecendo wikilinks [[...]] e tags @citação
#[must_use]
pub fn parse_inline_spans(line: &str) -> Vec<Span<'static>> {
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

/// Renderiza linhas de Markdown na TUI com auto-incremento de listas ordenadas e blocos ricos.
#[must_use]
pub fn render_markdown_lines(
    raw_body: &str,
    number_headings: bool,
    numbering_pattern: Option<&str>,
    card_width: usize,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    let processed_body = if number_headings {
        let pat = numbering_pattern.unwrap_or("1.1.1.1.1.1");
        mandu_core::vault::apply_heading_numbering_with_pattern(raw_body, pat)
    } else {
        raw_body.to_string()
    };

    let mut body_lines = processed_body.lines().peekable();
    let mut ordered_list_counter: usize = 0;

    #[inline]
    fn parse_ordered_item(line: &str) -> Option<&str> {
        let trimmed = line.trim_start();
        let digits_len = trimmed.chars().take_while(|c| c.is_ascii_digit()).count();
        if digits_len > 0 {
            let after_digits = &trimmed[digits_len..];
            if let Some(rest) = after_digits.strip_prefix(". ") {
                return Some(rest);
            }
        }
        None
    }

    while let Some(line) = body_lines.next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            ordered_list_counter = 0;
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
                    lines.extend(render_link_card(title, url, desc, card_width));
                }
                "table" | "csv" => {
                    lines.extend(render_table_block(&code, card_width));
                }
                "pedigree" | "heredograma" | "kinship2" | "pedtools" => {
                    lines.extend(render_pedigree_summary(&code, card_width));
                }
                "typst" | "joaju" => {
                    lines.extend(render_typst_preview(&code, card_width));
                }
                _ => {
                    let label = if engine.is_empty() { "código" } else { engine };
                    lines.extend(render_code_box(label, &code, card_width));
                }
            }
            continue;
        }

        if (trimmed.starts_with("http://") || trimmed.starts_with("https://"))
            && !trimmed.contains(' ')
        {
            ordered_list_counter = 0;
            lines.extend(render_link_card(trimmed, trimmed, None, card_width));
            continue;
        }

        if let Some(stripped) = trimmed.strip_prefix("# ") {
            ordered_list_counter = 0;
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
            ordered_list_counter = 0;
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
            ordered_list_counter = 0;
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
        } else if let Some(stripped) = trimmed.strip_prefix("#### ") {
            ordered_list_counter = 0;
            lines.push(Line::from(vec![
                Span::styled(
                    "▏ ",
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = trimmed.strip_prefix("##### ") {
            ordered_list_counter = 0;
            lines.push(Line::from(vec![
                Span::styled(
                    "▏ ",
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = trimmed.strip_prefix("###### ") {
            ordered_list_counter = 0;
            lines.push(Line::from(vec![
                Span::styled(
                    "▏ ",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    stripped.to_string(),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        } else if let Some(stripped) = trimmed.strip_prefix('>') {
            ordered_list_counter = 0;
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
            ordered_list_counter = 0;
            let mut list_spans = vec![Span::styled("  • ", Style::default().fg(Color::Cyan))];
            list_spans.extend(parse_inline_spans(stripped));
            lines.push(Line::from(list_spans));
        } else if let Some(rest) = parse_ordered_item(trimmed) {
            ordered_list_counter += 1;
            let prefix = format!("  {ordered_list_counter}. ");
            let mut list_spans = vec![Span::styled(
                prefix,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )];
            list_spans.extend(parse_inline_spans(rest));
            lines.push(Line::from(list_spans));
        } else if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            ordered_list_counter = 0;
            lines.push(Line::from(vec![Span::styled(
                "──────────────────────────────────────────",
                Style::default().fg(Color::DarkGray),
            )]));
        } else if trimmed.is_empty() {
            ordered_list_counter = 0;
            lines.push(Line::from(""));
        } else {
            ordered_list_counter = 0;
            lines.push(Line::from(parse_inline_spans(line)));
        }
    }

    lines
}
