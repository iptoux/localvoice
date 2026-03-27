#!/usr/bin/env bash
# create-release.sh — Build LocalVoice locally and publish a GitHub release.
#
# Usage:
#   ./scripts/create-release.sh [OPTIONS]
#
# Options:
#   --version <x.y.z>       Explicit version (default: read from package.json)
#   --bump major|minor|patch Bump version before releasing
#   --append                 Upload artifacts to an existing release/tag (no new tag)
#   --draft                  Publish as draft (ignored with --append)
#   --prerelease             Mark as pre-release (ignored with --append)
#   --skip-whisper           Skip whisper.cpp build (use existing binaries)
#   --dry-run                Print actions without executing them
#   -h, --help               Show this help
#
# Examples:
#   ./scripts/create-release.sh                    # release current version
#   ./scripts/create-release.sh --bump minor       # bump minor, build, release
#   ./scripts/create-release.sh --version 1.0.0    # release explicit version
#   ./scripts/create-release.sh --append           # add artifacts to existing release
#   ./scripts/create-release.sh --dry-run          # preview only

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# ── Defaults ──────────────────────────────────────────────────────────────────
VERSION=""
BUMP=""
DRAFT=false
PRERELEASE=false
APPEND=false
SKIP_WHISPER=false
DRY_RUN=false

# ── Colours ───────────────────────────────────────────────────────────────────
CYAN='\033[0;36m'; GREEN='\033[0;32m'; RED='\033[0;31m'
YELLOW='\033[0;33m'; GRAY='\033[0;37m'; MAGENTA='\033[0;35m'; NC='\033[0m'

step()  { echo -e "\n${CYAN}==> $*${NC}"; }
ok()    { echo -e "  ${GREEN}[OK]${NC} $*"; }
fail()  { echo -e "  ${RED}[FAIL]${NC} $*" >&2; exit 1; }
info()  { echo -e "  ${GRAY}$*${NC}"; }
dry()   { echo -e "  ${YELLOW}[DRY-RUN]${NC} $*"; }

run() {
    if $DRY_RUN; then dry "$*"; return; fi
    "$@" || fail "'$*' failed (exit $?)"
}

# ── Argument parsing ──────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case "$1" in
        --version)    VERSION="$2"; shift 2 ;;
        --bump)       BUMP="$2";    shift 2 ;;
        --append)     APPEND=true;  shift ;;
        --draft)      DRAFT=true;   shift ;;
        --prerelease) PRERELEASE=true; shift ;;
        --skip-whisper) SKIP_WHISPER=true; shift ;;
        --dry-run|-n) DRY_RUN=true; shift ;;
        -h|--help)
            sed -n '2,/^$/p' "$0" | grep '^#' | sed 's/^# \?//'
            exit 0 ;;
        *) fail "Unknown option: $1" ;;
    esac
done

# ── Detect platform ───────────────────────────────────────────────────────────
OS="$(uname -s)"
ARCH="$(uname -m)"
case "$OS" in
    Linux)   PLATFORM="linux" ;;
    Darwin)  PLATFORM="macos" ;;
    MINGW*|MSYS*|CYGWIN*) PLATFORM="windows" ;;
    *) fail "Unsupported platform: $OS" ;;
esac
info "Detected platform: $PLATFORM ($ARCH)"

# ── Pre-flight ────────────────────────────────────────────────────────────────
step "Checking prerequisites"

REQUIRED_TOOLS=(git pnpm gh jq)
[[ "$PLATFORM" != "windows" ]] && REQUIRED_TOOLS+=(cmake)

for tool in "${REQUIRED_TOOLS[@]}"; do
    command -v "$tool" &>/dev/null || fail "$tool not found in PATH."
    ok "$tool found"
done

gh auth status &>/dev/null || fail "gh CLI not authenticated. Run: gh auth login"
ok "gh authenticated"

[[ -z "$(git status --porcelain)" ]] || fail "Working tree has uncommitted changes. Commit or stash first."
ok "Working tree clean"

# ── Resolve version ───────────────────────────────────────────────────────────
step "Resolving version"

