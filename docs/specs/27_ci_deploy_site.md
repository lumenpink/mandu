# Especificação 27: Esteira de Build e Deploy Contínuo do Site (Ticket #20)

- **ID do Ticket:** #20
- **Branch:** `ci/20-deploy-continuo-site`
- **Status:** Em Especificação (Fase 3 DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (CI/CD)

---

## 1. Contexto & Motivação

Com a criação do site estático minimalista do Mandu (Ticket #19), é indispensável automatizar a publicação contínua dos artefatos estáticos (`site/`) na nuvem de borda (Cloudflare Workers via `wrangler.jsonc` para o domínio `mandu.2lp.in`).

Atualmente, o pipeline `.forgejo/workflows/ci.yml` do Mandu executa apenas os testes automatizados e o deploy de binários da Banca (`deploy-banca`), deixando o site sem atualização automática após pushes na branch principal (`main`).

---

## 2. Requisitos Arquiteturais & Contrato da Esteira

1. **Gatilhos e Dependência Estrita:**
   - O deploy contínuo do site deve ocorrer exclusivamente quando os testes do job `test` forem aprovados com sucesso (`needs: [test]` e `if: success()`).
   - O acionamento ocorre na branch `main` ou em tags versionadas (`v*`).
2. **Execução Hermética e Resiliente:**
   - O step de deploy do site deve utilizar `npx wrangler deploy` apontando para a raiz do repositório onde reside `wrangler.jsonc` e a pasta `site/`.
   - Injeção das credenciais padrão da organização: `CLOUDFLARE_API_TOKEN` e `CLOUDFLARE_ACCOUNT_ID`.
   - Tratamento de tolerância a oscilações de rede (WAN) com fallback gracioso (`npx wrangler deploy || echo "Aviso: Falha no deploy Cloudflare Workers."`), preservando o padrão de resiliência adotado nos outros projetos (`ayvu`, `navalha`, `fuxico`).
3. **Conformidade e Testabilidade:**
   - Atualização do script de teste de conformidade de CI `tests/test_ci_conformance.sh` para verificar a presença obrigatória da etapa de deploy do site via `wrangler deploy` e o uso das credenciais do Cloudflare.

---

## 3. Fases DD-TDD

- [x] Fase 1: Ticket #20 formalizado.
- [x] Fase 2: Branch `ci/20-deploy-continuo-site` criada.
- [x] Fase 3: Especificação `docs/specs/27_ci_deploy_site.md` concluída.
- [ ] Fase 4: Red Test atualizando `tests/test_ci_conformance.sh` exigindo o deploy do site.
- [ ] Fase 5: Green Implement adicionando o step de deploy no `.forgejo/workflows/ci.yml`.
- [ ] Fase 6: Refactor e limpeza de dependências do workflow.
- [ ] Fase 7: Testes de mutação validando comportamento sob ausência de variáveis.
- [ ] Fase 8: Quality Gate com execução de `tests/test_ci_conformance.sh` e testes unitários.
- [ ] Fase 9: Changelog atualizado e fechamento da issue #20.
