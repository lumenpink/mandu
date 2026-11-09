# Spec Ticket #44: Sincronizar Versão Estática de Joaju no build.rs

## 1. Contexto e Motivação
A esteira de CI do `mandu` roda de forma hermética dentro de um contêiner Docker (`gost-ci-cross`), onde o diretório local do repositório `joaju` não está disponível. Por projeto, o `crates/mandu-core/build.rs` baixa a biblioteca pré-compilada `libjoaju-x86_64-linux.a` do registro de pacotes genéricos do Forgejo (`https://gambi.2lp.in/api/packages/lumen/generic/joaju-static/<VERSION>/...`).

Com a padronização e exportação de novos motores C-ABI no `joaju` (Ticket #84) e a inclusão de seus cartões e wrappers no `mandu` (Ticket #43), a versão publicada pelo `joaju` no Forgejo avançou para `0.2.0-dev.137+20261109.df2603d`. Como `build.rs` ainda requeria a versão anterior `0.2.0-dev.137+20261106.6eda140`, a compilação na esteira falhava durante a linkedição (`rust-lld: error: undefined symbol: joaju_render_*`).

## 2. Escopo da Solução
- Atualizar a constante `JOAJU_STATIC_REQUIRED_VERSION` em `crates/mandu-core/build.rs` para `"0.2.0-dev.137+20261109.df2603d"`.
- Atualizar o teste de contrato e asserções de mutação em `crates/mandu-core/tests/joaju_staticlib_ffi_spec.rs` para garantir fidelidade ao SemVer e hash canônicos.

## 3. Critérios de Aceitação
- `cargo fmt -- --check`: Formatação sem desvios.
- `cargo clippy --all-targets -- -D warnings`: Zero avisos.
- `cargo test --package mandu-core --test joaju_staticlib_ffi_spec`: 100% dos testes aprovados.
- `cccp check --strict`: 100% de conformidade com a norma GOST.
