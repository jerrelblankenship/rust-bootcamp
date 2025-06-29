@echo off
REM Setup script for Rust Bootcamp (Windows)

echo Setting up Rust Bootcamp repository...

REM Check if git is initialized
if not exist .git (
    echo Initializing git repository...
    git init
    git add .
    git commit -m "Initial commit: Rust Bootcamp curriculum"
)

REM Check Rust installation
where rustc >nul 2>nul
if %errorlevel% == 0 (
    echo Rust is installed:
    rustc --version
) else (
    echo ERROR: Rust is not installed. Please follow the setup guide in 00-setup/
    exit /b 1
)

REM Check cargo
where cargo >nul 2>nul
if %errorlevel% == 0 (
    echo Cargo is installed:
    cargo --version
) else (
    echo ERROR: Cargo is not installed. Please follow the setup guide in 00-setup/
    exit /b 1
)

REM Create .env file if it doesn't exist
if not exist .env (
    echo Creating .env file...
    (
        echo # Environment variables for development
        echo RUST_BACKTRACE=1
        echo RUST_LOG=debug
    ) > .env
)

echo.
echo Would you like to install recommended cargo extensions? (Y/N)
set /p response=

if /i "%response%"=="Y" (
    echo Installing cargo extensions...
    cargo install cargo-watch 2>nul || echo cargo-watch already installed
    cargo install cargo-edit 2>nul || echo cargo-edit already installed
    cargo install cargo-expand 2>nul || echo cargo-expand already installed
)

echo.
echo Detected Windows. Check 00-setup/windows-11-setup.md for platform-specific instructions.
echo.
echo Basic setup complete!
echo.
echo Next steps:
echo 1. Read the main README.md
echo 2. Follow the setup guide in 00-setup/ for your platform
echo 3. Configure VS Code using 00-setup/vscode-configuration.md
echo 4. Start with Module 01: Foundations
echo.
echo Happy learning!
