# Especificação Técnica — Ticket #13: Soft Wrap no Editor TUI, Renderização Tipográfica Rica e Numeração de Seções

## 1. Visão Geral
Esta especificação estabelece os requisitos para:
1. Ativação de quebra de linha suave (*soft wrap*) no editor de terminal (`mandu-tui`), impedindo que textos longos saiam da janela visual;
2. Renderização tipográfica aprimorada no painel de visualização (Split e Leitura), ocultando o frontmatter YAML bruto em favor de metadados refinados, e conferindo destaque a links `[[...]]`, citações `@...`, listas e cercaduras de código;
3. Suporte a numeração automática hierárquica de seções e títulos (`1.`, `1.1.`, `1.1.1.`), ativada declarativamente pelo YAML Frontmatter (`numerar_titulos: true`).

## 2. Numeração Automática de Seções (`mandu-core`)

### 2.1 Contrato no Frontmatter
O frontmatter da nota pode conter:
```yaml
---
titulo: Introdução ao Conhecimento
numerar_titulos: true
---
```
Sinônimos aceitos para interoperabilidade acadêmica: `numerar_titulos: true`, `number_headings: true`, ou `numbering: "1.1"` / `numeracao: "1.1"`.

### 2.2 Algoritmo de Numeração Hierárquica
No módulo `mandu_core::vault` (ou utilitário de formatação):
```rust
pub fn apply_heading_numbering(content: &str) -> String
```
- Cabeçalhos de Nível 1 (`# Título`): `1. Título`, `2. Título`
- Cabeçalhos de Nível 2 (`## Subtítulo`): `1.1. Subtítulo`, `1.2. Subtítulo`
- Cabeçalhos de Nível 3 (`### Tópico`): `1.1.1. Tópico`, `1.1.2. Tópico`
- O algoritmo deve ignorar cercaduras de código (````...````) para não numerar comentários ou strings de código.
- Títulos que já possuem numeração explícita fornecida pelo autor são preservados sem duplicar numeração.

### 2.3 Injeção no Compilador PDF Joaju/Typst
Na função `compile_markdown_to_pdf` e `export_note_to_pdf`:
Se a nota possuir `numerar_titulos: true`, o Mandu deve garantir que o preâmbulo Typst inclua `#set heading(numbering: "1.1")\n`.

## 3. Soft Wrap no Editor TUI (`mandu-tui`)
1. Substituir a dependência `tui-textarea` por `ratatui-textarea = "0.9"`.
2. No `crates/mandu-tui/src/editor.rs`, inicializar o `TextArea` com:
   ```rust
   textarea.set_wrap_mode(WrapMode::WordOrGlyph);
   ```
3. O editor quebrará visualmente as linhas sem alterar os bytes do arquivo em disco, permitindo navegação fluida com as setas do teclado.

## 4. Renderização Tipográfica Rica no Preview
1. No `crates/mandu-tui/src/ui.rs`:
   - Detectar e ocultar o bloco de YAML Frontmatter no início do documento;
   - Se `numerar_titulos` estiver ativo, aplicar a numeração hierárquica automática nos títulos;
   - Processar spans inline:
     - `[[alvo]]` e `[[alvo|apelido]]`: Estilizados em Ciano com sublinhado;
     - `@citekey`: Estilizados em Amarelo negrito;
     - `**negrito**`: Estilizado com `Modifier::BOLD`;
     - `*itálico*`: Estilizado com `Modifier::ITALIC`;
     - ` `código` `: Estilizado com fundo destacado e cor diferenciada.
   - Listas (`- `, `* `): Marcador `• ` colorido.
   - Citações (`> `): Barra vertical `▎ ` estilizada.
   - Blocos de código / Joaju: Molduras elegantes `┌── Bloco [engine] ────` e `└──`.
