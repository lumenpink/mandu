# Especificação Técnica - Ticket #16: Renderização Rica de Blocos Joaju e Correção Geométrica na TUI

## 1. Contexto e Motivação
Na interface de terminal (`mandu-tui`), os blocos de visualização (como prévias de links, metadados e blocos de código/Joaju) apresentavam desalinhamento visual ("bloco torto"). Esse problema decorre de:
1. Utilização de contagem ingênua de bytes (`len()`) ou caracteres (`chars().count()`) em vez de largura visual de colunas (`unicode-width`). Símbolos e emojis como `🔗`, `🌐`, `📄`, `🏷` ocupam 2 colunas visuais no terminal, causando deslocamento na borda direita (`│`).
2. Ausência de bordas direitas e fechamentos geométricos calculados dinamicamente em relação à largura do painel.
3. Blocos específicos do Joaju (`table`/`csv`, `pedigree`/`heredograma`, `unfurl`) sendo renderizados como código bruto ou dump de texto, em vez de componentes textuais estruturados e agradáveis para o terminal.

## 2. Princípios Arquiteturais e Fronteiras
- **`joaju-core` Permanece Agnóstico**: O núcleo do Joaju (`joaju`) atua como orquestrador e compilador Typst/PDF. Nenhuma lógica, dependência de terminal ou estruturas do Ratatui devem ser inseridas no `joaju-core`.
- **Separação de Responsabilidades**: As crates modulares (`joaju-unfurl`, `joaju-table`, `joaju-pedigree`) cuidam de suas respectivas extrações e modelos de domínio.
- **Camada de Apresentação TUI (`mandu-tui::blocks`)**: O crate `mandu-tui` implementa um módulo dedicado `blocks.rs` encarregado de calcular larguras com precisão via `unicode-width` e formatar linhas (`ratatui::text::Line`) perfeitamente alinhadas.

## 3. Contratos e Algoritmo de Alinhamento Geométrico
Para qualquer card ou caixa retangular desenhada com bordas (`┌──┐`, `│  │`, `└──┘` ou `╭──╮`, `╰──╯`):
1. **Largura Alvo (`width`)**: Determinada pelo painel ou largura máxima configurada (ex: `width = 54` colunas).
2. **Largura Interna (`inner_width`)**: `width.saturating_sub(4)` (1 coluna borda esq + 1 espaço margem esq + 1 espaço margem dir + 1 coluna borda dir).
3. **Cálculo de Colunas**: Para cada string de conteúdo `s`, sua largura visual é obtida via `UnicodeWidthStr::width(s)`.
   - Se `visual_len > inner_width`, a string é truncada preservando o limite e recebendo reticências `…` (largura 1).
   - O preenchimento de espaços à direita é exatamente `inner_width - visual_len`.
   - A linha final é montada como: `│ <conteúdo><espaços> │`.
4. **Garantia Geométrica**: Cada linha produzida possui largura visual rigorosamente idêntica a `width`.

## 4. Renderizadores de Blocos Especializados
- **`render_link_card` / `render_unfurl_block`**:
  Card fechado contendo:
  - Borda superior com título estilizado `╭─ 🔗 Link / Prévia ─...─╮`
  - Linha de título da página (`📄 <titulo>`)
  - Linha de domínio/origem (`🌐 <dominio>`)
  - Linha de descrição resumida (`<descricao>`)
  - Borda inferior fechada `╰─...─╯`
- **`render_table_block`**:
  Tabela com cabeçalho formatado, separador `├─...─┤` e linhas alinhadas por coluna, em vez de texto bruto de CSV/diretivas.
- **`render_pedigree_summary`**:
  Ficha clínica resumida exibindo indivíduos, gerações e indicadores de probando em formato de cartão tabular limpo.
- **`render_code_box`**:
  Caixa fechada com bordas completas (superior com nome do motor/linguagem, laterais e inferior) alinhada à largura do painel.

## 5. Critérios de Aceite
- [x] Zero warnings em compilação e clippy.
- [x] Testes unitários comprovando que as linhas geradas possuem a mesma largura de colunas visual, inclusive na presença de caracteres acentuados e emojis de largura dupla.
