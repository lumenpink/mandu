# Especificação Técnica — Delegar prepare_markdown_pipeline para joaju::prepare_markdown (Ticket #47)

## 1. Contexto e Motivação
A orquestração do pipeline de Markdown (`prepare_markdown_pipeline`) residia em `crates/mandu-core/src/preview.rs`, encadeando manualmente `joaju_refs::process_references` e as funções de `joaju_markdown` (`preprocess_comments`, `preprocess_callouts`, `preprocess_highlights`, `preprocess_wikilinks`).
Essa responsabilidade de encadeamento canônico foi migrada para o compilador `joaju` (Ticket #89), que introduziu `ReferenceSource` e a função pública `joaju::prepare_markdown`.
Este ticket emagrece `mandu-core`, transformando `prepare_markdown_pipeline` em um adaptador de domínio que mapeia `ReferenceMd` para `joaju_refs::ReferenceNote` e delega a transformação para `joaju::prepare_markdown(markdown, Some(ReferenceSource::Memory(&map)))`.

## 2. Decisões de Arquitetura e Árvore de Escolhas

### 2.1 Preservação da Interface Pública
A assinatura da função em `mandu-core`:
```rust
pub fn prepare_markdown_pipeline(
    markdown: &str,
    references: Option<&HashMap<String, ReferenceMd>>,
) -> String
```
permanece estritamente idêntica. Isso garante que nenhum consumidor (`mandu-tui`, `mandu-server`, `mandu-gui`, `mandu-mobile`, `mandu-web`) necessite de modificação em sua camada de chamada.

### 2.2 Desacoplamento da Esteira
- Remove-se a chamada manual sequencial das etapas de `joaju_markdown` e `joaju_refs` de dentro de `crates/mandu-core/src/preview.rs`.
- Adiciona-se a dependência `joaju = { version = "=0.2.0-dev.138", registry = "gambi", default-features = false, path = "../../../joaju" }`.
- O adaptador converte o mapa de `ReferenceMd` para `joaju_refs::ReferenceNote` e invoca `joaju::prepare_markdown(markdown, Some(ReferenceSource::Memory(&refs_map)))` (ou `None` quando ausente).

## 3. Preservação de Manutenibilidade (Para Natasha)
- **Por que manter `prepare_markdown_pipeline` em `mandu-core` em vez de apagar?** Para manter compatibilidade retroativa com todos os frontends e encapsular a conversão do modelo de domínio (`ReferenceMd`) do Mandu.
- **Por que `default-features = false` no `joaju`?** Para evitar puxar dependências desnecessárias de CLI (`clap`) ou I/O não suportado em ambientes como WASM.
- **Alternativas descartadas:**
  - *Forçar cada frontend do Mandu a importar `joaju` e fazer a conversão de `ReferenceMd`*: Descartada por espalhar código boilerplate de conversão em 5 repositórios diferentes.
