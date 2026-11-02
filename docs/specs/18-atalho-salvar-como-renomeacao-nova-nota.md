# Spec #18 — Atalho Salvar Como com Renomeação de nova_nota

## 1. Contexto e Motivação
Quando o editor inicializa em `nova_nota.md`, o usuário inicia um rascunho. Ao decidir o título definitivo, a ação "Salvar Como" deve mover/renomear o arquivo `nova_nota.md` para o novo caminho no cofre, evitando duplicatas ou arquivos temporários órfãos deixados para trás no cofre. Caso o arquivo atual já tenha outro nome, o comportamento padrão de salvar como uma nova cópia ativa é preservado.

## 2. Requisitos e Critérios de Aceite
1. Atalho de teclado: `Ctrl+Shift+S` ativa o modal de entrada de texto "Salvar Como".
2. Modal interativo: permite digitar o nome do novo arquivo (extensão `.md` adicionada automaticamente se omitida), com navegação (`Backspace`, digitação de caracteres, `Esc` para cancelar, `Enter` para confirmar).
3. Renomeação atômica: se o arquivo ativo for `nova_nota.md`, a ação move/renomeia fisicamente o arquivo em disco para o novo destino. Se o arquivo já tiver outro nome, grava a cópia no novo destino.
4. Estado do editor: atualiza `app.current_file`, buffers e indexação SQLite FTS para o novo arquivo.
5. Modal de ajuda: atualizado para incluir o atalho `Ctrl+Shift+S`.

## 3. Ciclo GOST DD-TDD
- Fase 1: Ticket #18 delimitado.
- Fase 2: Branch `feat/18-salvar-como-rename`.
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho comprovando a renomeação de `nova_nota.md` e persistência.
- Fase 5: Implementação verde no `mandu-tui`.
- Fase 6: Refatoração.
- Fase 7: Testes de mutação sobre paths relativos e renomeação.
- Fase 8: Quality Gate (`cargo fmt`, `clippy`, testes).
- Fase 9: Fechamento com laudo via `cccp close`.
