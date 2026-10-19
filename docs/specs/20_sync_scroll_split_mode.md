# Especificação Técnica - Ticket #20: Scroll Sincronizado entre Editor e Preview no Modo Split

## 1. Contexto e Motivação

O Mandu adota a arquitetura de visualização tripartida inspirada no padrão *HedgeDoc*:
1. Modo Escrita (Source) — 100% da tela para edição pura.
2. Modo Split (HedgeDoc) — Painel dividido: Editor à esquerda e Preview à direita.
3. Modo Leitura (Preview) — 100% da tela para visualização rica diagramada.

No Modo Split, a experiência do usuário requer que a rolagem do editor de texto e a visualização do painel renderizado acompanhem proporcionalmente a posição do cursor e da leitura. Como documentos Markdown contêm cabeçalhos que quebram linhas, metadados colapsados em badges no topo e blocos estruturados Joaju/Typst (tabelas, árvores genealógicas, diagramas de blocos) que ocupam mais linhas no preview do que no código-fonte, a contagem de linhas entre o código e a prévia raramente é 1:1.

Portanto, um algoritmo de rolagem sincronizada proporcional baseado na posição vertical relativa do documento garante navegação fluida, sem sobressaltos e sem oscilações bruscas.

---

## 2. Especificação Arquitetural e Algorítmica

### 2.1. Cálculo da Proporção Vertical Relativa ($\text{ratio}$)
A fração de progresso vertical no editor é calculada normalizada no intervalo $[0.0, 1.0]$:

$$\text{ratio} = \frac{\text{cursor\_row}}{\max(1, \text{total\_linhas\_editor} - 1)}$$

Onde:
- $\text{cursor\_row}$: Linha atual do cursor no editor (0-indexed).
- $\text{total\_linhas\_editor}$: Total de linhas no buffer do editor.

### 2.2. Cálculo do Offset de Rolagem do Preview na TUI (`mandu-tui`)
O painel de prévia no terminal renderiza as linhas estilizadas (`preview_lines`). A área disponível do terminal possui altura $\text{viewport\_height} = \text{height} - 2$ (descontando bordas superior e inferior do bloco Ratatui).

O deslocamento vertical do preview é calculado como:

$$\text{max\_scroll} = \max(0, \text{preview\_lines.len}() - \text{viewport\_height})$$
$$\text{preview\_scroll} = (\text{ratio} \times \text{max\_scroll}).\text{round}() \text{ as } u16$$

O `Paragraph` do Ratatui recebe `.scroll((preview_scroll, 0))`.

### 2.3. Navegação Exclusiva no Modo Preview na TUI
Quando o usuário entra no modo Leitura (`ViewMode::Preview`):
- O editor está oculto. As teclas <kbd>Up</kbd>, <kbd>Down</kbd>, <kbd>PageUp</kbd>, <kbd>PageDown</kbd>, <kbd>Home</kbd>, <kbd>End</kbd> e eventos de roda do mouse (`MouseEventKind::ScrollUp`, `MouseEventKind::ScrollDown`) controlam diretamente o offset vertical do preview.
- O offset é delimitado por `clamp(0, max_scroll)`.
- Ao alternar entre Split e Leitura com <kbd>Ctrl</kbd>+<kbd>T</kbd>, a posição percentual é retida para manter a continuidade cognitiva da leitura.

### 2.4. Sincronização de Rolagem na GUI (`mandu-gui`)
Na interface gráfica (`egui`), o editor e o preview são hospedados em áreas roláveis `egui::ScrollArea::vertical()`:
- `ViewsState` armazena `scroll_ratio: f32` (de `0.0` a `1.0`).
- No modo Split, ao rolar o editor, calcula-se o percentual do editor a partir do deslocamento vertical e da altura do conteúdo.
- O `ScrollArea` do preview aplica `vertical_scroll_offset(ratio * max_preview_offset)`, garantindo sincronia suave.

---

## 3. Critérios de Aceitação e Casos de Borda

1. **Notas curtas (< altura do painel):** Quando o conteúdo cabe integralmente na tela, $\text{max\_scroll} = 0$, mantendo o scroll fixo em `0` sem divisão por zero.
2. **Notas longas (> 500 linhas):** Performance estável $O(1)$ por frame nos cálculos de scroll, sem alocações repetitivas.
3. **Cursor no início e no fim:**
   - $\text{cursor\_row} = 0 \implies \text{ratio} = 0.0 \implies \text{preview\_scroll} = 0$.
   - $\text{cursor\_row} = \text{fim} \implies \text{ratio} = 1.0 \implies \text{preview\_scroll} = \text{max\_scroll}$.
4. **Isolamento de Erros e Jitter:** Zero trepidações entre frames sucessivos quando o cursor não se mover.
