# Spec #31 — Tela de Sobre e Diagnóstico de Versões dos Componentes

## 1. Contexto e Motivação
O Mandu integra múltiplos motores de renderização, compilação tipográfica, cálculo químico e análise genealógica. Usuários e desenvolvedores necessitam inspecionar as versões exatas de cada componente embutido, tanto através da linha de comando quanto de forma interativa na interface TUI.

## 2. Requisitos e Critérios de Aceite
1. Módulo canônico no `mandu-core` (`about.rs`):
   - Expõe a lista estruturada de componentes e versões: Mandu Core, Motor Joaju, Typst PDF Engine, Química e Pedigree clínico.
   - Fornece função de formatação textual amigável.
2. Comando CLI `mandu --version --all`:
   - Quando invocado com `--version --all`, imprime a árvore formatada de versões dos componentes e encerra com código 0.
3. Interface TUI:
   - Suporte a modal "Sobre o Mandu" (`is_about_open`) ativado via atalho `Ctrl+I` ou fechado com `Esc` / `q`.
   - Renderização limpa e legível dos dados de versão dos componentes.
   - Atualização do modal de ajuda para documentar o atalho `Ctrl+I`.

## 3. Ciclo GOST DD-TDD
- Fase 1: Ticket #31 delimitado.
- Fase 2: Branch `feat/31-sobre-diagnostico-versoes`.
- Fase 3: Especificação técnica (este documento).
- Fase 4: Teste vermelho unitário e de integração.
- Fase 5: Implementação verde no `mandu-core`, `mandu-cli` e `mandu-tui`.
- Fase 6: Refatoração.
- Fase 7: Testes de mutação sobre formatação e flags CLI.
- Fase 8: Quality Gate (`cargo fmt`, `clippy`, testes).
- Fase 9: Fechamento com laudo via `cccp close`.
