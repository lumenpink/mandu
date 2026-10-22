# Especificação 24: Otimização de Tamanho Binário e Linking Estático musl

- **ID do Ticket:** #14
- **Status:** Aprovado
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

## 1. Contexto & Motivação
Os binários compilados do Mandu gerados pela esteira de CI/CD apresentavam dois problemas críticos:
1. **Tamanho excessivo:** Binários com ~65 MB a ~78 MB devido à ausência de LTO, símbolos de depuração DWARF mantidos intactos, 16 unidades de compilação paralelas e unwinding desnecessário de exceções.
2. **Incompatibilidade de execução em distros glibc:** O `cargo zigbuild` compilava para o target `musl` sem a diretiva de linking estático integral (`+crt-static`), gerando executáveis com dependência de interpretador dinâmico ELF (`/lib/ld-musl-x86_64.so.1`), causando `cannot execute: required file not found` ou Falha de Segmentação em distros baseadas em glibc (como Debian e Ubuntu).

## 2. Requisitos Técnicos

### 2.1 Perfil de Release (`Cargo.toml`)
O arquivo raiz `Cargo.toml` deve declarar a seção `[profile.release]` com:
- `opt-level = 3` (ou `"s"`/`"z"` se aplicável): Otimização plena de código.
- `lto = "fat"`: Link-Time Optimization em todo o grafo de crates para eliminação de código morto (DCE).
- `codegen-units = 1`: Redução a uma única unidade LLVM para permitir poda cross-crate e inline agressivo.
- `panic = "abort"`: Eliminação das pesadas tabelas DWARF de unwinding de exceção.
- `strip = true`: Remoção automática de todos os símbolos de depuração e seções auxiliares.

### 2.2 Configuração Hermética de Build (`.cargo/config.toml`)
O repositório deve conter `.cargo/config.toml` forçando o linking estático da C runtime para os alvos musl:
- `[target.x86_64-unknown-linux-musl]` com `rustflags = ["-C", "target-feature=+crt-static"]`
- `[target.aarch64-unknown-linux-musl]` com `rustflags = ["-C", "target-feature=+crt-static"]`

### 2.3 Workflow de CI/CD (`.forgejo/workflows/ci.yml`)
Garantir que a compilação multiarch de release execute com essas variáveis ativas e os binários sejam gerados como `static-pie linked`.

## 3. Critérios de Aceitação
- Teste automatizado validando a presença e integridade das diretivas em `Cargo.toml` e `.cargo/config.toml`.
- O comando `file` nos binários gerados para musl deve reportar `statically linked` ou `static-pie linked` e `stripped`.
- Zero ocorrência de interpretador dinâmico `/lib/ld-musl*` nos binários gerados.
