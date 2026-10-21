# Spec 23: Correção de Argumentos do rotate-downloads e Remote da Banca

## 1. Contexto & Motivação
Na esteira unificada de CI/CD do Mandu (`.forgejo/workflows/ci.yml`), a etapa de rotação de manifesto e registro de releases na Banca (`Rotate Manifest & Commit to Banca`) falhou com o erro:
```
rotate-downloads.py: error: unrecognized arguments: add --app-name Mandu
```
Além disso, a esteira ainda referenciava o repositório legado no Codeberg (`codeberg.org/lumenpink/banca.git`), violando a diretriz de soberania do ecossistema que determina o uso exclusivo de `git.2lp.in`.

Uma análise do utilitário `scripts/rotate-downloads.py` no repositório da Banca revelou que a CLI não implementa subcomandos posicionais (como `add`), utiliza `--version-str` em vez de `--version`, e `--app` em vez de `--app-name`.

## 2. Requisitos Técnicos
1. Em `.forgejo/workflows/ci.yml`, no job `deploy-banca` -> etapa `Rotate Manifest & Commit to Banca`:
   - Configurar a variável de ambiente `GIT_2LP_TOKEN: ${{ secrets.GIT_2LP_TOKEN }}`.
   - Clonar o repositório da Banca autenticado via `git.2lp.in`:
     `git clone --depth 1 "https://lumen:${GIT_2LP_TOKEN}@git.2lp.in/lumen/banca.git" /tmp/banca`
   - Chamar `python3 scripts/rotate-downloads.py` passando:
     - `--manifest site/downloads/mandu/index.json`
     - `--channel "${CHANNEL}"`
     - `--arch "${ARCH}"`
     - `--version-str "${VERSION}"`
     - `--url "${DOWNLOAD_URL}"`
     - `--sha256 "${HASH}"`
     - `--tarball "${PKG_FILE}"`
     - `--app "mandu"`
   - Manter a configuração de autor de commit para o bot (`Mandu Release Bot <bot@2lp.in>`) e realizar o push direto autenticado para `origin main`.
2. Remover qualquer referência residual a `codeberg.org` ou `CODEBERG_TOKEN` no workflow.

## 3. Critérios de Aceite
- Execução local do parser de argumentos do `rotate-downloads.py` com a sintaxe corrigida é bem-sucedida.
- Zero ocorrências de `codeberg.org` no repositório do Mandu.
- `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` sem violações.
- `cccp check --strict` aprovado com 100% de conformidade.
