# Especificação Técnica — Esteira de Release Multi-Arquitetura e Publicação Soberana na Banca (#5)

## 1. Contexto e Motivação
O Mandu distribui binários nativos para CLI (`mandu`), interface de terminal (`mandu-tui`) e interface gráfica GPU (`mandu-gui`).
Em conformidade com a expansão da vitrine de downloads e updater da Banca (`https://banca.2lp.in/downloads/mandu/`), a esteira de publicação do Mandu deve produzir artefatos para a matriz multi-arquitetura:
- `linux-amd64` (`x86_64-unknown-linux-musl`)
- `linux-arm64` (`aarch64-unknown-linux-musl`)
- `windows-amd64` (`x86_64-pc-windows-gnu`)

A esteira compila hermeticamente no contêiner `gost-ci-cross`, envia os artefatos para o bucket Cloudflare R2 (`twolp-bucket` / `bucket.2lp.in`), atualiza o manifesto `site/downloads/mandu/index.json` via commit na Banca e purga os objetos podados do R2.

---

## 2. Contrato de Compilação & Empacotamento

### 2.1. Ambiente de Execução
- **Contêiner:** `docker.io/lumenpink/gost-ci-cross:latest`
- **Alvos Rust:**
  - `x86_64-unknown-linux-musl`
  - `aarch64-unknown-linux-musl`
  - `x86_64-pc-windows-gnu`

### 2.2. Comandos de Compilação
```bash
# Linux x86_64 Estático
cargo zigbuild --release --target x86_64-unknown-linux-musl -p mandu-cli -p mandu-tui -p mandu-gui || \
cargo build --release --target x86_64-unknown-linux-musl -p mandu-cli -p mandu-tui -p mandu-gui

# Linux ARM64 Estático
cargo zigbuild --release --target aarch64-unknown-linux-musl -p mandu-cli -p mandu-tui -p mandu-gui || \
cargo build --release --target aarch64-unknown-linux-musl -p mandu-cli -p mandu-tui -p mandu-gui

# Windows x86_64 MinGW
cargo zigbuild --release --target x86_64-pc-windows-gnu -p mandu-cli -p mandu-tui -p mandu-gui || \
cargo build --release --target x86_64-pc-windows-gnu -p mandu-cli -p mandu-tui -p mandu-gui
```

### 2.3. Estrutura dos Pacotes
Para cada versão `${VERSION}`:
- `dist/packages/mandu-${VERSION}-linux-amd64.tar.gz`: contém `mandu`, `mandu-tui`, `mandu-gui`, `README.md`, `LICENSE`.
- `dist/packages/mandu-${VERSION}-linux-arm64.tar.gz`: contém `mandu`, `mandu-tui`, `mandu-gui`, `README.md`, `LICENSE`.
- `dist/packages/mandu-${VERSION}-windows-amd64.zip`: contém `mandu.exe`, `mandu-tui.exe`, `mandu-gui.exe`, `README.md`, `LICENSE`.
- `dist/packages/SHA256SUMS`: lista canônica de hashes SHA-256 de todos os pacotes.

---

## 3. Publicação no R2 e Commit na Banca

### 3.1. Upload no Cloudflare R2
1. Envio dos 3 pacotes versionados:
   `twolp-bucket/downloads/mandu/${CHANNEL}/mandu-${VERSION}-${ARCH}.(tar.gz|zip)`
2. Envio de links/aliases para o canal mais recente (`${CHANNEL}-latest`):
   - `twolp-bucket/downloads/mandu/${CHANNEL}-latest/mandu-${ARCH}.(tar.gz|zip)`
3. Envio do `SHA256SUMS` para o canal.

### 3.2. Rotação & Poda Soberana (Banca)
A esteira clona o repositório `banca` e invoca `scripts/rotate-downloads.py` para as 3 arquiteturas:
```bash
for ARCH in "linux-amd64" "linux-arm64" "windows-amd64"; do
  EXT="tar.gz"
  [ "$ARCH" = "windows-amd64" ] && EXT="zip"
  python3 scripts/rotate-downloads.py add \
    --manifest site/downloads/mandu/index.json \
    --channel "${CHANNEL}" \
    --arch "${ARCH}" \
    --version "${VERSION}" \
    --url "https://bucket.2lp.in/downloads/mandu/${CHANNEL}/mandu-${VERSION}-${ARCH}.${EXT}" \
    --sha256 "${HASH}" \
    --tarball "mandu-${VERSION}-${ARCH}.${EXT}" \
    --app-name "Mandu"
done
```

### 3.3. Expulso do 6º/11º Elemento no R2
Caso o manifesto tenha sido podado, os arquivos correspondentes das versões eliminadas são deletados do bucket R2 via `wrangler r2 object delete`.

### 3.4. Commit e Trigger de Deploy da Banca
A esteira realiza o commit em `site/downloads/mandu/index.json` no repositório `banca` e faz o push, acionando o deploy imediato da vitrine e API da Banca na borda.