CURRENT_VERSION=$(jq -r '.version' "$ROOT_DIR/package.json")

bump_semver() {
    local v="$1" type="$2"
    IFS='.' read -r ma mi pa <<< "$v"
    case "$type" in
        major) ((ma++)); mi=0; pa=0 ;;
        minor) ((mi++)); pa=0 ;;
        patch) ((pa++)) ;;
    esac
    echo "$ma.$mi.$pa"
}

if [[ -n "$VERSION" ]]; then
    RELEASE_VERSION="$VERSION"
    info "Using explicit version: $RELEASE_VERSION"
elif [[ -n "$BUMP" ]]; then
    RELEASE_VERSION=$(bump_semver "$CURRENT_VERSION" "$BUMP")
    info "Bumping $BUMP: $CURRENT_VERSION -> $RELEASE_VERSION"
else
    RELEASE_VERSION="$CURRENT_VERSION"
    info "Using current version: $RELEASE_VERSION"
fi

TAG="v$RELEASE_VERSION"

# ── Tag / release existence check ─────────────────────────────────────────────
TAG_EXISTS_REMOTE=false
if git ls-remote --tags origin "refs/tags/$TAG" | grep -q "$TAG"; then
    TAG_EXISTS_REMOTE=true
fi

if $APPEND; then
    # In append mode the tag/release must already exist.
    $TAG_EXISTS_REMOTE || fail "Tag $TAG does not exist on remote. Cannot append. Create the release first."
    info "Append mode: will upload artifacts to existing release $TAG"
else
    # Normal mode: fail if tag already exists to prevent accidental overwrite.
    $TAG_EXISTS_REMOTE && fail "Tag $TAG already exists on remote. Use --append to add artifacts, or choose a different version."
fi

# ── Bump version files if needed ──────────────────────────────────────────────
if [[ "$RELEASE_VERSION" != "$CURRENT_VERSION" ]] && ! $APPEND; then
    step "Updating version files to $RELEASE_VERSION"

    set_version() {
        local file="$1"
        [[ -f "$file" ]] || return
        if $DRY_RUN; then
            dry "Would update version in $(basename "$file"): $CURRENT_VERSION -> $RELEASE_VERSION"
            return
        fi
        sed -i "s/\"version\": \"$CURRENT_VERSION\"/\"version\": \"$RELEASE_VERSION\"/g" "$file"
        info "Updated $(basename "$file")"
    }

    set_version "$ROOT_DIR/package.json"
    set_version "$ROOT_DIR/src-tauri/tauri.conf.json"
    set_version "$ROOT_DIR/src-tauri/Cargo.toml"

    if ! $DRY_RUN; then
        (cd "$ROOT_DIR/src-tauri" && cargo update --workspace -q 2>/dev/null || true)
        git add "$ROOT_DIR/package.json" \
                "$ROOT_DIR/src-tauri/tauri.conf.json" \
                "$ROOT_DIR/src-tauri/Cargo.toml" \
                "$ROOT_DIR/src-tauri/Cargo.lock" 2>/dev/null || true
        run git commit -m "chore: bump version to $RELEASE_VERSION"
        run git push
    fi
fi

# ── Create and push tag (normal mode only) ────────────────────────────────────
if ! $APPEND; then
    step "Creating tag $TAG"
    run git tag -a "$TAG" -m "Release $TAG"
    run git push origin "$TAG"
    ok "Tag $TAG pushed"
fi

