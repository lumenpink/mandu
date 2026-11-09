# Spec Ticket #41: Consolidar Aplicação Terminal em mandu e Eliminar Despachante

## 1. Contexto e Motivação
Historicamente, o binário `mandu` atuava como um despachante de linha de comando que tentava inspecionar o ambiente (`DISPLAY`, `WAYLAND_DISPLAY`) e varrer o sistema em busca de executáveis como `mandu-gui` e `mandu-tui` para executá-los em subprocessos.
Com a decisão arquitetural de separar o pacote desktop para o repositório `mandu-gui` (que agora se chamará `mandu-desktop`), o pacote de distribuição do repositório `mandu` é estritamente a aplicação de terminal. Portanto, manter lógica de despachante para processos externos ou duplicar binários (`mandu` e `mandu-tui`) gera overhead desnecessário e confusão para usuários de ambientes headless.

## 2. Escopo da Solução
- Consolidar `mandu` (gerado por `crates/mandu-cli`) como a aplicação direta e exclusiva de terminal.
- Remover busca por executáveis externos no `PATH` e invocação via `Command::exec` / `Command::status`.
- Executar a TUI diretamente no processo corrente quando acionado como `mandu` ou `mandu [nota.md]`.
- Se `--gui` for passado, informar educadamente que a interface gráfica pertence ao pacote `mandu-desktop`.
- Manter comandos de utilitário CLI: `--help`, `--version --all`, `check-update`, `update`.
- Remover a geração do binário duplicado `[[bin]] name = "mandu-tui"` de `crates/mandu-tui/Cargo.toml` (mantendo-o puramente como `mandu_tui` library).
- Atualizar a esteira `.forgejo/workflows/ci.yml` para compilar e empacotar unicamente o binário `mandu`.

## 3. Critérios de Aceitação
- `cargo fmt -- --check`: Formatação em conformidade sem desvios.
- `cargo clippy --all-targets -- -D warnings`: Zero advertências.
- `cargo test --all-targets`: Todos os testes unitários e de integração aprovados.
- Testes confirmando que `mandu` executa diretamente a TUI in-process e não depende de binários externos no `PATH`.
- `cccp check --strict`: Auditoria Gosplan 100% aprovada.
