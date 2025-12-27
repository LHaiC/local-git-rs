#!/bin/bash
# local-git-rs Installation Script

set -e

echo "🚀 Installing local-git-rs..."

# Build project
echo "📦 Building..."
cargo build --release

# Install to ~/.local/bin
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

cp target/release/local-git-rs "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/local-git-rs"

echo "✓ Installation complete!"
echo ""
echo "Usage:"
echo "  local-git-rs init                    # Initialize Hub"
echo "  local-git-rs create <name>           # Create repository"
echo "  local-git-rs add-remote <name>       # Add to current project"
echo "  local-git-rs --help                  # Show help"
echo ""
echo "If ~/.local/bin is not in PATH, add:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo "  to your ~/.bashrc or ~/.zshrc"