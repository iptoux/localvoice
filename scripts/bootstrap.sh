#!/bin/bash
# LocalVoice Development Environment Bootstrap Script
# Run this script to set up the development environment on macOS or Linux.

set -e

SKIP_WHISPER=false
SKIP_PARAKEET=false
SKIP_VERIFICATION=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-whisper)
            SKIP_WHISPER=true
            shift
            ;;
        --skip-parakeet)
            SKIP_PARAKEET=true
            shift
            ;;
        --skip-verification)
            SKIP_VERIFICATION=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--skip-whisper] [--skip-verification]"
            echo "  --skip-whisper       Skip downloading/building whisper.cpp binaries"
            echo "  --skip-parakeet      Skip downloading parakeet.cpp sidecar"
            echo "  --skip-verification  Skip verification steps"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WHISPER_DIR="$REPO_ROOT/src-tauri/binaries"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        PLATFORM="macos"
        case "$ARCH" in
            arm64)  RUST_TRIPLE="aarch64-apple-darwin" ;;
            x86_64) RUST_TRIPLE="x86_64-apple-darwin" ;;
            *)      RUST_TRIPLE="aarch64-apple-darwin" ;;
        esac
        ;;
    Linux)
        PLATFORM="linux"
        case "$ARCH" in
            aarch64|arm64) RUST_TRIPLE="aarch64-unknown-linux-gnu" ;;
            *)             RUST_TRIPLE="x86_64-unknown-linux-gnu" ;;
        esac
        ;;
    *)
        echo "Unsupported platform: $OS"
        exit 1
        ;;
esac

WHISPER_BIN="$WHISPER_DIR/whisper-cli-${RUST_TRIPLE}"
PARAKEET_BIN="$WHISPER_DIR/parakeet-cli-${RUST_TRIPLE}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

print_step() {
    echo -e "\n${CYAN}==> $1${NC}"
}

print_success() {
    echo -e "${GREEN}[OK] $1${NC}"
}

print_warn() {
    echo -e "${YELLOW}[WARN] $1${NC}"
}

print_fail() {
    echo -e "${RED}[FAIL] $1${NC}"
}

check_prerequisite() {
    local name="$1"
    local cmd="$2"

    echo -n "Checking for $name... "
    if ! command -v "$cmd" &> /dev/null; then
        print_fail "$name not found. Please install $name first."
        return 1
    fi
    print_success "$name found"
    return 0
}

echo -e "${MAGENTA}========================================"
echo -e "  LocalVoice Dev Environment Bootstrap"
echo -e "  Platform: $OS / $ARCH ($RUST_TRIPLE)"
echo -e "========================================${NC}"

# ── Prerequisites ─────────────────────────────────────────────────────────────
print_step "Checking prerequisites..."

check_prerequisite "Node.js" "node" || exit 1
check_prerequisite "Rust" "rustc" || exit 1

if ! command -v pnpm &> /dev/null; then
    echo -n "Installing pnpm... "
    if command -v npm &> /dev/null; then
        npm install -g pnpm 2>/dev/null && print_success "pnpm installed" || {
            print_fail "pnpm installation failed"
            exit 1
        }
    else
        print_fail "npm not found. Cannot install pnpm."
        exit 1
    fi
else
    print_success "pnpm found"
fi

