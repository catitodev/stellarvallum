
# STELLARVALLUM PROJECT SUMMARY
## Complete Project Structure for KiroIDE Development

---

## 📁 PROJECT STRUCTURE

```
stellarvallum/
├── .gitignore                    # Git ignore rules
├── Cargo.toml                    # Rust project configuration
├── LICENSE                       # MIT + Disclaimer
├── README.md                     # Main project documentation
├── SECURITY.md                   # Security policy
├── stellarvallum.code-workspace  # KiroIDE/VSCode workspace
│
├── config/
│   └── vallum.toml              # Main configuration file
│
├── contracts/
│   └── testnet/                 # Test contracts for testing
│       └── .gitkeep
│
├── docs/
│   ├── architecture/
│   │   └── ARCHITECTURE.md      # Technical architecture docs
│   ├── guides/
│   │   └── GETTING_STARTED.md   # User getting started guide
│   └── steering/                # OFFICIAL STEERING CONTENT
│       ├── README.md             # Steering committee overview
│       ├── GOVERNANCE.md         # Project governance
│       ├── ROADMAP.md            # Development roadmap
│       ├── DECISIONS/            # Architecture Decision Records
│       │   ├── 001-testnet-first.md
│       │   ├── 002-ai-agnostic.md
│       │   ├── 003-no-ai-default.md
│       │   └── 004-openrouter-binding.md
│       └── MEETINGS/             # Meeting notes (future)
│
├── reports/                      # Generated reports (gitignored)
│   └── .gitkeep
│
├── scripts/
│   └── setup.sh                  # Setup script for new devs
│
├── src/                          # MAIN SOURCE CODE
│   ├── main.rs                   # CLI entry point
│   ├── config.rs                 # Configuration management
│   ├── shield.rs                 # SHIELD: Transaction inspector
│   ├── spear.rs                  # SPEAR: Adversarial testing
│   ├── chain.rs                  # CHAIN: Audit trail
│   ├── network.rs                # Network operations (testnet only)
│   ├── dashboard.rs              # Dashboard interface
│   └── ai/                       # AI provider system
│       ├── mod.rs                # Trait + factory
│       ├── no_ai.rs              # Default: Pure Rust heuristics
│       ├── openrouter.rs         # Optional: OpenRouter binding
│       └── local.rs              # Optional: Local LLM
│
└── tests/                        # Test files
    └── .gitkeep
```

---

## 🎯 KEY FEATURES

### 1. Testnet-First (Beta)
- ALL operations on Stellar Testnet
- Mainnet HARD-BLOCKED until v1.0
- Safe environment for validation

### 2. AI-Agnostic
- **Default**: No-AI mode (free, private, instant)
- **Optional**: OpenRouter (binded, transparent)
- **Optional**: Local LLM (100% private)
- **Always**: Fallback to No-AI if provider fails

### 3. Three Security Layers
- **SHIELD**: Static analysis + vulnerability detection
- **SPEAR**: 7 attack vectors for adversarial testing
- **CHAIN**: Immutable SHA-256 audit trail

### 4. Steering System
- `docs/steering/`: Official governance content
- `DECISIONS/`: Architecture Decision Records (ADRs)
- `GOVERNANCE.md`: Project governance model
- `ROADMAP.md`: Development roadmap

---

## 🚀 QUICK START IN KIROIDE

### 1. Open Project
```bash
cd stellarvallum
# Open in KiroIDE: File → Open Folder → stellarvallum/
```

### 2. Setup Environment
```bash
./scripts/setup.sh
```

### 3. Build
```bash
cargo build
```

### 4. Test
```bash
cargo test
```

### 5. Run
```bash
# Scan a contract
stellarvallum scan --wasm ./contract.wasm

# Deploy to testnet
stellarvallum deploy-testnet --wasm ./contract.wasm --source-account G...

# Run adversarial tests
stellarvallum spear --contract CD... --attacks all

# Start dashboard
stellarvallum dashboard
```

---

## 📊 WORKFLOW FOR DEVELOPMENT

### Adding New Heuristic (No-AI)
1. Edit `src/ai/no_ai.rs`
2. Implement `Heuristic` trait
3. Add to `heuristics` vector in `NoAIMode::new()`
4. Test with `cargo test`
5. Update documentation

### Adding New AI Provider
1. Create `src/ai/<provider>.rs`
2. Implement `AIProvider` trait
3. Add to `AIProviderFactory::create()` in `src/ai/mod.rs`
4. Update `config/vallum.toml` template
5. Document in steering ADRs

### Adding New Attack Vector (SPEAR)
1. Edit `src/spear.rs`
2. Add to `attack_list` in `run()`
3. Implement in `run_attack()` match
4. Update documentation

---

## 🤝 CONTRIBUTING

### Steering Content (Official)
- Location: `docs/steering/`
- Process: RFC → Discussion → Vote → Merge
- Authority: Steering Committee

### Code Contributions
- Location: `src/`
- Process: Fork → Branch → PR → Review → Merge
- Authority: Maintainers

### Documentation
- Location: `docs/guides/`
- Process: Fork → PR → Review → Merge
- Authority: Maintainers

---

## 📚 DOCUMENTATION MAP

| Document | Purpose | Audience |
|----------|---------|----------|
| `README.md` | Project overview | Everyone |
| `docs/guides/GETTING_STARTED.md` | First steps | New users |
| `docs/architecture/ARCHITECTURE.md` | Technical design | Developers |
| `docs/steering/README.md` | Governance overview | Contributors |
| `docs/steering/GOVERNANCE.md` | Rules and roles | Contributors |
| `docs/steering/ROADMAP.md` | Future plans | Everyone |
| `docs/steering/DECISIONS/*.md` | Why we chose X | Developers |

---

## ⚡ IMPORTANT REMINDERS

1. **Testnet Only**: Beta is testnet-only for safety
2. **No-AI Default**: AI is optional, never mandatory
3. **Open Source**: MIT licensed, community driven
4. **Security First**: This is a security tool - we take it seriously
5. **Official Steering**: All official decisions in `docs/steering/`

---

## 🔗 LINKS

- **Repository**: https://github.com/catitodev/stellarvallum
- **Original Vallum**: https://github.com/catitodev/vallum
- **Stellar**: https://stellar.org
- **Soroban**: https://soroban.stellar.org
- **OpenRouter**: https://openrouter.ai

---

*Project created: 2026-05-15*
*Version: 0.1.0-testnet*
*Status: Beta - Testnet Only*
