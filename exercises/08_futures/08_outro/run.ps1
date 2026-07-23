Set-Location $PSScriptRoot
Write-Host "Starting Ticket console (Tauri + Vue Vite)..."
pnpm install
pnpm tauri:dev
if ($LASTEXITCODE -ne 0) {
    Read-Host "Failed, press Enter to exit"
}
