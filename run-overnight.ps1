# Claude Code overnight runner for Windows.
# Repeatedly runs non-interactive Claude Code sessions that continue the roadmap
# (ROADMAP.md header -> next task) until DONE.txt exists or the retry cap is hit.
#
# Usage: powershell -NoProfile -File run-overnight.ps1 [-Model sonnet] [-MaxRuns 8] [-PermissionMode auto] [-WaitSeconds 120]
# Stop early: create DONE.txt in the repository root (the agent does this when no READY task remains).
param(
	[string]$Model = 'sonnet',
	[int]$MaxRuns = 8,
	[ValidateSet('auto', 'acceptEdits', 'dontAsk', 'plan')][string]$PermissionMode = 'auto',
	[int]$WaitSeconds = 120
)
$ErrorActionPreference = 'Stop'
$projectPath = $PSScriptRoot
Set-Location $projectPath
$logFile = Join-Path $projectPath 'run.log'
$doneFile = Join-Path $projectPath 'DONE.txt'

$prompt = @'
Overnight run. Follow AGENTS.md and CLAUDE.md, including the session start routine.
Start at the ROADMAP.md header (Next task) and section 1 rules; work the next task's microtasks in order. If none is READY, promote the first TODO task
whose dependencies are all DONE, after the READY audit in docs/validation/quality-gates.md (exact files, test names per
acceptance criterion), and record the promotion. Implement it within its allowed paths,
run its validation commands and scripts/check.ps1, record evidence under docs/validation/evidence/<TASK-ID>/,
check the boxes, update the task status, the ROADMAP.md header and CHANGELOG.md [Unreleased], and commit with a Conventional Commit.
Do not push, tag or publish. If a task is blocked, record why and move to the next independent READY task.
Stop at the end of Phase 2 (Prototype) exit: QA-P2-001 needs a human native playtest; record it as BLOCKED for the owner.
If no task can be made READY, write a one-line summary to DONE.txt and stop.
'@

function Write-Log([string]$Message) {
	$line = "[{0}] {1}" -f (Get-Date -Format 'yyyy-MM-dd HH:mm:ss'), $Message
	Add-Content -Path $logFile -Value $line -Encoding UTF8
	Write-Host $line
}

if (-not (Get-Command claude -ErrorAction SilentlyContinue)) { throw 'claude CLI not found on PATH' }

# Keep the machine awake only while this process runs; Windows restores the
# previous behavior automatically when the process exits, so no power settings
# are changed or need restoring.
Add-Type -Namespace Omega -Name Power -MemberDefinition @'
[System.Runtime.InteropServices.DllImport("kernel32.dll")]
public static extern uint SetThreadExecutionState(uint esFlags);
'@
$ES_CONTINUOUS = [uint32]'0x80000000'
$ES_SYSTEM_REQUIRED = [uint32]'0x00000001'
[void][Omega.Power]::SetThreadExecutionState($ES_CONTINUOUS -bor $ES_SYSTEM_REQUIRED)

# Native stderr lines must not become terminating errors inside the loop (PowerShell 5.1).
$ErrorActionPreference = 'Continue'
Write-Log "Starting overnight runner in $projectPath (model: $Model, max runs: $MaxRuns, permission mode: $PermissionMode)"
try {
	for ($run = 1; $run -le $MaxRuns; $run++) {
		if (Test-Path $doneFile) { Write-Log 'DONE.txt found - stopping.'; break }
		Write-Log "=== Run $run of $MaxRuns ==="
		$claudeArgs = @('--print', $prompt, '--model', $Model, '--permission-mode', $PermissionMode)
		if ($run -gt 1) { $claudeArgs = @('--continue') + $claudeArgs }
		& claude @claudeArgs 2>&1 | ForEach-Object { Add-Content -Path $logFile -Value $_ -Encoding UTF8; $_ }
		Write-Log "Claude exited with code $LASTEXITCODE"
		if (Test-Path $doneFile) { Write-Log 'DONE.txt found - work complete.'; break }
		if ($run -lt $MaxRuns) {
			Write-Log "Waiting $WaitSeconds seconds before the next run..."
			Start-Sleep -Seconds $WaitSeconds
		}
	}
} finally {
	[void][Omega.Power]::SetThreadExecutionState($ES_CONTINUOUS)
	Write-Log "Runner finished. Log: $logFile"
}
