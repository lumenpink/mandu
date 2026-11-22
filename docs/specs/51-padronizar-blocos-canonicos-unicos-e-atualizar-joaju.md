# Especificação Técnica: Fix #51 - Padronizar Blocos Canônicos Únicos e Atualizar Joaju

## Contexto e Histórico
No ecossistema Mandu/Joaju, os blocos de código com identificadores especiais (tabelas, diagramas, química, versos) possuíam aliases variados como `pedtools`, `kinship2`, `openchemlib`, `openbabel`, `salt`, `flowchart`, etc.
Para evitar confusão e inconsistências de renderização entre o compilador `joaju`, a CLI, a TUI do `mandu`, o visualizador desktop `mandu-gui` e o app `mandu-mobile`, travamos rigorosamente identificadores canônicos únicos sem qualquer alias:
- `table` (tabelas)
- `pedigree` (heredogramas)
- `chem` (química e SMILES)
- `lirica` (poesia e escansão)
- `wireframe` (mockups de interface)
- `sequence` (diagramas de sequência)
- `graph` (grafos e fluxogramas)
- `typst` (código Typst)
- `unfurl` (enriquecimento de links)

Neste ticket, atualizamos a dependência `joaju` em `crates/mandu-core` para a versão `0.2.0-dev.140` e alinhamos a TUI (`crates/mandu-tui/src/blocks.rs`) e o parser de preview (`crates/mandu-core/src/preview.rs`) para reconhecer apenas esses motores canônicos.

Para Natasha manter este código: qualquer menção a nomes legados (`pedtools`, `salt`, `flowchart`, `openchemlib`) foi removida dos detectores de bloco ativo para garantir previsibilidade e ausência de bugs sutis.

## Alternativas Descartadas
- *Suportar aliases legados na TUI:* Descartado para manter integridade com as diretivas do Joaju #92 e evitar divergência de comportamento entre preview HTML e TUI.

## Validação
- Testes de compilação e unidade em `crates/mandu-core` (incluindo FFI estático `joaju_staticlib_ffi_spec` e cache de preview `preview_engine_test`) e `crates/mandu-tui` (`tui_satellite_cards_spec`, `tui_blocks_test`, `tui_layout_test`).
- `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` aprovados com zero warnings.
