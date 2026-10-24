# Runs every local quality gate in the same order as CI (quality-gates.md).
# Usage: powershell -NoProfile -File scripts/check.ps1 [-SkipClient] [-SkipDeny]
param(
	[switch]$SkipClient,
	[switch]$SkipDeny
)
$ErrorActionPreference = 'Stop'
Set-Location (Resolve-Path (Join-Path $PSScriptRoot '..'))
$zero = '0' * 64

$steps = @(
	@{ Name = 'fmt'; Cmd = { cargo fmt --all -- --check } },
	@{ Name = 'clippy core'; Cmd = { cargo clippy --workspace --no-default-features --all-targets --locked -- -D warnings } },
	@{ Name = 'test core'; Cmd = { cargo test --workspace --no-default-features --locked } },
	@{ Name = 'content full'; Cmd = { cargo run -q -p omega_game --no-default-features --locked --bin omega-content -- validate crates/omega_game/tests/fixtures/content/full --profile full } },
	@{ Name = 'content prototype'; Cmd = { cargo run -q -p omega_game --no-default-features --locked --bin omega-content -- validate crates/omega_game/tests/fixtures/content/prototype --profile prototype } },
	@{ Name = 'content shipped'; Cmd = { cargo run -q -p omega_game --no-default-features --locked --bin omega-content -- validate assets/content --profile full } },
	@{ Name = 'content prototype subset'; Cmd = { cargo run -q -p omega_game --no-default-features --locked --bin omega-content -- validate assets/content-prototype --profile prototype } },
	@{ Name = 'balance smoke'; Cmd = { cargo run -q -p omega_game --no-default-features --locked --bin omega-sim -- --scenario balance --policies idle,human_like --seeds 2 --label smoke --out artifacts/balance/smoke.json } },
	@{ Name = 'sim smoke'; Cmd = { cargo run -q -p omega_game --no-default-features --locked --bin omega-sim -- --scenario repeatability --seed $zero --ticks 600 } },
	@{ Name = 'assets'; Cmd = { powershell -NoProfile -File scripts/validate-assets.ps1 -Manifest assets/presentation/proof-manifest.json } }
)
if (-not $SkipClient) {
	$steps += @(
		@{ Name = 'check client'; Cmd = { cargo check -p omega_game --features client --locked } },
		@{ Name = 'clippy client'; Cmd = { cargo clippy -p omega_game -p omega_2d --features client --all-targets --locked -- -D warnings } },
		@{ Name = 'test client contracts'; Cmd = { cargo test -p omega_2d --features client --locked --test visual_contract --test asset_contract } },
		@{ Name = 'test game client contracts'; Cmd = { cargo test -p omega_game --features client --locked --test e2e_contract --test e2e_loop_contract --test presentation_contract } },
		@{ Name = 'e2e scripts'; Cmd = { foreach ($f in Get-ChildItem tests/e2e/*.json) { cargo run -q -p omega_game --features client --locked --bin omega-game -- --headless --e2e-script $f.FullName --out-dir artifacts/e2e; if ($LASTEXITCODE -ne 0) { break } } } },
		@{ Name = 'qa bot prototype'; Cmd = { foreach ($p in 'cautious','aggressive','chaotic') { cargo run -q -p omega_game --features client --locked --bin omega-game -- --headless --qa-bot $p --seed ('5a' * 32) --frames 60000 --out-dir artifacts/e2e; if ($LASTEXITCODE -ne 0) { break } } } }
	)
}
if (-not $SkipDeny) { $steps += @{ Name = 'cargo deny'; Cmd = { cargo deny check } } }
if (Get-Command bun -ErrorAction SilentlyContinue) { $steps += @{ Name = 'markdown'; Cmd = { bun run lint } } }

$failed = @()
foreach ($s in $steps) {
	Write-Host "==> $($s.Name)"
	& $s.Cmd
	if ($LASTEXITCODE -ne 0) { $failed += $s.Name; Write-Host "FAILED: $($s.Name)" }
}
if ($failed.Count -gt 0) {
	Write-Host "failed gates: $($failed -join ', ')"
	exit 1
}
Write-Host 'all gates passed'
