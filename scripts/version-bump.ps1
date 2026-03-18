#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Cross-platform monorepo version bump script.

.DESCRIPTION
    Increments the version number across all packages in the npm workspace.
    Supports semantic versioning with --major, --minor, --patch flags.

.PARAMETER BumpType
    The type of version bump: Major, Minor, or Patch (default: Patch)

.PARAMETER DryRun
    Preview changes without modifying files.

.EXAMPLE
    .\version-bump.ps1 -BumpType Patch
    Bumps patch version: 1.0.0 -> 1.0.1

.EXAMPLE
    .\version-bump.ps1 -BumpType Minor -DryRun
    Preview minor version bump without making changes.
#>

param(
    [Parameter(Position = 0)]
    [ValidateSet('Major', 'Minor', 'Patch')]
    [string]$BumpType = 'Patch',

    [Parameter()]
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Join-Path $ScriptDir '..'

function Get-Usage {
    @"
Usage: $([System.IO.Path]::GetFileName($MyInvocation.MyCommand.Path)) [OPTIONS]

Options:
    -BumpType <type>  Version bump type: Major, Minor, Patch (default: Patch)
    -DryRun           Preview changes without modifying files
    -Help             Show this help message

Shortcuts (can be combined):
    --major           Bump major version (e.g., 1.0.0 -> 2.0.0)
    --minor           Bump minor version (e.g., 1.0.0 -> 1.1.0)
    --patch           Bump patch version (e.g., 1.0.0 -> 1.0.1)
    --dry-run         Preview changes without modifying files

Examples:
    $([System.IO.Path]::GetFileName($MyInvocation.MyCommand.Path))           # 1.0.0 -> 1.0.1
    $([System.IO.Path]::GetFileName($MyInvocation.MyCommand.Path)) --minor    # 1.0.0 -> 1.1.0
    $([System.IO.Path]::GetFileName($MyInvocation.MyCommand.Path)) --major    # 1.0.0 -> 2.0.0
    $([System.IO.Path]::GetFileName($MyInvocation.MyCommand.Path)) --dry-run  # Preview only
"@
}

function Parse-Arguments {
    $script:BumpType = 'Patch'
    $script:DryRun = $false

    foreach ($arg in $args) {
        switch ($arg) {
            '--major' { $script:BumpType = 'Major' }
            '-M' { $script:BumpType = 'Major' }
            '--minor' { $script:BumpType = 'Minor' }
            '-m' { $script:BumpType = 'Minor' }
            '--patch' { $script:BumpType = 'Patch' }
            '-p' { $script:BumpType = 'Patch' }
            '--dry-run' { $script:DryRun = $true }
            '-n' { $script:DryRun = $true }
            '--help' { Get-Usage; exit 0 }
            '-h' { Get-Usage; exit 0 }
            '-?' { Get-Usage; exit 0 }
            default {
                if ($arg -match '^-+') {
                    Write-Warning "Unknown option: $arg"
                    Get-Usage
                    exit 1
                }
            }
        }
    }
}

function Test-Semver {
    param([string]$Version)
    return $Version -match '^\d+\.\d+\.\d+$'
}

function Invoke-BumpVersion {
    param(
        [string]$Version,
        [string]$Bump
    )

    $parts = $Version.Split('.')
    $major = [int]$parts[0]
    $minor = [int]$parts[1]
    $patch = [int]$parts[2]

    switch ($Bump) {
        'Major' { $major++; $minor = 0; $patch = 0 }
        'Minor' { $minor++; $patch = 0 }
        'Patch' { $patch++ }
    }

    return "$major.$minor.$patch"
}

function Get-PackageVersion {
    param([string]$FilePath)

    if (-not (Test-Path $FilePath)) {
        return ''
    }

    try {
        $content = Get-Content $FilePath -Raw | ConvertFrom-Json
        if ($null -ne $content.version) {
            return $content.version
        }
        return ''
    }
    catch {
        return ''
    }
}

function Update-PackageJson {
    param(
        [string]$FilePath,
        [string]$OldVersion,
        [string]$NewVersion
    )

    if (-not (Test-Path $FilePath)) {
        return $false
    }

    $content = Get-Content $FilePath -Raw

    $escapedOld = [regex]::Escape($OldVersion)
    $pattern = '("version":\s*)"' + $escapedOld + '"'
    $replacement = '$1"' + $NewVersion + '"'
    $content = $content -replace $pattern, $replacement

    $packages = @(
        'localvoice',
        '@automaker/utils',
        '@automaker/types',
        '@automaker/spec-parser',
        '@automaker/prompts',
        '@automaker/platform',
        '@automaker/model-resolver',
        '@automaker/git-utils',
        '@automaker/dependency-resolver'
    )

    foreach ($pkg in $packages) {
        $escapedPkg = [regex]::Escape($pkg)
        $pattern = '("' + $pkg + '":\s*)"' + $escapedOld + '"'
        $replacement = '$1"' + $NewVersion + '"'
        $content = $content -replace $pattern, $replacement
    }

    if (-not $DryRun) {
        $content | Set-Content $FilePath -NoNewline -Encoding UTF8
    }

    return $true
}

function Find-Packages {
    $packages = @()

    $searchDirs = @('apps', 'libs')
    foreach ($dir in $searchDirs) {
        $path = Join-Path $RootDir $dir
        if (Test-Path $path) {
            $found = Get-ChildItem -Path $path -Filter 'package.json' -Recurse -File -ErrorAction SilentlyContinue
            $packages += $found | Select-Object -ExpandProperty FullName
        }
    }

    $rootPkg = Join-Path $RootDir 'package.json'
    if (Test-Path $rootPkg) {
        $packages += $rootPkg
    }

    return ,$packages
}

function Main {
    Parse-Arguments @args

    $rootPkg = Join-Path $RootDir 'package.json'
    if (-not (Test-Path $rootPkg)) {
        Write-Error "Root package.json not found at $rootPkg"
        exit 1
    }

    $currentVersion = Get-PackageVersion -FilePath $rootPkg
    if (-not (Test-Semver $currentVersion)) {
        Write-Error "Invalid version in root package.json: $currentVersion"
        exit 1
    }

    $newVersion = Invoke-BumpVersion -Version $currentVersion -Bump $BumpType

    Write-Host "=== Version Bump Summary ==="
    Write-Host "Current version:  $currentVersion"
    Write-Host "New version:      $newVersion"
    Write-Host "Bump type:        $BumpType"
    Write-Host "Dry run:          $DryRun"
    Write-Host ""
    Write-Host "Changes:"

    $changesMade = 0
    $packages = Find-Packages

    foreach ($pkgJson in $packages) {
        $pkgVersion = Get-PackageVersion -FilePath $pkgJson

        if ([string]::IsNullOrEmpty($pkgVersion)) {
            continue
        }

        if ($pkgVersion -eq $currentVersion) {
            $relPath = $pkgJson.Replace($RootDir, '').TrimStart('\', '/')
            Write-Host "  [UPDATE] $relPath : $pkgVersion -> $newVersion"

            if (-not $DryRun) {
                Update-PackageJson -FilePath $pkgJson -OldVersion $currentVersion -NewVersion $newVersion
            }
            $changesMade++
        }
    }

    if ($changesMade -eq 0) {
        Write-Host "  (no packages found with version $currentVersion)"
    }

    Write-Host ""
    Write-Host "Total packages updated: $changesMade"

    if ($DryRun) {
        Write-Host ""
        Write-Host "[DRY-RUN] No files were modified."
    }
}

Main @args
