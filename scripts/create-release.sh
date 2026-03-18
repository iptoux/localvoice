#!/usr/bin/env bash
# create-release.sh — Build LocalVoice locally and publish a GitHub release.
#
# Usage:
#   ./scripts/create-release.sh [OPTIONS]
#
# Options:
#   --version <x.y.z>       Explicit version (default: read from package.json)
#   --bump major|minor|patch Bump version before releasing
#   --draft                  Publish as draft
#   --prerelease             Mark as pre-release
#   --dry-run                Print actions without executing them
#   -h, --help               Show this help
#
# Examples:
#   ./scripts/create-release.sh                    # release current version
#   ./scripts/create-release.sh --bump minor       # bump minor, build, release
#   ./scripts/create-release.sh --version 1.0.0    # release explicit version
#   ./scripts/create-release.sh --dry-run          # preview only

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# ── Defaults ──────────────────────────────────────────────────────────────────
VERSION=""
BUMP=""
DRAFT=false
PRERELEASE=false
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
        --version)   VERSION="$2"; shift 2 ;;
        --bump)      BUMP="$2";    shift 2 ;;
        --draft)     DRAFT=true;   shift ;;
        --prerelease) PRERELEASE=true; shift ;;
        --dry-run|-n) DRY_RUN=true; shift ;;
        -h|--help)
            sed -n '2,/^$/p' "$0" | grep '^#' | sed 's/^# \?//'
            exit 0 ;;
        *) fail "Unknown option: $1" ;;
    esac
done

# ── Pre-flight ────────────────────────────────────────────────────────────────
step "Checking prerequisites"

for tool in git pnpm gh jq; do
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

# Check tag doesn't already exist remotely
if git ls-remote --tags origin "refs/tags/$TAG" | grep -q "$TAG"; then
    fail "Tag $TAG already exists on remote. Use a different version."
fi

# ── Bump version files if needed ──────────────────────────────────────────────
if [[ "$RELEASE_VERSION" != "$CURRENT_VERSION" ]]; then
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

# ── Create and push tag ───────────────────────────────────────────────────────
step "Creating tag $TAG"
run git tag -a "$TAG" -m "Release $TAG"
run git push origin "$TAG"
ok "Tag $TAG pushed"

# ── Build ─────────────────────────────────────────────────────────────────────
step "Building Tauri app (this takes a few minutes)"
cd "$ROOT_DIR"
run pnpm tauri build
ok "Build complete"

# ── Collect artifacts ─────────────────────────────────────────────────────────
step "Collecting build artifacts"

BUNDLE_DIR="$ROOT_DIR/src-tauri/target/release/bundle"
ARTIFACTS=()

while IFS= read -r -d '' f; do ARTIFACTS+=("$f"); done < <(
    find "$BUNDLE_DIR" \( -name "*${RELEASE_VERSION}*.msi" -o -name "*${RELEASE_VERSION}*setup.exe" \) -print0 2>/dev/null
)

[[ ${#ARTIFACTS[@]} -gt 0 ]] || fail "No artifacts found for version $RELEASE_VERSION in $BUNDLE_DIR"

for f in "${ARTIFACTS[@]}"; do
    size=$(du -m "$f" | cut -f1)
    info "$(basename "$f")  (${size} MB)"
done

# ── Publish GitHub release ────────────────────────────────────────────────────
step "Publishing GitHub release $TAG"

NOTES="## LocalVoice $TAG

### Installation
Download the \`.msi\` (Windows Installer) or the \`-setup.exe\` (NSIS installer) below and run it.

> **Note:** Whisper models are not bundled. Download them from the **Models** page inside the app after installation.

### Changes
See [commits since last release](https://github.com/iptoux/localvoice/commits/$TAG) for the full changelog."

GH_ARGS=('release' 'create' "$TAG" '--title' "LocalVoice $TAG" '--notes' "$NOTES")
$DRAFT      && GH_ARGS+=('--draft')
$PRERELEASE && GH_ARGS+=('--prerelease')
GH_ARGS+=("${ARTIFACTS[@]}")

run gh "${GH_ARGS[@]}"

# ── Done ──────────────────────────────────────────────────────────────────────
echo ""
echo -e "${MAGENTA}========================================${NC}"
if $DRY_RUN; then
    echo -e "${YELLOW}  DRY-RUN complete — no changes made.${NC}"
else
    echo -e "${GREEN}  Release $TAG published successfully!${NC}"
    echo -e "${CYAN}  https://github.com/iptoux/localvoice/releases/tag/$TAG${NC}"
fi
echo -e "${MAGENTA}========================================${NC}"
