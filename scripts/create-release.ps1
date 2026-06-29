#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build LocalVoice locally and publish a GitHub release.

.PARAMETER Bump
    Bump the version before releasing: Major, Minor, or Patch.

.PARAMETER Version
    Explicit version to release (e.g. "1.2.3"). Skips auto-detection.

.PARAMETER Draft
    Publish the release as a draft.

.PARAMETER Prerelease
    Mark the release as a pre-release.

.PARAMETER LocalBuild
    Build installers locally without creating a tag or publishing a GitHub release.
    Updater artifacts are disabled by default in this mode, so no signing key is required.

.PARAMETER UpdaterArtifacts
    Create signed updater artifacts during a local build. If TAURI_SIGNING_PRIVATE_KEY
    is not set, the script loads the key from SigningKeyPath.

.PARAMETER SigningKeyPath
    Local updater signing key file used when signed updater artifacts are requested.

.PARAMETER DryRun
    Print what would happen without making any changes.

.EXAMPLE
    .\scripts\create-release.ps1 -Bump Patch
    Bump patch version, build, and publish release.

.EXAMPLE
    .\scripts\create-release.ps1 -Version 1.0.0 -Draft
    Release v1.0.0 as a draft without bumping.

.EXAMPLE
    .\scripts\create-release.ps1 -LocalBuild
    Build local installers without GitHub release publishing or updater signing.

.EXAMPLE
    .\scripts\create-release.ps1 -LocalBuild -UpdaterArtifacts
    Build local installers and signed updater artifacts using the local updater key file.
#>

param(
    [ValidateSet('Major','Minor','Patch','')]
    [string]$Bump       = '',
    [string]$Version    = '',
    [switch]$Draft,
    [switch]$Prerelease,
    [switch]$LocalBuild,
    [switch]$UpdaterArtifacts,
    [string]$SigningKeyPath = (Join-Path $env:USERPROFILE '.tauri\localvoice-updater.key'),
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir   = Join-Path $ScriptDir '..'
$script:SigningKeyLoadedFromFile = $false
$script:TempTauriConfigPath = $null

function Clear-LoadedUpdaterSigningKey {
    if ($script:SigningKeyLoadedFromFile) {
        Remove-Item Env:\TAURI_SIGNING_PRIVATE_KEY -ErrorAction SilentlyContinue
        $script:SigningKeyLoadedFromFile = $false
    }
}

function Clear-TemporaryTauriConfig {
    if ($script:TempTauriConfigPath -and (Test-Path $script:TempTauriConfigPath)) {
        Remove-Item $script:TempTauriConfigPath -Force -ErrorAction SilentlyContinue
        $script:TempTauriConfigPath = $null
    }
}

function Clear-TransientState {
    Clear-LoadedUpdaterSigningKey
    Clear-TemporaryTauriConfig
}

function Write-Step { param([string]$m) Write-Host "" ; Write-Host "==> $m" -ForegroundColor Cyan }
function Write-Ok   { param([string]$m) Write-Host "  [OK] $m" -ForegroundColor Green }
function Write-Fail { param([string]$m) Clear-TransientState; Write-Host "  [FAIL] $m" -ForegroundColor Red; exit 1 }
function Write-Info { param([string]$m) Write-Host "  $m" -ForegroundColor Gray }
function Write-Dry  { param([string]$m) Write-Host "  [DRY-RUN] $m" -ForegroundColor Yellow }

function Invoke-Cmd {
    param([string]$Cmd, [string[]]$CmdArgs)
    if ($DryRun) { Write-Dry "$Cmd $($CmdArgs -join ' ')"; return }
    & $Cmd @CmdArgs
    if ($LASTEXITCODE -ne 0) { Write-Fail "'$Cmd $($CmdArgs -join ' ')' failed (exit $LASTEXITCODE)" }
}

function Get-BumpedVersion {
    param([string]$v, [string]$type)
    $p  = $v.Split('.')
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
    $escapedOld = [regex]::Escape($Old)
    $c = Get-Content $File -Raw
    $c = $c -replace "(`"version`"\s*:\s*)`"$escapedOld`"", "`$1`"$New`""
    $c = $c -replace "(?m)^(version\s*=\s*)`"$escapedOld`"", "`$1`"$New`""
    if (-not $DryRun) {
        # Use UTF8NoBOM to avoid BOM that breaks JSON parsers
        $enc = New-Object System.Text.UTF8Encoding $false
        [System.IO.File]::WriteAllText((Resolve-Path $File).Path, $c, $enc)
    }
    Write-Info "Updated $(Split-Path $File -Leaf): $Old -> $New"
}

function Ensure-UpdaterSigningKey {
    if (Test-Path Env:\TAURI_SIGNING_PRIVATE_KEY) {
        Write-Ok "Updater signing key available from environment"
        return
    }

    if ($DryRun) {
        Write-Dry "Would load updater signing key from $SigningKeyPath"
        return
    }

    if (-not (Test-Path $SigningKeyPath)) {
        Write-Fail "Updater artifacts require TAURI_SIGNING_PRIVATE_KEY or a key file at $SigningKeyPath. Use -LocalBuild without -UpdaterArtifacts to skip updater signing."
    }

    $env:TAURI_SIGNING_PRIVATE_KEY = Get-Content $SigningKeyPath -Raw
    $script:SigningKeyLoadedFromFile = $true
    Write-Ok "Updater signing key loaded from local key file"
}

function Ensure-ParakeetSidecars {
    $setupScript = Join-Path $ScriptDir 'setup-parakeet-cpp.ps1'
    if ($DryRun) {
        Write-Dry "$setupScript"
        return
    }
    if (-not (Test-Path $setupScript)) {
        Write-Fail "Missing Parakeet setup script: $setupScript"
    }
    & $setupScript
    if ($LASTEXITCODE -ne 0) {
        Write-Fail "Parakeet sidecar setup failed."
    }
}

function Get-TauriBuildArgs {
    param([bool]$CreateUpdaterArtifacts)

    $args = [System.Collections.Generic.List[string]]::new()
    $args.Add('tauri')
    $args.Add('build')

    if (-not $CreateUpdaterArtifacts) {
        $configPath = Join-Path ([System.IO.Path]::GetTempPath()) "localvoice-tauri-local-build-$PID.json"
        if (-not $DryRun) {
            $enc = New-Object System.Text.UTF8Encoding $false
            [System.IO.File]::WriteAllText($configPath, '{"bundle":{"createUpdaterArtifacts":false}}', $enc)
            $script:TempTauriConfigPath = $configPath
        }
        $args.Add('--config')
        $args.Add($configPath)
    }

    return $args.ToArray()
}

# --- Pre-flight checks --------------------------------------------------------

Write-Step "Checking prerequisites"

$requiredTools = if ($LocalBuild) { @('pnpm') } else { @('git', 'pnpm', 'gh') }

foreach ($tool in $requiredTools) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        Write-Fail "$tool not found in PATH."
    }
    Write-Ok "$tool found"
}

