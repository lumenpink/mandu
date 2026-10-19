# Especificação Técnica - Ticket #17: Renderizador Tipográfico Joaju/Typst e Proteção de Buffer

## 1. Contexto e Motivação
Durante a validação prática no cofre (`mandu-vault`), dois comportamentos críticos foram identificados:
1. **Perda de Frontmatter e Destruição de Buffer no Salvamento/Autosave**:
   Ao editar qualquer nota no `mandu-tui` (como `indice.md`), o arquivo foi truncado e teve seus metadados de frontmatter YAML (`title`, `tags`, datas) eliminados, restando apenas um frontmatter vazio com `modified: ...` seguido de fragmentos de digitação (ex: caractere `f`).
   **Causa Raiz**:
   - `open_note` e `new_with_vault` carregavam apenas `note.body` no `Editor`, descartando o frontmatter da visão do usuário e do buffer em memória.
   - Ao executar `save_current_note`, `Linter::apply_save_hooks` recebia um texto sem delimitadores `---`, interpretando que o arquivo não possuía frontmatter e gerando um cabeçalho mínimo contendo unicamente `modified: ...`.
   - Adicionalmente, `save_current_note` reatribuía `self.editor = Editor::from_text(&note.body)` a cada salvamento (inclusive no debounce de 1000ms de autosave), reinicializando o estado do cursor para `(0, 0)` e corrompendo a edição concorrente durante a digitação.
2. **Blocos Typst/Joaju Não Publicados na TUI (Item 3 do Índice)**:
   A nota `demonstracao-joaju.md` possui blocos tipográficos contendo diretivas `#set`, títulos (`== `), fórmulas matemáticas (`$ ... $`), listas e tabelas (`#table(...)`). No terminal, estes blocos caíam no fallback genérico `render_code_box`, exibindo código bruto dentro de uma caixa amarela/ciano truncada em vez de serem "publicados" visualmente de forma rica e legível.

## 2. Princípios Arquiteturais e Isolamento de Responsabilidades
- **Imutabilidade do `joaju-core`**: A compilação PDF e o pipeline do Joaju permanecem no `joaju-core`, sem qualquer acoplamento a Ratatui, terminais ou TUI.
- **Camada de Apresentação TUI (`mandu-tui`)**: O crate `mandu-tui` é o único responsável por interpretar blocos de marcação tipográfica e transformá-los em linhas estilizadas (`ratatui::text::Line`).
- **Segurança e Integridade do Buffer**: O editor deve carregar e manipular o documento integral (`raw content`), mantendo as seções de frontmatter intactas e jamais reinicializando a instância de `TextArea` durante saves rotineiros ou autosaves em background.

## 3. Especificação da Proteção de Buffer e Salvamento Seguro
1. **Carga Completa no Editor**:
   Em `App::new_with_vault` e `App::open_note`, o `editor` é inicializado com `&content` integral do arquivo lido em disco, permitindo edição e visibilidade completas do frontmatter YAML.
2. **Não Reinicialização no Salvamento**:
   Em `App::save_current_note`:
   - O conteúdo atual é obtido via `self.editor.get_text()`.
   - `Linter::apply_save_hooks` atualiza o campo `modified` no frontmatter existente e assegura terminação POSIX newline.
   - O conteúdo formatado é gravado em disco e reindexado no banco SQLite (`db.index_note`).
   - `self.editor` **NÃO** é reatribuído nem reinicializado. O cursor, histórico e seleção permanecem intocados.
   - `self.is_dirty` é atualizado para `false`.

## 4. Especificação do Renderizador Tipográfico (`render_typst_preview`)
Em `crates/mandu-tui/src/blocks.rs`, a função `render_typst_preview(code: &str, width: usize) -> Vec<Line<'static>>` processa blocos com identificador `typst` ou `joaju`:
1. **Filtragem de Diretivas de Compilação**:
   Linhas iniciadas por `#set `, `#show `, `#import ` são filtradas da exibição visual para não poluir o terminal.
2. **Hierarquia de Títulos Typst**:
   - `= Título` -> `▌ Título` (Ciano, Negrito - H1)
   - `== Subtítulo` -> `▍ Subtítulo` (Verde, Negrito - H2)
   - `=== Seção` -> `▎ Seção` (Magenta, Negrito - H3)
3. **Fórmulas Matemáticas (`$ ... $`)**:
   - Fórmulas em bloco ou expressões matemáticas são destacadas com prefixo tipográfico (ex: `  ∫  ` ou `  ∑  `) e estilo distinto (Amarelo / Itálico / Negrito), com substituições legíveis como `log_2` -> `log₂`.
4. **Listas e Marcadores**:
   - Linhas iniciadas por `- ` são formatadas como `  • ` com spans inline preservados.
5. **Estruturas Tabulares (`#table(...)`)**:
   - Blocos `#table(...)` têm suas células extraídas (blocos delimitados por `[...]`) e são delegados para o renderizador de tabelas tabulares (`render_table_block`), garantindo visualização em colunas alinhadas.
6. **Integração no Roteador de Blocos (`ui.rs`)**:
   - Blocos com engine `"typst"` ou `"joaju"` são direcionados diretamente para `render_typst_preview(&code, card_width)`.

## 5. Critérios de Aceite e Verificação
- [ ] Teste unitário comprovando que `save_current_note` preserva campos existentes de frontmatter (`title`, `tags`, `criado`).
- [ ] Teste unitário comprovando que `render_typst_preview` converte títulos (`==`), equações matemáticas e listas, eliminando diretivas `#set`.
- [ ] Teste unitário comprovando a conversão de `#table(...)` Typst em visualização tabular formatada.
- [ ] Validação prática no cofre local `/home/lumen/mandu-vault` com a nota `demonstracao-joaju.md`.
