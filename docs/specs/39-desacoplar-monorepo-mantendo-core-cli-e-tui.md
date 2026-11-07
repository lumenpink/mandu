# Especificação: Desacoplar Monorepo Mantendo Core, CLI e TUI

## 1. Contexto e Motivação
O repositório `mandu` acumulava múltiplos crates pesados (`mandu-gui` em egui, `mandu-mobile` em Dioxus Android, `mandu-server` em Axum e `mandu-web` em Dioxus WASM) em um único workspace. As dependências gráficas e do Dioxus Desktop (WebKit2GTK/GTK3) sobrecarregavam a esteira de CI, excedendo o timeout estrito de 45 minutos do runner. Conforme decisão de arquitetura modular similar à adotada no ecossistema `joaju`, os crates de interface gráfica, mobile, web e servidor foram extraídos para repositórios independentes (`mandu-gui`, `mandu-mobile`, `mandu-server`, `mandu-web`). O repositório `mandu` passa a concentrar exclusivamente o núcleo de domínio (`mandu-core`), o despachante de linha de comando (`mandu-cli`) e a interface de terminal (`mandu-tui`).

## 2. Requisitos Técnicos
1. **Reestruturação do Workspace (`Cargo.toml`)**:
   - Manter apenas `crates/mandu-core`, `crates/mandu-cli` e `crates/mandu-tui` no workspace.
   - Remover os diretórios dos crates extraídos do repositório `mandu`.
2. **Atualização da Esteira de CI (`.forgejo/workflows/ci.yml`)**:
   - O job `test` passa a executar `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` e `cargo test --all-targets` apenas sobre `core`, `cli` e `tui`, concluindo em menos de 3 minutos.
   - Os jobs de compilação multiarch na Banca passam a empacotar os binários `mandu` (CLI despachante) e `mandu-tui`.
   - Remoção dos passos de APK Android e WebKit desnecessários deste repositório (transferidos para os respectivos repositórios).
3. **Testes de Especificação & Isolamento**:
   - Assegurar que `mandu-cli` compila e executa o despachante com sucesso.
   - Assegurar que `mandu-tui` compila e executa os testes do terminal sem depender de Dioxus ou egui.
   - Assegurar que o workspace não referencia mais os crates extraídos.
4. **Qualidade e Estilo**:
   - Formatação estrita com `cargo fmt`.
   - Linter sem avisos com `cargo clippy --all-targets -- -D warnings`.
   - 100% de testes aprovados com `cargo test --all-targets`.

## 3. Registro de Execução e Fechamento
- Fase 3 (Spec): Especificação elaborada e versionada.
- Fase 4 (Red Test): Teste de integridade de workspace sem dependências externas adicionado comprovando falha.
- Fase 5 (Green Implement): Remoção dos crates do workspace e ajuste de CI concluído; testes aprovados em 6s.
- Fase 6 (Refactor): Limpeza e otimização.
- Fase 7 (Mutation Test): Teste de mutação validando isolamento aprovado.
- Fase 8 (Quality Gate): Testes e clippy aprovados com 100% de sucesso em 8s.
- Fase 9 (Finish & Close): Ciclo concluído e pronto para integração.
