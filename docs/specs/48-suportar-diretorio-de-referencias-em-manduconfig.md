# Especificação 48: Suportar Diretório de Referências em ManduConfig

- **ID do Ticket:** #48
- **Status:** Em Andamento (DD-TDD Fase 3)
- **Autora:** Lumen
- **Alinhamento:** GOST 3.0.0 (Rust)

## 1. Contexto & Motivação
Até o momento, o Mandu utilizava uma convenção estática baseada em nomes de pasta (`refs/`) e declaração explícita de metadados (`type: reference` / `tipo: referencia`) para identificar notas bibliográficas no cofre.
Contudo, para vaults com convenções semânticas prévias ou preferências de organização (ex.: pastas `bibliografia/`, `sources/` ou `leituras/`), é indispensável permitir a configuração de um caminho relativo arbitrário dentro do cofre.

## 2. Decisões Arquiteturais (Para Natasha)
- **Caminho Relativo ao Cofre Exclusivo:**
  O diretório configurado em `[references] dir` é estritamente relativo à raiz do cofre ativo (`vault_root`).
  *Por que essa decisão foi tomada:*
  1. **Hermeticidade e Portabilidade:** O cofre permanece auto-suficiente; ao clonar ou sincronizar o repositório do cofre em outro computador ou dispositivo móvel (Android), não há caminhos absolutos quebrados (como `/home/usuario/...`).
  2. **Compatibilidade com Sandboxes Móveis:** Em dispositivos móveis (Android), diretórios fora da sandbox do cofre requerem permissões especiais de armazenamento e quebram isolamento de dados.
- **Hierarquia de Configuração:**
  1. `<vault>/.mandu/config.toml` (específico do cofre ativo);
  2. `~/.config/mandu/config.toml` (fallback global);
  3. Padrão zero-config (`refs/` e `type: reference`).
- **Retrocompatibilidade:**
  Mesmo que `[references] dir = "bibliografia"` seja configurado, a detecção de notas em pastas `refs/` e notas com `type: reference` continua ativa como rede de segurança.

## 3. Requisitos Técnicos
1. **Em `crates/mandu-core/src/config.rs`:**
   - Adicionar struct `ReferencesConfig`:
     ```rust
     #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
     pub struct ReferencesConfig {
         #[serde(default)]
         pub dir: Option<String>,
     }
     ```
   - Adicionar campo `references: ReferencesConfig` em `ManduConfig`.
2. **Em `crates/mandu-core/src/refs.rs`:**
   - Adicionar função `extract_reference_from_note_with_custom_dir(note: &Note, custom_dir: Option<&str>) -> Option<ReferenceMd>`.
   - Normalizar caminhos para reconhecer prefixos e subpastas correspondentes ao `custom_dir`.
   - Manter `extract_reference_from_note(note: &Note)` delegando com `None`.
3. **Em `crates/mandu-tui/src/app.rs`:**
   - Carregar `ManduConfig::load_hierarchical` ao inicializar o cofre e passar o `references.dir` na extração de referências.
4. **Testes:**
   - Criar `crates/mandu-core/tests/references_config_spec.rs` validando o carregamento do TOML, a extração em diretório customizado e o fallback.