# ── Build whisper.cpp sidecar ─────────────────────────────────────────────────
if [[ "$PLATFORM" != "windows" ]]; then
    step "Setting up whisper.cpp sidecar"

    # Normalise arch for Rust/Tauri triple (Apple reports "arm64").
    RUST_ARCH="$ARCH"
    [[ "$RUST_ARCH" = "arm64" ]] && RUST_ARCH="aarch64"

    if [[ "$PLATFORM" = "linux" ]]; then
        WHISPER_TRIPLE="${RUST_ARCH}-unknown-linux-gnu"
    else
        WHISPER_TRIPLE="${RUST_ARCH}-apple-darwin"
    fi

    WHISPER_BIN="$ROOT_DIR/src-tauri/binaries/whisper-cli-${WHISPER_TRIPLE}"

    if $SKIP_WHISPER && [[ -f "$WHISPER_BIN" ]]; then
        ok "Skipping whisper.cpp build — binary already present: $(basename "$WHISPER_BIN")"
    else
        if $SKIP_WHISPER; then
            info "--skip-whisper set but binary not found; building anyway"
        fi

        # Resolve latest whisper.cpp tag via redirect (no API auth needed).
        info "Resolving latest whisper.cpp release tag..."
        WHISPER_TAG=$(curl -fsSI "https://github.com/ggml-org/whisper.cpp/releases/latest" \
            | grep -i "^location:" \
            | sed 's|.*releases/tag/||' \
            | tr -d '[:space:]')
        [[ -n "$WHISPER_TAG" ]] || fail "Could not resolve latest whisper.cpp tag"
        info "Latest whisper.cpp: $WHISPER_TAG"

        if $DRY_RUN; then
            dry "Would clone whisper.cpp $WHISPER_TAG and build whisper-cli-${WHISPER_TRIPLE}"
        else
            mkdir -p "$ROOT_DIR/src-tauri/binaries"

            echo "  Cloning whisper.cpp $WHISPER_TAG (shallow)..."
            git clone https://github.com/ggml-org/whisper.cpp \
                --branch "$WHISPER_TAG" --depth 1 /tmp/whisper-src

            cmake -S /tmp/whisper-src -B /tmp/whisper-build \
                -DCMAKE_BUILD_TYPE=Release \
                -DWHISPER_BUILD_TESTS=OFF \
                -DWHISPER_BUILD_EXAMPLES=ON \
                -DBUILD_SHARED_LIBS=OFF \
                -Wno-dev

            cmake --build /tmp/whisper-build \
                --config Release \
                -j"$(nproc 2>/dev/null || sysctl -n hw.ncpu)"

            cp /tmp/whisper-build/bin/whisper-cli "$WHISPER_BIN"
            chmod +x "$WHISPER_BIN"

            # macOS: also provide the opposite-arch copy so Tauri finds the
            # binary regardless of which --target is used.
            if [[ "$PLATFORM" = "macos" ]]; then
                for alt in aarch64-apple-darwin x86_64-apple-darwin; do
                    tgt="$ROOT_DIR/src-tauri/binaries/whisper-cli-${alt}"
                    [[ ! -f "$tgt" ]] && cp "$WHISPER_BIN" "$tgt"
                    chmod +x "$tgt"
                done
            fi

            rm -rf /tmp/whisper-src /tmp/whisper-build
            ok "whisper-cli-${WHISPER_TRIPLE} placed"
        fi
    fi
else
    if ! $SKIP_WHISPER; then
        info "Windows: whisper.cpp sidecar must be set up manually (use --skip-whisper if already present)"
    fi
fi

# ── Build ─────────────────────────────────────────────────────────────────────
step "Building Tauri app (this takes a few minutes)"
cd "$ROOT_DIR"
# APPIMAGE_EXTRACT_AND_RUN: linuxdeploy is itself an AppImage; without FUSE
# it refuses to run. Extract-and-run bypasses that.
# NO_STRIP: the strip binary bundled inside linuxdeploy is outdated and fails on
# modern ELF libraries that use .relr.dyn sections (SHT_RELR = 0x13), which are
# standard on Arch Linux and other cutting-edge distros.
if [[ "$PLATFORM" = "linux" ]]; then
    export APPIMAGE_EXTRACT_AND_RUN=1
    export NO_STRIP=1
fi
run pnpm tauri build
ok "Build complete"

# ── Collect artifacts ─────────────────────────────────────────────────────────
step "Collecting build artifacts"

BUNDLE_DIR="$ROOT_DIR/src-tauri/target/release/bundle"
ARTIFACTS=()

