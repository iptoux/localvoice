# LocalVoice Development Environment Bootstrap Script
# Run this script to set up the development environment on Windows

param(
    [switch]$SkipWhisper,
    [switch]$SkipVerification
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$REPO_ROOT = Split-Path -Parent $PSScriptRoot
$WHISPER_URL = "https://github.com/ggerganov/whisper.cpp/releases/download/v1.7.1/whisper-bin-win-x64.zip"
$WHISPER_DIR = Join-Path $REPO_ROOT "src-tauri\binaries"

function Write-Step {
    param([string]$Message)
    Write-Host "`n==> $Message" -ForegroundColor Cyan
}

function Write-Success {
    param([string]$Message)
    Write-Host "[OK] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Fail {
    param([string]$Message)
    Write-Host "[FAIL] $Message" -ForegroundColor Red
}

function Test-Prerequisite {
    param([string]$Name, [string]$Command, [string]$VersionArg, [string]$MinVersion)
    
    Write-Host "Checking for $Name..." -NoNewline
    $result = Get-Command $Command -ErrorAction SilentlyContinue
    if (-not $result) {
        Write-Fail "$Name not found. Please install $Name first."
        return $false
    }
    
    if ($VersionArg) {
        $versionOutput = & $Command $VersionArg 2>&1
        $version = ($versionOutput | Out-String).Trim() -replace '[^0-9.]', ''
        if ($version -lt $MinVersion) {
            Write-Fail "$Name version $version is below minimum $MinVersion"
            return $false
        }
    }
    
    Write-Success "$Name found"
    return $true
}

Write-Host "========================================" -ForegroundColor Magenta
Write-Host "  LocalVoice Dev Environment Bootstrap" -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta

# Check prerequisites
Write-Step "Checking prerequisites..."

$nodeOk = Test-Prerequisite "Node.js" "node" "-v" "20"
$rustOk = Test-Prerequisite "Rust" "rustc" "--version" "1.77"
$pnpmOk = Get-Command "pnpm" -ErrorAction SilentlyContinue

if (-not $pnpmOk) {
    Write-Host "Checking for npm..." -NoNewline
    $npmOk = Get-Command "npm" -ErrorAction SilentlyContinue
    if ($npmOk) {
        Write-Success "npm found"
    } else {
        Write-Fail "npm not found. Cannot install pnpm."
        exit 1
    }
} else {
    Write-Success "pnpm found"
}

if (-not ($nodeOk -and $rustOk)) {
    Write-Fail "Missing prerequisites. Please install the required tools."
    exit 1
}

# Install pnpm if not present
if (-not $pnpmOk) {
    Write-Step "Installing pnpm..."
    npm install -g pnpm
    if ($LASTEXITCODE -ne 0) {
        Write-Fail "Failed to install pnpm"
        exit 1
    }
    Write-Success "pnpm installed"
}

# Install Node.js dependencies
Write-Step "Installing Node.js dependencies..."
Set-Location $REPO_ROOT
pnpm install
if ($LASTEXITCODE -ne 0) {
    Write-Fail "Failed to install dependencies"
    exit 1
}
Write-Success "Dependencies installed"

# Download whisper.cpp binaries
if (-not $SkipWhisper) {
    Write-Step "Downloading whisper.cpp binaries..."
    
    if (Test-Path $WHISPER_DIR) {
        $existing = Get-ChildItem $WHISPER_DIR -Filter "whisper-*.exe" -ErrorAction SilentlyContinue
        if ($existing) {
            Write-Success "whisper.cpp binaries already exist"
        }
    }
    
    if (-not (Test-Path $WHISPER_DIR) -or -not (Get-ChildItem $WHISPER_DIR -Filter "whisper-*.exe" -ErrorAction SilentlyContinue)) {
        $tempZip = Join-Path $env:TEMP "whisper-bin-win-x64.zip"
        
        Write-Host "Downloading from $WHISPER_URL..."
        try {
            Invoke-WebRequest -Uri $WHISPER_URL -OutFile $tempZip -UseBasicParsing
            Write-Host "Extracting..."
            Expand-Archive -Path $tempZip -DestinationPath $WHISPER_DIR -Force
            Remove-Item $tempZip -Force
            Write-Success "whisper.cpp binaries downloaded and extracted"
        } catch {
            Write-Warn "Failed to download whisper.cpp binaries: $_"
            Write-Host "  You can manually download from: $WHISPER_URL"
            Write-Host "  Extract to: $WHISPER_DIR"
        }
    }
}

# Verify Tauri CLI is installed
Write-Step "Checking Tauri CLI..."
$tauriCmd = Get-Command "pnpm" -ErrorAction SilentlyContinue
if ($tauriCmd) {
    $tauriCheck = pnpm tauri --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Tauri CLI v$tauriCheck installed"
    } else {
        Write-Step "Installing Tauri CLI..."
        pnpm add -D @tauri-apps/cli
        if ($LASTEXITCODE -ne 0) {
            Write-Warn "Failed to install Tauri CLI. Run 'pnpm tauri dev' to install it."
        } else {
            Write-Success "Tauri CLI installed"
        }
    }
}

# Install VS Code extensions
Write-Step "Checking VS Code extensions..."
$vscodeExtPath = Join-Path $env:APPDATA "Code\User\extensions.json"
if (Test-Path (Split-Path $vscodeExtPath)) {
    Write-Host "Recommended extensions are defined in .vscode/extensions.json"
    Write-Host "  - tauri-apps.tauri-vscode"
    Write-Host "  - rust-lang.rust-analyzer"
    Write-Success "VS Code extensions listed in .vscode/extensions.json"
} else {
    Write-Warn "VS Code not detected. Extensions listed in .vscode/extensions.json"
}

# Verify installation
if (-not $SkipVerification) {
    Write-Step "Verifying installation..."
    
    Write-Host "Checking Rust compilation..."
    Set-Location (Join-Path $REPO_ROOT "src-tauri")
    cargo check 2>&1 | Out-Null
    if ($LASTEXITCODE -eq 0) {
        Write-Success "Rust code compiles successfully"
    } else {
        Write-Warn "Rust compilation check had issues. This may be normal on first run."
    }
    
    Set-Location $REPO_ROOT
}

Write-Host "`n========================================" -ForegroundColor Magenta
Write-Host "  Bootstrap Complete!" -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta
Write-Host "`nNext steps:"
Write-Host "  1. Run 'pnpm tauri dev' to start development"
Write-Host "  2. If whisper binaries are missing, copy them to src-tauri\binaries\"
Write-Host "  3. See docs/dev/index.md for developer documentation"
Write-Host ""
