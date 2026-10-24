# Exports SVG sources to PNG with the pinned resvg (SP-013, Art Bible 8.3).
# Every source is validated before the exporter runs; outputs are rendered into a
# temporary folder, checked, and only then published together with the lock file.
param(
	[Parameter(Mandatory = $true)][string]$Manifest
)
$ErrorActionPreference = 'Stop'
Import-Module (Join-Path $PSScriptRoot 'lib\Assets.psm1') -Force

$m = Read-AssetManifest $Manifest
$version = Get-ResvgVersion
if ($version -ne $m.exporter.version) { throw "resvg $version found, manifest pins $($m.exporter.version)" }

$failed = $false
foreach ($a in $m.assets) {
	$src = Get-RepoPath $a.source
	if (-not (Test-Path $src)) { Write-Error "missing source $($a.source)" -ErrorAction Continue; $failed = $true; continue }
	foreach ($p in (Test-SvgSource $src $a)) { Write-Host "REJECT $($a.source): $p"; $failed = $true }
}
if ($failed) { Write-Host 'export aborted: no output was written'; exit 1 }

$tmp = Join-Path ([IO.Path]::GetTempPath()) ("omega-export-" + [guid]::NewGuid())
New-Item -ItemType Directory $tmp | Out-Null
try {
	$entries = @()
	$i = 0
	foreach ($a in $m.assets) {
		$size = Get-ExportSize $a $m
		$out = Join-Path $tmp "$i.png"
		Invoke-Resvg (Get-RepoPath $a.source) $out $size[0] $size[1]
		$info = Get-PngInfo $out
		if ($info.Width -ne $size[0] -or $info.Height -ne $size[1] -or -not $info.Rgba8) {
			throw "invalid output for $($a.source): $($info.Width)x$($info.Height) rgba8=$($info.Rgba8)"
		}
		$entries += [pscustomobject]@{
			visual_id = $a.visual_id
			source = $a.source
			output = $a.output
			width = $size[0]
			height = $size[1]
			source_sha256 = Get-Sha256 (Get-RepoPath $a.source)
			output_sha256 = Get-Sha256 $out
			command = "resvg --skip-system-fonts --width $($size[0]) --height $($size[1]) <source> <output>"
		}
		$i++
	}
	$i = 0
	foreach ($e in $entries) {
		$dest = Get-RepoPath $e.output
		New-Item -ItemType Directory -Force (Split-Path $dest) | Out-Null
		Copy-Item (Join-Path $tmp "$i.png") $dest -Force
		$i++
	}
	$lock = [ordered]@{
		schema_version = 1
		generated_by = 'scripts/export-assets.ps1'
		exporter = [ordered]@{ name = 'resvg'; version = $version }
		assets = $entries
	}
	$json = $lock | ConvertTo-Json -Depth 6
	[IO.File]::WriteAllText((Get-LockPath $Manifest), ($json -replace "`r`n", "`n") + "`n", (New-Object Text.UTF8Encoding $false))
	Write-Host "exported $($entries.Count) assets with resvg $version"
} finally {
	Remove-Item -Recurse -Force $tmp
}
