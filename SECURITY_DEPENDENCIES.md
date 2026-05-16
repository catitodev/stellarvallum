# Security Dependencies Audit

> Last reviewed: 2026-05-16
> StellarVallum v0.2.0-testnet

This document justifies every direct dependency used by StellarVallum, with security considerations for each.

## Crate-Level Security

```rust
#![forbid(unsafe_code)]  // No unsafe Rust in our codebase
```

All dependencies may use unsafe internally (Rust's standard library does), but our code never does.

---

## Direct Dependencies

| Crate | Version | Purpose | Unsafe Internally? | Justification |
|-------|---------|---------|-------------------|---------------|
| `tokio` | 1.x | Async runtime | Yes (I/O primitives) | Industry standard, heavily audited, required for async Rust |
| `axum` | 0.7 | HTTP server (dashboard) | Minimal | Built on tokio+hyper, maintained by tokio team |
| `reqwest` | 0.12 | HTTP client (RPC/Horizon) | Via rustls | Using `rustls-tls` feature (no OpenSSL). TLS 1.2+ enforced |
| `tower` | 0.4 | Middleware framework | No | Pure Rust service abstractions |
| `tower-http` | 0.5 | HTTP middleware (CORS) | No | CORS and tracing layers |
| `serde` | 1.0 | Serialization | Yes (performance) | Universal Rust serialization, extremely well audited |
| `serde_json` | 1.0 | JSON parsing | Minimal | Standard JSON library for Rust |
| `toml` | 0.8 | TOML config parsing | No | Pure Rust TOML parser |
| `sha2` | 0.10 | SHA-256 hashing (CHAIN) | No | RustCrypto project, pure Rust, no unsafe |
| `hex` | 0.4 | Hex encoding | No | Simple, no unsafe |
| `ed25519-dalek` | 2.x | Ed25519 signatures | Yes (crypto ops) | Well-audited cryptography crate, used for Stellar key operations |
| `stellar-strkey` | 0.0.9 | StrKey address encoding | No | Official Stellar crate for address validation |
| `stellar-xdr` | 22 | Stellar XDR types | Minimal | Official Stellar crate for transaction types |
| `thiserror` | 1.0 | Error derive macros | No | Compile-time only, zero runtime cost |
| `anyhow` | 1.0 | Error handling (CLI) | Minimal | Thin wrapper, widely used |
| `tracing` | 0.1 | Structured logging | No | Tokio ecosystem standard |
| `tracing-subscriber` | 0.3 | Log output formatting | Minimal | Standard companion to tracing |
| `clap` | 4.x | CLI argument parsing | No | Industry standard CLI framework |
| `chrono` | 0.4 | Date/time handling | Minimal | Standard time library |
| `sqlx` | 0.8 | SQLite database | Minimal | **Configured with `default-features = false`** to exclude MySQL (which pulled in vulnerable `rsa` crate). Only SQLite feature enabled. |
| `wasmparser` | 0.118 | WASM binary analysis | No | Bytecode Alliance project, pure Rust |
| `regex` | 1.x | Pattern matching (heuristics) | Yes (SIMD) | Standard regex library, unsafe only for SIMD optimization |
| `async-trait` | 0.1 | Async trait support | No | Proc macro, compile-time only |
| `uuid` | 1.x | Unique scan IDs | Minimal | Standard UUID generation |
| `base32` | 0.5 | StrKey validation | No | Simple base32 encoding |

---

## Security Decisions

### Why `rustls-tls` instead of `native-tls`?

- `native-tls` depends on system OpenSSL which may have unpatched vulnerabilities
- `rustls` is pure Rust, memory-safe, and actively maintained
- We enforce TLS 1.2 minimum via `reqwest::ClientBuilder`

### Why `sqlx` with `default-features = false`?

- Default features include MySQL and PostgreSQL support
- `sqlx-mysql` pulls in `rsa` crate which has RUSTSEC-2023-0071 (no fix available)
- We only use SQLite — no need for MySQL/PostgreSQL drivers
- This eliminates the `rsa` vulnerability entirely from our dependency tree

### Why `ed25519-dalek` with unsafe?

- Cryptographic operations require unsafe for performance-critical constant-time comparisons
- This is standard practice for all crypto libraries
- The crate is well-audited and maintained by the dalek-cryptography team

---

## Dependency Audit Process

1. `cargo audit` runs on every CI build (GitHub Actions)
2. Any vulnerability with severity >= Medium blocks the build
3. Dependencies are reviewed quarterly for unmaintained status
4. New dependencies require justification in this document before merging

---

## Known Accepted Risks

| Advisory | Crate | Risk | Mitigation |
|----------|-------|------|------------|
| None currently | — | — | — |

---

*This document is updated whenever dependencies change.*
