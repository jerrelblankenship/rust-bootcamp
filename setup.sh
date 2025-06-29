#!/bin/bash
# Setup script for Rust Bootcamp

set -e  # Exit on any error

echo "🦀 Setting up Rust Bootcamp repository..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found. Run this script from the repository root."
    exit 1
fi

# Initialize git repository
if [ ! -d .git ]; then
    echo "📂 Initializing git repository..."
    git init
    git add .
    git commit -m "Initial commit: Rust Bootcamp curriculum"
else
    echo "✅ Git repository already initialized"
fi

# Check Rust installation
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✅ Rust is installed: $RUST_VERSION"

    # Check minimum version (1.70.0 or higher recommended)
    RUST_MAJOR=$(echo "$RUST_VERSION" | grep -oE '[0-9]+\.[0-9]+' | head -1)
    if [[ "$(printf '%s\n' "1.70" "$RUST_MAJOR" | sort -V | head -n1)" != "1.70" ]]; then
        echo "⚠️  Warning: Rust $RUST_MAJOR detected. Recommend 1.70+ for best compatibility."
    fi
else
    echo "❌ Rust is not installed. Please follow the setup guide in 00-setup/"
    exit 1
fi

# Check cargo
if command -v cargo &> /dev/null; then
    echo "✅ Cargo is installed: $(cargo --version)"
else
    echo "❌ Cargo is not installed. Please follow the setup guide in 00-setup/"
    exit 1
fi

# Verify workspace builds
echo "🔧 Verifying workspace configuration..."
if cargo check --workspace --quiet; then
    echo "✅ Workspace configuration is valid"
else
    echo "❌ Workspace has configuration issues"
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    cat > .env << 'EOF'
# Environment variables for development
RUST_BACKTRACE=1
RUST_LOG=debug
CARGO_TERM_COLOR=always
EOF
else
    echo "✅ .env file already exists"
fi

# Install useful cargo extensions (optional)
echo ""
echo "🛠️  Would you like to install recommended cargo extensions? (y/n)"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo "Installing cargo extensions..."

    # Updated cargo extensions for 2024/2025
    cargo install cargo-watch 2>/dev/null || echo "✅ cargo-watch already installed"
    cargo install cargo-edit 2>/dev/null || echo "✅ cargo-edit already installed"
    cargo install cargo-expand 2>/dev/null || echo "✅ cargo-expand already installed"
    cargo install cargo-clippy 2>/dev/null || echo "✅ cargo-clippy already installed"
    cargo install cargo-fmt 2>/dev/null || echo "✅ cargo-fmt already installed"

    echo "✅ Cargo extensions installed"
fi

# Platform-specific setup
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo ""
    echo "🍎 Detected macOS. Check 00-setup/macos-setup.md for platform-specific instructions."
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo ""
    echo "🪟 Detected Windows. Check 00-setup/windows-11-setup.md for platform-specific instructions."
else
    echo ""
    echo "🐧 Detected Linux. The setup should work out of the box."
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "📖 Next steps:"
echo "1. Read the main README.md"
echo "2. Follow the setup guide in 00-setup/ for your platform"
echo "3. Configure VS Code using 00-setup/vscode-configuration.md"
echo "4. Start with Module 01: Foundations"
echo ""
echo "🚀 Quick start commands:"
echo "  cargo build --workspace    # Build all projects"
echo "  cargo test --workspace     # Run all tests"
echo "  cargo run -p calculator    # Run calculator project"
echo ""
echo "Happy learning! 🦀"
