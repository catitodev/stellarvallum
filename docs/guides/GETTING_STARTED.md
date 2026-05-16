# Getting Started with StellarVallum

## Prerequisites

- [Rust 1.74+](https://rust-lang.org/tools/install)
- A Stellar testnet account (we'll create one)
- (Optional) OpenRouter API key for AI features

## Installation

### Step 1: Clone Repository

```bash
git clone https://github.com/catitodev/stellarvallum.git
cd stellarvallum
```

### Step 2: Run Setup Script

```bash
./scripts/setup.sh
```

Or manually:

```bash
# Install dependencies
cargo fetch

# Build
cargo build --release

# Install binary
cargo install --path .
```

### Step 3: Verify Installation

```bash
stellarvallum --version
# Output: stellarvallum 0.1.0-testnet
```

## Your First Scan

### 1. Create a Test Contract

```bash
# Create new Soroban project
soroban contract init hello-world
cd hello-world
```

### 2. Write a Vulnerable Contract

Edit `src/lib.rs`:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // VULNERABLE: Missing auth check
    pub fn hello(env: Env) -> Symbol {
        Symbol::new(&env, "Hello")
    }

    // VULNERABLE: Using panic! instead of panic_with_error!
    pub fn check_value(env: Env, value: i128) {
        if value < 0 {
            panic!("negative value");  // ❌ Bad!
        }
    }
}
```

### 3. Compile

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 4. Scan with StellarVallum

```bash
stellarvallum scan   --wasm ./target/wasm32-unknown-unknown/release/hello_world.wasm   --mode no-ai   --output report.json
```

### 5. View Results

```
🔍 SHIELD SCAN COMPLETE
═══════════════════════════════════════
Score: 45/100
Findings: 2 vulnerabilities
Mainnet Ready: ❌ NO

[HIGH] Missing Authorization Check
  Description: Function performs operation without require_auth
  Fix: Add env.require_auth(caller_address)

[HIGH] Improper Error Handling
  Description: Uses panic! instead of panic_with_error!
  Fix: Replace with panic_with_error!(env, Error::NegativeValue, 1)

📄 Report saved: report.json
```

### 6. Fix and Re-scan

Edit `src/lib.rs`:

```rust
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, caller: Address) -> Symbol {
        env.require_auth(caller);  // ✅ Fixed!
        Symbol::new(&env, "Hello")
    }

    pub fn check_value(env: Env, value: i128) {
        if value < 0 {
            panic_with_error!(env, Error::NegativeValue, 1);  // ✅ Fixed!
        }
    }
}
```

Re-compile and scan:

```bash
cargo build --target wasm32-unknown-unknown --release
stellarvallum scan --wasm ./target/wasm32-unknown-unknown/release/hello_world.wasm
```

Result: `Score: 95/100 ✅ MAINNET READY`

## Next Steps

- [Deploy to testnet](guides/deploy-testnet.md)
- [Run adversarial tests](guides/spear-testing.md)
- [Set up monitoring](guides/monitoring.md)
- [Configure AI providers](guides/ai-configuration.md)

## Troubleshooting

### "Rust version too old"
```bash
rustup update
```

### "Soroban CLI not found"
```bash
cargo install soroban-cli
```

### "Testnet XLM needed"
```bash
curl https://friendbot.stellar.org/?addr=<YOUR_ADDRESS>
```

## Getting Help

- [GitHub Issues](https://github.com/catitodev/stellarvallum/issues)
- [Stellar Dev Discord](https://discord.gg/stellardev)
- [Documentation](https://docs.stellarvallum.dev)

---

*Happy scanning! 🔍*
