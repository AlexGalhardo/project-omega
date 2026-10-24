# Validates exported assets against their manifest and lock file (SP-013).
# -Reproduce re-renders every source into a temporary folder and requires
# byte-identical output (SP-013-AC3).
param(
	[Parameter(Mandatory = $true)][string]$Manifest,
	[switch]$Reproduce
)
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSScriptRoot 'lib\Assets.psm1') -Force

$m = Read-AssetManifest $Manifest
$lockPath = Get-LockPath $Manifest
if (-not (Test-Path $lockPath)) { Write-Host "missing lock file $lockPath; run export-assets.ps1"; exit 1 }
$lock = Get-Content -Raw -Encoding UTF8 $lockPath | ConvertFrom-Json

$errors = @()
if ($lock.assets.Count -ne $m.assets.Count) { $errors += "lock has $($lock.assets.Count) entries, manifest $($m.assets.Count)" }
for ($i = 0; $i -lt [math]::Min($lock.assets.Count, $m.assets.Count); $i++) {
	$a = $m.assets[$i]; $e = $lock.assets[$i]
	$src = Get-RepoPath $a.source
	foreach ($p in (Test-SvgSource $src $a)) { $errors += "$($a.source): $p" }
	if ($e.output -ne $a.output) { $errors += "entry $i output mismatch"; continue }
	if ((Get-Sha256 $src) -ne $e.source_sha256) { $errors += "$($a.source) changed since export; run export-assets.ps1" }
	$out = Get-RepoPath $a.output
	if (-not (Test-Path $out)) { $errors += "missing output $($a.output)"; continue }
	$size = Get-ExportSize $a $m
	$info = Get-PngInfo $out
	if ($info.Width -ne $size[0] -or $info.Height -ne $size[1]) { $errors += "$($a.output) is $($info.Width)x$($info.Height), expected $($size[0])x$($size[1])" }
	if (-not $info.Rgba8) { $errors += "$($a.output) is not 8-bit RGBA" }
	if ((Get-Sha256 $out) -ne $e.output_sha256) { $errors += "$($a.output) hash differs from lock (hand edit?)" }
}

if ($Reproduce -and $errors.Count -eq 0) {
	$tmp = Join-Path ([IO.Path]::GetTempPath()) ("omega-repro-" + [guid]::NewGuid())
	New-Item -ItemType Directory $tmp | Out-Null
	try {
		for ($i = 0; $i -lt $m.assets.Count; $i++) {
			$a = $m.assets[$i]; $size = Get-ExportSize $a $m
			$out = Join-Path $tmp "$i.png"
			Invoke-Resvg (Get-RepoPath $a.source) $out $size[0] $size[1]
			if ((Get-Sha256 $out) -ne $lock.assets[$i].output_sha256) { $errors += "$($a.output) is not reproducible" }
		}
	} finally {
		Remove-Item -Recurse -Force $tmp
	}
}

if ($errors.Count -gt 0) {
	$errors | ForEach-Object { Write-Host "INVALID $_" }
	exit 1
}
Write-Host "valid: $($m.assets.Count) assets$(if ($Reproduce) { ', reproducible' })"
