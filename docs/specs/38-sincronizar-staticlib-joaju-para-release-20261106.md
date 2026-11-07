# Especificação: Sincronizar Staticlib Joaju para Release 20261106

## 1. Contexto e Motivação
A biblioteca estática pré-compilada `joaju-static` foi atualizada com correções de injeção de versão e sincronização com o catálogo da Banca (release `0.2.0-dev.137+20261106.6eda140`). O Mandu utiliza o motor do Joaju via FFI (`crates/mandu-core`) para renderização química, heredogramas e compilação de notas Markdown para PDF. Para propagar essas atualizações e disponibilizar novas compilações desktop e Android na Banca, é necessário atualizar a constante `JOAJU_STATIC_REQUIRED_VERSION`.

## 2. Requisitos Técnicos
1. **Resolução de Staticlib (`crates/mandu-core/build.rs`)**:
   - Atualizar `pub const JOAJU_STATIC_REQUIRED_VERSION: &str = "0.2.0-dev.137+20261106.6eda140";`.
   - Manter a cadeia de fallback e flags de linkedição (`-Wl,--allow-multiple-definition`).
2. **Testes de Especificação**:
   - Adicionar asserções em `crates/mandu-core/tests/joaju_staticlib_ffi_spec.rs` garantindo que a versão requerida coincide com a release canônica ativa.
   - Adicionar teste de mutação confirmando a sensibilidade caso ocorra regressão para versões antigas.
3. **Qualidade e Estilo**:
   - Formatação estrita com `cargo fmt`.
   - Linter sem avisos com `cargo clippy -p mandu-core --all-targets -- -D warnings`.

## 3. Registro de Execução e Fechamento
- Fase 3 (Spec): Especificação elaborada e versionada.
- Fase 4 (Red Test): Teste `test_joaju_static_required_version_is_current` introduzido com falha comprovada.
- Fase 5 (Green Implement): `JOAJU_STATIC_REQUIRED_VERSION` atualizado para `0.2.0-dev.137+20261106.6eda140` e testes aprovados.
- Fase 6 (Refactor): Formatação via `cargo fmt` e validação com `cargo clippy`.
- Fase 7 (Mutation Test): Teste de mutação `test_mutation_version_mismatch_detected` aprovado.
- Fase 8 (Quality Gate): Testes de FFI e quality gate aprovados com 100% de sucesso.
- Fase 9 (Finish & Close): Ciclo concluído e integrado via `cccp merge`.
