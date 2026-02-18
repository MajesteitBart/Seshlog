@echo off
cd /d "%~dp0"

echo Checking for pnpm...
where pnpm >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo pnpm not found, installing globally...
    call npm install -g pnpm
)

echo Cleaning Next.js cache...
rd /s /q .next 2>nul
rd /s /q out 2>nul

echo Cleaning npm dependencies...
rd /s /q node_modules 2>nul
del /f /q package-lock.json 2>nul
del /f /q pnpm-lock.yaml 2>nul

echo Cleaning Rust build cache...
rd /s /q src-tauri\target 2>nul

echo Installing dependencies...
call pnpm install

echo Building the project...
call pnpm run tauri:dev