if ($LocalBuild) {
    Write-Info "Local build mode: skipping GitHub auth, clean working tree check, tag creation, and release publishing."
} else {
    $authCheck = gh auth status 2>&1
    if ($LASTEXITCODE -ne 0) { Write-Fail "gh CLI not authenticated. Run: gh auth login" }
    Write-Ok "gh authenticated"

    $dirty = git status --porcelain
    if ($dirty) { Write-Fail "Working tree has uncommitted changes. Commit or stash first." }
    Write-Ok "Working tree clean"
}

# --- Resolve version ----------------------------------------------------------

Write-Step "Resolving version"

$pkgJson        = Join-Path $RootDir 'package.json'
$currentVersion = (Get-Content $pkgJson -Raw | ConvertFrom-Json).version

if ($Version -ne '') {
    $releaseVersion = $Version
    Write-Info "Using explicit version: $releaseVersion"
} elseif ($Bump -ne '') {
    $releaseVersion = Get-BumpedVersion -v $currentVersion -type $Bump
    Write-Info "Bumping ${Bump}: $currentVersion -> $releaseVersion"
} else {
    $releaseVersion = $currentVersion
    Write-Info "Using current version: $releaseVersion"
}

$tag = "v$releaseVersion"

if ($LocalBuild -and $releaseVersion -ne $currentVersion) {
    Write-Fail "Local builds use the checked-out version ($currentVersion). Run the version bump separately or use release mode for $releaseVersion."
}

if (-not $LocalBuild) {
    $existingTag = git ls-remote --tags origin "refs/tags/$tag" 2>&1
    if ($existingTag -match [regex]::Escape($tag)) {
        Write-Fail "Tag $tag already exists on remote. Use a different version."
    }
}

