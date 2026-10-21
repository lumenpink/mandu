# Especificação Técnica — OpenSSL Vendored para Multiarch MUSL (#9)

## 1. Contexto e Motivação
A publicação soberana de artefatos multi-arquitetura na Banca (`https://banca.2lp.in/downloads/mandu/`) empacota binários estáticos para `x86_64-unknown-linux-musl` e `aarch64-unknown-linux-musl`.
O ecossistema utiliza `joaju` e `reqwest`, os quais transitivamente dependem do crate `openssl-sys`. Em alvos estáticos baseados em musl, o OpenSSL dinâmico não está disponível via pkg-config padrão, exigindo que o código-fonte C do OpenSSL seja compilado estaticamente via a feature `vendored` do crate `openssl`.

Adicionalmente, na esteira `.forgejo/workflows/ci.yml`, o job `deploy-banca` deve exigir formalmente a cláusula `success()` do upstream `test` para garantir que qualquer falha nos testes bloqueie imediatamente a publicação de pacotes.

## 2. Requisitos Técnicos
1. **Ativação do OpenSSL Vendored:**
   Em `crates/mandu-core/Cargo.toml`, declarar a dependência `openssl = { version = "0.10", features = ["vendored"] }`.
2. **Garantia de Dependência no CI:**
   Em `.forgejo/workflows/ci.yml`, atualizar o predicado `if:` de `deploy-banca` para:
   `if: success() && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/tags/v')) && (github.event_name == 'push' || github.event_name == 'workflow_dispatch')`

## 3. Critérios de Aceite
1. Verificação automatizada via script de teste `tests/test_openssl_vendored.sh`.
2. Teste de mutação confirmando sensibilidade caso `vendored` seja removido.
3. Validação do predicado `success()` em `.forgejo/workflows/ci.yml`.
4. Conformidade estrita GOST via `cccp check --strict`.

## 4. Evidências de Validação
- Compilação dos alvos `x86_64-unknown-linux-musl` (`mandu-cli`, `mandu-tui`, `mandu-gui`) executada com sucesso total dentro do contêiner hermético `gost-ci-cross:latest`.
- Teste de mutação e verificação em `tests/test_openssl_vendored.sh` 100% aprovados.

