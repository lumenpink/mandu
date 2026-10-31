#!/usr/bin/env bash
# Red test para o Ticket #27:
# Valida a presença de max-parallel: 1, flags completas de RUSTFLAGS com allow-multiple-definition,
# inclusão do target musl no build.rs e ausência de termos imperialistas.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKFLOW="$ROOT_DIR/.forgejo/workflows/ci.yml"
BUILD_RS="$ROOT_DIR/crates/mandu-core/build.rs"
TUI_MAIN="$ROOT_DIR/crates/mandu-tui/src/main.rs"

FAILED=0

echo "=== [TEST] Auditoria de Flags, Concorrência e Terminologia (Ticket #27) ==="

# 1. max-parallel: 1 na matriz do deploy-banca
if ! grep -q "max-parallel: 1" "$WORKFLOW"; then
    echo "FALHA: .forgejo/workflows/ci.yml deve conter 'max-parallel: 1' na estratégia do deploy-banca" >&2
    FAILED=1
else
    echo "✔ max-parallel: 1 configurado corretamente"
fi

# 2. RUSTFLAGS com crt-static E allow-multiple-definition
if ! grep -F -- '-Wl,--allow-multiple-definition' "$WORKFLOW" | grep -q 'target-feature=+crt-static'; then
    echo "FALHA: .forgejo/workflows/ci.yml deve definir RUSTFLAGS com +crt-static e allow-multiple-definition" >&2
    FAILED=1
else
    echo "✔ RUSTFLAGS contém crt-static e allow-multiple-definition"
fi

# 3. build.rs deve priorizar target musl
if ! grep -Fq 'target/x86_64-unknown-linux-musl/release' "$BUILD_RS"; then
    echo "FALHA: crates/mandu-core/build.rs deve verificar o diretório musl de libjoaju.a" >&2
    FAILED=1
else
    echo "✔ build.rs verifica target musl local"
fi

# 4. Ausência de termos proibidos em ci.yml e mandu-tui
if grep -iE 'soberan[ao]' "$WORKFLOW" >/dev/null 2>&1; then
    echo "FALHA: .forgejo/workflows/ci.yml ainda contém termos proibidos ('soberano')" >&2
    FAILED=1
else
    echo "✔ .forgejo/workflows/ci.yml higienizado"
fi

if grep -iE 'soberan[ao]' "$TUI_MAIN" >/dev/null 2>&1; then
    echo "FALHA: mandu-tui ainda contém termos proibidos ('soberana')" >&2
    FAILED=1
else
    echo "✔ mandu-tui higienizado"
fi

if [ $FAILED -ne 0 ]; then
    echo "❌ [RED TEST] Auditoria falhou conforme esperado na Fase 4." >&2
    exit 1
fi

echo "SUCESSO: Todas as validações da esteira e conformidade foram aprovadas."
exit 0
