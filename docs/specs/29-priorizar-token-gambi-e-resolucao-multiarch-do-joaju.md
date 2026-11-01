# Spec do Ticket #29: Priorizar Token Gambi e Resolução Multiarch do Joaju

## 1. Contexto e Motivação
Durante a compilação multiarch do Mandu (`crates/mandu-core/build.rs`), o script de build deve:
1. Buscar primeiro a biblioteca estática compilada localmente para o target exato (`/home/lumen/src/joaju/target/{target}/release`), evitando que um build local x86_64 contamine compilações cross (ex.: `aarch64` ou `windows`).
2. Priorizar de maneira estrita o token `GAMBI_2LP_IN_TOKEN`, permitindo fallback ordenado para `GIT_2LP_TOKEN`, `REGISTRY_TOKEN`, `LUMEN_REGISTRY_TOKEN` e `FORGEJO_TOKEN`, higienizando prefixos (`Bearer `).
3. Na ausência de binário local, baixar da Forgejo Generic Packages o artefato estático correspondente ao target especificado (`libjoaju-x86_64-linux.a`, `libjoaju-aarch64-linux.a`, `libjoaju-x86_64-windows.a`), suportando versionamento parametrizado via `JOAJU_STATIC_VERSION` (padrão `0.2.0`), com flags de rede robustas (`--http1.1 --retry 3 -s -f`).
4. Repassar as variáveis de autenticação (`GAMBI_2LP_IN_TOKEN`, `GIT_2LP_TOKEN`, etc.) na etapa de compilação e deploy da esteira `.forgejo/workflows/ci.yml`.

## 2. Escopo Técnico
- `crates/mandu-core/build.rs`:
  - Corrigir a ordem de `local_paths` para priorizar `{target}` antes de fallbacks genéricos x86_64.
  - Implementar resolução com prioridade para `GAMBI_2LP_IN_TOKEN`.
  - Remover fallback indiscriminado para `libjoaju-x86_64-linux.a` quando o target for Windows ou ARM64 (evitando erro de linkagem "file in wrong format").
  - Adicionar `--http1.1 --retry 3` na requisição `curl`.
- `.forgejo/workflows/ci.yml`:
  - Repassar `GAMBI_2LP_IN_TOKEN` em todos os jobs e etapas que consom o registro ou a Banca.
- Testes:
  - Adicionar teste canônico em `tests/test_joaju_multiarch_build_script.sh` validando a resolução de paths e tokens do script de build.

## 3. Plano de Fases DD-TDD
- [x] Fase 1 (Ticket): #29 criado
- [x] Fase 2 (Branch): `fix/29-priorizar-token-gambi-e-resolucao-multiarch-do-joaju` ativada
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Criação do teste de resolução do script de build
- [x] Fase 5 (Green Implement): Atualização de `build.rs` e `ci.yml`
- [x] Fase 6 (Refactor): Limpeza de código e clippy zero warnings
- [x] Fase 7 (Mutation/Adversarial): Execução dos testes da suíte e checagem de alvos
- [x] Fase 8 (Quality Gate): Testes canônicos, `cargo fmt`, `cargo clippy` e `cccp check --strict`
- [x] Fase 9 (Finish): Encerramento com `cccp close` e merge via `cccp merge`
