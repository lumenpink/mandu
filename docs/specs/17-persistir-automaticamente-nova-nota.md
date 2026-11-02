# Spec #17 — Persistir Automaticamente nova_nota ao Abrir Pasta com Índice

## 1. Contexto e Motivação
Ao abrir o Mandu em uma pasta com cofre contendo `indice.md` sem especificar um arquivo de nota, o editor abria diretamente o arquivo de índice para edição, expondo-o ao risco de corrupção ou perda de cabeçalhos frontmatter acidental durante rascunhos. Além disso, em pastas sem arquivos, a criação e persistência inicial de `nova_nota.md` dependia de um ciclo que não persistia o arquivo no disco em auto-save.

## 2. Requisitos e Critérios de Aceite
1. Em `resolve_vault_and_file`, se `file_arg` for `None` e existir `indice.md` no cofre, o arquivo `nova_nota.md` deve ser persistido automaticamente em disco (caso ainda não exista) com frontmatter canônico e retornado como a nota ativa.
2. Caso o diretório de destino não possua permissões de escrita, a falha de escrita deve ser tratada de forma silenciosa ou informativa sem provocar pânico no processo.
3. No ciclo de auto-save da TUI (`save_current_note`), caso `current_file` seja `None`, deve salvar no arquivo `nova_nota.md` dentro do cofre e atualizar `current_file`.

## 3. Ciclo GOST DD-TDD
- Fase 1: Ticket #17 formalizado.
- Fase 2: Branch `fix/17-auto-save-nova-nota`.
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho em `vault_resolution_test.rs`.
- Fase 5: Implementação verde em `mandu-core` e `mandu-tui`.
- Fase 6: Refatoração.
- Fase 7: Testes de mutação.
- Fase 8: Quality Gate (`cargo fmt`, `clippy`, testes).
- Fase 9: Fechamento com laudo via `cccp close`.
