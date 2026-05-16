#!/bin/bash
# StellarVallum Setup Script for KiroIDE
# Run this after cloning the repository

set -e

echo "🚀 StellarVallum Setup"
echo "═══════════════════════════════════════"

# Check Rust installation
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "✅ Rust version: $RUST_VERSION"

# Check minimum version
MIN_VERSION="1.74.0"
if [ "$(printf '%s\n' "$MIN_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$MIN_VERSION" ]; then 
    echo "⚠️  Rust version $RUST_VERSION is below minimum $MIN_VERSION"
    echo "   Please update: rustup update"
    exit 1
fi

# Install dependencies
echo "📦 Installing dependencies..."
cargo fetch

# Create config from template
echo "⚙️  Setting up configuration..."
if [ ! -f "config/vallum.toml" ]; then
    cp config/vallum.toml config/vallum.toml
    echo "✅ Configuration created"
else
    echo "✅ Configuration already exists"
fi

# Create reports directory
mkdir -p reports

# Build project
echo "🔨 Building StellarVallum..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

# Install binary locally
echo "💿 Installing stellarvallum binary..."
cargo install --path . --force

echo ""
echo "✅ Setup Complete!"
echo "═══════════════════════════════════════"
echo ""
echo "Next steps:"
echo "   1. Configure testnet: stellarvallum config --network testnet"
echo "   2. Get testnet XLM: curl https://friendbot.stellar.org/?addr=<ADDRESS>"
echo "   3. Scan contract: stellarvallum scan --wasm ./contract.wasm"
echo "   4. Start dashboard: stellarvallum dashboard"
echo ""
echo "Documentation:"
echo "   - README.md: Project overview"
echo "   - docs/steering/: Governance and decisions"
echo "   - docs/guides/: User guides (coming soon)"
echo ""
echo "⚡ Remember: This is TESTNET ONLY during Beta"
echo "   Mainnet support coming in v1.0"
