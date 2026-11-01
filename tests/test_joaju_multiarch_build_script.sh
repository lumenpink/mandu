#!/usr/bin/env bash
# ==============================================================================
# Mandu — Teste de Validação do Script de Build Multiarch e Token Gambi (#29)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_RS="$REPO_ROOT/crates/mandu-core/build.rs"
WORKFLOW="$REPO_ROOT/.forgejo/workflows/ci.yml"

echo "================================================================="
echo "🧪 [TESTE] Validação do Script de Build Multiarch e Token Gambi (#29)"
echo "================================================================="

FAILURES=0

# 1. Existência do build.rs
echo -n "-> 1. Verificando crates/mandu-core/build.rs... "
if [ -f "$BUILD_RS" ]; then
    echo "✅ OK"
else
    echo "❌ FALHOU (build.rs inexistente)"
    FAILURES=$((FAILURES + 1))
fi

# 2. Priorização do token GAMBI_2LP_IN_TOKEN no build.rs
echo -n "-> 2. Verificando priorização de GAMBI_2LP_IN_TOKEN no build.rs... "
if grep -q 'GAMBI_2LP_IN_TOKEN' "$BUILD_RS"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (GAMBI_2LP_IN_TOKEN não encontrado em build.rs)"
    FAILURES=$((FAILURES + 1))
fi

# 3. Ordem de busca de caminhos locais priorizando {target}
echo -n "-> 3. Verificando prioridade de busca por target específico... "
python3 -c "
content = open('$BUILD_RS').read()
idx_target = content.find('{target}')
idx_x86 = content.find('x86_64-unknown-linux-musl')
if idx_target == -1 or idx_x86 == -1 or idx_target > idx_x86:
    exit(1)
exit(0)
" && echo "✅ OK" || {
    echo "❌ FALHOU ({target} deve ser verificado antes de alvos genéricos x86_64)"
    FAILURES=$((FAILURES + 1))
}

# 4. Flags de resiliência de download (--http1.1 e --retry 3)
echo -n "-> 4. Verificando flags de rede (--http1.1 e --retry) no curl do build.rs... "
if grep -q -- '--http1.1' "$BUILD_RS" && grep -q -- '--retry' "$BUILD_RS"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (flags --http1.1 e --retry ausentes em build.rs)"
    FAILURES=$((FAILURES + 1))
fi

# 5. GAMBI_2LP_IN_TOKEN no workflow ci.yml
echo -n "-> 5. Verificando repasse de GAMBI_2LP_IN_TOKEN no ci.yml... "
if grep -q 'GAMBI_2LP_IN_TOKEN' "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (GAMBI_2LP_IN_TOKEN ausente em ci.yml)"
    FAILURES=$((FAILURES + 1))
fi

# 6. Eliminação de fallback incorreto para x86_64 em compilações aarch64/windows
echo -n "-> 6. Verificando ausência de fallback incorreto para x86_64 em build.rs... "
if grep -q 'fallback_url.*libjoaju-x86_64-linux.a' "$BUILD_RS"; then
    echo "❌ FALHOU (fallback cego para x86_64 ainda presente)"
    FAILURES=$((FAILURES + 1))
else
    echo "✅ OK"
fi

# 7. [MUTAÇÃO] Teste de mutação: remoção de GAMBI_2LP_IN_TOKEN
echo -n "-> 7. [MUTAÇÃO] Testando sensibilidade à ausência de GAMBI_2LP_IN_TOKEN... "
MUT_TOKEN=$(python3 -c "
text = open('$BUILD_RS').read()
mutated = text.replace('GAMBI_2LP_IN_TOKEN', 'OUTRO_TOKEN')
assert 'GAMBI_2LP_IN_TOKEN' not in mutated
print('OK')
" 2>&1)
if [ "$MUT_TOKEN" == "OK" ]; then
    echo "✅ OK (Detecção confirmada)"
else
    echo "❌ FALHOU no teste de mutação: $MUT_TOKEN"
    FAILURES=$((FAILURES + 1))
fi

# 8. [MUTAÇÃO] Teste de mutação: inversão da ordem de target
echo -n "-> 8. [MUTAÇÃO] Testando sensibilidade à inversão de prioridade de target... "
MUT_ORDER=$(python3 -c "
text = open('$BUILD_RS').read()
idx_target = text.find('{target}')
idx_x86 = text.find('x86_64-unknown-linux-musl')
assert idx_target != -1 and idx_x86 != -1 and idx_target < idx_x86
mutated = text.replace('x86_64-unknown-linux-musl', 'PLACEHOLDER_ARCH').replace('{target}', 'x86_64-unknown-linux-musl')
mutated = mutated.replace('PLACEHOLDER_ARCH', '{target}')
assert mutated.find('{target}') > mutated.find('x86_64-unknown-linux-musl'), 'Mutação falhou ao inverter alvos'
print('OK')
" 2>&1)
if [ "$MUT_ORDER" == "OK" ]; then
    echo "✅ OK (Sensibilidade confirmada)"
else
    echo "❌ FALHOU na validação de inversão de ordem: $MUT_ORDER"
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
