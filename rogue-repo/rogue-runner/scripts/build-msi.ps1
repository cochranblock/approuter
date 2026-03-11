# Copyright (c) 2026 The Cochran Block. All rights reserved.
# Build Rogue Runner MSI installer. Run on Windows. Requires: WiX Toolset, Rust.
# If exe from WSL: copy target/x86_64-pc-windows-gnu/release/rogue-runner.exe here first.
param(
    [switch]$Sign,
    [string]$CertThumbprint
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$RogueRunner = Join-Path $RepoRoot "rogue-runner"
$WixDir = Join-Path $RogueRunner "wix"
$TargetDir = Join-Path $RepoRoot "target"

# Build exe if not present (Windows native)
$ExeGnu = Join-Path $TargetDir "x86_64-pc-windows-gnu\release\rogue-runner.exe"
$ExeRelease = Join-Path $TargetDir "release\rogue-runner.exe"
$SourceExe = $null

if (Test-Path $ExeGnu) {
    $SourceExe = $ExeGnu
} elseif (Test-Path $ExeRelease) {
    $SourceExe = $ExeRelease
} else {
    Write-Host "Building rogue-runner..."
    Push-Location $RepoRoot
    cargo build -p rogue-runner --release --target x86_64-pc-windows-gnu 2>$null
    if (-not $?) { cargo build -p rogue-runner --release }
    Pop-Location
    $SourceExe = if (Test-Path $ExeGnu) { $ExeGnu } else { $ExeRelease }
}

$SourceDir = Split-Path -Parent $SourceExe
Write-Host "Using exe: $SourceExe"

# Run WiX (candle + light)
$WixObjDir = Join-Path $TargetDir "wix\build"
$WixOutDir = Join-Path $TargetDir "wix"
New-Item -ItemType Directory -Force -Path $WixObjDir | Out-Null
New-Item -ItemType Directory -Force -Path $WixOutDir | Out-Null

$Candle = "candle.exe"
$Light = "light.exe"
$WixPath = $env:WIX
if (-not $WixPath) {
    $WixPath = "C:\Program Files (x86)\WiX Toolset v3.11\bin"
    if (-not (Test-Path $WixPath)) { $WixPath = "C:\Program Files (x86)\WiX Toolset v4\bin" }
}
$CandleExe = Join-Path $WixPath $Candle
$LightExe = Join-Path $WixPath $Light

if (-not (Test-Path $CandleExe)) {
    Write-Error "WiX Toolset not found. Install from https://wixtoolset.org/"
}

& $CandleExe -dSourceDir=$SourceDir -out $WixObjDir\ -arch x64 (Join-Path $WixDir "main.wxs")
if (-not $?) { exit 1 }

& $LightExe -out (Join-Path $WixOutDir "rogue-runner-windows-x64.msi") -b $SourceDir (Join-Path $WixObjDir "main.wixobj")
if (-not $?) { exit 1 }

$MsiPath = Join-Path $WixOutDir "rogue-runner-windows-x64.msi"
Write-Host "MSI: $MsiPath"

# Copy to downloads for embedding
$DownloadsDir = Join-Path $RepoRoot "rogue-repo\assets\downloads"
if (Test-Path $DownloadsDir) {
    Copy-Item $MsiPath (Join-Path $DownloadsDir "rogue-runner-windows-x64.msi") -Force
    Write-Host "Copied to $DownloadsDir"
}

if ($Sign -and $CertThumbprint) {
    & signtool sign /sha1 $CertThumbprint /tr http://timestamp.digicert.com /td sha256 /fd sha256 $MsiPath
    Write-Host "Signed: $MsiPath"
}
