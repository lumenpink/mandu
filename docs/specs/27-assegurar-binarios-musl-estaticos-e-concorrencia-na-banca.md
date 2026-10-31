# Especificação 27: Assegurar Binários Musl Estáticos e Concorrência na Banca

- **ID do Ticket:** #27
- **Status:** Em Andamento (DD-TDD Fase 3)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

---

## 1. Contexto & Motivação

No Mandu, a esteira de integração contínua (`.forgejo/workflows/ci.yml`) executa o job `deploy-banca` para compilar e empacotar binários nativos e estáticos (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl` e `x86_64-pc-windows-gnu`).

Identificaram-se dois problemas críticos no pipeline de release:
1. **Sobrescrita de `RUSTFLAGS`:** O comando `export RUSTFLAGS="-C target-feature=+crt-static"` na esteira suprimia a flag `-C link-arg=-Wl,--allow-multiple-definition` configurada no `.cargo/config.toml`, resultando em falha de linkedição devido a duplicidade de símbolos entre a biblioteca estática pura musl `libjoaju.a` e a `libstd`.
2. **Concorrência de Matriz na Banca:** Os jobs da matriz (`linux-amd64`, `linux-arm64`, `windows-amd64`) tentavam efetuar clone, rotação e `git push` concorrentemente no repositório da Banca, gerando conflitos de concorrência não triviais.
3. **Resíduos Terminológicos:** Ocorrências de termos imperialistas ("soberano") foram localizadas em scripts de teste, mensagens e no CLI do TUI.

---

## 2. Requisitos Técnicos

1. **Configuração de Linkedição no CI:**
   - Em `.forgejo/workflows/ci.yml`, definir `RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-Wl,--allow-multiple-definition"` durante a compilação de binários estáticos.
   - Definir `max-parallel: 1` na estratégia da matriz em `deploy-banca`.

2. **Priorização de Biblioteca Musl no `build.rs` do `mandu-core`:**
   - Assegurar que `crates/mandu-core/build.rs` busque `/home/lumen/src/joaju/target/x86_64-unknown-linux-musl/release` prioritariamente ao linkar com a biblioteca local.

3. **Higienização Terminológica:**
   - Substituir qualquer ocorrência de "soberano" por "canônico" ou "centralizado" nos arquivos de workflow, scripts de teste e no cabeçalho do `mandu-tui`.

---

## 3. Checklist GOST DD-TDD

- [x] Fase 1 (Ticket): #27 criado na forja
- [x] Fase 2 (Branch): `fix/27-assegurar-binarios-musl-estaticos-e-concorrencia-na-banca`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Teste automatizado validando a presença de flags no workflow e ausência de concorrência
- [ ] Fase 5 (Green Implement): Ajustes no ci.yml, build.rs e mandu-tui
- [ ] Fase 6 (Refactor): Limpeza e garantia de zero advertências
- [ ] Fase 7 (Mutation Test): Testes de mutação com injeção de flags incompletas
- [ ] Fase 8 (Quality Gate): Validação com fmt, clippy e testes direcionados
- [ ] Fase 9 (Finish & Close): Registro e conclusão do ciclo (closes #27)
