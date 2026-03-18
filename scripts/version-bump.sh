#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
DRY_RUN=false
BUMP_TYPE="patch"

usage() {
    cat <<EOF
Usage: $(basename "$0") [OPTIONS]

Options:
    --major    Bump major version (e.g., 1.0.0 -> 2.0.0)
    --minor    Bump minor version (e.g., 1.0.0 -> 1.1.0)
    --patch    Bump patch version (e.g., 1.0.0 -> 1.0.1) [default]
    --dry-run  Preview changes without modifying files
    -h, --help Show this help message

Examples:
    $(basename "$0") --patch       # Default: 1.0.0 -> 1.0.1
    $(basename "$0") --minor       # 1.0.0 -> 1.1.0
    $(basename "$0") --major       # 1.0.0 -> 2.0.0
    $(basename "$0") --dry-run     # Preview only
EOF
}

parse_semver() {
    local version="$1"
    if [[ ! "$version" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)(-([0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*))?(\+([0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*))?$ ]]; then
        echo "ERROR: Invalid semver format: $version" >&2
        return 1
    fi
    echo "${BASH_REMATCH[1]} ${BASH_REMATCH[2]} ${BASH_REMATCH[3]}"
}

bump_version() {
    local version="$1"
    local bump="$2"
    local parts
    parts=($(parse_semver "$version"))
    local major="${parts[0]}" minor="${parts[1]}" patch="${parts[2]}"

    case "$bump" in
        major) ((major++)); minor=0; patch=0 ;;
        minor) ((minor++)); patch=0 ;;
        patch) ((patch++)) ;;
    esac

    echo "$major.$minor.$patch"
}

validate_version() {
    local version="$1"
    if [[ ! "$version" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
        return 1
    fi
    return 0
}

get_package_version() {
    local pkg_json="$1"
    if [[ -f "$pkg_json" ]]; then
        local normalized
        normalized="$(cygpath -m "$pkg_json" 2>/dev/null || echo "$pkg_json")"
        normalized="${normalized//\\//}"
        node -e "try{const p=require('$normalized');console.log(p.version||'')}catch(e){console.log('')}"
    fi
}

update_package_json() {
    local file="$1"
    local old_ver="$2"
    local new_ver="$3"
    local pkg_name="$4"

    if [[ ! -f "$file" ]]; then
        return 1
    fi

    local temp_file="${file}.tmp"
    local content
    content=$(cat "$file")

    content="${content//\"version\": \"${old_ver}\"/\"version\": \"${new_ver}\"}"
    content="${content//'\"version": "'"${old_ver}"'\"'/'\"version": "'"${new_ver}"'\"'}"

    local all_pkgs=("localvoice" "@automaker/utils" "@automaker/types" "@automaker/spec-parser" "@automaker/prompts" "@automaker/platform" "@automaker/model-resolver" "@automaker/git-utils" "@automaker/dependency-resolver")
    for dep_pkg in "${all_pkgs[@]}"; do
        local dep_pattern="\"${dep_pkg}\": \"${old_ver}\""
        local dep_replacement="\"${dep_pkg}\": \"${new_ver}\""
        content="${content//${dep_pattern}/${dep_replacement}}"
    done

    echo "$content" > "$temp_file"
    mv "$temp_file" "$file"
}

find_packages() {
    local dirs=("apps" "libs")
    local packages=()

    for dir in "${dirs[@]}"; do
        if [[ -d "$ROOT_DIR/$dir" ]]; then
            while IFS= read -r -d '' pkg_json; do
                packages+=("$pkg_json")
            done < <(find "$ROOT_DIR/$dir" -name "package.json" -type f -print0 2>/dev/null || true)
        fi
    done

    if [[ -f "$ROOT_DIR/package.json" ]]; then
        packages+=("$ROOT_DIR/package.json")
    fi

    printf '%s\n' "${packages[@]}"
}

main() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --major) BUMP_TYPE="major"; shift ;;
            --minor) BUMP_TYPE="minor"; shift ;;
            --patch) BUMP_TYPE="patch"; shift ;;
            --dry-run) DRY_RUN=true; shift ;;
            -h|--help) usage; exit 0 ;;
            *) echo "Unknown option: $1"; usage; exit 1 ;;
        esac
    done

    local root_pkg="$ROOT_DIR/package.json"
    if [[ ! -f "$root_pkg" ]]; then
        echo "ERROR: Root package.json not found at $root_pkg" >&2
        exit 1
    fi

    local current_version
    current_version=$(get_package_version "$root_pkg")

    if ! validate_version "$current_version"; then
        echo "ERROR: Invalid version in root package.json: $current_version" >&2
        exit 1
    fi

    local new_version
    new_version=$(bump_version "$current_version" "$BUMP_TYPE")

    echo "=== Version Bump Summary ==="
    echo "Current version:  $current_version"
    echo "New version:      $new_version"
    echo "Bump type:        $BUMP_TYPE"
    echo "Dry run:          $DRY_RUN"
    echo ""
    echo "Changes:"

    local changes_made=0
    while IFS= read -r pkg_json; do
        local pkg_version
        pkg_version=$(get_package_version "$pkg_json")

        if [[ -z "$pkg_version" ]]; then
            continue
        fi

        if [[ "$pkg_version" == "$current_version" ]]; then
            local rel_path="${pkg_json#$ROOT_DIR/}"
            echo "  [UPDATE] $rel_path: $pkg_version -> $new_version"

            if [[ "$DRY_RUN" == "false" ]]; then
                update_package_json "$pkg_json" "$current_version" "$new_version" ""
            fi
            ((changes_made++)) || true
        fi
    done < <(find_packages)

    if [[ $changes_made -eq 0 ]]; then
        echo "  (no packages found with version $current_version)"
    fi

    echo ""
    echo "Total packages updated: $changes_made"

    if [[ "$DRY_RUN" == "true" ]]; then
        echo ""
        echo "[DRY-RUN] No files were modified."
    fi
}

main "$@"
