#!/bin/bash
# Setup script for Rust Bootcamp

echo "🦀 Setting up Rust Bootcamp repository..."

# Initialize git repository
if [ ! -d .git ]; then
    echo "Initializing git repository..."
    git init
    git add .
    git commit -m "Initial commit: Rust Bootcamp curriculum"
fi

# Check Rust installation
if command -v rustc &> /dev/null; then
    echo "✅ Rust is installed: $(rustc --version)"
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

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cat > .env << EOF
# Environment variables for development
RUST_BACKTRACE=1
RUST_LOG=debug
EOF
fi

# Install useful cargo extensions (optional)
echo ""
echo "Would you like to install recommended cargo extensions? (y/n)"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo "Installing cargo extensions..."
    cargo install cargo-watch 2>/dev/null || echo "cargo-watch already installed"
    cargo install cargo-edit 2>/dev/null || echo "cargo-edit already installed"
    cargo install cargo-expand 2>/dev/null || echo "cargo-expand already installed"
fi

# Platform-specific setup
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo ""
    echo "📱 Detected macOS. Check 00-setup/macos-m4-setup.md for platform-specific instructions."
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo ""
    echo "🪟 Detected Windows. Check 00-setup/windows-11-setup.md for platform-specific instructions."
else
    echo ""
    echo "🐧 Detected Linux. The setup should work out of the box."
fi

echo ""
echo "✅ Basic setup complete!"
echo ""
echo "📖 Next steps:"
echo "1. Read the main README.md"
echo "2. Follow the setup guide in 00-setup/ for your platform"
echo "3. Configure VS Code using 00-setup/vscode-configuration.md"
echo "4. Start with Module 01: Foundations"
echo ""
echo "Happy learning! 🚀"
