# Spec #33 — Corrigir Formatação Rustfmt em Shortcuts do Modal de Ajuda

## 1. Contexto e Motivação
Após a integração cumulativa dos modais de ajuda e atalhos na TUI, o array de tuplas em `crates/mandu-tui/src/ui.rs` continha uma quebra de linha manual divergente do padrão oficial do `rustfmt`, resultando em falha na etapa `Format Check` da esteira de integração contínua (`cargo fmt -- --check`).

## 2. Requisitos e Critérios de Aceite
1. Executar formatação canônica via `cargo fmt` garantindo aderência estrita à norma de estilo.
2. `cargo fmt -- --check` deve passar com saída limpa (código de saída 0).
3. Todas as 9 fases do GOST DD-TDD devem ser cumpridas para restabelecer o sinal verde no pipeline.

## 3. Ciclo GOST DD-TDD
- Fase 1: Ticket #33 formalizado.
- Fase 2: Branch `fix/33-corrigir-formatacao-rustfmt-em-shortcuts-do-modal-de-ajuda`.
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho (`cargo fmt -- --check` falha).
- Fase 5: Implementação verde (`cargo fmt`).
- Fase 6: Refatoração.
- Fase 7: Testes de mutação.
- Fase 8: Quality Gate (`cargo fmt`, `clippy`, testes, `cccp check --strict`).
- Fase 9: Fechamento com laudo via `cccp close`.
