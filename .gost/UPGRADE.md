# Relatório Consolidado de Migração (GOST DD-TDD)

- **Perfil:** `lilya`
- **Baseline Instalada:** `2.1.0`
- **Versão Alvo:** `2.2.0`
- **Cadeia Percorrida:** 2.1.0 ➔ 2.2.0
- **Quebras de Contrato:** Sim (atenção redobrada requerida)

---

## Delta de Migração para 2.2.0

# Guia de Migração: Perfil Lilya 2.1.0 ➔ 2.2.0

Este documento formaliza as alterações contratuais e os passos operacionais para elevar um repositório regulado da versão `2.1.0` para a versão `2.2.0` do perfil operacional Lilya.

---

## 1. Quebras de Contrato (*Breaking Changes*)

1. **Herança do Perfil Tático Kollontai 2.2.0 e Requisito rustc 1.100.0:**
   - O perfil Lilya eleva sua base para o perfil tático Kollontai `2.2.0`, incorporando compulsoriamente a exigência de compilador `rustc >= 1.100.0` para projetos na stack `rust`.
2. **Serviço de Regras sob Rotas Imutáveis Versionadas:**
   - As regras operacionais canônicas são servidas exclusivamente nas rotas versionadas `{base_url}/2.2.0/RULES.md` e `RULES.toml`.

---

## 2. Mutação de Regras Declarativas (`.rules/rules.toml`)

- **Preservadas e Revalidadas:**
  - As 9 regras canônicas operacionais (LILYA-001 a LILYA-009) permanecem vigentes, adaptadas à base Kollontai 2.2.0.

---

## 3. Ações Manuais Necessárias

1. **Atualizar a Toolchain Rust:**
   ```sh
   rustup update stable
   # ou fixar em 1.100.0
   rustup default 1.100.0
   ```
2. **Confirmar Versão:**
   ```sh
   rustc --version
   ```

---

## 4. Ações Automatizáveis via Ferramenta

Execute os comandos do Maestro local para atualizar as definições normativas e recompilar o perfil:
```sh
cccp profile set lilya
cccp profile assemble
```
Após a montagem, execute a auditoria rigorosa de conformidade:
```sh
cccp check --strict
```

---

