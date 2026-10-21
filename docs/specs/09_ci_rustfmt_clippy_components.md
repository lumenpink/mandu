# Especificação Técnica: Instalação Garantida de Componentes Rustfmt e Clippy na CI

## 1. Contexto e Diagnóstico
A esteira unificada `.forgejo/workflows/ci.yml` falhou no job `test` (execução 60) porque o executor do runner instala o toolchain Rust com perfil mínimo (`--profile minimal`), omitindo os utilitários `cargo-fmt` e `clippy`. Consequentemente, o passo subsequente `cargo fmt -- --check` abortou a esteira.

## 2. Requisitos Técnicos
1. **Instalação Explicita de Componentes:**
   O passo de setup da toolchain Rust no job `test` deve invocar explicitamente:
   ```bash
   rustup component add rustfmt clippy 2>/dev/null || true
   ```
2. **Robustez de Toolchain:**
   Remover `--profile minimal` ou assegurar que `rustfmt` e `clippy` sejam adicionados independentemente do perfil de instalação prévio.
3. **Validação de Conformidade:**
   O script de teste `tests/test_ci_conformance.sh` deve conter asserção estrita de que `rustup component add rustfmt clippy` está presente no `.forgejo/workflows/ci.yml`.
