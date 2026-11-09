# Spec Ticket #43: Suporte a Cartões Visuais de Lírica e Motores Joaju na TUI

## 1. Contexto e Motivação
O `mandu` utiliza a biblioteca estática `joaju` (`libjoaju.a`) para orquestração tipográfica, compilação de documentos e renderização de blocos visuais especializados. Enquanto blocos de tabelas (`table`/`csv`), heredogramas (`pedigree`/`heredograma`) e Typst (`typst`/`joaju`) já possuem cartões dedicados com bordas e estilização na TUI (`crates/mandu-tui/src/blocks.rs`), os demais motores satélites do ecossistema caíam na caixa de código genérica (`render_code_box`):
1. **Lírica e Escansão Poética** (`lirica`, `poema`, `poesia`): Necessita de visualização elegante com título do poema, indicação de métrica/esquema estrófico e contagem de versos e sílabas.
2. **Química e SMILES** (`chem`, `smiles`): Necessita de cartão estilizado destacando a notação SMILES/código químico e formato detectado.
3. **Protótipos de Interface** (`wireframe`, `salt`): Necessita de cartão estruturado de UI mockup.
4. **Diagramas Temporais de Sequência** (`sequence`): Necessita de cartão visual evidenciando participantes e fluxo de mensagens.
5. **Grafos Conexos e Fluxogramas** (`graph`, `flowchart`): Necessita de cartão exibindo nós, conexões e direção.

Além disso:
- A camada C-ABI em `crates/mandu-core/src/joaju_ffi.rs` deve expor as chamadas seguras para os novos símbolos C-ABI (`joaju_render_lirica`, `joaju_render_wireframe`, `joaju_render_sequence`, `joaju_render_graph`).
- O script `crates/mandu-core/build.rs` deve incluir o caminho local de compilação `/home/lumen/src/joaju/target/debug` para localização imediata do `libjoaju.a` gerado no ambiente de desenvolvimento.
- O catálogo de componentes em `crates/mandu-core/src/about.rs` (`mandu --version --all`) deve listar com exatidão os 11 motores e componentes satélites do ecossistema.

## 2. Escopo da Solução

### 2.1 Camada FFI em `crates/mandu-core` (`src/joaju_ffi.rs` e `build.rs`)
- Declarar as assinaturas `extern "C"`:
  - `joaju_render_lirica(code, out_typst, out_error) -> i32`
  - `joaju_render_wireframe(code, out_typst, out_error) -> i32`
  - `joaju_render_sequence(code, out_typst, out_error) -> i32`
  - `joaju_render_graph(code, out_typst, out_error) -> i32`
- Implementar wrappers seguros:
  - `render_lirica(code: &str) -> Result<String, String>`
  - `render_wireframe(code: &str) -> Result<String, String>`
  - `render_sequence(code: &str) -> Result<String, String>`
  - `render_graph(code: &str) -> Result<String, String>`
- Atualizar `crates/mandu-core/build.rs` para inspecionar `/home/lumen/src/joaju/target/debug` entre as buscas locais prioritárias.

### 2.2 Cartões Especializados na TUI (`crates/mandu-tui/src/blocks.rs`)
- `render_lirica_card(code: &str, width: usize) -> Vec<Line<'static>>`:
  - Extrai metadados opcionais (`%% titulo = ...`, `%% autor = ...`).
  - Apresenta borda magenta/rosa estilizada `╭─ ✒ Lírica: <Título> ───╮`.
  - Exibe versos indentados e rodapé informativo com total de versos/estrofes.
- `render_chem_card(code: &str, width: usize) -> Vec<Line<'static>>`:
  - Borda verde/cianita `╭─ 🧪 Química: SMILES ───╮`.
  - Exibe a fórmula ou blocos de ligação estilizados.
- `render_wireframe_card(code: &str, width: usize) -> Vec<Line<'static>>`:
  - Borda azul/amarela `╭─ 📐 Wireframe (UI) ───╮`.
  - Pré-visualização formatada dos elementos de interface.
- `render_sequence_card(code: &str, width: usize) -> Vec<Line<'static>>`:
  - Borda ciano/azul `╭─ ⇄ Diagrama de Sequência ───╮`.
  - Exibição de interações `A -> B`.
- `render_graph_card(code: &str, width: usize) -> Vec<Line<'static>>`:
  - Borda verde/amarela `╭─ ☊ Grafo / Fluxograma ───╮`.
  - Exibição estruturada dos relacionamentos `A --> B`.
- No despachador `render_markdown_lines`, rotear as linguagens correspondentes (`lirica`, `poema`, `poesia`, `chem`, `smiles`, `wireframe`, `salt`, `sequence`, `graph`, `flowchart`).

### 2.3 Catálogo de Ecossistema (`crates/mandu-core/src/about.rs`)
- Expandir a listagem de componentes satélites para incluir explicitamente:
  `joaju-pdf`, `joaju-chem`, `joaju-pedigree`, `joaju-table`, `joaju-markdown`, `joaju-templates`, `joaju-lirica`, `joaju-slides`, `joaju-wireframe`, `joaju-sequence`, `joaju-graph`.

## 3. Critérios de Aceitação
- `cargo fmt -- --check`: Zero discrepâncias de formatação.
- `cargo clippy --all-targets -- -D warnings`: Zero advertências.
- `cargo test --all-targets`: Todos os testes da workspace `mandu` aprovados.
- Testes unitários comprovando a renderização correta de cada novo tipo de cartão na TUI.
- Testes unitários para as novas funções FFI em `mandu-core`.
- `cccp check --strict`: 100% de conformidade com as diretivas GOST.
