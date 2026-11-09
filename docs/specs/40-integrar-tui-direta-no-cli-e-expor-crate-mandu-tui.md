# Especificação Técnica — Ticket #40

## feat(tui): integrar tui direta no cli e expor crate mandu-tui

### 1. Objetivo
Permitir que o executável `mandu` (produzido por `crates/mandu-cli`) execute o motor TUI (`mandu_tui::run`) diretamente em seu processo, sem depender de um subprocesso externo `mandu-tui` localizado no disco ou no `$PATH`. Além disso, expor o crate de biblioteca `mandu-tui` com metadados adequados para publicação no registro Gambi, viabilizando o consumo posterior pelo `mandu-gui`.

### 2. Mudanças de Arquitetura
1. **`crates/mandu-cli`**:
   - Adicionar dependência interna `mandu-tui = { path = "../mandu-tui" }`.
   - Modificar o despachante em `crates/mandu-cli/src/main.rs`:
     - Se o modo de destino resolvido for `TargetMode::Tui` (passado `--tui`, ou invocado sem argumentos em ambiente headless sem `$DISPLAY`/`$WAYLAND_DISPLAY`): invocar a função `mandu_tui::run(vault)` diretamente no processo corrente.
     - Se o modo de destino for `TargetMode::Gui`: continuar procurando o binário `mandu-gui` externo para delegar via `exec()`.
     - Se o modo for `CheckUpdate`, `Update` ou `VersionAll`: manter a execução canônica já existente.
2. **`crates/mandu-tui`**:
   - Assegurar que `mandu-tui` exporte de forma pública `pub fn run(vault_path: Option<PathBuf>) -> anyhow::Result<()>`.
   - Atualizar `crates/mandu-tui/Cargo.toml` para garantir metadados de publicação no registro Gambi.
3. **Distribuição**:
   - O binário `mandu` passa a ser auto-suficiente para terminais e servidores headless.

### 3. Critérios de Aceite
- Executar `mandu --tui` ou `mandu` em headless não requer a presença do arquivo `mandu-tui` no sistema de arquivos.
- Testes unitários comprovam a invocação in-process e o despacho consistente.
- Qualidade GOST 100% auditada por `cccp check --strict`.
