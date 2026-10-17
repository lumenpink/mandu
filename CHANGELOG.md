# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Inicialização do repositório em conformidade com o padrão GOST (`profiles/lumen.md`).
- Especificação arquitetural canônica em `SPEC.md` detalhando a filosofia *Lumen-Style*, o padrão *HedgeDoc*, a integração com o *Joaju* e a migração de plugins do Obsidian.
- Workspace Cargo multi-crate com `mandu-core`, `mandu-tui` e `mandu-gui`.
- Núcleo `mandu-core` com schema SQLite in-memory (FTS5), hashing BLAKE3, linter POSIX, extrator de referências e stub do Joaju.
- Cascas iniciais `mandu-tui` (Ratatui/Crossterm) e `mandu-gui` (egui/eframe).
- Testes unitários e de integração de contrato com zero warnings e validação rigorosa de linters.
- Deploy de documentação configurado em `wrangler.jsonc` para `mandu.2lp.in` e registro no `catalog.json`.
