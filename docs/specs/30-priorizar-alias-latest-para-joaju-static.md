# Spec #30 — Priorizar Alias Latest para Joaju-Static no Build

## 1. Contexto e Objetivo
Garantir que a compilação do crate `mandu-core` resolva `joaju-static` priorizando por padrão o alias `latest` em Generic Packages (ou a variável de ambiente `JOAJU_STATIC_VERSION`), mantendo fallback resiliente para a versão estável `0.2.0`.

## 2. Requisitos
1. Em `crates/mandu-core/build.rs`, definir `"latest"` como valor padrão em `env::var("JOAJU_STATIC_VERSION").unwrap_or_else(|_| "latest".to_string())`.
2. Caso a resolução primária sob `latest` falhe ou o arquivo não exista após o download, executar fallback automático e transparente para a versão canônica `0.2.0`.
3. Atualizar `tests/test_joaju_multiarch_build_script.sh` para verificar a configuração de `latest` como padrão e testar mutação.

## 3. Plano de Ação (GOST DD-TDD)
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho validando a exigência de `latest` em `tests/test_joaju_multiarch_build_script.sh`.
- Fase 5: Implementação verde em `crates/mandu-core/build.rs`.
- Fase 6: Refatoração e limpeza.
- Fase 7: Teste de mutação.
- Fase 8: Quality Gate (`cargo fmt`, `cargo clippy`, testes de conformidade, `cccp check --strict`).
- Fase 9: Conclusão, merge e push.
