# Especificação Técnica - Ticket #21: Publicação na Banca e Auto-Atualização Atômica

## 1. Contexto e Motivação

O Mandu adota a infraestrutura soberana da **Banca** (`https://banca.2lp.in/downloads/mandu/`) para compilação remota e entrega contínua de executáveis e pacotes tarball, eliminando o acoplamento com o armazenamento de releases do Codeberg.

Adicionalmente, os usuários precisam de um mecanismo nativo, seguro e atômico para verificar atualizações e efetuar upgrade de todos os binários da ferramenta (`mandu`, `mandu-tui` e `mandu-gui`), com suporte a canais de distribuição (`stable` e `nightly`), versões travadas (*pin*) e persistência em arquivo de configuração local ou global.

---

## 2. Configuração do Atualizador (`mandu-core::config`)

### 2.1. Estrutura de Metadados no `config.toml`

A configuração reside hierarquicamente em:
1. `.mandu/config.toml` (configuração específica do cofre/projeto ativo)
2. `~/.config/mandu/config.toml` (ou `$XDG_CONFIG_HOME/mandu/config.toml`, configuração global do usuário)

~~~toml
[update]
channel = "stable"       # "stable" (padrão) ou "nightly"
pin = "0.1.0"            # opcional: fixa a versão impedindo updates automáticos
auto_check = true        # verificação não-bloqueante periódica
~~~

### 2.2. Precedência de Resolução de Canal

1. Flag explicita na linha de comando (`--channel stable` ou `--channel nightly`).
2. Variável de ambiente `MANDU_UPDATE_CHANNEL`.
3. Arquivo de configuração local `.mandu/config.toml`.
4. Arquivo de configuração global `~/.config/mandu/config.toml`.
5. Fallback padrão: canal `stable`.

---

## 3. Motor de Atualização (`mandu-core::updater`)

### 3.1. Endpoints Oficiais na Banca

- **Base de Downloads:** `https://banca.2lp.in/downloads/mandu/`
- **Consulta de Versão Stable:** `https://banca.2lp.in/downloads/mandu/version`
- **Consulta de Versão Nightly:** `https://banca.2lp.in/downloads/mandu/nightly-version`
- **Binários Stable:** `https://banca.2lp.in/downloads/mandu/stable-latest/{bin_name}`
- **Binários Nightly:** `https://banca.2lp.in/downloads/mandu/nightly-latest/{bin_name}`

Onde `{bin_name}` corresponde a `mandu`, `mandu-tui` ou `mandu-gui` (ou sufixado com a arquitetura `linux-amd64` / `linux-arm64`).

### 3.2. Comparação SemVer e Travamento (*Pin*)

- Se a configuração contiver `pin = "<versão>"`, a verificação aborta preventivamente, informando que o binário está fixado na versão indicada.
- A comparação SemVer determina com rigor se a versão remota é superior à instalada (`remote_version > current_version`).

### 3.3. Atualização Atômica com Rollback Seguro

1. Identifica quais executáveis estão instalados no mesmo diretório do executável em execução (`mandu`, `mandu-tui`, `mandu-gui`).
2. Realiza o download do novo binário em arquivo temporário (`.{bin_name}.tmp`).
3. Define permissões de execução Unix (`0o755`).
4. Cria cópia de segurança (`{bin_name}.old`).
5. Substitui o executável original através de renomeação atômica (`fs::rename`).
6. Se qualquer etapa falhar, restaura automaticamente os binários a partir do backup (`.old`).

---

## 4. Interface de Linha de Comando (`mandu-cli`)

O despachante `mandu` incorpora os subcomandos de manutenção:
- `mandu check-update [--channel stable|nightly]`: Inspeciona remotamente a versão mais recente na Banca e informa se há atualização pendente.
- `mandu update [--channel stable|nightly] [--pin <ver>] [--unpin]`: Baixa e substitui atomicamente os binários locais instalados.

---

## 5. Pipeline de CI/CD para a Banca (`.forgejo/workflows/release-banca.yaml`)

O workflow é disparado em tags `v*` (estável) e em pushes na branch `main` (nightly):
- Compila os 3 executáveis (`mandu`, `mandu-tui`, `mandu-gui`).
- Gera checksums `SHA256SUMS`.
- Publica no diretório da Banca `/srv/banca/downloads/mandu/` com rota para `stable/`, `nightly/` e manifestos de versão.
- Aplica retenção de 5 versões estáveis e 10 nightlies.
