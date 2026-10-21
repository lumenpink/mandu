# Especificação Técnica: Execução Hermética de Testes com Contêiner gost-ci-cross

## 1. Contexto e Motivação
A diretriz GOST impõe que as imagens de CI/CD sejam herméticas, atômicas e reproduzíveis. Esteiras não devem realizar instalações ad-hoc de ferramentas ou dependências de runtime (`curl | sh`, `rustup component add`) durante os passos de validação.

Com a publicação oficial da imagem `docker.io/lumenpink/gost-ci-cross:latest` contendo o Rust 1.98.1 completo acompanhado de `rustfmt` e `clippy`, o job `test` do arquivo `.forgejo/workflows/ci.yml` deve executar integralmente dentro desse contêiner.

## 2. Requisitos Técnicos
1. **Ambiente Contêiner Oficial:**
   O job `test` deve especificar expressamente:
   ```yaml
   container:
     image: docker.io/lumenpink/gost-ci-cross:latest
   ```
2. **Remoção de Instalações em Tempo de Execução:**
   Eliminar completamente o step `Setup Rust Toolchain & Linters` com comandos `curl ... rustup.rs` ou `rustup component add`.
3. **Consumo Direto de Linters e Testes:**
   Executar diretamente:
   - `cargo fmt -- --check`
   - `cargo clippy --all-targets -- -D warnings`
   - `cargo test --all-targets`
4. **Dependência Estrita Preservada:**
   O job `deploy-banca` mantém rigorosamente a dependência `needs: [test]`.
5. **Auditoria em Testes:**
   O script `tests/test_ci_conformance.sh` deve validar que:
   - O job `test` usa o contêiner `docker.io/lumenpink/gost-ci-cross:latest`.
   - Não há comandos `sh.rustup.rs` no `ci.yml`.