# --- Bump version files -------------------------------------------------------

if ((-not $LocalBuild) -and $releaseVersion -ne $currentVersion) {
    Write-Step "Updating version files to $releaseVersion"

    Set-VersionInFile -File $pkgJson -Old $currentVersion -New $releaseVersion
    Set-VersionInFile -File (Join-Path $RootDir 'src-tauri\tauri.conf.json') -Old $currentVersion -New $releaseVersion
    Set-VersionInFile -File (Join-Path $RootDir 'src-tauri\Cargo.toml')      -Old $currentVersion -New $releaseVersion

    if (-not $DryRun) {
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

# --- Create and push tag ------------------------------------------------------

if (-not $LocalBuild) {
    Write-Step "Creating tag $tag"
    Invoke-Cmd git @('tag', '-a', $tag, '-m', "Release $tag")
    Invoke-Cmd git @('push', 'origin', $tag)
    Write-Ok "Tag $tag pushed"
}

# --- Build --------------------------------------------------------------------

Write-Step "Building Tauri app (this takes a few minutes)"
$createUpdaterArtifacts = (-not $LocalBuild) -or $UpdaterArtifacts
if ($createUpdaterArtifacts) {
    Ensure-UpdaterSigningKey
} else {
    Write-Info "Updater artifacts disabled for local build; no updater signing key is required."
}

Write-Step "Preparing Parakeet sidecars"
Ensure-ParakeetSidecars

$buildArgs = Get-TauriBuildArgs -CreateUpdaterArtifacts $createUpdaterArtifacts
Push-Location $RootDir
Invoke-Cmd pnpm $buildArgs
Pop-Location
Write-Ok "Build complete"

# --- Collect artifacts --------------------------------------------------------

Write-Step "Collecting build artifacts"

$bundleDir = Join-Path $RootDir 'src-tauri\target\release\bundle'
$artifacts = @(
    Get-ChildItem "$bundleDir\msi"  -Filter "*$releaseVersion*.msi" -ErrorAction SilentlyContinue
    Get-ChildItem "$bundleDir\nsis" -Filter "*$releaseVersion*.exe" -ErrorAction SilentlyContinue
)

if ($artifacts.Count -eq 0) {
    if ($DryRun) {
        Write-Dry "Would collect artifacts for version $releaseVersion in $bundleDir"
    } else {
        Write-Fail "No artifacts found for version $releaseVersion in $bundleDir"
    }
}

foreach ($f in $artifacts) {
    Write-Info "$($f.Name)  ($([math]::Round($f.Length/1MB, 1)) MB)"
}

# --- Publish GitHub release ---------------------------------------------------

if (-not $LocalBuild) {
    Write-Step "Publishing GitHub release $tag"

    $notes = "## LocalVoice $tag`n`n" +
             "### Installation`n" +
             "Download the .msi (Windows Installer) or the -setup.exe (NSIS installer) below.`n`n" +
             "Whisper and Parakeet sidecars are bundled. Transcription models are not bundled - download them from the Models page inside the app. .nemo models require an optional local NVIDIA NeMo Python runtime.`n`n" +
             "### Changes`n" +
             "https://github.com/iptoux/localvoice/commits/$tag"

    $ghArgs = [System.Collections.Generic.List[string]]@(
        'release', 'create', $tag,
        '--title', "LocalVoice $tag",
        '--notes', $notes
    )

    if ($Draft)      { $ghArgs.Add('--draft') }
    if ($Prerelease) { $ghArgs.Add('--prerelease') }
    foreach ($f in $artifacts) { $ghArgs.Add($f.FullName) }

    Invoke-Cmd gh $ghArgs.ToArray()
}

Clear-TransientState

# --- Done ---------------------------------------------------------------------

Write-Host ""
Write-Host "======================================" -ForegroundColor Magenta
if ($DryRun) {
    Write-Host "  DRY-RUN complete - no changes made." -ForegroundColor Yellow
} elseif ($LocalBuild) {
    Write-Host "  Local build complete for $tag." -ForegroundColor Green
    Write-Host "  Artifacts: $bundleDir" -ForegroundColor Cyan
} else {
    Write-Host "  Release $tag published!" -ForegroundColor Green
    Write-Host "  https://github.com/iptoux/localvoice/releases/tag/$tag" -ForegroundColor Cyan
}
Write-Host "======================================" -ForegroundColor Magenta
