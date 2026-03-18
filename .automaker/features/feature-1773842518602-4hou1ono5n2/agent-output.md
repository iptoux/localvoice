
🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice"
}

🔧 Tool: read
Input: {
  "filePath": "C:\\Users\\Maik Roland Damm\\Entwicklung\\localvoice\\package.json"
}

🔧 Tool: glob
Input: {
  "pattern": "**/package.json"
}

🔧 Tool: bash
Input: {
  "command": "ls -la apps libs 2>/dev/null || echo \"No apps/ or libs/ directories found\"",
  "description": "Check for apps/ and libs/ directories"
}
## Planning Outline

**Goal**: Create a cross-platform version bump script for a monorepo structure.

**Approach**: The requested structure (apps/ and libs/) doesn't exist in this project. I'll create the script with the directory structure mentioned in the feature request, but note that this appears to be a new structure to be created or the script is meant for a different context. I'll create a robust script that:
1. Works cross-platform using bash-compatible syntax
2. Supports semantic versioning flags
3. Finds and updates all package.json files in specified directories
4. Updates inter-package dependencies
5. Includes dry-run and validation

**Files to Touch**:
- `scripts/version-bump.sh` (new) - main script

**Tasks**:
1. Create `scripts/` directory
2. Write version-bump.sh with semver logic, dry-run, cross-platform support
3. Add help/usage documentation within script
4. Create placeholder package.json files in apps/ and libs/ for testing (or note they need to be created)

**Risks**:
- Windows compatibility (prefer bash, but mention PowerShell fallback)
- JSON parsing with `jq` (will check if available)
- Inter-package dependency regex matching

---

[SPEC_GENERATED] Please review the planning outline above. The project currently has a flat Tauri structure (no apps/libs). Should I:
1. Create the apps/libs structure and populate with package.json files?
2. Write the script for a hypothetical monorepo structure as specified?
3. Adapt to work with the existing single-package structure?## Revised Planning Outline

**Goal**: Create cross-platform version bump scripts for a monorepo structure (bash/sh + PowerShell).

**Approach**: Create two scripts that work identically across platforms:
1. `scripts/version-bump.sh` - bash/sh script for Linux/macOS/Git Bash/WSL
2. `scripts/version-bump.ps1` - PowerShell script for Windows native

Both scripts will:
- Support `--major`, `--minor`, `--patch` flags (default: patch)
- Update version in all package.json files in apps/ and libs/ directories
- Update inter-package dependencies
- Include `--dry-run` preview mode
- Validate semver format
- Print change summary

**Files to Touch**:
- `scripts/version-bump.sh` (new)
- `scripts/version-bump.ps1` (new)

```tasks
- [ ] T001: Create scripts/ directory | File: scripts/
- [ ] T002: Write cross-platform version-bump.sh (bash) | File: scripts/version-bump.sh
- [ ] T003: Write cross-platform version-bump.ps1 (PowerShell) | File: scripts/version-bump.ps1
```

**Risks**:
- `jq` dependency for bash script (use Node.js JSON parsing as fallback)
- PowerShell JSON handling is native, no extra deps needed
- Cross-platform line endings (LF vs CRLF)

---

[SPEC_GENERATED] Please review the revised specification above.[TASK_COMPLETE] T001: Created scripts/ directory at the root of the project.[TASK_COMPLETE] T002: Created `scripts/version-bump.sh` with:
- `--major`, `--minor`, `--patch` flags (default: patch)
- `--dry-run` preview mode
- Semver validation
- Cross-platform Windows path handling via `cygpath -m`
- Inter-package dependency updates for all specified packages
- Help documentation[TASK_COMPLETE] T003: Created `scripts/version-bump.ps1` with:
- `-BumpType` parameter (Major/Minor/Patch) and `--major/--minor/--patch` shortcuts
- `-DryRun` switch for preview mode
- Cross-platform compatible (PowerShell 5.1 and 7+)
- Native JSON handling via `ConvertFrom-Json`
- Inter-package dependency updates for all specified packages
- Help documentation

<summary>
## Summary: Cross-Platform Monorepo Version Bump Script

### Changes Implemented
- Created `scripts/` directory
- Implemented `scripts/version-bump.sh` (bash) with:
  - `--major`, `--minor`, `--patch` flags (default: patch)
  - `--dry-run` preview mode
  - Cross-platform Windows path handling via `cygpath -m`
  - Semver validation
  - Inter-package dependency updates
  - Help documentation
- Implemented `scripts/version-bump.ps1` (PowerShell) with:
  - `-BumpType` parameter and shortcut flags (`--major/--minor/--patch`)
  - `-DryRun` switch for preview mode
  - PowerShell 5.1 and 7+ compatibility
  - Native JSON handling
  - Inter-package dependency updates
  - Help documentation

### Files Modified
- `scripts/` (directory created)
- `scripts/version-bump.sh` (new)
- `scripts/version-bump.ps1` (new)
- `package.json` (version reset to 0.1.0 after testing)

### Notes for Developer
- Run bash script: `bash scripts/version-bump.sh [--major|--minor|--patch] [--dry-run]`
- Run PowerShell script: `powershell -File scripts/version-bump.ps1 [-BumpType Major|Minor|Patch] [-DryRun]`
- Both scripts scan `apps/` and `libs/` directories for package.json files
- Scripts update inter-package dependencies for all @automaker/* packages
</summary>