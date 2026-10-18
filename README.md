# 🧠 mandu

> **Mandu** (do Guarani *mandu'a*: memória, lembrança, recordação) é um ambiente de escrita acadêmica, gestão de conhecimento e publicação científica em **100% Rust**, projetado sob a filosofia de fluxo honesto (*HedgeDoc-style*), zero dependências web e pipeline único de renderização via **Joaju**.

---

## 📋 Conformidade GOST

Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (Gosplan Standard for Technical Software).
- **Versão GOST:** `1.1.0`
- **Perfil:** `profiles/lumen.md`
- **Linguagens declaradas:** `rust`
- **Licença:** `Apache-2.0`
- **Documentação e Especificação Técnica:** Consulte [SPEC.md](./SPEC.md)

---

## 🏗 Arquitetura do Workspace

```text
mandu/
├── Cargo.toml                  # Manifesto do Workspace
├── SPEC.md                     # Especificação Arquitetural
└── crates/
    ├── mandu-core/             # Cérebro do cofre (Zero dependências de UI)
    ├── mandu-tui/              # Interface de Terminal (Debian st/Kitty + Android Termux)
    └── mandu-gui/              # Interface Gráfica GPU (Debian Desktop + Android APK)
```

---

## 🚀 Como Iniciar

```bash
# Verificar conformidade com as normas GOST
cccp check

# Executar testes unitários e de integração
cargo test

# Executar a interface TUI
cargo run -p mandu-tui

# Executar a interface GUI nativa GPU
cargo run -p mandu-gui
```

---

## 📄 Licença

Este projeto é software livre licenciado sob a [GNU General Public License v3.0](LICENSE) (GPL-3.0-or-later).

| Componente | Licença | Arquivo |
| :--- | :--- | :--- |
| **Código-fonte e binários** | GPL-3.0-or-later | [LICENSE](LICENSE) |
| **Logotipos e marcas** | CC-BY-ND-4.0 | [LICENSE-LOGO](LICENSE-LOGO) |
| **Mídias e documentação** | CC-BY-SA-4.0 | [LICENSE-MEDIA](LICENSE-MEDIA) |
