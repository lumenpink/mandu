# Especificação Técnica — Desacoplar storage-local, Pipeline Joaju e Coleções (Ticket #45)

## 1. Contexto e Motivação
O `mandu-core` é o núcleo de domínio do ecossistema Mandu. Historicamente, ele dependia diretamente e incondicionalmente de `rusqlite`, `notify`, `walkdir` e `rayon`.
Essa dependência direta impedia a compilação do crate para alvos puros como `wasm32-unknown-unknown` (usado pelo `mandu-web`), forçando a duplicação indevida de tipos e lógica em subprojetos.

Além disso, as notas de literatura sincronizadas pelo Tatupeba incluem o campo `collections` no YAML frontmatter, e o pipeline de compilação precisa resolver referências (`joaju-refs`) e adaptar sintaxes do Obsidian como callouts, comentários e realces (`joaju-markdown`) antes da renderização nos frontends.

## 2. Decisões de Arquitetura e Árvore de Escolhas

### 2.1 Feature Flags no `Cargo.toml`
- `default = ["storage-local"]`
- `storage-local = ["dep:rusqlite", "dep:notify", "dep:walkdir", "dep:rayon"]`
- O domínio puro (`ReferenceMd`, `Note`, `InternalLink`, `AutocompleteItem`, extração de metadados, formatação de citações e motor fuzzy com `nucleo`) não depende de SO e compila para qualquer alvo.

### 2.2 Dependências Compartilhadas do Joaju
Adicionar ao `mandu-core` as crates já publicadas no registry Gambi:
- `joaju-refs = "=0.1.0-dev.12"` (resolução de citações e bibliografia)
- `joaju-markdown = "=0.1.0-dev.34"` (callouts, destaques, comentários e wikilinks)
Ambas são puras (baseadas em regex e manipulação de strings) e rodam nativamente em desktop, Android e WASM.

### 2.3 Metadados de Coleções (`collections`)
- `ReferenceMd`:
  ```rust
  pub struct ReferenceMd {
      pub citekey: String,
      pub note_path: String,
      pub doi: Option<String>,
      pub title: String,
      pub authors_json: Option<String>,
      pub year: Option<i32>,
      pub csl_yaml: Option<String>,
      pub collections: Vec<String>,
  }
  ```
- Extração no frontmatter: ler o array `collections` do YAML/JSON gerado pelo Tatupeba.
- No `ManduDb`: adicionar coluna `collections_json TEXT` à tabela `references_md` e suportar listagem filtrada por coleção.

### 2.4 Pipeline Canônico de Pré-Processamento
Expor em `mandu_core::preview` (ou `mandu_core::pipeline`):
```rust
pub fn prepare_markdown_pipeline(
    markdown: &str,
    references: Option<&HashMap<String, ReferenceNote>>,
) -> String
```
Executa a esteira padronizada:
1. `joaju_refs::process_references(markdown, references)` (se houver referências fornecidas)
2. `joaju_markdown::preprocess_comments(md)`
3. `joaju_markdown::preprocess_callouts(md)`
4. `joaju_markdown::preprocess_highlights(md)`
5. `joaju_markdown::preprocess_wikilinks(md)`

## 3. Preservação de Manutenibilidade (Para Natasha)
- **Por que feature flag e não outro crate?** Evita proliferação de microrrepositórios mantendo os tipos canônicos em `mandu-core`.
- **Por que pipeline centralizado?** Garante que o preview da TUI, da GUI, do Mobile e da Web renderizem exatamente o mesmo documento, sem surpresas de formatação.

## 4. Validação e Quality Gate (Fase 8)
- `cargo fmt --check`: Aprovado com zero divergências.
- `cargo clippy --all-targets -- -D warnings`: Aprovado com zero alertas no workspace completo.
- `cargo clippy --no-default-features -p mandu-core -- -D warnings`: Aprovado garantindo compilação pura para WASM sem dependências de SO.
- `cargo test -p mandu-core --test collections_and_pipeline_spec`: 4 testes aprovados cobrindo extração de coleções, filtragem por coleções no SQLite, pré-processamento do pipeline Joaju e casos de borda de mutação.

## 5. Laudo de Fechamento (Fase 9)
- **Status:** Concluído e aprovado.
- **Resultado:** `mandu-core` modularizado sob feature flag `storage-local`, compatível com alvos nativos e WASM. Pipeline Joaju integrado para resolução de citações e sintaxe Obsidian. Coleções do Tatupeba indexadas e consultáveis no autocompletar.
