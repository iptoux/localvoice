#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Prepare the Windows parakeet.cpp CLI, streaming worker, and runtime DLLs.

.DESCRIPTION
    Mirrors the Windows path of .github/actions/setup-parakeet-cpp for local
    Tauri builds. The script is idempotent: if the CLI, worker, and required
    runtime DLLs already exist, it only runs smoke checks.

.PARAMETER Force
    Re-download the CLI and rebuild the streaming worker even when files exist.

.PARAMETER SkipSmokeTest
    Skip CLI and worker smoke tests after setup.
#>

param(
    [switch]$Force,
    [switch]$SkipSmokeTest
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Resolve-Path (Join-Path $ScriptDir "..")
$BinariesDir = Join-Path $RootDir "src-tauri\binaries"
$RuntimeDir = Join-Path $RootDir "src-tauri\parakeet-runtime"
$WorkerSourceDir = Join-Path $RootDir "src-tauri\sidecars\parakeet-stream-worker"

$ParakeetTag = "v0.3.2"
$ParakeetAsset = "parakeet-v0.3.2-bin-win-cpu-x64.zip"
$ParakeetSha256 = "71bceab8cd2a27ada6abb1234e6b0b81dc043964345219492f0d1d8522517a2b"
$ParakeetUrl = "https://github.com/mudler/parakeet.cpp/releases/download/$ParakeetTag/$ParakeetAsset"

$ParakeetCli = Join-Path $BinariesDir "parakeet-cli-x86_64-pc-windows-msvc.exe"
$WorkerBin = Join-Path $BinariesDir "parakeet-stream-worker-x86_64-pc-windows-msvc.exe"
$RuntimeDllsToCopy = @("ggml.dll", "ggml-base.dll", "ggml-cpu.dll", "parakeet.dll")

function Write-Step { param([string]$Message) Write-Host ""; Write-Host "==> $Message" -ForegroundColor Cyan }
function Write-Ok { param([string]$Message) Write-Host "  [OK] $Message" -ForegroundColor Green }
function Write-Info { param([string]$Message) Write-Host "  $Message" -ForegroundColor Gray }
function Write-Fail { param([string]$Message) Write-Host "  [FAIL] $Message" -ForegroundColor Red; exit 1 }

function Invoke-External {
    param([string]$Command, [string[]]$Arguments)
    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        Write-Fail "'$Command $($Arguments -join ' ')' failed with exit code $LASTEXITCODE"
    }
}

function Invoke-CapturedProcess {
    param(
        [string]$FileName,
        [string]$Arguments = "",
        [string]$PathPrefix = ""
    )

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $FileName
    $psi.Arguments = $Arguments
    $psi.UseShellExecute = $false
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    if ($PathPrefix) {
        $psi.EnvironmentVariables["PATH"] = "$PathPrefix;$($psi.EnvironmentVariables["PATH"])"
    }

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $psi
    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()

    return [pscustomobject]@{
        ExitCode = $process.ExitCode
        Output = "$stdout`n$stderr"
    }
}

function Test-CommandAvailable {
    param([string]$Command)
    if (-not (Get-Command $Command -ErrorAction SilentlyContinue)) {
        return $false
    }
    return $true
}

function Download-ParakeetCli {
    if ((Test-Path $ParakeetCli) -and (-not $Force)) {
        Write-Ok "parakeet.cpp CLI already exists"
        return
    }

    Write-Step "Downloading parakeet.cpp CLI"
    New-Item -ItemType Directory -Force -Path $BinariesDir | Out-Null

    $zip = Join-Path $env:TEMP $ParakeetAsset
    $extractDir = Join-Path $env:TEMP "localvoice-parakeet-cli"

    Remove-Item $zip -Force -ErrorAction SilentlyContinue
    Remove-Item $extractDir -Recurse -Force -ErrorAction SilentlyContinue

    Invoke-WebRequest -Uri $ParakeetUrl -OutFile $zip -UseBasicParsing
    $actual = (Get-FileHash -Algorithm SHA256 -Path $zip).Hash.ToLowerInvariant()
    if ($actual -ne $ParakeetSha256) {
        Write-Fail "Checksum mismatch for $ParakeetAsset. Expected $ParakeetSha256, got $actual"
    }

    Expand-Archive -Path $zip -DestinationPath $extractDir -Force
    $cli = Get-ChildItem $extractDir -Recurse -Filter "parakeet-cli.exe" | Select-Object -First 1
    if (-not $cli) {
        Write-Fail "parakeet-cli.exe not found in $ParakeetAsset"
    }

    Copy-Item $cli.FullName $ParakeetCli -Force
    Remove-Item $zip -Force -ErrorAction SilentlyContinue
    Remove-Item $extractDir -Recurse -Force -ErrorAction SilentlyContinue
    Write-Ok "parakeet.cpp CLI downloaded and verified"
}

