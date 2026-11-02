# Spec #32 — Fixar Versão Estrita SemVer de Joaju-Static no Build.rs

## 1. Contexto e Objetivo
Garantir que a compilação do crate `mandu-core` resolva a biblioteca estática `joaju-static` usando uma versão canônica estrita SemVer (`JOAJU_STATIC_REQUIRED_VERSION = "0.2.0-dev.137+20261102.e5c6878"`), eliminando a dependência do alias flutuante `latest` e o fallback legado para `0.2.0`, assegurando compilações determinísticas com fallback estrito para `0.2.0-dev.137`.

## 2. Requisitos e Diretrizes
1. Definir a constante `pub const JOAJU_STATIC_REQUIRED_VERSION: &str = "0.2.0-dev.137+20261102.e5c6878";` no `build.rs`.
2. O `build.rs` deve utilizar `env::var("JOAJU_STATIC_VERSION").unwrap_or_else(|_| JOAJU_STATIC_REQUIRED_VERSION.to_string())`.
3. Não deve existir referência a `"latest"` como valor padrão ou fallback.
4. Não deve existir referência à versão legada `"0.2.0"` como fallback.
5. Caso o download primário falhe, o fallback estrito deve ser para a versão sem metadata `"0.2.0-dev.137"`.
6. Atualizar a suíte de testes `tests/test_joaju_multiarch_build_script.sh` para refletir a nova exigência normativa.

## 3. Ciclo GOST DD-TDD
- Fase 1: Ticket #32 validado.
- Fase 2: Branch `chore/32-joaju-static-semver`.
- Fase 3: Doc/Spec (este documento).
- Fase 4: Red test em `tests/test_joaju_multiarch_build_script.sh`.
- Fase 5: Implementação verde em `crates/mandu-core/build.rs`.
- Fase 6: Refactor.
- Fase 7: Testes de mutação.
- Fase 8: Quality Gate (fmt, clippy, testes).
- Fase 9: Commit de fechamento convencional.
