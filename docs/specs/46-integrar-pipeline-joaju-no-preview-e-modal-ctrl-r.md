# Especificação Técnica — Integrar Pipeline Joaju no Preview e Modal Ctrl+R (Ticket #46)

## 1. Contexto e Motivação
Na TUI do Mandu (`mandu-tui`), os utilizadores precisam inserir referências bibliográficas rapidamente sem ter que lembrar decor o `@citekey` exato.
Com a sincronização de literatura vinda do Zotero via Tatupeba, cada nota de referência possui um ou mais identificadores de coleção (`collections: [...]`).
Quando um autor está redigindo um artigo ou tese, seu documento de trabalho define no frontmatter:
```yaml
---
title: "Meu Artigo"
ref_collections:
  - "Tese Doutorado"
---
```
O atalho `Ctrl+R` deve invocar um modal seletor de citações bibliográficas:
1. Detectar as coleções permitidas no frontmatter da nota atual (`ref_collections`).
2. Se houver `ref_collections`, filtrar inicialmente as referências pertencentes a essas coleções.
3. Permitir alternar para "todas as referências" a qualquer momento usando a tecla `Tab`.
4. Ao teclar `Enter`, inserir `@citekey` na posição atual do cursor no editor de texto.

Adicionalmente, o painel de preview da TUI (`render_preview_pane`) deve passar o corpo Markdown pelo `mandu_core::preview::prepare_markdown_pipeline(raw_body, references)`, garantindo:
- Resolução de citações e compilação da lista de referências bibliográficas.
- Expurgo de comentários `%%...%%`.
- Conversão de callouts Obsidian (`> [!tip]`, etc.) e realces (`==...==`).

## 2. Decisões de Arquitetura e Árvore de Escolhas

### 2.1 Modal de Citações (`CitationModalState`)
- Em `crates/mandu-tui/src/app.rs`:
  ```rust
  #[derive(Debug, Clone, Default)]
  pub struct CitationModalState {
      pub query: String,
      pub results: Vec<mandu_core::refs::ReferenceMd>,
      pub selected_index: usize,
      pub all_references: Vec<mandu_core::refs::ReferenceMd>,
      pub allowed_collections: Option<Vec<String>>,
      pub filter_by_collections: bool,
  }
  ```
- **Por que alternar com `Tab`?**
  Usuários frequentemente precisam de uma referência fora do escopo da coleção sem ter que editar o frontmatter. O `Tab` atua como chave de escape simples e intuitiva.
- **Formato de Inserção:**
  Insere `@citekey ` na posição do cursor na textarea do editor e marca o buffer como modificado (`is_dirty = true`).

### 2.2 Pipeline Joaju no Preview TUI
- Em `crates/mandu-tui/src/ui.rs`:
  `render_preview_pane` recebe `references: Option<&HashMap<String, ReferenceMd>>`.
  O texto do corpo bruto é transformado via `mandu_core::preview::prepare_markdown_pipeline(raw_body, references)` antes da tokenização em blocos ratatui.

## 3. Preservação de Manutenibilidade (Para Natasha)
- **Por que `ref_collections` no documento e `collections` na nota bibliográfica?**
  A nota bibliográfica pertence a uma ou mais `collections`. O documento de trabalho cita referências de um conjunto de `ref_collections` (coleções referenciadas). O parser do frontmatter aceita ambos ou lista/string para prevenir falhas por digitação livre.
- **Sem mutação destrutiva do buffer:**
  O modal `Ctrl+R` apenas lê o frontmatter para filtrar a busca; ele não reescreve ou formata o frontmatter automaticamente.

## 4. Validação e Quality Gate (Fase 8)
- `cargo fmt --check`: Aprovado sem nenhuma divergência de estilo.
- `cargo clippy --all-targets -- -D warnings`: Aprovado com zero advertências em todo o workspace.
- `cargo test -p mandu-tui --test tui_citation_modal_and_preview_spec`: 4 testes aprovados cobrindo abertura de modal, filtro por coleções, alternância com Tab, inserção com Enter, expurgo de comentários e renderização Joaju no preview, além de casos de mutação e banco vazio.

## 5. Laudo de Fechamento (Fase 9)
- **Status:** Concluído e aprovado.
- **Resultado:** Implementado seletor de citações interativo via atalho `Ctrl+R` com suporte a filtro por coleções do frontmatter (`ref_collections`) e alternância com `Tab`. Painel de preview integrado ao pipeline canônico Joaju (`prepare_markdown_pipeline`), expurgando comentários `%%...%%` e resolvendo referências bibliográficas.
