# Especificação 28: Decomposição da Esteira Multiarch em Matrix e Isolamento de deploy-site (Ticket #24)

- **ID do Ticket:** #24
- **Branch:** `feat/24-decompor-esteira-multiarch-matrix-deploy-site`
- **Status:** Em Especificação (Fase 3 DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (CI/CD Soberano)

---

## 1. Contexto & Motivação

Atualmente, o pipeline `.forgejo/workflows/ci.yml` do Mandu agrupa no job `deploy-banca` a compilação sequencial dos 3 pacotes (`mandu-cli`, `mandu-tui` e `mandu-gui`) para as 3 arquiteturas suportadas (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl` e `x86_64-pc-windows-gnu`), além da publicação do site estático em Cloudflare Workers.

O somatório do tempo de compilação ultrapassou 54 minutos. No ecossistema de infraestrutura, há uma rotina rígida no cron (`prune-stuck-tasks.sh`) que expurga containers de CI ativos por mais de 45 minutos para evitar que processos travados bloqueiem a fila única do runner. Consequentemente, o container do job foi encerrado via SIGKILL (exit code 137).

Para respeitar rigorosamente o limite de 45 minutos e destravar a fila de forma eficiente:
1. A publicação do site estático (`mandu.2lp.in`) deve ser desacoplada em um job independente `deploy-site` (execução ultra-rápida, < 1 minuto).
2. O job `deploy-banca` deve ser decomposto através de `strategy.matrix` com 3 alvos (`linux-amd64`, `linux-arm64`, `windows-amd64`), reduzindo o tempo de cada container para 15–25 minutos.

---

## 2. Requisitos Arquiteturais & Contrato da Esteira

1. **Job `deploy-site` Dedicado:**
   - Dependência estrita: `needs: [test]` e `if: success() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/tags/v')) && (github.event_name == 'push' || github.event_name == 'workflow_dispatch')`.
   - Executa `npx wrangler deploy` para publicar `site/` no Cloudflare Workers de forma imediata após os testes.
2. **Job `deploy-banca` Matricial:**
   - Dependência estrita: `needs: [test]` e `if: success() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/tags/v')) && (github.event_name == 'push' || github.event_name == 'workflow_dispatch')`.
   - Estratégia `strategy.matrix` com `fail-fast: false`:
     - Alvo 1: `target: x86_64-unknown-linux-musl`, `arch: linux-amd64`, `pkg_ext: tar.gz`
     - Alvo 2: `target: aarch64-unknown-linux-musl`, `arch: linux-arm64`, `pkg_ext: tar.gz`
     - Alvo 3: `target: x86_64-pc-windows-gnu`, `arch: windows-amd64`, `pkg_ext: zip`
   - Cada instância da matriz compila os binários de seu respectivo alvo, empacota, envia para o Cloudflare R2 e atualiza o manifesto na Banca via `rotate-downloads.py`.
   - Adição de mecanismo de retentativa resiliente no push do manifesto da Banca (`git pull --rebase` com até 5 tentativas).
3. **Conformidade de Imagem Soberana:**
   - Manutenção do container `gambi.2lp.in/lumen/gost-ci-cross:latest`.

---

## 3. Fases DD-TDD

- [x] Fase 1: Ticket #24 formalizado.
- [x] Fase 2: Branch `feat/24-decompor-esteira-multiarch-matrix-deploy-site` criada.
- [x] Fase 3: Especificação `docs/specs/28_decompor_esteira_multiarch_matrix.md` concluída.
- [ ] Fase 4: Red Test atualizando `tests/test_ci_conformance.sh` e `tests/test-pipeline-multiarch.sh` exigindo `deploy-site` e a matriz de `deploy-banca`.
- [ ] Fase 5: Green Implement adicionando o job `deploy-site` e o `strategy.matrix` no `.forgejo/workflows/ci.yml`.
- [ ] Fase 6: Refactor de scripts e retentativas de push na Banca.
- [ ] Fase 7: Testes de mutação validando sensibilidade a regressões nos alvos e jobs.
- [ ] Fase 8: Quality Gate com execução das suítes de validação nativas.
- [ ] Fase 9: Changelog atualizado e fechamento da issue #24.
