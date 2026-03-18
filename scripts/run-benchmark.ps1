#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Run LocalVoice transcription benchmarks.

.DESCRIPTION
    Executes the run_transcription_benchmark Tauri command and reports timing metrics.
    Requires the LocalVoice app to be running with Tauri IPC enabled.

.PARAMETER Language
    Language code (de, en, auto). Defaults to settings default.

.PARAMETER ModelPath
    Optional explicit model path override.

.PARAMETER DurationMs
    Synthetic audio duration in milliseconds. Defaults to 1000.

.PARAMETER Iterations
    Number of benchmark iterations to run. Defaults to 1.

.EXAMPLE
    .\run-benchmark.ps1
    .\run-benchmark.ps1 -Language de -Iterations 3
#>

param(
    [string]$Language,
    [string]$ModelPath,
    [int]$DurationMs = 1000,
    [int]$Iterations = 1
)

$ErrorActionPreference = "Stop"

function Invoke-Benchmark {
    param(
        [string]$Lang,
        [string]$Model,
        [int]$Duration
    )

    $params = @{
        durationMs = $Duration
    }
    if ($Lang) { $params.language = $Lang }
    if ($Model) { $params.modelPath = $Model }

    $body = @{
        cmd = "run_transcription_benchmark"
        args = $params
    } | ConvertTo-Json -Depth 3

    $response = Invoke-RestMethod -Uri "http://localhost:9222/json/invoke" `
        -Method POST `
        -ContentType "application/json" `
        -Body $body `
        -TimeoutSec 120

    return $response
}

Write-Host "LocalVoice Transcription Benchmark" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan
Write-Host "Duration: ${DurationMs}ms | Iterations: $Iterations" -ForegroundColor Yellow
Write-Host ""

$results = @()
for ($i = 1; $i -le $Iterations; $i++) {
    Write-Host "Running iteration $i/$Iterations..." -NoNewline
    try {
        $result = Invoke-Benchmark -Lang $Language -Model $ModelPath -Duration $DurationMs
        $results += $result
        Write-Host " Done" -ForegroundColor Green
    }
    catch {
        Write-Host " Failed: $_" -ForegroundColor Red
        exit 1
    }
}

if ($results.Count -eq 0) {
    Write-Host "No results collected." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Results" -ForegroundColor Cyan
Write-Host "-------" -ForegroundColor Cyan
Write-Host ""

$avgMicToText = ($results | Measure-Object -Property micToTextMs -Average).Average
$avgWhisperInit = ($results | Measure-Object -Property whisperInitMs -Average).Average
$avgPostProcess = ($results | Measure-Object -Property postProcessingMs -Average).Average
$avgTotal = ($results | Measure-Object -Property totalTranscriptionMs -Average).Average

Write-Host "Model:         $($results[0].modelId)"
Write-Host "Language:      $($results[0].language)"
Write-Host "Audio Duration: $($results[0].audioDurationMs)ms @ $($results[0].audioSampleRate)Hz"
Write-Host ""
Write-Host "Average Timings (ms):" -ForegroundColor Yellow
Write-Host "  Mic-to-Text:    $($avgMicToText.ToString('F2'))"
Write-Host "  Whisper Init:   $($avgWhisperInit.ToString('F2'))"
Write-Host "  Post-Processing: $($avgPostProcess.ToString('F2'))"
Write-Host "  Total:          $($avgTotal.ToString('F2'))"
Write-Host ""

if ($Iterations -gt 1) {
    Write-Host "Per-Iteration Results:" -ForegroundColor Yellow
    for ($i = 0; $i -lt $results.Count; $i++) {
        $r = $results[$i]
        Write-Host "  Iteration $($i+1): Total=$($r.totalTranscriptionMs)ms | Init=$($r.whisperInitMs)ms | PostProc=$($r.postProcessingMs)ms"
    }
}

Write-Host ""
Write-Host "Text Output: `"$($results[0].textOutput)`"" -ForegroundColor DarkGray
