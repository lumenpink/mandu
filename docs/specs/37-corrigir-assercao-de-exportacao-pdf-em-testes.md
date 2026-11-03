# Especificação Técnica — Ticket #37: Corrigir Asserção de Exportação PDF em Testes TUI

## Contexto e Motivação
Na suíte de testes de integração do TUI (`crates/mandu-tui/tests/tui_layout_test.rs`), o teste `test_tui_ctrl_shortcuts_and_no_fkeys` valida o acionamento de atalhos de teclado.
Durante o teste:
1. `Ctrl+S` é acionado em uma nota sem nome prévio, o que resulta na gravação de `nova_nota.md` e na atualização de `self.current_file = Some("nova_nota.md")`.
2. Em seguida, `Ctrl+D` é acionado para disparar a exportação de PDF (`app.export_pdf()`). Como `self.current_file` está definido como `nova_nota.md`, a exportação bem-sucedida gera `nova_nota.pdf` e define a mensagem de status como `Documento exportado via Joaju: nova_nota.pdf`.
3. A asserção antiga do teste verificava estritamente `Some("Documento exportado via Joaju")`, causando falha de asserção (`assertion left == right failed`).

## Requisitos
1. Ajustar a asserção no teste `test_tui_ctrl_shortcuts_and_no_fkeys` em `crates/mandu-tui/tests/tui_layout_test.rs` para verificar `Some("Documento exportado via Joaju: nova_nota.pdf")` ou validar o prefixo correspondente ao arquivo exportado.
2. Garantir que arquivos temporários de teste (`nova_nota.md`, `nova_nota.pdf`) criados durante a execução do teste sejam adequadamente isolados ou limpos caso necessário.
3. Assegurar que 100% dos testes da suíte `tui_layout_test` e demais testes passem sem falhas.

## Plano de Fases (GOST DD-TDD)
- Fase 1: Ticket #37 criado.
- Fase 2: Branch `fix/37-corrigir-assercao-de-exportacao-pdf-em-testes`.
- Fase 3: Especificação técnica documentada.
- Fase 4: Red test confirmado (falha em `tui_layout_test.rs:114:5`).
- Fase 5: Implementação verde (Green test).
- Fase 6: Refatoração e limpeza.
- Fase 7: Teste mutacional.
- Fase 8: Quality Gate (fmt, clippy, test).
- Fase 9: Conclusão, merge e push.
