# Especificação Técnica - Ticket #4: Aplicativo Mobile com Dioxus 0.7 e Integração Pratilera

## 1. Contexto e Motivação

O Mandu é um ecossistema de escrita e visualização hiperlocal de notas acadêmicas e zettelkasten, integrando blocos ricos do Joaju e compilação tipográfica Typst. Após a consolidação do despachante unificado (`mandu-cli`), da interface de terminal (`mandu-tui`), da interface gráfica desktop (`mandu-gui`) e da infraestrutura de publicação na Banca (Ticket #21), torna-se mandatória a disponibilização do aplicativo móvel para o sistema operacional Android.

Para garantir digitação ergonômica em telas sensíveis ao toque, integração completa com teclados virtuais (Gboard, SwiftKey), autocorreção, suporte a acentuação em português e seleção touch nativa, o aplicativo móvel adota **Dioxus 0.7** (`crates/mandu-mobile`).

O gerenciamento de notas opera estritamente sobre **diretórios locais do dispositivo**, delegando a sincronização entre dispositivos a ferramentas autônomas descentralizadas já consagradas pelo usuário (como o Syncthing), mantendo o `mandu-core` como a única autoridade de parsing, indexação SQLite FTS5, gerenciamento de referências e escrita atômica.

A distribuição mobile é realizada soberanamente na **Pratilera** (catálogo F-Droid da Banca) sob o identificador canônico `in.twolp.mandu`.

---

## 2. Arquitetura do Crate `crates/mandu-mobile`

### 2.1. Estrutura de Arquivos e Manifesto Cargo

O crate é integrado ao workspace raiz do Mandu:

```
crates/mandu-mobile/
├── Cargo.toml
├── Dioxus.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── editor.rs
│   │   ├── toolbar.rs
│   │   ├── preview.rs
│   │   └── vault.rs
│   └── state.rs
└── assets/
    └── icon.png
```

### 2.2. Dependências e Features

```toml
[package]
name = "mandu-mobile"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
description = "Aplicativo Mobile Android (Dioxus 0.7) para o Mandu"

[dependencies]
mandu-core = { path = "../mandu-core" }
dioxus = { version = "0.7", default-features = false, features = ["macro", "html", "signals", "hooks", "launch", "document"] }
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
anyhow = "1.0"
chrono = "0.4"
```

### 2.3. Manifesto `Dioxus.toml`

```toml
[application]
name = "mandu-mobile"
default_platform = "mobile"
out_dir = "dist"
asset_dir = "assets"

[android]
app_title = "Mandu"
package = "in.twolp.mandu"
min_sdk = 26
target_sdk = 34
```

---

## 3. Experiência de Uso Mobile (UX / UI)

### 3.1. Modos de Visualização Mobile
Em dispositivos móveis de tela vertical (*portrait*):
- **Modo Editor (`ViewMode::Editor`):** Foco absoluto na área de digitação do `<textarea>` nativo com a barra de ferramentas sobre o teclado.
- **Modo Prévia (`ViewMode::Preview`):** Visualização rica em tela cheia do Markdown renderizado, fórmulas matemáticas Typst e diagramas vetoriais Joaju em SVG.
- **Alternância Fluida:** Botão de alternância e abas inferiores com resposta tátil instantânea.

### 3.2. Barra de Ações Rápidas (Toolbar sobre o Teclado)
Uma barra horizontal scrollável com botões táteis projetados para digitação ágil:
1. `[[ ]]` — Inserção de link wiki ou encapsulamento de seleção.
2. `#` — Cabeçalhos markdown sequenciais.
3. `- [ ]` — Caixa de marcação de lista de tarefas.
4. `**` — Negrito (*bold*).
5. `*` — Itálico (*italic*).
6. `$` — Fórmulas matemáticas em notação Typst/LaTeX.
7. ```` — Blocos cercados de código ou blocos especializados Joaju (`joaju table`, `pedigree`, `unfurl`).

### 3.3. Gestão Local do Cofre (Zero Cloud)
- O aplicativo armazena a preferência de caminho do cofre ativo (com fallback padrão seguro para pasta de notas local em `Documents/Mandu`).
- O `mandu-core::scanner::VaultScanner` carrega o índice em milissegundos sem travar a interface gráfica.
- As gravações acionam o pipeline de segurança de `mandu-core` com normalização de terminação de linha POSIX (`\n`).

---

## 4. Distribuição na Pratilera (`in.twolp.mandu`)

### 4.1. Metadados Fastlane
Estruturados em `fastlane/metadata/android/in.twolp.mandu/`:
- `pt-BR/title.txt`: `Mandu`
- `pt-BR/short_description.txt`: `Editor e Visualizador Hiperlocal de Notas com Joaju`
- `pt-BR/full_description.txt`: Detalhamento completo do Mandu, escrita honesta, zettelkasten, busca FTS5, blocos Joaju e soberania digital.
- `en-US/*`: Paridade de metadados em língua inglesa.
- `icon.png`: Ícone canônico 512x512 no formato RGBA 8-bit estrito.

### 4.2. Script de Empacotamento Soberano (`scripts/package-apk.sh`)
- Executa compilação do APK via `dx build --platform android --release` (ou fallback hermético).
- Assina o pacote gerado com `~/.android/in_twolp_release.keystore` e `apksigner`.
- Suporta a flag `--sync-pratilera [CAMINHO_BANCA]` para atualizar automaticamente a loja e disparar a regeneração do catálogo F-Droid via `gerar-pratilera.py`.

---

## 5. Critérios de Aceite e Conformidade GOST

1. Presença do crate `crates/mandu-mobile` configurado no workspace raiz com dependência em `mandu-core` e `dioxus 0.7`.
2. Presença do manifesto `Dioxus.toml` com pacote `in.twolp.mandu`.
3. Testes automatizados validando o estado do aplicativo, inserções da toolbar e integridade do cofre.
4. Existência dos metadados Fastlane (`pt-BR`, `en-US` e `icon.png`) em conformidade com as diretrizes da Pratilera.
5. Script de empacotamento `scripts/package-apk.sh` funcional e com validação de assinatura.
6. Zero warnings em `cargo clippy --all-targets -- -D warnings` e formatação validada via `cargo fmt --check`.