function Build-ParakeetWorker {
    if ((Test-Path $WorkerBin) -and (-not $Force)) {
        Write-Ok "Parakeet streaming worker already exists"
        Copy-ExistingRuntimeDlls
        return
    }

    Write-Step "Building Parakeet streaming worker"

    foreach ($tool in @("git", "cmake")) {
        if (-not (Test-CommandAvailable $tool)) {
            Write-Fail "$tool not found in PATH. Install it before building the Parakeet streaming worker."
        }
    }

    if (-not (Test-Path $WorkerSourceDir)) {
        Write-Fail "Worker source directory not found: $WorkerSourceDir"
    }

    New-Item -ItemType Directory -Force -Path $BinariesDir | Out-Null
    New-Item -ItemType Directory -Force -Path $RuntimeDir | Out-Null

    $src = Join-Path $env:TEMP "localvoice-parakeet-src-$PID"
    $build = Join-Path $env:TEMP "localvoice-parakeet-build-$PID"
    $workerDst = Join-Path $src "examples\localvoice-stream-worker"

    Remove-Item $src -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item $build -Recurse -Force -ErrorAction SilentlyContinue

    try {
        Invoke-External git @(
            "clone",
            "--depth", "1",
            "--recurse-submodules",
            "--shallow-submodules",
            "--branch", $ParakeetTag,
            "https://github.com/mudler/parakeet.cpp.git",
            $src
        )

        Copy-Item $WorkerSourceDir $workerDst -Recurse
        Add-Content -Path (Join-Path $src "CMakeLists.txt") -Value "add_subdirectory(examples/localvoice-stream-worker)"

        Invoke-External cmake @(
            "-S", $src,
            "-B", $build,
            "-DCMAKE_BUILD_TYPE=Release",
            "-DBUILD_SHARED_LIBS=OFF",
            "-DGGML_NATIVE=OFF",
            "-DGGML_AVX2=OFF",
            "-DGGML_FMA=OFF",
            "-DGGML_F16C=OFF",
            "-DGGML_BMI2=OFF",
            "-DPARAKEET_BUILD_CLI=OFF",
            "-DPARAKEET_BUILD_SERVER=OFF"
        )
        Invoke-External cmake @("--build", $build, "--config", "Release", "--target", "parakeet-stream-worker", "--parallel")

        $worker = Get-ChildItem $build -Recurse -Filter "parakeet-stream-worker.exe" | Select-Object -First 1
        if (-not $worker) {
            Write-Fail "parakeet-stream-worker.exe not found after build"
        }
        Copy-Item $worker.FullName $WorkerBin -Force

        foreach ($dll in $RuntimeDllsToCopy) {
            $runtimeDll = Get-ChildItem $build -Recurse -Filter $dll -ErrorAction SilentlyContinue | Select-Object -First 1
            if ($runtimeDll) {
                Copy-Item $runtimeDll.FullName (Join-Path $RuntimeDir $dll) -Force
                Copy-Item $runtimeDll.FullName (Join-Path $BinariesDir $dll) -Force
                Write-Info "Copied $dll"
            }
        }
        Copy-ExistingRuntimeDlls

        Write-Ok "Parakeet streaming worker built"
    } finally {
        Remove-Item $src -Recurse -Force -ErrorAction SilentlyContinue
        Remove-Item $build -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Copy-ExistingRuntimeDlls {
    New-Item -ItemType Directory -Force -Path $RuntimeDir | Out-Null
    foreach ($dll in $RuntimeDllsToCopy) {
        $source = Join-Path $BinariesDir $dll
        $target = Join-Path $RuntimeDir $dll
        if ((Test-Path $source) -and (-not (Test-Path $target))) {
            Copy-Item $source $target -Force
            Write-Info "Copied existing $dll into parakeet-runtime"
        }
    }
}

function Test-ParakeetSidecars {
    if ($SkipSmokeTest) {
        Write-Info "Skipping Parakeet sidecar smoke tests"
        return
    }

    Write-Step "Smoke testing Parakeet sidecars"

    $cliOutput = Invoke-CapturedProcess -FileName $ParakeetCli
    if ($cliOutput.Output -notmatch "parakeet-cli transcribe") {
        Write-Host $cliOutput.Output
        Write-Fail "parakeet-cli smoke test failed"
    }

    $workerOutput = Invoke-CapturedProcess `
        -FileName $WorkerBin `
        -Arguments "--health" `
        -PathPrefix "$RuntimeDir;$BinariesDir"
    if ($workerOutput.ExitCode -ne 0 -or $workerOutput.Output -notmatch "parakeet.cpp streaming worker") {
        Write-Host $workerOutput.Output
        Write-Fail "parakeet-stream-worker smoke test failed"
    }

    Write-Ok "Parakeet sidecars passed smoke tests"
}

if ($env:OS -ne "Windows_NT") {
    Write-Fail "setup-parakeet-cpp.ps1 currently supports Windows only. Use CI/bootstrap on macOS and Linux."
}

Download-ParakeetCli
Build-ParakeetWorker
Test-ParakeetSidecars
