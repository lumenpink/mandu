# Especificação 49: Atualizar Joaju para 0.2.0-dev.139

- **ID do Ticket:** #49
- **Status:** Concluído (DD-TDD Fase 9)
- **Autora:** Lumen
- **Alinhamento:** GOST 3.0.0 (Rust)

## 1. Contexto & Motivação
A dependência `joaju` em `crates/mandu-core/Cargo.toml` estava configurada com `path = "../../../joaju"`. Isso fazia com que os runners de CI falhassem ao tentar carregar o manifesto de uma pasta inexistente no contêiner isolado do Forgejo (`/workspace/lumen/joaju/Cargo.toml`).

## 2. Decisão Arquitetural (Para Natasha)
Com o pacote `joaju` versão `0.2.0-dev.139` devidamente publicado no registry `gambi` (contendo `ReferenceSource` e `prepare_markdown`), remover a chave `path` e fixar a versão oficial no registry:
```toml
joaju = { version = "=0.2.0-dev.139", registry = "gambi", default-features = false }
```

## 3. Requisitos Técnicos
1. Atualizar `crates/mandu-core/Cargo.toml`.
2. Sincronizar `Cargo.lock` contra o registry `gambi`.
3. Validar qualidade (`cargo fmt --check`, `cargo clippy`, `cargo test`).
