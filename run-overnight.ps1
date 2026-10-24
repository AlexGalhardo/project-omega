# Claude Code Overnight Runner for Windows
# Runs Claude Code in a loop, resuming context and retrying on failure

$projectPath = Get-Location
$logFile = Join-Path $projectPath "run.log"
$doneFile = Join-Path $projectPath "DONE.txt"
$maxRetries = 0

# Log function
function Write-Log {
	param([string]$message)
	$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
	"[$timestamp] $message" | Tee-Object -FilePath $logFile -Append
}

Write-Log "Starting overnight runner for project: $projectPath"
Write-Log "Claude Code will continue work from PLAN.md"
Write-Log "Terminal will retry on failure (max retries: $maxRetries per attempt)"

# Disable PC sleep (Windows 11)
Write-Log "Disabling system sleep..."
powercfg /change monitor-timeout-ac 0 2>&1 | Out-Null
powercfg /change disk-timeout-ac 0 2>&1 | Out-Null
powercfg /change standby-timeout-ac 0 2>&1 | Out-Null

# Main loop: run Claude Code until DONE.txt exists
while (-not (Test-Path $doneFile)) {
	Write-Log "=== Resuming Claude Code session ==="
	Write-Log "Working directory: $(Get-Location)"

	# Run claude code - resume latest conversation
	# Flags: -c (continue), default permission mode requires approval for destructive actions
	& claude code --continue 2>&1 | Tee-Object -FilePath $logFile -Append
	$exitCode = $LASTEXITCODE

	Write-Log "Claude exited with code: $exitCode"

	if (Test-Path $doneFile) {
		Write-Log "DONE.txt found - work complete!"
		break
	}

	Write-Log "Waiting 120 seconds before retry..."
	Start-Sleep -Seconds 120
}

Write-Log "Overnight runner finished successfully"

# Restore system sleep settings
Write-Log "Restoring system sleep settings..."
powercfg /change monitor-timeout-ac 10 2>&1 | Out-Null
powercfg /change disk-timeout-ac 20 2>&1 | Out-Null
powercfg /change standby-timeout-ac 30 2>&1 | Out-Null

Write-Log "All done. Run log saved to: $logFile"
