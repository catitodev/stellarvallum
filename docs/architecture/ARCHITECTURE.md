# StellarVallum Architecture

## Overview

StellarVallum is a Rust-based security framework for Soroban smart contracts, organized into modular components following clean architecture principles.

## Directory Structure

```
src/
├── main.rs           # CLI entry point, command routing
├── config.rs         # Configuration management (TOML)
├── ai/               # AI provider system
│   ├── mod.rs        # Trait definitions and factory
│   ├── no_ai.rs      # Pure Rust heuristics (default)
│   ├── openrouter.rs # OpenRouter API integration
│   └── local.rs      # Local LLM (Ollama) integration
├── shield.rs         # Transaction inspector
├── spear.rs          # Adversarial testing engine
├── chain.rs          # Audit trail and reporting
├── network.rs        # Stellar network operations
└── dashboard.rs       # Web dashboard interface
```

## Module Responsibilities

### main.rs
- CLI argument parsing (clap)
- Command routing
- Logging initialization
- Configuration loading

### config.rs
- TOML configuration parsing
- Environment variable substitution
- Default configuration generation
- Network validation

### ai/ Module
**Philosophy**: AI is optional, never mandatory.

| File | Purpose |
|------|---------|
| `mod.rs` | `AIProvider` trait, `AIProviderFactory`, shared types |
| `no_ai.rs` | Default provider: 7 heuristics for vulnerability detection |
| `openrouter.rs` | Optional: Multi-model AI via OpenRouter |
| `local.rs` | Optional: 100% private local LLM |

### shield.rs
- WASM static analysis
- Transaction simulation via Soroban RPC
- Vulnerability scoring (0-100)
- Report generation (JSON/CSV/PDF)

### spear.rs
- Attack vector execution
- Fuzzing contract deployment
- Behavior recording
- Exploitability assessment

### chain.rs
- SHA-256 hash chain for tamper evidence
- Testnet contract storage
- SQLite local cache
- Report export

### network.rs
- Testnet validation (hard-coded)
- Mainnet blocking (Beta safety)
- Contract deployment
- RPC communication

### dashboard.rs
- Streamlit server management
- Metrics aggregation
- Real-time updates

## Data Flow

```
User Input (CLI)
    ↓
main.rs (parse command)
    ↓
config.rs (load settings)
    ↓
[Command Router]
    ├── scan → shield.rs → ai/ → Report
    ├── deploy-testnet → network.rs → Contract ID
    ├── spear → spear.rs → Attack Results
    ├── monitor → shield.rs → Continuous
    ├── report → chain.rs → Export File
    └── dashboard → dashboard.rs → Web UI
```

## AI Provider Architecture

```
┌─────────────────────────────────────────┐
│         AIProvider Trait                │
│  - analyze_contract()                   │
│  - generate_tests()                     │
│  - explain_vulnerability()             │
│  - suggest_fix()                        │
└─────────────────────────────────────────┘
           ↑
    ┌──────┴──────┬──────────────┐
    │             │              │
┌───┴───┐   ┌───┴────┐   ┌────┴───┐
│ No-AI │   │OpenRouter│   │ Local  │
│(Default)│   │(Optional)│   │(Optional)│
└───────┘   └────────┘   └────────┘
```

## Security Considerations

### Testnet-Only Enforcement
- Hard-coded in `network.rs`
- Checked before every operation
- Cannot be bypassed via configuration

### AI Privacy
- No-AI mode: Zero external communication
- Local LLM: Zero external communication
- OpenRouter: Only with explicit opt-in

### Secret Management
- API keys via environment variables only
- Never logged or stored in code
- `.gitignore` prevents accidental commits

## Testing Strategy

| Level | Tools | Coverage |
|-------|-------|----------|
| Unit | `cargo test` | Individual functions |
| Integration | `tokio-test` | Module interactions |
| E2E | Shell scripts | Full CLI workflows |
| Security | `cargo-audit` | Dependency vulnerabilities |

## Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| Scan (No-AI) | < 2s | Pure Rust heuristics |
| Scan (AI) | < 10s | Network dependent |
| Deploy | < 30s | RPC dependent |
| Spear (7 attacks) | < 5m | Configurable |
| Dashboard load | < 1s | Local server |

## Future Architecture Evolution

### v0.2.0 (Community)
- Plugin system for custom heuristics
- Webhook integrations
- Community-contributed attack vectors

### v0.3.0 (Audit)
- Formal verification bridge
- Audit Bank API integration
- STRIDE automation

### v1.0.0 (Mainnet)
- Mainnet operation unlock
- Enterprise monitoring
- SLA guarantees

---

*Last updated: 2026-05-15*
*Architecture version: 0.1.0*
