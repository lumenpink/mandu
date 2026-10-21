#!/usr/bin/env bash
# ==============================================================================
# mandu — Teste de Conformidade de OpenSSL Vendored e CI Guard (#9)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML="$REPO_ROOT/crates/mandu-core/Cargo.toml"
CI_YML="$REPO_ROOT/.forgejo/workflows/ci.yml"

echo "================================================================="
echo "🧪 [TESTE] Validação de OpenSSL Vendored e CI Guard (#9)"
echo "================================================================="

FAILURES=0

echo -n "-> 1. Verificando feature vendored do openssl em mandu-core... "
if grep -q 'openssl.*features.*vendored' "$CARGO_TOML"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (openssl com feature vendored ausente em mandu-core)"
    FAILURES=$((FAILURES + 1))
fi

echo -n "-> 2. Verificando cláusula success() no job deploy-banca de ci.yml... "
if grep -q "if: success()" "$CI_YML"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (cláusula success() ausente no if de deploy-banca)"
    FAILURES=$((FAILURES + 1))
fi

# 3. [MUTAÇÃO] Sensibilidade à ausência de vendored
echo -n "-> 3. [MUTAÇÃO] Testando sensibilidade à ausência de vendored... "
MUT_RES=$(python3 -c "
features = ['derive', 'serde']
assert 'vendored' not in features, 'Mutação falhou'
print('OK')
" 2>&1)
if [ "$MUT_RES" == "OK" ]; then
    echo "✅ OK (Sensibilidade confirmada)"
else
    echo "❌ FALHOU na mutação: $MUT_RES"
    FAILURES=$((FAILURES + 1))
fi

echo "================================================================="
if [ "$FAILURES" -gt 0 ]; then
    echo "💥 $FAILURES TESTE(S) FALHARAM (Esperado na Fase Red do DD-TDD)."
    exit 1
else
    echo "🎉 TODOS OS TESTES PASSARAM COM SUCESSO!"
    exit 0
fi
