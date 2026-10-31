# 🧠 mandu

> **Mandu** (do Guarani *mandu'a*: memória, lembrança, recordação) é um caderno pessoal seguro e bloco de notas para organizar pensamentos, estudos e anotações com privacidade total e publicação elegante integrada ao motor **Joaju**.

## 📋 Conformidade GOST
Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (versão `1.5.0`, perfil `profiles/lumen.md`).
Documentação e especificação técnica: consulte [SPEC.md](./SPEC.md).

## 🏗 Arquitetura do Workspace
- `crates/mandu-core`: Núcleo de domínio, cofre, indexador SQLite (FTS5) e resolvedor de notas.
- `crates/mandu-tui`: Interface TUI nativa de terminal.
- `crates/mandu-gui`: Interface gráfica nativa acelerada por GPU.
- `crates/mandu-cli`: Utilitários CLI e processamento.
- `crates/mandu-server`: Servidor HTTP e API de sincronização.

## 🚀 Como Iniciar
```bash
# Executar conformidade e testes
cccp check --strict
cargo test --all-targets

# Executar a interface TUI ou GUI
cargo run -p mandu-tui
cargo run -p mandu-gui
```

## 📄 Licença
Software livre sob a [GNU General Public License v3.0](LICENSE) (GPL-3.0-or-later).
- **Código-fonte:** GPL-3.0-or-later ([LICENSE](LICENSE))
- **Logotipos:** CC-BY-ND-4.0 ([LICENSE-LOGO](LICENSE-LOGO))
- **Mídias e documentação:** CC-BY-SA-4.0 ([LICENSE-MEDIA](LICENSE-MEDIA))
