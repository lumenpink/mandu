# Especificação Técnica — Ticket #34: Remoção de OpenSSL e OpenSSL-Src do Mandu Core

## 1. Contexto e Motivação
O Mandu é um editor de notas em texto simples e Markdown. O crate central de domínio (`crates/mandu-core`) possuía uma dependência direta declarada:
```toml
openssl = { version = "0.10", features = ["vendored"] }
```
Essa dependência não é consumida por nenhum arquivo `.rs` de `crates/mandu-core` ou da aplicação (`mandu-tui`, `mandu-cli`, `mandu-gui`). No entanto, a flag `features = ["vendored"]` força o Cargo a invocar o build script de `openssl-sys`, baixando o código-fonte C de `openssl-src` e disparando um processo completo de compilação em C via `make`.

Esse build redundante:
1. Consome mais de 5 minutos adicionais de compilação em ambientes limpos.
2. Dispara dezenas de processos do compilador C (`cc`) em paralelo, gerando picos massivos de memória e contenção com o `sccache`.
3. Provocou estouro de memória (`SIGKILL: kill`) no container de CI.

## 2. Requisitos Técnicos
1. **Remoção Estrita de Dependência:**
   Remover a entrada `openssl = { version = "0.10", features = ["vendored"] }` de `crates/mandu-core/Cargo.toml`.
2. **Higienização do Cargo.lock:**
   Executar `cargo check -p mandu-core` para atualizar `Cargo.lock` eliminando a compilação de `openssl-src` em builds que utilizam apenas o núcleo.
3. **Garantia de Não-Regressão e Integridade:**
   Todos os crates (`mandu-core`, `mandu-tui`, `mandu-cli`, `mandu-gui`) devem compilar sem warnings e passar em todos os testes unitários e de integração.
4. **Verificação de Ausência de OpenSSL:**
   Nenhum código Rust em `crates/*/src/` deve conter referências ao crate `openssl`.

## 3. Plano de Fases DD-TDD
- [x] Fase 1 (Ticket): Ticket #34 aberto via `cccp ticket new`
- [x] Fase 2 (Branch): Branch `chore/34-remover-openssl-de-mandu-core` ativa
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [ ] Fase 4 (Red Test): Teste de conformidade verificando que `crates/mandu-core/Cargo.toml` não declara openssl
- [ ] Fase 5 (Green Implement): Remoção da dependência em `Cargo.toml`
- [ ] Fase 6 (Refactor): Limpeza e alinhamento
- [ ] Fase 7 (Mutation Test): Validação cruzada dos testes de conformidade
- [ ] Fase 8 (Quality Gate): Validação com `cargo fmt --check`, `cargo clippy` e `cargo test`
- [ ] Fase 9 (Finish & Close): Fechamento com `cccp close` e merge via `cccp merge`
