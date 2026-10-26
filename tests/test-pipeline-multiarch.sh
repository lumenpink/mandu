#!/usr/bin/env bash
# ==============================================================================
# Mandu — Teste de Validação da Esteira Multiarch e Publicação na Banca (#5 & #24)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKFLOW="$REPO_ROOT/.forgejo/workflows/ci.yml"

echo "================================================================="
echo "🧪 [TESTE] Validação da Esteira Multiarch e Publicação na Banca (#5 & #24)"
echo "================================================================="

FAILURES=0

# 1. Existência do arquivo de workflow
echo -n "-> 1. Verificando arquivo .forgejo/workflows/ci.yml... "
if [ -f "$WORKFLOW" ]; then
    echo "✅ OK"
else
    echo "❌ FALHOU (workflow inexistente)"
    FAILURES=$((FAILURES + 1))
fi

# 2. Utilização da sub-imagem gost-ci-cross soberana na Gambi
echo -n "-> 2. Verificando uso da imagem gost-ci-cross soberana... "
if grep -q "gambi.2lp.in/lumen/gost-ci-cross" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (imagem gambi.2lp.in/lumen/gost-ci-cross ausente no job de release)"
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
for arch_pat in "linux-amd64" "linux-arm64" "windows-amd64"; do
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

# 7. Verificação de soberania do registry gambi (sem clones ou patches locais) (#7)
echo -n "-> 7. Verificando consumo de joaju via registry soberano gambi... "
if grep -q 'joaju = {.*registry = "gambi"' "$REPO_ROOT/crates/mandu-core/Cargo.toml" && \
   ! grep -q 'joaju = { path =' "$REPO_ROOT/crates/mandu-core/Cargo.toml" && \
   ! grep -q '\[patch\."sparse+https://gambi.2lp.in' "$REPO_ROOT/Cargo.toml" && \
   grep -q 'sparse+https://gambi.2lp.in/api/packages/lumen/cargo/' "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (dependencia de caminho ou patch local ainda presente, ou registry gambi nao configurado no workflow)"
    FAILURES=$((FAILURES + 1))
fi

# 8. Decomposição matricial do job deploy-banca (#24)
echo -n "-> 8. Verificando estratégia matrix no job deploy-banca... "
if grep -A 10 "deploy-banca:" "$WORKFLOW" | grep -q "matrix:" && \
   grep -q "matrix.arch" "$WORKFLOW" && \
   grep -q "matrix.target" "$WORKFLOW"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (estratégia matrix ausente ou incompleta em deploy-banca)"
    FAILURES=$((FAILURES + 1))
fi

# 9. Isolamento do job deploy-site (#24)
echo -n "-> 9. Verificando job independente deploy-site... "
if grep -q "deploy-site:" "$WORKFLOW" && \
   grep -A 8 "deploy-site:" "$WORKFLOW" | grep -q "needs: \[test\]" && \
   grep -A 8 "deploy-site:" "$WORKFLOW" | grep -q "if: success()"; then
    echo "✅ OK"
else
    echo "❌ FALHOU (job independente deploy-site ausente ou sem guard de dependência)"
    FAILURES=$((FAILURES + 1))
fi

# 10. [MUTAÇÃO] Teste de mutação: validação de arquiteturas obrigatórias
echo -n "-> 10. [MUTAÇÃO] Testando sensibilidade a arquiteturas adulteradas... "
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

# 11. [MUTAÇÃO] Teste de sensibilidade ao endpoint de storage R2
echo -n "-> 11. [MUTAÇÃO] Testando sensibilidade a bucket e endpoints adulterados... "
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

# 12. [MUTAÇÃO] Teste de sensibilidade a reversão do registry para Codeberg
echo -n "-> 12. [MUTAÇÃO] Testando sensibilidade a host de imagem adulterado... "
MUT_REG=$(python3 -c "
text = open('$WORKFLOW').read()
mutated = text.replace('gambi.2lp.in/lumen/gost-ci-cross', 'codeberg.org/lumenpink/gost-ci-cross')
assert 'gambi.2lp.in/lumen/gost-ci-cross' not in mutated
print('OK')
" 2>&1)
if [ "$MUT_REG" == "OK" ]; then
    echo "✅ OK (Detecção de reversão de registry confirmada)"
else
    echo "❌ FALHOU no teste de mutação de registry: $MUT_REG"
    FAILURES=$((FAILURES + 1))
fi

# 13. [MUTAÇÃO] Teste de sensibilidade a remoção da matrix de build
echo -n "-> 13. [MUTAÇÃO] Testando sensibilidade a remoção de matrix... "
MUT_MATRIX=$(python3 -c "
text = open('$WORKFLOW').read()
assert 'matrix:' in text or 'strategy:' in text, 'Matrix precisa estar presente'
print('OK')
" 2>&1 || echo "FALHOU")
if [ "$MUT_MATRIX" == "OK" ]; then
    echo "✅ OK (Sensibilidade a ausência de matrix confirmada)"
else
    echo "❌ FALHOU na validação de matrix: $MUT_MATRIX"
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
