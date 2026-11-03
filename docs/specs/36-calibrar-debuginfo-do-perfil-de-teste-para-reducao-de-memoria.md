# Especificação Técnica — Ticket #36: Calibrar Debuginfo do Perfil de Teste para Redução de Memória

## 1. Contexto e Motivação
A esteira de CI (`Checks, Tests & Quality Gates`) compila e executa todos os testes de integração do repositório (`cargo test --all-targets`).
Por padrão, os perfis `dev` e `test` do Cargo utilizam `debug = 2` (informações completas de depuração DWARF).
Em projetos com múltiplas bibliotecas e interfaces gráficas complexas (`egui`, `ratatui`, `dioxus`, `gtk`), a geração de tabelas DWARF completas gera binários de teste que exigem vários gigabytes de memória RAM por instância durante a linkedição (`ld`/`rustc`).
Quando o Cargo linka múltiplos binários de teste simultaneamente em `crates/mandu-tui/tests/`, o consumo de memória excede os limites de cgroups, provocando `SIGKILL: kill` no linker.

Ao configurar `debug = 1` (line tables only), o compilador gera informações suficientes para backtraces completos (arquivo e número de linha), reduzindo o uso de memória do linker em até 75-80% e acelerando significativamente a fase de teste.

## 2. Requisitos Técnicos
1. **Configuração nos Perfis de Workspace (`Cargo.toml`):**
   Definir `debug = 1` para os perfis `dev` e `test`:
   ```toml
   [profile.dev]
   debug = 1

   [profile.test]
   debug = 1
   ```
2. **Garantia de Ambiente na CI (`.forgejo/workflows/ci.yml`):**
   Injetar variáveis explícitas no job `test`:
   ```yaml
   env:
     CARGO_BUILD_JOBS: "2"
     CARGO_PROFILE_DEV_DEBUG: "1"
     CARGO_PROFILE_TEST_DEBUG: "1"
   ```
3. **Validação Automatizada (TDD):**
   Adicionar teste de conformidade (`tests/test_debuginfo_config.sh`) validando que:
   - Os perfis `profile.dev` e `profile.test` em `Cargo.toml` contêm `debug = 1`.
   - O workflow `.forgejo/workflows/ci.yml` possui `CARGO_PROFILE_TEST_DEBUG: "1"`.

## 3. Plano de Fases DD-TDD
- [x] Fase 1 (Ticket): Ticket #36 aberto via `cccp ticket new`
- [x] Fase 2 (Branch): Branch `chore/36-calibrar-debuginfo-do-perfil-de-teste-para-reducao-de-memoria` ativa
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Criação de `tests/test_debuginfo_config.sh` falhando antes da implementação
- [x] Fase 5 (Green Implement): Ajuste em `Cargo.toml` e `.forgejo/workflows/ci.yml`
- [x] Fase 6 (Refactor): Limpeza e verificação
- [ ] Fase 7 (Mutation Test): Teste mutacional de reversão da configuração
- [ ] Fase 8 (Quality Gate): Validação com ferramentas nativas (`cargo fmt --check`, `cargo clippy`, testes)
- [ ] Fase 9 (Finish & Close): Fechamento do ticket e merge via `cccp` sob autorização
