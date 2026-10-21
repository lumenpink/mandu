#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORKFLOW="$ROOT_DIR/.forgejo/workflows/ci.yml"
BANCA_ROTATE_SCRIPT="/home/lumen/src/banca/scripts/rotate-downloads.py"

echo "=== [TEST] Validação dos Argumentos de rotate-downloads e Remote Soberano da Banca (#11) ==="

FAILURES=0

# 1. Proibição absoluta de codeberg.org no workflow
echo -n "-> 1. Verificando ausência de referências a Codeberg no ci.yml... "
if grep -qi "codeberg" "$WORKFLOW"; then
    echo "❌ FALHOU: Referência a codeberg encontrada no ci.yml"
    FAILURES=$((FAILURES + 1))
else
    echo "✅ OK"
fi

# 2. Uso do git.2lp.in com GIT_2LP_TOKEN
echo -n "-> 2. Verificando clone soberano via git.2lp.in e GIT_2LP_TOKEN... "
if grep -q "git.2lp.in/lumen/banca.git" "$WORKFLOW" && grep -q "GIT_2LP_TOKEN" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU: Remote soberano git.2lp.in ou GIT_2LP_TOKEN ausente"
    FAILURES=$((FAILURES + 1))
fi

# 3. Eliminação do subcomando inexistente 'add'
echo -n "-> 3. Verificando ausência do subcomando inexistente 'add'... "
if grep -q "rotate-downloads.py add" "$WORKFLOW"; then
    echo "❌ FALHOU: 'rotate-downloads.py add' ainda presente"
    FAILURES=$((FAILURES + 1))
else
    echo "✅ OK"
fi

# 4. Uso do argumento --version-str em vez de --version
echo -n "-> 4. Verificando uso de --version-str em rotate-downloads.py... "
if grep -q -- "--version-str" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU: '--version-str' ausente na invocação"
    FAILURES=$((FAILURES + 1))
fi

# 5. Uso do argumento --app em vez de --app-name
echo -n "-> 5. Verificando uso de --app em vez de --app-name... "
if grep -q -- "--app " "$WORKFLOW" && ! grep -q -- "--app-name" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU: '--app' ausente ou '--app-name' ainda presente"
    FAILURES=$((FAILURES + 1))
fi

# 6. Teste com o próprio parser de argumentos de scripts/rotate-downloads.py
if [ -f "$BANCA_ROTATE_SCRIPT" ]; then
    echo -n "-> 6. Validando parser real de scripts/rotate-downloads.py contra argumentos esperados... "
    TMP_MANIFEST=$(mktemp)
    cat << 'JSON' > "$TMP_MANIFEST"
{
  "app": "mandu",
  "name": "Mandu",
  "stables": [],
  "nightlies": []
}
JSON
    if python3 "$BANCA_ROTATE_SCRIPT" \
        --manifest "$TMP_MANIFEST" \
        --channel "nightly" \
        --arch "linux-amd64" \
        --version-str "dev.999" \
        --url "https://bucket.2lp.in/downloads/mandu/nightly/dummy.tar.gz" \
        --sha256 "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" \
        --tarball "dummy.tar.gz" \
        --app "mandu" >/dev/null 2>&1; then
        echo "✅ OK"
    else
        echo "❌ FALHOU: Argumentos rejeitados pelo parser de rotate-downloads.py"
        FAILURES=$((FAILURES + 1))
    fi
    rm -f "$TMP_MANIFEST"
fi

if [ "$FAILURES" -gt 0 ]; then
    echo "💥 $FAILURES TESTE(S) FALHARAM (Esperado na Fase Red do DD-TDD)."
    exit 1
else
    echo "🎉 TODOS OS TESTES PASSARAM COM SUCESSO!"
    exit 0
fi
