@echo off
REM Setup script for Rust Bootcamp (Windows)
setlocal enabledelayedexpansion

echo 🦀 Setting up Rust Bootcamp repository...
echo.

REM Check if we're in the right directory
if not exist Cargo.toml (
    echo ❌ Error: Cargo.toml not found. Run this script from the repository root.
    exit /b 1
)

REM Initialize git repository
if not exist .git (
    echo 📂 Initializing git repository...
    git init
    git add .
    git commit -m "Initial commit: Rust Bootcamp curriculum"
) else (
    echo ✅ Git repository already initialized
)

REM Check Rust installation
where rustc >nul 2>nul
if %errorlevel% == 0 (
    echo ✅ Rust is installed:
    rustc --version
) else (
    echo ❌ Error: Rust is not installed. Please follow the setup guide in 00-setup/
    exit /b 1
)

REM Check cargo
where cargo >nul 2>nul
if %errorlevel% == 0 (
    echo ✅ Cargo is installed:
    cargo --version
) else (
    echo ❌ Error: Cargo is not installed. Please follow the setup guide in 00-setup/
    exit /b 1
)

REM Verify workspace builds
echo 🔧 Verifying workspace configuration...
cargo check --workspace --quiet >nul 2>nul
if %errorlevel% == 0 (
    echo ✅ Workspace configuration is valid
) else (
    echo ❌ Workspace has configuration issues
    exit /b 1
)

REM Create .env file if it doesn't exist
if not exist .env (
    echo 📝 Creating .env file...
    (
        echo # Environment variables for development
        echo RUST_BACKTRACE=1
        echo RUST_LOG=debug
        echo CARGO_TERM_COLOR=always
    ) > .env
) else (
    echo ✅ .env file already exists
)

echo.
echo 🛠️  Would you like to install recommended cargo extensions? (Y/N^)
set /p response=

if /i "!response!"=="Y" (
    echo Installing cargo extensions...
    
    REM Updated cargo extensions for 2024/2025
    cargo install cargo-watch 2>nul || echo ✅ cargo-watch already installed
    cargo install cargo-edit 2>nul || echo ✅ cargo-edit already installed  
    cargo install cargo-expand 2>nul || echo ✅ cargo-expand already installed
    cargo install cargo-clippy 2>nul || echo ✅ cargo-clippy already installed
    cargo install cargo-fmt 2>nul || echo ✅ cargo-fmt already installed
    
    echo ✅ Cargo extensions installed
)

echo.
echo 🪟 Detected Windows. Check 00-setup/windows-11-setup.md for platform-specific instructions.
echo.
echo ✅ Setup complete!
echo.
echo 📖 Next steps:
echo 1. Read the main README.md
echo 2. Follow the setup guide in 00-setup/ for your platform
echo 3. Configure VS Code using 00-setup/vscode-configuration.md
echo 4. Start with Module 01: Foundations
echo.
echo 🚀 Quick start commands:
echo   cargo build --workspace    # Build all projects
echo   cargo test --workspace     # Run all tests
echo   cargo run -p calculator    # Run calculator project
echo.
echo Happy learning! 🦀

pause
