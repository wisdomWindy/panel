# Fetches the shared LGPL FFmpeg build that is bundled with the installer.
#
# The binaries are never committed: this script (or the release workflow step)
# populates src-tauri/binaries/ffmpeg before packaging, because
# tauri.conf.json bundles that directory as a resource.
#
# Usage:
#   pwsh -NoProfile -File scripts/fetch-ffmpeg.ps1 [-Force]
param(
    [switch]$Force,
    [string]$Destination = '',
    [string]$SourceUrl = 'https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-lgpl-shared.zip'
)

$ErrorActionPreference = 'Stop'
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

if (-not $Destination) {
    $packageRoot = Split-Path $PSScriptRoot -Parent
    $Destination = Join-Path $packageRoot 'src-tauri/binaries/ffmpeg'
}

$executable = Join-Path $Destination 'ffmpeg.exe'
if ((Test-Path $executable) -and -not $Force) {
    Write-Host "ffmpeg.exe already present in $Destination (use -Force to refresh)"
    exit 0
}

New-Item -ItemType Directory -Path $Destination -Force | Out-Null
$staging = Join-Path ([System.IO.Path]::GetTempPath()) ('ffmpeg-bundle-' + [guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $staging -Force | Out-Null

try {
    $archive = Join-Path $staging 'ffmpeg.zip'
    Write-Host "downloading $SourceUrl"
    Invoke-WebRequest -Uri $SourceUrl -OutFile $archive -UseBasicParsing
    Write-Host ('downloaded {0:N1} MB' -f ((Get-Item $archive).Length / 1MB))

    $extracted = Join-Path $staging 'extracted'
    New-Item -ItemType Directory -Path $extracted -Force | Out-Null
    tar -xf $archive -C $extracted
    if ($LASTEXITCODE -ne 0) { throw 'tar could not extract the FFmpeg archive' }

    $bin = Get-ChildItem $extracted -Recurse -Directory -Filter 'bin' |
        Where-Object { Test-Path (Join-Path $_.FullName 'ffmpeg.exe') } |
        Select-Object -First 1
    if (-not $bin) { throw 'the archive does not contain a bin/ffmpeg.exe' }

    # The shared build is ffmpeg.exe plus the av*/sw* DLLs; they must stay
    # side by side or the executable will not start. ffplay and ffprobe are
    # dropped to keep the installer small: the app only shells out to ffmpeg.
    Copy-Item (Join-Path $bin.FullName 'ffmpeg.exe') $Destination -Force
    Copy-Item (Join-Path $bin.FullName '*.dll') $Destination -Force

    $license = Get-ChildItem $extracted -Recurse -File -Filter 'LICENSE*' | Select-Object -First 1
    if ($license) {
        Copy-Item $license.FullName (Join-Path $Destination 'LICENSE.txt') -Force
    }

    if (-not (Test-Path $executable)) { throw 'ffmpeg.exe is missing after extraction' }
    $libraries = (Get-ChildItem $Destination -Filter '*.dll' | Measure-Object).Count
    Write-Host "bundled ffmpeg.exe with $libraries shared libraries into $Destination"
}
finally {
    Remove-Item $staging -Recurse -Force -ErrorAction SilentlyContinue
}
