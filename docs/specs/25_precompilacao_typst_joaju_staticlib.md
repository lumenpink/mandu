# Especificação 25: Pré-compilação e Vinculação Estática do Joaju/Typst (Opção 2)

- **ID do Ticket:** #25
- **Status:** Em Progresso (DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

---

## 1. Contexto & Motivação

Atualmente, o subsistema de renderização acadêmica e exportação via Typst (trazido pelo `joaju`) é compilado a partir do código-fonte Rust em todos os builds do Mandu (`mandu-tui`, `mandu-gui`, etc.).

Como o Typst é uma suíte extensa, a sua compilação e a linkedição com `LTO = "fat"` consomem grande quantidade de tempo (15–20 minutos) e memória RAM (>3.5 GB), gerando gargalos e até abortos por OOM no runner de CI.

O objetivo deste ticket é adotar a **Opção 2** do ticket #25:
- Pré-compilar o `joaju` e suas dependências pesadas (`typst-as-lib`, `typst-pdf`, etc.) como biblioteca estática multiarch C-ABI (`libjoaju.a` / `joaju.lib`) publicada na infraestrutura soberana (Forgejo Generic Packages).
- Vincular o binário estático no Cargo do `mandu-core` via `build.rs`, gerando um executável **100% único, autônomo e estático**.
- Reduzir o tempo de compilação do Mandu no CI de ~18 minutos para **menos de 1 minuto**.

---

## 2. Requisitos Técnicos

1. **Automação no `build.rs` de `crates/mandu-core`:**
   - Detectar arquitetura alvo (`TARGET`).
   - Baixar ou localizar a biblioteca estática correspondente (`libjoaju-<target>.a` ou `target/release/libjoaju.a` local caso em desenvolvimento monorepo).
   - Emitir diretivas `cargo:rustc-link-search` e `cargo:rustc-link-lib=static=joaju`.

2. **Desacoplamento de Dependência em `crates/mandu-core/Cargo.toml`:**
   - Remover dependência direta `joaju = { ... }` que arrasta Typst para a compilação do Mandu.
   - Criar wrapper FFI seguro em `crates/mandu-core/src/joaju_ffi.rs` consumindo as funções da C-ABI:
     - `joaju_compile_markdown_to_pdf`
     - `joaju_render_pedigree`
     - `joaju_render_chem`
     - `joaju_free_bytes`
     - `joaju_free_string`

3. **Integração no `crates/mandu-core/src/preview.rs`:**
   - Atualizar `render_block_svg` e `compile_markdown_to_pdf` para utilizar o wrapper FFI de forma transparente.

---

## 3. Critérios de Aceitação

1. `mandu-core` e `mandu-tui` compilam sem a crate `typst` in-tree.
2. Testes de renderização SVG e compilação PDF passando com sucesso.
3. Binários resultantes continuam 100% autônomos e estáticos (`musl-static` no Linux).
4. `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem avisos.
5. `cccp check --strict` 100% conforme.

---

## 4. Registro de Fases do DD-TDD
- [x] Fase 1 (Ticket): #25
- [x] Fase 2 (Branch): `feat/25-vincular-joaju-staticlib`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Criar teste red para vinculação estática do joaju
- [ ] Fase 5 (Green Implement): Implementar build.rs, wrapper FFI e desacoplar crate joaju
- [ ] Fase 6 (Refactor): Limpeza de memória e segurança FFI
- [ ] Fase 7 (Mutation Test): Validação de erros e ponteiros nulos
- [ ] Fase 8 (Quality Gate): Conformidade Gosplan e testes verdes
- [ ] Fase 9 (Finish & Close): Concluído e pronto para merge
