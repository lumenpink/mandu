# Especificação Técnica: Autosave Transparente com Debounce de Inatividade e Save-on-Switch

## 1. Contexto e Motivação
Em ferramentas de notas em rede como o Obsidian, a escrita acadêmica e a exploração do grafo de conhecimento não devem exigir esforço mental contínuo para persistir alterações (`Ctrl+S`).
Ao navegar continuamente entre notas (através de links internos `[[nota]]`, citações `@citekey` ou busca rápida `Ctrl+O`), a ausência de persistência automática gera atrito ou perda acidental de ideias.

Este ticket formaliza o mecanismo de **Autosave Transparente**:
1. Debounce por inatividade temporal (~1.000 ms).
2. Salvamento compulsório imediato em qualquer troca de contexto ou encerramento (*Save-on-Switch*).
3. Feedback discreto na barra de status de 1 linha.

---

## 2. Requisitos Arquiteturais e Funcionais

### 2.1. Debounce por Inatividade
- Toda tecla ou edição que altere o buffer marca `is_dirty = true` e atualiza `last_edit_time = Some(Instant::now())`.
- O event loop do terminal (`mandu-tui`) utiliza `crossterm::event::poll(Duration::from_millis(100))` para garantir ticks frequentes mesmo quando não há eventos de teclado.
- Em cada ciclo do loop, se `is_dirty` for verdadeiro e `last_edit_time.elapsed() >= autosave_delay` (1.000 ms):
  - Aciona `app.save_current_buffer()`.
  - Reseta `is_dirty = false`.
  - Ativa o feedback visual `autosave_feedback_until = Some(Instant::now() + Duration::from_secs(2))`.

### 2.2. Salvamento Imediato em Troca de Contexto (*Save-on-Switch*)
Para eliminar completamente janelas de inconsistência:
1. **Busca Rápida (`Ctrl+O`):** Ao confirmar uma nova nota na paleta modal fuzzy (`open_selected_search_result`), se `is_dirty` for verdadeiro, o buffer atual é salvo antes de ler e carregar o novo arquivo.
2. **Navegação Direta de Links (`Ctrl+G`):** Ao saltar para o link ou citação sob o cursor, o buffer atual é salvo no disco antes da transição.
3. **Abertura a partir do Peek (`Enter` na modal):** Salva o buffer atual antes de mudar para a nota inspecionada.
4. **Encerramento (`Ctrl+Q`):** Se `is_dirty` for verdadeiro, executa o salvamento silencioso e limpo antes de encerrar o loop e desabilitar o modo raw do terminal.

### 2.3. Feedback Visual na Barra de Status
- Durante digitação (`is_dirty == true`):
  `[● Modificado]` (ou `[●]`).
- Imediatamente após o salvamento automático (por 2 segundos):
  `[✓ Salvo]`.
- Estado neutro:
  Sem aviso de pendência.

---

## 3. Paridade TUI (`mandu-tui`) e GUI (`mandu-gui`)
- Ambos os executáveis compartilham rigorosamente as regras de debounce (1.000 ms) e save-on-switch.
- O pipeline de salvamento aciona `Linter::apply_save_hooks`, garantindo normalização POSIX, injeção de `modified: <timestamp>` e reindexação in-memory no SQLite.