if [[ "$PLATFORM" == "linux" ]]; then
    print_step "Checking Linux system dependencies..."
    MISSING_PKGS=()
    for pkg in libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libasound2-dev; do
        if ! dpkg -s "$pkg" &>/dev/null 2>&1; then
            MISSING_PKGS+=("$pkg")
        fi
    done
    if [[ ${#MISSING_PKGS[@]} -gt 0 ]]; then
        print_warn "Missing system packages: ${MISSING_PKGS[*]}"
        echo "  Install them with:"
        echo "  sudo apt-get install -y ${MISSING_PKGS[*]}"
        echo "  (Skipping — you may see build errors without these packages)"
    else
        print_success "All required system packages are installed"
    fi
fi

# ── Node.js dependencies ──────────────────────────────────────────────────────
print_step "Installing Node.js dependencies..."
cd "$REPO_ROOT"
pnpm install || { print_fail "Failed to install dependencies"; exit 1; }
print_success "Dependencies installed"

# ── whisper.cpp binaries ──────────────────────────────────────────────────────
if [[ "$SKIP_WHISPER" != "true" ]]; then
    print_step "Setting up whisper.cpp binaries..."
    mkdir -p "$WHISPER_DIR"

    if [[ -f "$WHISPER_BIN" ]]; then
        print_success "whisper-cli already present at $WHISPER_BIN"
    else
        if [[ "$PLATFORM" == "linux" ]]; then
            # Try pre-built Linux binary from whisper.cpp releases.
            WHISPER_URL="https://github.com/ggerganov/whisper.cpp/releases/download/v1.7.1/whisper-bin-linux-x64.tar.gz"
            TEMP_ARCHIVE="/tmp/whisper-bin-linux.tar.gz"
            echo "Downloading Linux binary from $WHISPER_URL..."
            if curl -L --progress-bar "$WHISPER_URL" -o "$TEMP_ARCHIVE" 2>/dev/null; then
                tar -xzf "$TEMP_ARCHIVE" -C "$WHISPER_DIR" 2>/dev/null || true
                rm -f "$TEMP_ARCHIVE"
                # Look for whisper-cli in extracted tree and rename.
                FOUND_BIN=$(find "$WHISPER_DIR" -name "whisper-cli" -type f | head -1)
                if [[ -n "$FOUND_BIN" ]]; then
                    mv "$FOUND_BIN" "$WHISPER_BIN"
                    chmod +x "$WHISPER_BIN"
                    print_success "whisper-cli installed to $WHISPER_BIN"
                else
                    print_warn "whisper-cli not found in archive — falling back to source build"
                    BUILD_FROM_SOURCE=true
                fi
            else
                print_warn "Download failed — building from source"
                BUILD_FROM_SOURCE=true
            fi
        else
            # macOS: build from source (no official pre-built macOS binaries at this URL).
            BUILD_FROM_SOURCE=true
        fi

        if [[ "${BUILD_FROM_SOURCE:-false}" == "true" ]]; then
            if ! command -v cmake &>/dev/null; then
                print_warn "cmake not found — cannot build whisper.cpp from source."
                if [[ "$PLATFORM" == "macos" ]]; then
                    echo "  Install cmake with: brew install cmake"
                else
                    echo "  Install cmake with: sudo apt-get install cmake"
                fi
                echo "  Then re-run this script, or manually place the binary at:"
                echo "  $WHISPER_BIN"
            else
                echo "Building whisper.cpp from source (this may take a few minutes)..."
                WHISPER_SRC="/tmp/whisper-src-$$"
                git clone --depth 1 --branch v1.7.1 \
                    https://github.com/ggerganov/whisper.cpp "$WHISPER_SRC"
                cmake -S "$WHISPER_SRC" -B "$WHISPER_SRC/build" \
                    -DCMAKE_BUILD_TYPE=Release \
                    -DWHISPER_BUILD_TESTS=OFF \
                    -DWHISPER_BUILD_EXAMPLES=ON
                CORES=$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)
                cmake --build "$WHISPER_SRC/build" --config Release --target whisper-cli -j"$CORES"
                cp "$WHISPER_SRC/build/bin/whisper-cli" "$WHISPER_BIN"
                chmod +x "$WHISPER_BIN"
                rm -rf "$WHISPER_SRC"
                print_success "whisper-cli built and installed to $WHISPER_BIN"
            fi
        fi
    fi
fi

# ── Tauri CLI ─────────────────────────────────────────────────────────────────
# parakeet.cpp sidecar
if [[ "$SKIP_PARAKEET" != "true" ]]; then
    print_step "Setting up parakeet.cpp sidecar..."
    mkdir -p "$WHISPER_DIR"

    if [[ -f "$PARAKEET_BIN" ]]; then
        print_success "parakeet-cli already present at $PARAKEET_BIN"
    else
        PARAKEET_TAG="v0.3.2"
        if [[ "$PLATFORM" == "linux" ]]; then
            if [[ "$RUST_TRIPLE" == "aarch64-unknown-linux-gnu" ]]; then
                PARAKEET_ASSET="parakeet-v0.3.2-bin-linux-cpu-arm64.tar.gz"
                PARAKEET_SHA256="a6f4fd94d373cc7d7f863074e0707e696c3c364dd9cc448deeb4bea350e41c17"
            else
                PARAKEET_ASSET="parakeet-v0.3.2-bin-linux-cpu-x64.tar.gz"
                PARAKEET_SHA256="d84385fa934dc05cd18e94b85069b7b7664569baea0a05fb6e3c09b06613d23d"
            fi
        else
            if [[ "$RUST_TRIPLE" == "aarch64-apple-darwin" ]]; then
                PARAKEET_ASSET="parakeet-v0.3.2-bin-macos-metal-arm64.tar.gz"
                PARAKEET_SHA256="665cc533f504e3ee1b887a42492176ce0aecdd38f692f5bbaefcab669471c035"
            else
                PARAKEET_ASSET="parakeet-v0.3.2-bin-macos-cpu-x64.tar.gz"
                PARAKEET_SHA256="04ff73ed21b29bb9e05c5475c42128a523c399e084de312219b9fce1f6f4e179"
            fi
        fi

        PARAKEET_URL="https://github.com/mudler/parakeet.cpp/releases/download/${PARAKEET_TAG}/${PARAKEET_ASSET}"
        TEMP_ARCHIVE="/tmp/${PARAKEET_ASSET}"
        EXTRACT_DIR="/tmp/localvoice-parakeet-$$"

        echo "Downloading from $PARAKEET_URL..."
        if curl -L --fail --progress-bar "$PARAKEET_URL" -o "$TEMP_ARCHIVE"; then
            if command -v sha256sum &>/dev/null; then
                ACTUAL_SHA256="$(sha256sum "$TEMP_ARCHIVE" | awk '{print $1}')"
            else
                ACTUAL_SHA256="$(shasum -a 256 "$TEMP_ARCHIVE" | awk '{print $1}')"
            fi

            if [[ "$ACTUAL_SHA256" != "$PARAKEET_SHA256" ]]; then
                rm -f "$TEMP_ARCHIVE"
                print_warn "Checksum mismatch for $PARAKEET_ASSET"
            else
                rm -rf "$EXTRACT_DIR"
                mkdir -p "$EXTRACT_DIR"
                tar -xzf "$TEMP_ARCHIVE" -C "$EXTRACT_DIR"
                FOUND_BIN=$(find "$EXTRACT_DIR" -type f -name "parakeet-cli" | head -1)
                if [[ -n "$FOUND_BIN" ]]; then
                    cp "$FOUND_BIN" "$PARAKEET_BIN"
                    chmod +x "$PARAKEET_BIN"
                    print_success "parakeet-cli installed to $PARAKEET_BIN"
                else
                    print_warn "parakeet-cli not found in archive"
                fi
                rm -f "$TEMP_ARCHIVE"
                rm -rf "$EXTRACT_DIR"
            fi
        else
            print_warn "Failed to download parakeet.cpp sidecar"
            echo "  Manually download: $PARAKEET_URL"
            echo "  Place parakeet-cli at: $PARAKEET_BIN"
        fi
    fi
fi

print_step "Checking Tauri CLI..."
if pnpm tauri --version &> /dev/null; then
    print_success "Tauri CLI installed"
else
    print_step "Installing Tauri CLI..."
    pnpm add -D @tauri-apps/cli || print_warn "Failed to install Tauri CLI."
fi

# ── Rust compilation check ────────────────────────────────────────────────────
if [[ "$SKIP_VERIFICATION" != "true" ]]; then
    print_step "Verifying Rust compilation..."
    cd "$REPO_ROOT/src-tauri"
    if cargo check 2>&1 | grep -q "^error"; then
        print_warn "Rust compilation check had errors. See output above."
    else
        print_success "Rust code compiles successfully"
    fi
    cd "$REPO_ROOT"
fi

echo -e "${MAGENTA}"
echo "========================================"
echo "  Bootstrap Complete!"
echo "========================================"
echo -e "${NC}"
echo "Next steps:"
echo "  1. Run 'pnpm tauri dev' to start development"
if [[ ! -f "$WHISPER_BIN" ]]; then
    echo "  2. whisper-cli is missing — place it at:"
    echo "     $WHISPER_BIN"
    echo "     Or set WHISPER_BIN_PATH=/path/to/whisper-cli in your environment."
fi
if [[ ! -f "$PARAKEET_BIN" ]]; then
    echo "  3. parakeet-cli is missing - place it at:"
    echo "     $PARAKEET_BIN"
    echo "     Or set PARAKEET_BIN_PATH=/path/to/parakeet-cli in your environment."
fi
echo "  See docs/dev/index.md for developer documentation"
echo ""
