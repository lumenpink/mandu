# Spec Ticket #42: Formatação Inline no Preview, Salvar Como com Sugestão e Renumeração de Listas

## 1. Contexto e Motivação
Durante a validação prática da interface TUI do `mandu`, foram identificadas três oportunidades de aprimoramento de usabilidade fundamentais para a experiência de edição e leitura:
1. **Visualização de estilos no preview**: O renderizador de linhas markdown no painel de pré-visualização tratava apenas wikilinks (`[[...]]`) e citações (`@...`), deixando marcações padrão de formatação (`**negrito**`, `*itálico*`, código inline) como texto cru.
2. **Salvamento manual e compatibilidade de terminal**: Atalhos como `Ctrl+Shift+S` são descartados por emuladores de terminal padrão (sem extensões de protocolo de teclado), e `Ctrl+A`/`Ctrl+B` colidem com multiplexadores como Byobu e tmux. Como o salvamento contínuo já persiste o arquivo automaticamente, unificar o `Ctrl+S` para acionar a confirmação de salvamento/salvar como com sugestão pré-preenchida do nome atual proporciona transparência e ergonomia universal.
3. **Continuação e renumeração inteligente de listas**: Ao teclar `Enter` em uma lista, o editor deve continuar o marcador (com indentação preservada), sair da lista ao teclar `Enter` em marcador vazio, e recalcular automaticamente a numeração sequencial das listas ordenadas (OL) no próprio texto Markdown bruto.

## 2. Escopo da Solução

### 2.1 Formatação Inline no Preview (`mandu-tui/src/blocks.rs`)
- Expandir a função `parse_inline_spans` para processar:
  - Negrito: `**texto**` e `__texto__` com `Modifier::BOLD` e cor destacada.
  - Itálico: `*texto*` e `_texto_` com `Modifier::ITALIC`.
  - Negrito + Itálico: `***texto***` ou `___texto___` com `Modifier::BOLD | Modifier::ITALIC`.
  - Código inline: `` `código` `` com cor ciano/estilo diferenciado.
  - Preservar wikilinks `[[alvo|título]]` e citações acadêmicas `@chave`.

### 2.2 Salvar / Salvar Como Unificado com Sugestão (`mandu-tui/src/app.rs`)
- O atalho `Ctrl+S` abre o modal de salvamento / renomeação (`is_save_as_open`).
- O campo de entrada é pré-preenchido com o nome atual do arquivo (ex.: `nota.md`, ou título extraído, ou `nova_nota.md`).
- Ao teclar `Enter` sem editar, o arquivo é confirmado e salvo com o nome sugerido.
- A barra de status exibe temporariamente o feedback explícito: `[✓ Nota salva: <arquivo>]`.
- Teclar `Esc` cancela a operação sem alterar o arquivo.

### 2.3 Continuação Inteligente e Renumeração de Listas (`mandu-tui/src/editor.rs`)
- Ao teclar `Enter` no editor:
  - Detectar se a linha atual inicia com:
    - Lista não-ordenada: `^([ \t]*)([-*+])\s+(.*)$`
    - Checklist / Tarefa: `^([ \t]*)([-*+]\s+\[[ xX]\])\s+(.*)$`
    - Lista ordenada: `^([ \t]*)(\d+)\.\s+(.*)$`
  - Se a linha contiver apenas o marcador (conteúdo vazio): apagar o marcador da linha corrente e sair da lista.
  - Se houver conteúdo: inserir quebra de linha com a mesma indentação e o marcador subsequente (`- `, `- [ ] ` ou `(N+1). `).
  - Para listas ordenadas (`OL`), recalcular a sequência numérica do bloco contíguo subsequente de modo que todo o Markdown fonte permaneça estritamente ordenado (`1.`, `2.`, `3.`, ...).

## 3. Critérios de Aceitação
- `cargo fmt -- --check`: Formatação sem violações.
- `cargo clippy --all-targets -- -D warnings`: Zero advertências.
- `cargo test --all-targets`: Todos os testes unitários e de integração aprovados.
- Testes unitários para `parse_inline_spans` comprovando estilos BOLD, ITALIC e código inline.
- Testes unitários para renumeração de listas ordenadas e continuação inteligente no editor.
- `cccp check --strict`: Auditoria Gosplan 100% aprovada.
