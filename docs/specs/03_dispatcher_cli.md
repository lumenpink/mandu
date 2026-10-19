# Especificação Técnica - Ticket #3: Despachante Unificado `mandu`

## 1. Contexto e Motivação

Atualmente, o Mandu disponibiliza duas interfaces distintas:
- `mandu-tui`: interface para terminal baseada em Ratatui/Crossterm.
- `mandu-gui`: interface gráfica nativa para desktop baseada em egui/eframe.

Para uma experiência de usuário sem fricção em ambientes mistos (Debian desktop com X11/Wayland, terminais puros TTY, sessões SSH e Android Termux), é necessário um ponto de entrada único: o comando **`mandu`**. O despachante unificado deve inspecionar o contexto de execução, detectar a presença de servidores gráficos e despachar a execução para o binário apropriado, repassando todos os parâmetros da linha de comando de forma transparente.

---

## 2. Arquitetura do Despachante (`crates/mandu-cli`)

O despachante é empacotado na crate `crates/mandu-cli`, gerando o binário executável `mandu`.

### 2.1. Heurística de Resolução do Alvo de Execução

1. **Flags de Override Explícito:**
   - `--tui`: força a execução de `mandu-tui`. A flag `--tui` é removida da lista de argumentos repassados.
   - `--gui`: força a execução de `mandu-gui`. A flag `--gui` é removida da lista de argumentos repassados.
   - `--help` / `-h` e `--version` / `-V`: se passadas isoladamente, o despachante exibe instruções de uso do despachante `mandu` ou repassa para o binário de destino.

2. **Detecção Automática de Ambiente Gráfico:**
   - Se nenhuma flag de override for especificada:
     - O despachante verifica se `WAYLAND_DISPLAY` ou `DISPLAY` estão definidas e não-vazias no ambiente.
     - Se ambas estiverem ausentes: o alvo padrão é **`mandu-tui`**.
     - Se qualquer uma estiver presente: o alvo padrão é preferencialmente **`mandu-gui`**.

3. **Descoberta do Binário e Fallback Gracioso:**
   - A busca pelo binário (`mandu-tui` ou `mandu-gui`) ocorre em duas etapas:
     1. Diretório adjacente ao próprio executável `mandu` (`current_exe().parent()`), permitindo execução imediata em compilações locais (`target/debug/` ou `target/release/`) e instalações em `~/.local/bin/`.
     2. Diretórios listados na variável de ambiente `PATH`.
   - **Fallback GUI $\rightarrow$ TUI:** Se o alvo detectado for `mandu-gui`, mas o binário `mandu-gui` não for encontrado, o despachante tenta localizar `mandu-tui`. Se encontrado, executa `mandu-tui` graciosamente.
   - Se o binário final não for localizado:
     - Exibe mensagem em `stderr`: `Erro: Executável '<binario>' não encontrado no PATH ou no mesmo diretório.`
     - Retorna código de saída `127` (padrão POSIX para comando não encontrado).

### 2.2. Execução Transparente e Substituição de Imagem (`exec`)

- No Linux/Unix, o despachante utiliza `std::os::unix::process::CommandExt::exec()` para substituir a imagem do processo `mandu` pelo binário filho.
- Isso assegura:
  - Zero sobrecarga de memória (sem processo pai dormente).
  - Herança imediata e desobstruída dos descritores de arquivo padrão (`stdin`, `stdout`, `stderr`).
  - Repasse automático e nativo de sinais do sistema operacional (`SIGINT`, `SIGTERM`, `SIGWINCH`).
- Em caso de falha no `exec()` (ou em plataformas não-Unix), utiliza `std::process::Command` aguardando a finalização com repasse estrito do `exit_code`.

---

## 3. Critérios de Aceitação e Casos de Borda

1. `mandu --tui [argumentos]` executa `mandu-tui` com os argumentos (sem `--tui`).
2. `mandu --gui [argumentos]` executa `mandu-gui` com os argumentos (sem `--gui`).
3. Em ambiente sem `DISPLAY` e sem `WAYLAND_DISPLAY`, `mandu` despacha para `mandu-tui`.
4. Em ambiente com `DISPLAY` ou `WAYLAND_DISPLAY`, `mandu` despacha para `mandu-gui`.
5. Se `mandu-gui` estiver ausente do sistema, ocorre fallback automático para `mandu-tui`.
6. Código de saída do processo filho é preservado integralmente.
7. Binário ausente resulta em mensagem de erro clara e exit code `127`.
