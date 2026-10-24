# Shared helpers for the offline SVG pipeline (SP-013, Art Bible section 8).
Set-StrictMode -Version Latest

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path

function Get-RepoPath([string]$Relative) {
	Join-Path $RepoRoot ($Relative -replace '/', '\')
}

function Read-AssetManifest([string]$Path) {
	$full = if ([IO.Path]::IsPathRooted($Path)) { $Path } else { Get-RepoPath $Path }
	if (-not (Test-Path $full)) { throw "manifest not found: $Path" }
	$m = Get-Content -Raw -Encoding UTF8 $full | ConvertFrom-Json
	if ($m.schema_version -ne 1) { throw "unsupported manifest schema_version $($m.schema_version)" }
	$m
}

function Get-LockPath([string]$ManifestPath) {
	$full = if ([IO.Path]::IsPathRooted($ManifestPath)) { $ManifestPath } else { Get-RepoPath $ManifestPath }
	$full -replace '\.json$', '.lock.json'
}

function Get-Sha256([string]$File) {
	(Get-FileHash -Algorithm SHA256 $File).Hash.ToLowerInvariant()
}

# Rejects anything that could pull external data or run code before the exporter
# sees the file (SP-013-AC1). Returns a list of problems; empty means safe.
function Test-SvgSource([string]$File, $Asset) {
	$problems = @()
	$text = Get-Content -Raw -Encoding UTF8 $File
	$rules = @(
		@{ Pattern = '<script'; Reason = 'script element' },
		@{ Pattern = '<foreignObject'; Reason = 'foreignObject element' },
		@{ Pattern = '<image'; Reason = 'embedded or external image' },
		@{ Pattern = '<text'; Reason = 'live text (UI text stays live, not rasterized)' },
		@{ Pattern = '<!DOCTYPE|<!ENTITY'; Reason = 'DTD or entity declaration' },
		@{ Pattern = '\bon[a-z]+\s*='; Reason = 'event handler attribute' },
		@{ Pattern = '(xlink:)?href\s*=\s*"(?!#)'; Reason = 'reference to an undeclared resource' },
		@{ Pattern = 'url\(\s*(?!#)'; Reason = 'external url() reference' },
		@{ Pattern = '@import'; Reason = 'CSS import' }
	)
	foreach ($r in $rules) {
		if ($text -match $r.Pattern) { $problems += "$($r.Reason)" }
	}
	if ($text -notmatch 'viewBox\s*=\s*"\s*([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s*"') {
		$problems += 'missing explicit viewBox'
	} else {
		$vw = [double]$Matches[3]; $vh = [double]$Matches[4]
		if ($text -notmatch 'width\s*=\s*"([\d.]+)"' ) { $problems += 'missing width' }
		if ($text -notmatch 'height\s*=\s*"([\d.]+)"') { $problems += 'missing height' }
		$extent = Get-AssetExtent $Asset
		if ([math]::Abs($vw / $vh - $extent[0] / $extent[1]) -gt 1e-6) {
			$problems += "viewBox aspect $vw/$vh does not match catalog extent aspect (unintended distortion)"
		}
	}
	$problems
}

function Get-AssetExtent($Asset) {
	if ($Asset.PSObject.Properties.Name -contains 'logical_extent_lu') { return @([double]$Asset.logical_extent_lu[0], [double]$Asset.logical_extent_lu[1]) }
	@([double]$Asset.ui_size_px[0], [double]$Asset.ui_size_px[1])
}

# Export size: ceil(extent * texels_per_lu) for world sprites, explicit size for UI.
function Get-ExportSize($Asset, $Manifest) {
	if ($Asset.PSObject.Properties.Name -contains 'ui_size_px') { return @([int]$Asset.ui_size_px[0], [int]$Asset.ui_size_px[1]) }
	$density = if ($Asset.PSObject.Properties.Name -contains 'texels_per_lu') { [double]$Asset.texels_per_lu } else { [double]$Manifest.texels_per_lu }
	@([int][math]::Ceiling($Asset.logical_extent_lu[0] * $density), [int][math]::Ceiling($Asset.logical_extent_lu[1] * $density))
}

# Reads the PNG header: returns width, height and whether it is 8-bit RGBA.
function Get-PngInfo([string]$File) {
	$bytes = [IO.File]::ReadAllBytes($File)
	$sig = [byte[]](137, 80, 78, 71, 13, 10, 26, 10)
	for ($i = 0; $i -lt 8; $i++) { if ($bytes[$i] -ne $sig[$i]) { throw "$File is not a PNG" } }
	$be = { param($o) ([int]$bytes[$o] -shl 24) -bor ([int]$bytes[$o + 1] -shl 16) -bor ([int]$bytes[$o + 2] -shl 8) -bor [int]$bytes[$o + 3] }
	[pscustomobject]@{
		Width = & $be 16
		Height = & $be 20
		Rgba8 = ($bytes[24] -eq 8 -and $bytes[25] -eq 6)
	}
}

function Get-ResvgVersion {
	$cmd = Get-Command resvg -ErrorAction SilentlyContinue
	if (-not $cmd) { throw 'resvg not found on PATH; install with: cargo install resvg --version 0.48.1 --locked' }
	(& resvg --version).Trim()
}

function Invoke-Resvg([string]$Source, [string]$Output, [int]$Width, [int]$Height) {
	# Explicit size, no system fonts (sources have no text), transparent background by default.
	& resvg --skip-system-fonts --width $Width --height $Height $Source $Output
	if ($LASTEXITCODE -ne 0) { throw "resvg failed ($LASTEXITCODE) for $Source" }
}

Export-ModuleMember -Function * -Variable RepoRoot
