# Especificação 25: Servidor de Aplicação (mandu-server) e Cliente Web Desacoplado (mandu-web)

- **ID do Ticket:** #12
- **Status:** Em Especificação (Fase 3 DD-TDD)
- **Autora:** Lumen
- **Alinhamento:** GOST 1.5.0 (Rust)

---

## 1. Contexto & Motivação

O **Mandu** foi arquitetado originalmente como uma aplicação de escrita acadêmica e gestão de conhecimento em 100% Rust voltada a ambientes locais (Desktop via `mandu-gui`/`mandu-tui` e Android via `mandu-mobile`), operando diretamente sobre o sistema de arquivos local e renderizando documentos via motor **Joaju** (Typst).

Contudo, para cenários em que a usuária necessita acessar sua base de conhecimento e recursos do Mandu através de navegadores em qualquer dispositivo (sem acesso direto ao sistema de arquivos nem sincronização prévia de gigabytes de notas e bancos de dados SQLite/FTS5 em memória do cliente), faz-se necessária uma arquitetura cliente-servidor desacoplada:

1. **Processamento Server-Side:** Manter o cofre de notas, a indexação full-text (SQLite FTS5), a renderização pesada de Typst/Joaju e futuros pipelines de IA no servidor (`mandu-server`).
2. **Cliente Web Leve e Acessível:** Fornecer uma interface Web (`mandu-web`) em **Dioxus 0.7** que compartilhe 100% da identidade visual, layout RSX e componentes do `mandu-mobile`, compilada para WebAssembly (WASM).
3. **Resiliência e Modo Offline:** O cliente web deve persistir rascunhos e alterações pendentes no armazenamento do navegador (`localStorage`), garantindo que oscilações de conectividade não provoquem perda de trabalho.
4. **Autenticação Soberana Desacoplada (Authelia):** Suporte nativo a proxy reverso com Authelia, com logout sanitizador (limpeza de cache local e redirecionamento dinâmico sem URLs hardcoded).

---

## 2. Arquitetura e Contrato das Crates

### 2.1. Crate `mandu-server` (Backend Axum)

A nova crate `crates/mandu-server` atua como daemon HTTP de alta performance integrado ao `mandu-core`:

* **Framework:** `axum` com runtime assíncrono `tokio`.
* **Configuração Hermética (Zero Hardcoding):**
  - `MANDU_PORT`: Porta TCP de escuta (padrão: `3000`).
  - `MANDU_BIND_ADDR`: Endereço de vinculação (padrão: `0.0.0.0`).
  - `MANDU_VAULT_PATH`: Diretório absoluto ou relativo do cofre de notas gerenciado.
  - `MANDU_AUTH_LOGOUT_URL`: URL externa para redirecionamento após logout (ex.: `https://auth.2lp.in/logout`).
  - `MANDU_CORS_ORIGIN`: Origens HTTP permitidas para CORS (padrão: permissivo ou configurável).

#### Endpoints REST

| Método | Rota | Descrição |
|---|---|---|
| `GET` | `/health` | Checagem de integridade e versão ativa do servidor. |
| `GET` | `/api/config` | Fornece ao frontend configurações públicas (ex.: URL de logout). |
| `GET` | `/api/tree` | Retorna a árvore hierárquica de pastas e notas `.md` do cofre. |
| `GET` | `/api/notes?path=<rel>` | Obtém o conteúdo e metadados da nota indicada por caminho relativo seguro. |
| `PUT` | `/api/notes?path=<rel>` | Grava ou atualiza a nota, executando o linter do `mandu-core`. |
| `GET` | `/api/search?q=<query>` | Consulta de busca textual FTS5 em todo o cofre. |
| `POST` | `/api/render/joaju` | Renderiza trecho de Markdown/Typst para visualização remota em SVG/HTML. |
| `POST` | `/api/ai/extract` | Endpoint de extração semântica e sumarização via IA. |

#### Segurança contra Path Traversal
Qualquer tentativa de navegação acima da raiz do cofre (`..`, caminhos absolutos arbitrários ou caracteres de escape) deve ser estritamente bloqueada pelo servidor, retornando código HTTP `400 Bad Request` ou `403 Forbidden`.

---

### 2.2. Abstração de Provedor de Cofre (`VaultProvider`)

Para viabilizar o reuso integral da interface gráfica do `mandu-mobile` entre plataformas nativas (Desktop/Android) e Web, o acesso aos dados é abstraído por meio de uma trait assíncrona desacoplada:

```rust
#[async_trait::async_trait]
pub trait VaultProvider: Send + Sync {
    async fn list_tree(&self) -> Result<Vec<VaultEntry>, ProviderError>;
    async fn get_note(&self, relative_path: &str) -> Result<NotePayload, ProviderError>;
    async fn save_note(&self, relative_path: &str, content: &str) -> Result<(), ProviderError>;
    async fn search(&self, query: &str) -> Result<Vec<SearchResultItem>, ProviderError>;
    async fn render_preview(&self, markdown: &str) -> Result<String, ProviderError>;
}
```

1. **`LocalVaultProvider`:** Utilizado pelo `mandu-mobile` (Android APK) e desktop local, invocando diretamente o subsistema de arquivos e o SQLite do `mandu-core`.
2. **`RemoteVaultProvider`:** Utilizado pelo `mandu-web` (e opcionalmente pelo `mandu-mobile` em modo conectado), despachando requisições HTTP para a API do `mandu-server` com camada de contingência em `localStorage`.

---

### 2.3. Crate `mandu-web` (Cliente Dioxus WASM)

* **Compilação:** Alvo `wasm32-unknown-unknown` utilizando as features Web do Dioxus 0.7 (`dioxus/web`).
* **Interface:** Reutilização total dos componentes visuais de `mandu-mobile` (`NoteEditor`, `QuickToolbar`, `NotePreview`, `VaultDrawer`), mantendo consistência visual idêntica (tema escuro, paleta Dracula/Lumen).
* **Gestão de Sessão & Logout:** Botão de encerramento de sessão que:
  1. Purga dados e rascunhos temporários salvos no `localStorage`/`IndexedDB`.
  2. Consulta a URL pública informada em `/api/config`.
  3. Redireciona o navegador para o endpoint de logout do Authelia.

---

## 3. Critérios de Aceitação

1. **Estrutura de Workspace:** As novas crates `crates/mandu-server` e `crates/mandu-web` devem constar como membros ativos de `[workspace]` no `Cargo.toml`.
2. **Sanitização de Caminhos:** O servidor deve impedir qualquer leitura ou escrita fora dos limites de `MANDU_VAULT_PATH`.
3. **Contrato de API REST:** Todos os endpoints `/health`, `/api/config`, `/api/tree`, `/api/notes`, `/api/search` e `/api/render/joaju` devem responder com payloads JSON válidos e códigos HTTP padronizados.
4. **Zero Hardcoding:** Nenhuma URL de serviço de autenticação, porta de escuta ou caminho de diretório pode ser fixada em código-fonte.
5. **Qualidade & GOST:** Compilação limpa com `cargo clippy --all-targets -- -D warnings`, formatação 100% via `cargo fmt --check` e aprovação rigorosa no `cccp check --strict`.
