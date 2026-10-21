# Specification: Unificação do Workflow em ci.yml e Atualização para GOST 1.5.0

- **Ticket:** #8
- **Branch:** `feat/8-unify-ci-yml-and-gost-1-5-0`
- **Standard:** GOST v1.5.0
- **Status:** In Progress (Phase 3: Spec)

---

## 1. Contexto e Objetivos

Esta especificação define a unificação dos pipelines de automação do Mandu em estrita conformidade com as seções 7, 8 e 9 do GOST v1.5.0:

1. **Arquivo Canônico Único:**
   - A automação de CI/CD reside obrigatoriamente no arquivo único `.forgejo/workflows/ci.yml`.
   - Remoção dos arquivos legados fragmentados `.forgejo/workflows/ci.yaml` e `.forgejo/workflows/release-banca.yaml`.

2. **Separação Rígida e Dependência Estrita de Jobs:**
   - Job de Validação (`test` / `check-and-test`): Executa `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` e `cccp check --strict`.
   - Job de Deploy (`deploy-banca`): Declara obrigatoriamente `needs: [test]`, rodando apenas após a aprovação integral dos testes em pushes na branch `main`, tags `v*` ou `workflow_dispatch`.

3. **Compilação Multiarch Soberana via Container:**
   - Execução em container `docker.io/lumenpink/gost-ci-cross:latest`.
   - Alvos: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, `x86_64-pc-windows-gnu`.
   - Upload de artefatos para Cloudflare R2 (`twolp-bucket/downloads/mandu/${CHANNEL}/...`).
   - Rotação do manifesto `site/downloads/mandu/index.json` na Banca com retenção máxima de 5 stables e 10 nightlies.

4. **Conformidade GOST v1.5.0:**
   - Atualização do `GOSTVER` para `1.5.0`.
   - Incorporação das normas `GOST-23-CI-EXCLUSIVE-BUILD` e `GOST-24-BANCA-DISTRIBUTION`.
