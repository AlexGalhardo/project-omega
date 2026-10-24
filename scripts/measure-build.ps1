# PERF-08 build baseline (performance plan): no-change build and single game-module
# edit rebuild, optionally a cold build. Writes a JSON report; no budget is implied.
# Usage: powershell -NoProfile -File scripts/measure-build.ps1 -Output PATH [-Cold] [-Runs 3]
param(
	[Parameter(Mandatory = $true)][string]$Output,
	[switch]$Cold,
	[int]$Runs = 3
)
$ErrorActionPreference = 'Stop'
Set-Location (Resolve-Path (Join-Path $PSScriptRoot '..'))
$touched = 'crates/omega_game/src/sim/mod.rs'
# cmd keeps cargo's stderr progress out of PowerShell's error stream.
$build = { cmd /c 'cargo build --locked -q -p omega_game --bin omega-game 2>&1' | Out-Null; if ($LASTEXITCODE -ne 0) { throw 'build failed' } }

function Measure-Seconds([scriptblock]$Block) {
	$sw = [Diagnostics.Stopwatch]::StartNew()
	& $Block
	$sw.Stop()
	[math]::Round($sw.Elapsed.TotalSeconds, 2)
}

$results = [ordered]@{}
if ($Cold) {
	cmd /c 'cargo clean 2>&1' | Out-Null
	$results.cold_seconds = @(Measure-Seconds $build)
}
& $build
$results.no_change_seconds = @(1..$Runs | ForEach-Object { Measure-Seconds $build })
$results.single_module_edit_seconds = @(1..$Runs | ForEach-Object {
	# Append and remove a comment so the file content returns to its original bytes.
	$original = [IO.File]::ReadAllText((Resolve-Path $touched))
	try {
		[IO.File]::WriteAllText((Resolve-Path $touched), $original + "`n// perf-08 touch $_`n")
		Measure-Seconds $build
	} finally {
		[IO.File]::WriteAllText((Resolve-Path $touched), $original)
	}
})
& $build
$report = [ordered]@{
	scenario = 'PERF-08'
	profile = 'dev (dependencies opt-level 2), MSVC default linker'
	target = 'omega_game --bin omega-game, client feature'
	commit = (git rev-parse HEAD)
	dirty = [bool](git status --porcelain)
	rustc = (rustc --version)
	cpu = $env:PROCESSOR_IDENTIFIER
	edited_file = $touched
	runs = $Runs
	results = $results
	not_measured = $(if ($Cold) { @() } else { @('cold_seconds (run with -Cold)') })
}
New-Item -ItemType Directory -Force (Split-Path $Output) | Out-Null
[IO.File]::WriteAllText($Output, (($report | ConvertTo-Json -Depth 5) -replace "`r`n", "`n") + "`n")
Write-Host ($report | ConvertTo-Json -Depth 5)
