# Especificação 26: Permitir Múltiplas Definições na Linkedição Estática do Joaju

- **ID do Ticket:** #26
- **Status:** Concluído (DD-TDD) (closes #26)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

---

## 1. Contexto & Motivação

No ticket #25, adotou-se a pré-compilação do subsistema `joaju` como biblioteca estática multiarch C-ABI (`libjoaju.a`), vinculada via `build.rs` no `mandu-core`.

Ao executar a esteira de CI (`cargo test --all-targets`), a compilação de testes de crates dependentes (como `crates/mandu-server/tests/server_api_test.rs`) falhou na etapa de linkedição com o erro do `rust-lld`:
```text
rust-lld: error: duplicate symbol: rust_eh_personality
>>> defined at gcc.rs:299 (library/std/src/sys/personality/gcc.rs:299)
>>> in archive libmandu_core.rlib (via libjoaju.a)
>>> defined at gcc.rs:329
>>> in archive libstd.rlib
```

Por ser uma biblioteca estática compilada pelo ecossistema Rust (`crate-type = ["staticlib"]`), a `libjoaju.a` embute cópias dos símbolos de runtime padrão da linguagem (`rust_eh_personality`), que colidem com os símbolos embutidos na `libstd.rlib` do crate consumidor quando o linker não tolera múltiplas definições.

---

## 2. Requisitos Técnicos

1. **Configuração de Linkedição no `.cargo/config.toml`:**
   - Adicionar o argumento `-C link-arg=-Wl,--allow-multiple-definition` nos alvos `gnu` e `musl` (`x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl` e `x86_64-pc-windows-gnu`).
   - Isso instrui o linker (`lld`, GNU `ld` ou `collect2`) a aceitar definições repetidas entre `libjoaju.a` e `libstd`, selecionando a primeira e evitando falha na compilação dos testes e binários.

2. **Verificação de Integridade:**
   - Executar os testes em `mandu-server` e `mandu-core` garantindo resolução sem erro de linkedição.

---

## 3. Checklist GOST DD-TDD

- [x] Fase 1 (Ticket): #26 criado na forja
- [x] Fase 2 (Branch): `fix/26-permitir-multiplas-definicoes-na-linkedicao-estatica-do-joaju`
- [x] Fase 3 (Spec): Especificação técnica elaborada
- [x] Fase 4 (Red Test): Validar falha de linkedição sem as flags adequadas
- [x] Fase 5 (Green Implement): Aplicar configuração de linker em `.cargo/config.toml`
- [x] Fase 6 (Refactor): Limpeza e documentação da configuração
- [x] Fase 7 (Mutation Test): Teste de consistência das flags e invariantes
- [x] Fase 8 (Quality Gate): Validação com cargo fmt, cargo clippy e testes
- [x] Fase 9 (Finish & Close): Registro e conclusão do ciclo (closes #26)
