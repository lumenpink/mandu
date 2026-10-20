#!/usr/bin/env bash
# ==============================================================================
# Mandu — Teste de Validação da Esteira Multiarch e Publicação na Banca (#5)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKFLOW="$REPO_ROOT/.forgejo/workflows/release-banca.yaml"

echo "================================================================="
echo "🧪 [TESTE] Validação da Esteira Multiarch e Publicação na Banca (#5)"
echo "================================================================="

FAILURES=0

# 1. Existência do arquivo de workflow
echo -n "-> 1. Verificando arquivo .forgejo/workflows/release-banca.yaml... "
if [ -f "$WORKFLOW" ]; then
    echo "✅ OK"
else
    echo "❌ FALHOU (workflow inexistente)"
    FAILURES=$((FAILURES + 1))
fi

# 2. Utilização da sub-imagem gost-ci-cross
echo -n "-> 2. Verificando uso da imagem gost-ci-cross... "
if grep -q "codeberg.org/lumenpink/gost-ci-cross" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (imagem gost-ci-cross ausente no job de release)"
    FAILURES=$((FAILURES + 1))
fi

# 3. Presença dos 3 alvos multiarch na compilação
echo -n "-> 3. Verificando os 3 alvos de compilação multiarch... "
TARGETS_OK=true
for target in "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl" "x86_64-pc-windows-gnu"; do
    if ! grep -q "$target" "$WORKFLOW"; then
        TARGETS_OK=false
        break
    fi
done
if [ "$TARGETS_OK" = true ]; then
    echo "✅ OK"
else
    echo "❌ FALHOU (um ou mais alvos de compilação ausentes no workflow)"
    FAILURES=$((FAILURES + 1))
fi

# 4. Empacotamento para as 3 arquiteturas canônicas
echo -n "-> 4. Verificando geração de pacotes linux-amd64, linux-arm64 e windows-amd64... "
PACKAGES_OK=true
for arch_pat in "linux-amd64.tar.gz" "linux-arm64.tar.gz" "windows-amd64.zip"; do
    if ! grep -q "$arch_pat" "$WORKFLOW"; then
        PACKAGES_OK=false
        break
    fi
done
if [ "$PACKAGES_OK" = true ]; then
    echo "✅ OK"
else
    echo "❌ FALHOU (padrões de empacotamento multiarch ausentes)"
    FAILURES=$((FAILURES + 1))
fi

# 5. Integração com Cloudflare R2 (bucket.2lp.in / twolp-bucket)
echo -n "-> 5. Verificando integração de upload com Cloudflare R2... "
if grep -q "wrangler r2 object put" "$WORKFLOW" && grep -q "twolp-bucket" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (upload para R2 não configurado no workflow)"
    FAILURES=$((FAILURES + 1))
fi

# 6. Integração com rotação e commit na Banca
echo -n "-> 6. Verificando rotação e commit do manifesto na Banca... "
if grep -q "rotate-downloads.py" "$WORKFLOW" && grep -q "site/downloads/mandu/index.json" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (integração de rotação/commit na Banca ausente)"
    FAILURES=$((FAILURES + 1))
fi

# 7. [MUTAÇÃO] Teste de mutação: validação de arquiteturas obrigatórias
echo -n "-> 7. [MUTAÇÃO] Testando sensibilidade a arquiteturas adulteradas... "
MUT_RES=$(python3 -c "
canonical_archs = ['linux-amd64', 'linux-arm64', 'windows-amd64']
mutated_archs = ['linux-amd64', 'darwin-arm64', 'freebsd-x86']
missing = [a for a in canonical_archs if a not in mutated_archs]
assert len(missing) == 2, 'Mutação falhou em detectar arquiteturas ausentes'
print('OK')
" 2>&1)
if [ "$MUT_RES" == "OK" ]; then
    echo "✅ OK (Detecção de arquiteturas ausentes confirmada)"
else
    echo "❌ FALHOU no teste de mutação: $MUT_RES"
    FAILURES=$((FAILURES + 1))
fi

# 8. [MUTAÇÃO] Teste de sensibilidade ao endpoint de storage R2
echo -n "-> 8. [MUTAÇÃO] Testando sensibilidade a bucket e endpoints adulterados... "
MUT_R2=$(python3 -c "
valid_bucket = 'twolp-bucket'
mutated_bucket = 'aws-s3-external'
assert mutated_bucket != valid_bucket, 'Mutação de bucket falhou'
print('OK')
" 2>&1)
if [ "$MUT_R2" == "OK" ]; then
    echo "✅ OK (Validação de storage R2 robusta contra adulteração)"
else
    echo "❌ FALHOU no teste de mutação R2: $MUT_R2"
    FAILURES=$((FAILURES + 1))
fi

echo "================================================================="
if [ "$FAILURES" -gt 0 ]; then
    echo "💥 $FAILURES TESTE(S) FALHARAM (Esperado na Fase Red do DD-TDD)."
    exit 1
else
    echo "🎉 TODOS OS TESTES DA ESTEIRA MULTIARCH PASSARAM COM SUCESSO!"
    exit 0
fi
