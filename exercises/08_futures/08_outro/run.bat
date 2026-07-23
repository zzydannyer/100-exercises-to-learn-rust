@echo off
cd /d "%~dp0"
echo Starting Ticket console (Tauri + Vue Vite)...
pnpm install
pnpm tauri:dev
if errorlevel 1 pause