case "$PLATFORM" in
    linux)
        while IFS= read -r -d '' f; do ARTIFACTS+=("$f"); done < <(
            find "$BUNDLE_DIR" \( -name "*.deb" -o -name "*.AppImage" -o -name "*.rpm" \) -print0 2>/dev/null
        )
        ;;
    macos)
        # macOS cross-target builds land under target/<triple>/release/bundle
        MACOS_BUNDLE_DIRS=("$BUNDLE_DIR")
        for triple_dir in "$ROOT_DIR/src-tauri/target/"*-apple-darwin/release/bundle; do
            [[ -d "$triple_dir" ]] && MACOS_BUNDLE_DIRS+=("$triple_dir")
        done
        for bdir in "${MACOS_BUNDLE_DIRS[@]}"; do
            while IFS= read -r -d '' f; do ARTIFACTS+=("$f"); done < <(
                find "$bdir" \( -name "*.dmg" -o -name "*.app.tar.gz" \) -print0 2>/dev/null
            )
        done
        ;;
    windows)
        while IFS= read -r -d '' f; do ARTIFACTS+=("$f"); done < <(
            find "$BUNDLE_DIR" \( -name "*.msi" -o -name "*-setup.exe" \) -print0 2>/dev/null
        )
        ;;
esac

# Deduplicate (macOS may find the same file via multiple paths).
SEEN=()
UNIQUE_ARTIFACTS=()
for f in "${ARTIFACTS[@]}"; do
    base="$(basename "$f")"
    if [[ ! " ${SEEN[*]} " =~ " ${base} " ]]; then
        SEEN+=("$base")
        UNIQUE_ARTIFACTS+=("$f")
    fi
done
ARTIFACTS=("${UNIQUE_ARTIFACTS[@]}")

[[ ${#ARTIFACTS[@]} -gt 0 ]] || fail "No artifacts found in $BUNDLE_DIR"

for f in "${ARTIFACTS[@]}"; do
    size=$(du -m "$f" | cut -f1)
    info "$(basename "$f")  (${size} MB)"
done

# ── Publish or append GitHub release ─────────────────────────────────────────
if $APPEND; then
    step "Uploading artifacts to existing release $TAG"
    if $DRY_RUN; then
        dry "gh release upload $TAG ${ARTIFACTS[*]}"
    else
        gh release upload "$TAG" "${ARTIFACTS[@]}" --clobber
        ok "Artifacts uploaded to $TAG"
    fi
else
    step "Publishing GitHub release $TAG"

    NOTES="## LocalVoice $TAG

### Downloads

| Platform | File |
|----------|------|
| Windows (MSI) | \`LocalVoice_*.msi\` |
| Windows (NSIS) | \`LocalVoice_*-setup.exe\` |
| macOS (Apple Silicon) | \`LocalVoice_*.dmg\` |
| Linux (Debian/Ubuntu) | \`local-voice_*.deb\` |
| Linux (AppImage) | \`LocalVoice_*.AppImage\` |

> **Note:** Whisper models are not bundled. Download them from the **Models** page inside the app after installation.

### Changes
See [commits since last release](https://github.com/iptoux/localvoice/commits/$TAG) for the full changelog."

    GH_ARGS=('release' 'create' "$TAG" '--title' "LocalVoice $TAG" '--notes' "$NOTES")
    $DRAFT      && GH_ARGS+=('--draft')
    $PRERELEASE && GH_ARGS+=('--prerelease')
    GH_ARGS+=("${ARTIFACTS[@]}")

    run gh "${GH_ARGS[@]}"
fi

# ── Done ──────────────────────────────────────────────────────────────────────
echo ""
echo -e "${MAGENTA}========================================${NC}"
if $DRY_RUN; then
    echo -e "${YELLOW}  DRY-RUN complete — no changes made.${NC}"
elif $APPEND; then
    echo -e "${GREEN}  Artifacts added to release $TAG.${NC}"
    echo -e "${CYAN}  https://github.com/iptoux/localvoice/releases/tag/$TAG${NC}"
else
    echo -e "${GREEN}  Release $TAG published successfully!${NC}"
    echo -e "${CYAN}  https://github.com/iptoux/localvoice/releases/tag/$TAG${NC}"
fi
echo -e "${MAGENTA}========================================${NC}"
