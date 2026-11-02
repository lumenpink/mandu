# Especificação Técnica — Ticket #35: Calibrar Concorrência do Cargo na CI

## 1. Contexto e Motivação
O runner de CI do Forgejo opera sob contenção estrita de recursos gerenciada por cgroup:
`--cpus=2.0 --memory=4.5g --memory-swap=8g`.
No entanto, dentro do container Docker, o comando `cargo` consulta a máquina hospedeira e detecta 4 núcleos disponíveis, iniciando por padrão 4 instâncias simultâneas de `rustc` durante checagens e compilações (`cargo clippy`, `cargo test`).

Essa discrepância gera contenção de CPU e estouro de memória sob picos de compilação simultânea com caches em disco.

## 2. Requisitos Técnicos
1. **Configuração Canônica de Concorrência:**
   Injetar a variável de ambiente:
   ```yaml
   env:
     CARGO_BUILD_JOBS: "2"
   ```
   no job `test` (Checks, Tests & Quality Gates) de `.forgejo/workflows/ci.yml`.
2. **Preservação de Cache:**
   O cache do `sccache` continua ativo e funcional, operando sob uma carga distribuída de forma equilibrada com 2 threads de compilação.
3. **Validação Automatizada:**
   Criar teste de conformidade de workflow (`tests/test_ci_concurrency_config.sh` ou em Rust) garantindo que `CARGO_BUILD_JOBS` esteja configurado no workflow de CI.

## 3. Plano de Fases DD-TDD
- [x] Fase 1 (Ticket): Ticket #35 aberto via `cccp ticket new`
- [x] Fase 2 (Branch): Branch `chore/35-calibrar-concorrencia-do-cargo-na-ci` ativa
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Teste de conformidade verificando a presença de `CARGO_BUILD_JOBS: "2"` em `ci.yml`
- [x] Fase 5 (Green Implement): Adição de `CARGO_BUILD_JOBS` no job `test` de `.forgejo/workflows/ci.yml`
- [x] Fase 6 (Refactor): Limpeza e auditoria de formatação
- [x] Fase 7 (Mutation Test): Teste cruzado de integridade do workflow
- [ ] Fase 8 (Quality Gate): Validação com `cargo fmt --check`, `cargo clippy`, testes e `cccp check --strict`
- [ ] Fase 9 (Finish & Close): Fechamento com `cccp close` e merge via `cccp merge`
