#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build LocalVoice locally and publish a GitHub release.

.DESCRIPTION
    1. Reads the current version from package.json (or accepts --version override).
    2. Optionally bumps the version (--bump major|minor|patch).
    3. Creates an annotated git tag vX.Y.Z and pushes it.
    4. Runs `pnpm tauri build` to produce the Windows installers.
    5. Creates a GitHub release via `gh` CLI and uploads the bundles.

.PARAMETER Version
    Explicit version to release (e.g. "1.2.3"). Skips auto-detection.

.PARAMETER Bump
    Bump the version before releasing: Major, Minor, or Patch.

.PARAMETER Draft
    Publish the release as a draft (default: false).

.PARAMETER Prerelease
    Mark the release as a pre-release.

.PARAMETER DryRun
    Print what would happen without making any changes.

.EXAMPLE
    .\scripts\create-release.ps1
    Release the current version from package.json.

.EXAMPLE
    .\scripts\create-release.ps1 -Bump Minor
    Bump minor version, build, and release.

.EXAMPLE
    .\scripts\create-release.ps1 -Version 1.0.0 -Draft
    Release v1.0.0 as a draft.
#>

param(
    [string]$Version    = "",
    [ValidateSet('Major','Minor','Patch','')]
    [string]$Bump       = "",
    [switch]$Draft,
    [switch]$Prerelease,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir   = Join-Path $ScriptDir '..'

# ── Helpers ───────────────────────────────────────────────────────────────────

function Write-Step  { param([string]$m) Write-Host "`n==> $m" -ForegroundColor Cyan }
function Write-Ok    { param([string]$m) Write-Host "  [OK] $m" -ForegroundColor Green }
function Write-Fail  { param([string]$m) Write-Host "  [FAIL] $m" -ForegroundColor Red; exit 1 }
function Write-Info  { param([string]$m) Write-Host "  $m" -ForegroundColor Gray }
function Write-Dry   { param([string]$m) Write-Host "  [DRY-RUN] $m" -ForegroundColor Yellow }

function Invoke-Cmd {
    param([string]$Cmd, [string[]]$Args)
    if ($DryRun) { Write-Dry "$Cmd $($Args -join ' ')"; return }
    & $Cmd @Args
    if ($LASTEXITCODE -ne 0) { Write-Fail "'$Cmd $($Args -join ' ')' failed (exit $LASTEXITCODE)" }
}

function Bump-Semver {
    param([string]$v, [string]$type)
    $p = $v.Split('.')
    $ma = [int]$p[0]; $mi = [int]$p[1]; $pa = [int]$p[2]
    switch ($type) {
        'Major' { $ma++; $mi = 0; $pa = 0 }
        'Minor' { $mi++; $pa = 0 }
        'Patch' { $pa++ }
    }
    return "$ma.$mi.$pa"
}

function Set-VersionInFile {
    param([string]$File, [string]$Old, [string]$New)
    if (-not (Test-Path $File)) { return }
    $c = Get-Content $File -Raw
    $c = $c -replace ([regex]::Escape("`"version`": `"$Old`"")), "`"version`": `"$New`""
    if (-not $DryRun) { $c | Set-Content $File -NoNewline -Encoding UTF8 }
    Write-Info "Updated version in $((Split-Path $File -Leaf)): $Old -> $New"
}

# ── Pre-flight checks ─────────────────────────────────────────────────────────

Write-Step "Checking prerequisites"

foreach ($tool in @('git','pnpm','gh')) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        Write-Fail "$tool not found in PATH. Please install it first."
    }
    Write-Ok "$tool found"
}

# Verify gh is authenticated
$ghAuth = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) { Write-Fail "gh CLI is not authenticated. Run: gh auth login" }
Write-Ok "gh authenticated"

# Verify working tree is clean
$dirty = git status --porcelain
if ($dirty) {
    Write-Fail "Working tree has uncommitted changes. Commit or stash them first."
}
Write-Ok "Working tree clean"

# ── Resolve version ───────────────────────────────────────────────────────────

Write-Step "Resolving version"

$pkgJson = Join-Path $RootDir 'package.json'
$currentVersion = (Get-Content $pkgJson -Raw | ConvertFrom-Json).version

if ($Version -ne "") {
    $releaseVersion = $Version
    Write-Info "Using explicit version: $releaseVersion"
} elseif ($Bump -ne "") {
    $releaseVersion = Bump-Semver -v $currentVersion -type $Bump
    Write-Info "Bumping $Bump: $currentVersion -> $releaseVersion"
} else {
    $releaseVersion = $currentVersion
    Write-Info "Using current version: $releaseVersion"
}

$tag = "v$releaseVersion"

# Check tag doesn't already exist remotely
$existingTag = git ls-remote --tags origin "refs/tags/$tag" 2>&1
if ($existingTag -match $tag) {
    Write-Fail "Tag $tag already exists on remote. Use a different version."
}

# ── Bump version files if needed ──────────────────────────────────────────────

if ($releaseVersion -ne $currentVersion) {
    Write-Step "Updating version files to $releaseVersion"
    Set-VersionInFile -File $pkgJson -Old $currentVersion -New $releaseVersion
    Set-VersionInFile -File (Join-Path $RootDir 'src-tauri\tauri.conf.json') -Old $currentVersion -New $releaseVersion
    Set-VersionInFile -File (Join-Path $RootDir 'src-tauri\Cargo.toml') -Old $currentVersion -New $releaseVersion

    if (-not $DryRun) {
        # Update Cargo.lock
        Push-Location (Join-Path $RootDir 'src-tauri')
        cargo update --workspace --quiet 2>$null
        Pop-Location

        git add (Join-Path $RootDir 'package.json') `
                (Join-Path $RootDir 'src-tauri\tauri.conf.json') `
                (Join-Path $RootDir 'src-tauri\Cargo.toml') `
                (Join-Path $RootDir 'src-tauri\Cargo.lock')
        Invoke-Cmd git @('commit', '-m', "chore: bump version to $releaseVersion")
        Invoke-Cmd git @('push')
    }
}

# ── Create and push tag ───────────────────────────────────────────────────────

Write-Step "Creating tag $tag"
Invoke-Cmd git @('tag', '-a', $tag, '-m', "Release $tag")
Invoke-Cmd git @('push', 'origin', $tag)
Write-Ok "Tag $tag pushed"

# ── Build ─────────────────────────────────────────────────────────────────────

Write-Step "Building Tauri app (this takes a few minutes)"
Push-Location $RootDir
Invoke-Cmd pnpm @('tauri', 'build')
Pop-Location
Write-Ok "Build complete"

# ── Collect artifacts ─────────────────────────────────────────────────────────

Write-Step "Collecting build artifacts"

$bundleDir = Join-Path $RootDir 'src-tauri\target\release\bundle'
$artifacts  = @(
    Get-ChildItem "$bundleDir\msi"  -Filter "*$releaseVersion*.msi"  -ErrorAction SilentlyContinue
    Get-ChildItem "$bundleDir\nsis" -Filter "*$releaseVersion*.exe"  -ErrorAction SilentlyContinue
)

if ($artifacts.Count -eq 0) {
    Write-Fail "No build artifacts found for version $releaseVersion in $bundleDir"
}

foreach ($f in $artifacts) {
    Write-Info "$($f.Name)  ($([math]::Round($f.Length/1MB,1)) MB)"
}

# ── Publish GitHub release ────────────────────────────────────────────────────

Write-Step "Publishing GitHub release $tag"

$releaseNotes = @"
## LocalVoice $tag

### Installation
Download the \``.msi\`` (Windows Installer) or the \``-setup.exe\`` (NSIS installer) below and run it.

> **Note:** Whisper models are not bundled. Download them from the **Models** page inside the app after installation.

### Changes
See [commits since last release](https://github.com/iptoux/localvoice/commits/$tag) for the full changelog.
"@

$ghArgs = @(
    'release', 'create', $tag
    '--title', "LocalVoice $tag"
    '--notes', $releaseNotes
)

if ($Draft)      { $ghArgs += '--draft' }
if ($Prerelease) { $ghArgs += '--prerelease' }

# Append artifact paths
foreach ($f in $artifacts) { $ghArgs += $f.FullName }

Invoke-Cmd gh $ghArgs

# ── Done ──────────────────────────────────────────────────────────────────────

Write-Host ""
Write-Host "========================================" -ForegroundColor Magenta
if ($DryRun) {
    Write-Host "  DRY-RUN complete — no changes made." -ForegroundColor Yellow
} else {
    Write-Host "  Release $tag published successfully!" -ForegroundColor Green
    Write-Host "  https://github.com/iptoux/localvoice/releases/tag/$tag" -ForegroundColor Cyan
}
Write-Host "========================================" -ForegroundColor Magenta
