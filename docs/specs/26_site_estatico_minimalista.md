# Especificação 26: Site Estático Minimalista do Mandu

- **ID do Ticket:** #19
- **Branch:** `feat/19-site-estatico-minimalista`
- **Status:** Em Especificação (Fase 3 DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust / Web)

---

## 1. Contexto & Motivação

O **Mandu** é um caderno pessoal seguro e bloco de notas para organizar pensamentos, estudos e anotações com privacidade total. Embora o projeto disponha de uma rota no Cloudflare Workers (`mandu.2lp.in` configurado em `wrangler.jsonc`), sua presença web estática ainda consistia apenas de um arquivo embrionário provisório, sem assets dedicados, sem estilo visual completo no padrão minimalista do projeto e sem os metadados canônicos do ecossistema.

O objetivo do Ticket #19 é criar uma experiência web estática acolhedora, minimalista, rápida e elegante, alinhada à identidade do Mandu, comunicando o valor do aplicativo como refúgio pessoal de escrita e estudo.

---

## 2. Princípios de Design & Arquitetura Web

1. **Minimalismo e Foco na Escrita:**
   - Paleta escura suave (`#0e0e11` / `#16161b`), evitando fadiga visual.
   - Tipografia limpa nativa do sistema com monoespaçamento legível para atalhos e códigos.
   - Sem bibliotecas externas ou CDNs (100% autocontido, sem rastreadores ou chamadas externas).
2. **Estrutura de Arquivos Estáticos (`site/`):**
   - `site/index.html`: Estrutura semântica HTML5 com meta tags canônicas (OpenGraph, viewport, theme-color).
   - `site/assets/style.css`: Estilização minimalista modular e responsiva.
   - `site/assets/favicon.svg`: Identidade visual vetorial leve.
   - `site/_headers`: Cabeçalhos de segurança (CSP, X-Content-Type-Options, Referrer-Policy, Cache-Control).
3. **Cópia e Apresentação (Conforme Spec 13 / Ticket #13):**
   - Apresentação como **caderno pessoal** e **bloco de notas seguro**.
   - Proibição estrita de jargões técnicos inadequados na comunicação principal (como `100% rust`, `ambiente soberano`, `sqlite in-memory`, `fts5`, `blake3`, `pilares arquiteturais`).
   - Seções claras:
     - *Apresentação (Hero)* com badge e introdução acolhedora.
     - *Destaques*: Foco na escrita, arquivos locais sob controle da usuária, busca ágil, multiplataforma (terminal, desktop e celular).
     - *Como Usar / Instalar*: Atalhos de comando e links para download na Banca (`https://banca.2lp.in/`).
     - *Atalhos Essenciais*: Tabela resumida de atalhos de produtividade do Mandu.
     - *Rodapé*: Identidade canônica do ecossistema e licença Apache-2.0.

---

## 3. Contrato de Testes Automatizados (Fase 4 - Red)

O teste de conformidade do site em Rust (`crates/mandu-cli/tests/site_conformance_test.rs`) deve validar estritamente:
1. Existência física dos artefatos: `site/index.html`, `site/assets/style.css`, `site/assets/favicon.svg` e `site/_headers`.
2. Presença de tags essenciais no HTML:
   - `<title>` e metadados OpenGraph (`og:title`, `og:description`, `og:url`).
   - Viewport responsivo e link para stylesheet local `assets/style.css`.
   - Link para favicon local `assets/favicon.svg`.
3. Conformidade da cópia institucional:
   - Presença dos termos acolhedores "caderno pessoal" ou "bloco de notas".
   - Ausência completa dos jargões técnicos banidos.
   - Menção aos atalhos e links válidos (sem `http://` inseguro nos links externos, usando `https://`).
4. Cabeçalhos de segurança em `site/_headers`:
   - `X-Content-Type-Options: nosniff`.
   - `Referrer-Policy: strict-origin-when-cross-origin`.
   - `X-Frame-Options: SAMEORIGIN` ou `DENY`.

---

## 4. Fases DD-TDD do Ciclo

- [x] Fase 1: Ticket #19 formalizado.
- [x] Fase 2: Branch `feat/19-site-estatico-minimalista` criada.
- [x] Fase 3: Especificação `docs/specs/26_site_estatico_minimalista.md` concluída.
- [ ] Fase 4: Teste Red em `crates/mandu-cli/tests/site_conformance_test.rs`.
- [ ] Fase 5: Implementação Green de `site/index.html`, `site/assets/style.css`, `site/assets/favicon.svg` e `site/_headers`.
- [ ] Fase 6: Refactor de acessibilidade e semântica.
- [ ] Fase 7: Testes de mutação e casos de borda.
- [ ] Fase 8: Quality Gate (`cargo fmt`, `cargo clippy`, testes de integração).
- [ ] Fase 9: Changelog atualizado e fechamento da issue #19.
