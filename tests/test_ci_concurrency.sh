#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOW="$ROOT_DIR/.forgejo/workflows/ci.yml"

echo "=== [TEST] Verificando calibracao de concorrencia na CI (#35) ==="

if [ ! -f "$WORKFLOW" ]; then
    echo "FALHA: $WORKFLOW não encontrado!" >&2
    exit 1
fi

echo -n "-> 1. Verificando presenca de CARGO_BUILD_JOBS no job test... "
if grep -A 15 "test:" "$WORKFLOW" | grep -q 'CARGO_BUILD_JOBS: "2"'; then
    echo "✅ OK"
else
    echo "❌ FALHOU: CARGO_BUILD_JOBS: \"2\" não encontrado no job test!"
    exit 1
fi

echo -n "-> 2. Validando sintaxe YAML... "
python3 -c "import yaml; data = yaml.safe_load(open('$WORKFLOW')); env = data['jobs']['test']['env']; assert env.get('CARGO_BUILD_JOBS') == '2'" 2>/dev/null || {
    echo "❌ FALHOU: Estrutura YAML de env.CARGO_BUILD_JOBS invalida!"
    exit 1
}
echo "✅ OK"

echo "🎉 Teste de concorrencia passou com sucesso!"
