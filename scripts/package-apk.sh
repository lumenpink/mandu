#!/usr/bin/env bash
# ==============================================================================
# Mandu — Empacotador Oficial Android APK e Integracao Pratilera
# Pacote Canonico: in.twolp.mandu
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CRATE_DIR="$REPO_ROOT/crates/mandu-mobile"
DIST_DIR="$REPO_ROOT/dist/android"
FINAL_APK="$DIST_DIR/mandu.apk"
FASTLANE_DIR="$REPO_ROOT/fastlane/metadata/android/in.twolp.mandu"

DO_SYNC_PRATILERA=false
PRATILERA_TARGET="/home/lumen/src/banca"

mkdir -p "$DIST_DIR"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --apk)
            shift
            ;;
        --sync-pratilera)
            DO_SYNC_PRATILERA=true
            if [[ $# -gt 1 && ! "$2" =~ ^-- ]]; then
                PRATILERA_TARGET="$2"
                shift
            fi
            shift
            ;;
        *)
            echo "Uso: $0 [--apk] [--sync-pratilera [CAMINHO_BANCA]]"
            exit 1
            ;;
    esac
done

echo "================================================================="
echo "Mandu — Empacotamento Android APK e Distribuicao Pratilera"
echo "Pacote: in.twolp.mandu"
echo "================================================================="

# 0. Obter versao canonica SemVer do CCCP
APP_VERSION="0.1.0-dev.1"
if [[ -f "$REPO_ROOT/VERSION" ]]; then
    APP_VERSION="$(cat "$REPO_ROOT/VERSION" | tr -d '[:space:]')"
fi
echo "-> Versao canonica CCCP: $APP_VERSION"

# 0.1 Autodeteccao de toolchains do Android (SDK e NDK)
if [ -z "${ANDROID_HOME:-}" ]; then
    if [ -d "/opt/android-sdk" ]; then
        export ANDROID_HOME="/opt/android-sdk"
    elif [ -d "/usr/lib/android-sdk" ]; then
        export ANDROID_HOME="/usr/lib/android-sdk"
    elif [ -d "$HOME/Android/Sdk" ]; then
        export ANDROID_HOME="$HOME/Android/Sdk"
    fi
fi

if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    if [ -d "/opt/android-ndk/android-ndk-r27c" ]; then
        export ANDROID_NDK_HOME="/opt/android-ndk/android-ndk-r27c"
    elif [ -d "/opt/android-ndk" ]; then
        export ANDROID_NDK_HOME="/opt/android-ndk"
    elif [ -n "${ANDROID_HOME:-}" ] && [ -d "$ANDROID_HOME/ndk" ]; then
        LATEST_NDK=$(ls -1d "$ANDROID_HOME/ndk/"* 2>/dev/null | sort -V | tail -n 1 || true)
        if [ -n "$LATEST_NDK" ]; then
            export ANDROID_NDK_HOME="$LATEST_NDK"
        fi
    fi
fi

if [ -n "${ANDROID_HOME:-}" ]; then
    echo "-> ANDROID_HOME: $ANDROID_HOME"
fi
if [ -n "${ANDROID_NDK_HOME:-}" ]; then
    echo "-> ANDROID_NDK_HOME: $ANDROID_NDK_HOME"
fi

# 1. Compilacao do APK via Dioxus CLI (dx)
BUILT_WITH_DX=false
if command -v dx >/dev/null 2>&1 && [ -d "$CRATE_DIR" ]; then
    echo "-> Compilando APK via Dioxus CLI..."
    # Desativa sccache instavel caso RUSTC_WRAPPER aponte para sccache
    export RUSTC_WRAPPER=""
    (cd "$CRATE_DIR" && dx build --platform android --release) || echo "Aviso: dx build reportou avisos"
    DX_APK="$(find "$REPO_ROOT/target/dx" -name "*.apk" 2>/dev/null | head -n 1 || true)"
    if [ -n "$DX_APK" ] && [ -f "$DX_APK" ]; then
        cp "$DX_APK" "$FINAL_APK"
        BUILT_WITH_DX=true
        echo "✓ APK gerado via Dioxus CLI: $FINAL_APK"
    fi
fi

if [ "$BUILT_WITH_DX" = false ]; then
    echo "ERRO: Falha ao compilar APK com Dioxus CLI. A toolchain de compilacao e obrigatoria." >&2
    exit 1
fi

# 1.1 Validacao estrita de tamanho (minimo 1MB)
APK_SIZE=$(stat -c%s "$FINAL_APK" 2>/dev/null || stat -f%z "$FINAL_APK" 2>/dev/null || echo 0)
if [ "$APK_SIZE" -lt 1048576 ]; then
    echo "ERRO: O APK gerado tem tamanho anomalo ($APK_SIZE bytes < 1MB). Abortando publicacao." >&2
    exit 1
fi
echo "✓ Validacao de tamanho aprovada: $APK_SIZE bytes (>= 1MB)"

# 2. Assinatura do APK com chave oficial in_twolp
KEYSTORE_PATH="${MANDU_KEYSTORE_PATH:-}"
if [ -z "$KEYSTORE_PATH" ]; then
    for k in "$HOME/.android/in_twolp_release.keystore" "/root/.android/in_twolp_release.keystore"; do
        if [ -f "$k" ]; then
            KEYSTORE_PATH="$k"
            break
        fi
    done
fi

APKSIGNER_CMD="$(command -v apksigner || true)"
if [ -z "$APKSIGNER_CMD" ] && [ -x "/opt/android-sdk/build-tools/34.0.0/apksigner" ]; then
    APKSIGNER_CMD="/opt/android-sdk/build-tools/34.0.0/apksigner"
fi

if [ -n "$KEYSTORE_PATH" ] && [ -f "$KEYSTORE_PATH" ] && [ -n "$APKSIGNER_CMD" ]; then
    echo "-> Assinando APK com in_twolp_release.keystore..."
    "$APKSIGNER_CMD" sign --ks "$KEYSTORE_PATH" --ks-pass "pass:in_twolp" "$FINAL_APK" 2>/dev/null || echo "Aviso: apksigner executado com avisos."
    "$APKSIGNER_CMD" verify "$FINAL_APK" && echo "✓ Assinatura verificada com sucesso!"
else
    echo "-> Keystore oficial nao encontrada ou apksigner ausente; mantendo pacote original."
fi

# 3. Sincronizacao com a Pratilera (Banca)
if [ "$DO_SYNC_PRATILERA" = true ]; then
    PRATILERA_DIR="$PRATILERA_TARGET/site/pratilera"
    if [ -d "$PRATILERA_DIR" ]; then
        echo "-> Sincronizando pacote e metadados com a Pratilera ($PRATILERA_DIR)..."
        cp "$FINAL_APK" "$PRATILERA_DIR/mandu.apk"

        # Copiar icone principal e diretorio estruturado Fastlane
        mkdir -p "$PRATILERA_DIR/icons" "$PRATILERA_DIR/in.twolp.mandu/pt-BR" "$PRATILERA_DIR/in.twolp.mandu/en-US"
        if [ -f "$FASTLANE_DIR/icon.png" ]; then
            cp "$FASTLANE_DIR/icon.png" "$PRATILERA_DIR/icons/in.twolp.mandu.png"
            cp "$FASTLANE_DIR/icon.png" "$PRATILERA_DIR/in.twolp.mandu/pt-BR/icon.png"
            cp "$FASTLANE_DIR/icon.png" "$PRATILERA_DIR/in.twolp.mandu/en-US/icon.png"
        fi

        if [ -d "$FASTLANE_DIR/pt-BR" ]; then
            cp "$FASTLANE_DIR/pt-BR/"*.txt "$PRATILERA_DIR/in.twolp.mandu/pt-BR/" 2>/dev/null || true
        fi
        if [ -d "$FASTLANE_DIR/en-US" ]; then
            cp "$FASTLANE_DIR/en-US/"*.txt "$PRATILERA_DIR/in.twolp.mandu/en-US/" 2>/dev/null || true
        fi

        GERADOR="$PRATILERA_TARGET/scripts/gerar-pratilera.py"
        if [ -f "$GERADOR" ]; then
            echo "-> Notificando/executando gerador da Pratilera em $GERADOR..."
            python3 "$GERADOR" 2>/dev/null || echo "Aviso: gerar-pratilera executado com avisos de catalogo."
        fi
        echo "✓ Mandu sincronizado com a Pratilera com sucesso."
    else
        echo "Aviso: Diretorio da Pratilera nao encontrado em $PRATILERA_DIR"
    fi
fi

echo "================================================================="
echo "✓ Empacotamento de in.twolp.mandu concluido com sucesso!"
echo "================================================================="
