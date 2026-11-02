# Spec #16 — Simplificar Barra de Status e Exibir Modal de Ajuda na TUI

## 1. Contexto e Motivação
Na interface de terminal (`mandu-tui`), a barra de status continha botões visuais e rótulos desconectados de atalhos correspondentes (ex: `[ 📁 Buscar/Abrir ]` e `[ 📄 Gerar PDF ]`). O objetivo é simplificar a barra de status, exibindo primariamente os dados essenciais da nota ativa, o modo atual e o atalho de ajuda (`^H Ajuda`), fornecendo uma janela modal suspensa de ajuda que documente todos os atalhos disponíveis sem teclas de função (F1-F12).

## 2. Requisitos e Critérios de Aceite
1. Barra de status simplificada: remover botões avulsos sem atalhos e incluir a indicação clara do atalho de ajuda (`^H Ajuda` ou `Ctrl+H Ajuda`).
2. Acionamento do Modal de Ajuda: `Ctrl+H` alterna o estado de `is_help_open` da aplicação.
3. Conteúdo do Modal de Ajuda: exibir tabela/lista com todos os atalhos ativos:
   - `Ctrl+O`: Buscar / Abrir nota
   - `Ctrl+S`: Salvar nota
   - `Ctrl+D`: Exportar PDF via Joaju
   - `Ctrl+T`: Alternar visualização (Escrita / Split / Leitura)
   - `Ctrl+P`: Espiar (Peek) link ou citação sob o cursor
   - `Ctrl+G`: Navegar para link sob o cursor
   - `Ctrl+E`: Editor externo (Escape Hatch)
   - `Ctrl+H`: Abrir / Fechar esta ajuda
   - `Ctrl+Q`: Sair do Mandu
4. Fechamento do Modal: pressionar `Esc`, `q` ou repetir `Ctrl+H` fecha o modal de ajuda.
5. Conformidade estrita: proibido o uso de teclas de função (F1 a F12).

## 3. Ciclo GOST DD-TDD
- Fase 1: Ticket #16 delimitado.
- Fase 2: Branch `feat/16-tui-help-status-bar`.
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho unitário/integração para toggle e atalhos de ajuda.
- Fase 5: Implementação verde no `mandu-tui`.
- Fase 6: Refatoração e alinhamento visual.
- Fase 7: Testes de mutação sobre teclas de saída e visibilidade.
- Fase 8: Quality Gate (`cargo fmt`, `clippy`, testes).
- Fase 9: Fechamento com laudo via `cccp close`.
