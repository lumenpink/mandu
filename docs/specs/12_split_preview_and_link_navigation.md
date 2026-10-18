# Especificação Técnica — Ticket #12: Renderização Dinâmica no Modo Split e Navegação Direta de Links

## 1. Visão Geral
Esta especificação estabelece os requisitos para resolver a ausência de conteúdo no painel de visualização (Split e Leitura) e providenciar navegação hipertextual direta para links internos `[[nota]]` e notas de referências bibliográficas `@citekey`.

## 2. Renderização Dinâmica no Modo Split e Preview

### 2.1 Terminal (`mandu-tui`)
1. No painel de visualização (`ViewMode::Split` no painel direito e `ViewMode::Preview` em tela inteira), o Mandu deve renderizar o buffer atual do editor (`app.editor.get_text()`) em vez de um texto estático.
2. Formatação visual dos elementos:
   - Títulos (`#`, `##`, `###`): Renderizados em negrito e com cor diferenciada (ex: Ciano/Amarelo).
   - Listas (`-`, `*`, `1.`): Recuo visual e marcador estilizado.
   - Citações em bloco (`>`): Renderizadas em itálico com barra vertical ou estilo atenuado.
   - Blocos de código / Joaju (````...````): Delimitados com borda ou cor de fundo distinta, exibindo a linguagem/engine no cabeçalho.
   - Quebra automática de linha (`ratatui::widgets::Wrap { trim: false }`) para legibilidade confortável em larguras variáveis.

### 2.2 Interface Gráfica (`mandu-gui`)
1. O painel de visualização `render_preview` deve formatar os blocos de texto com estilos tipográficos claros e títulos proporcionais.
2. Para blocos Joaju e diagramas, exibir caixas delimitadas elegantes indicando o status do motor.

## 3. Resolução e Navegação Direta de Links (`Ctrl+G` e Peek `Enter`)

### 3.1 Contrato no Núcleo (`mandu-core::peek`)
```rust
pub fn resolve_target_path(
    db: &ManduDb,
    raw_target: &str,
    vault_path: Option<&Path>,
) -> Option<PathBuf>
```
1. Se `raw_target` inicia com `@`: busca a referência com o citekey no banco de dados e retorna o caminho canônico do arquivo em disco (ex: `refs/@citekey.md`).
2. Se `raw_target` é um wikilink `[[alvo]]` ou `[[alvo|apelido]]`: busca pelo título, nome do arquivo ou caminho no banco SQLite e retorna o caminho canônico resolvido.

### 3.2 Atalho `Ctrl+G` (Go to Link)
1. Quando o usuário estiver editando e pressionar `Ctrl+G`:
   - O Mandu inspeciona a linha atual sob o cursor (ou o buffer ativo) em busca de `[[link]]` ou `@citekey`.
   - Se encontrar um alvo válido e o arquivo existir no cofre, chama imediatamente `open_note(path)`, carregando-o no editor.
   - Se o alvo não existir, exibe mensagem na barra de status informando que o destino não foi encontrado.

### 3.3 Ação `Enter` na Modal de Peek (`Ctrl+P`)
1. Quando o modal de espiar (`Ctrl+P`) estiver aberto sobre um alvo, pressionar `Enter` (ou clicar em "Abrir no Editor") deve:
   - Fechar a modal de Peek;
   - Carregar o arquivo do alvo no editor principal (`open_note`).
