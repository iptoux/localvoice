#!/bin/bash
# LocalVoice Development Environment Bootstrap Script
# Run this script to set up the development environment on Unix/macOS

set -e

SKIP_WHISPER=false
SKIP_VERIFICATION=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-whisper)
            SKIP_WHISPER=true
            shift
            ;;
        --skip-verification)
            SKIP_VERIFICATION=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--skip-whisper] [--skip-verification]"
            echo "  --skip-whisper       Skip downloading whisper.cpp binaries"
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
WHISPER_URL="https://github.com/ggerganov/whisper.cpp/releases/download/v1.7.1/whisper-bin-linux-x64.tar.gz"
WHISPER_DIR="$REPO_ROOT/src-tauri/binaries"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

print_step() {
    echo -e "\n==> $1" "$CYAN"
}

print_success() {
    echo -e "[OK] $1" "$GREEN"
}

print_warn() {
    echo -e "[WARN] $1" "$YELLOW"
}

print_fail() {
    echo -e "[FAIL] $1" "$RED"
}

check_prerequisite() {
    local name="$1"
    local cmd="$2"
    local min_version="$3"
    
    echo -n "Checking for $name... "
    if ! command -v "$cmd" &> /dev/null; then
        print_fail "$name not found. Please install $name first."
        return 1
    fi
    
    if [[ -n "$min_version" ]]; then
        local version
        version=$(command -v "$cmd" | xargs --no-run-if-empty "$cmd" --version 2>/dev/null | head -n1 | grep -oP '\d+\.\d+' | head -1 || echo "0.0")
        if [[ $(echo -e "$version\n$min_version" | sort -V | head -n1) != "$min_version" ]]; then
            if [[ "$version" < "$min_version" ]]; then
                print_fail "$name version $version is below minimum $min_version"
                return 1
            fi
        fi
    fi
    
    print_success "$name found"
    return 0
}

echo -e "${MAGENTA}========================================"
echo -e "  LocalVoice Dev Environment Bootstrap"
echo -e "========================================${NC}"

# Check prerequisites
print_step "Checking prerequisites..."

check_prerequisite "Node.js" "node" "20" || exit 1
check_prerequisite "Rust" "rustc" "1.77" || exit 1

if ! command -v pnpm &> /dev/null; then
    echo -n "Installing pnpm... "
    if command -v npm &> /dev/null; then
        npm install -g pnpm 2>/dev/null && print_success "pnpm installed" || print_fail "pnpm installation failed"
    else
        print_fail "npm not found. Cannot install pnpm."
        exit 1
    fi
else
    print_success "pnpm found"
fi

# Install Node.js dependencies
print_step "Installing Node.js dependencies..."
cd "$REPO_ROOT"
pnpm install || { print_fail "Failed to install dependencies"; exit 1; }
print_success "Dependencies installed"

# Download whisper.cpp binaries
if [[ "$SKIP_WHISPER" != "true" ]]; then
    print_step "Downloading whisper.cpp binaries..."
    
    if [[ -d "$WHISPER_DIR" ]]; then
        if ls "$WHISPER_DIR"/whisper-* 1> /dev/null 2>&1; then
            print_success "whisper.cpp binaries already exist"
        fi
    fi
    
    if [[ ! -d "$WHISPER_DIR" ]] || ! ls "$WHISPER_DIR"/whisper-* 1> /dev/null 2>&1; then
        mkdir -p "$WHISPER_DIR"
        
        temp_archive="/tmp/whisper-bin-linux-x64.tar.gz"
        
        echo "Downloading from $WHISPER_URL..."
        if curl -L --progress-bar "$WHISPER_URL" -o "$temp_archive"; then
            echo "Extracting..."
            tar -xzf "$temp_archive" -C "$WHISPER_DIR" || { print_warn "Failed to extract archive"; }
            rm -f "$temp_archive"
            print_success "whisper.cpp binaries downloaded and extracted"
        else
            print_warn "Failed to download whisper.cpp binaries"
            echo "  You can manually download from: $WHISPER_URL"
            echo "  Extract to: $WHISPER_DIR"
        fi
    fi
fi

# Verify Tauri CLI is installed
print_step "Checking Tauri CLI..."
if pnpm tauri --version &> /dev/null; then
    local version=$(pnpm tauri --version)
    print_success "Tauri CLI v$version installed"
else
    print_step "Installing Tauri CLI..."
    pnpm add -D @tauri-apps/cli || print_warn "Failed to install Tauri CLI. Run 'pnpm tauri dev' to install it."
fi

# Install VS Code extensions
print_step "Checking VS Code extensions..."
if [[ -d "$HOME/.config/Code/User" ]]; then
    echo "Recommended extensions are defined in .vscode/extensions.json"
    echo "  - tauri-apps.tauri-vscode"
    echo "  - rust-lang.rust-analyzer"
    print_success "VS Code extensions listed in .vscode/extensions.json"
else
    print_warn "VS Code not detected. Extensions listed in .vscode/extensions.json"
fi

# Verify installation
if [[ "$SKIP_VERIFICATION" != "true" ]]; then
    print_step "Verifying installation..."
    
    echo "Checking Rust compilation..."
    cd "$REPO_ROOT/src-tauri"
    if cargo check 2>&1 | tee /dev/null | grep -q "error"; then
        print_warn "Rust compilation check had issues. This may be normal on first run."
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
echo ""
echo "Next steps:"
echo "  1. Run 'pnpm tauri dev' to start development"
echo "  2. If whisper binaries are missing, copy them to src-tauri/binaries/"
echo "  3. See docs/dev/index.md for developer documentation"
echo ""
