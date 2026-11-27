# Spec #52 — Atualizar perfil Lilya para 2.2.0

- **Ticket:** `#52`
- **Escopo:** `.gost/GOST.md`, `.gost/rules.toml`, `.gost/metadata.json`, `.cccprc`, `README.md`, `LICENSE-LOGO`, `.forgejo/workflows/ci.yml`, `docs/specs/`

## Árvore de Decisões
- **Escolha adotada:** migração linear `2.0.0 -> 2.1.0 -> 2.2.0` via `cccp profile upgrade`, alinhamento de `.gost/metadata.json`, `.cccprc`, `LICENSE-LOGO`, `cargo fmt --check` no CI e revisão terminológica (`KOLLONTAI-011`).
- **Alternativa descartada:** manter perfil legado `profile.md` sem conformidade com `cccp check --strict`.
