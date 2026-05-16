# ADR-001: Testnet-First Strategy

## Status
Accepted

## Context
StellarVallum is a security tool for Soroban smart contracts. As a security tool, it must be trustworthy before handling mainnet contracts.

## Decision
StellarVallum will operate **exclusively on Testnet** during Beta (v0.x.x).

Mainnet support will only be enabled after:
1. Community validation
2. External security audit
3. Steering Committee approval

## Consequences

### Positive
- No risk to mainnet contracts during Beta
- Community can safely test and provide feedback
- Forces robust testnet infrastructure
- Builds confidence before mainnet

### Negative
- Limits initial user base (testnet only)
- Cannot monitor mainnet contracts yet
- Some features need mainnet for full validation

## Alternatives Considered

1. **Mainnet from day one**: Rejected - too risky for security tool
2. **Optional mainnet**: Rejected - could lead to accidental mainnet usage
3. **Testnet-only**: Accepted - safest approach

## References
- [Soroban Testnet](https://developers.stellar.org/docs/fundamentals/testnet-and-pubnet)
- [Stellar Friendbot](https://friendbot.stellar.org)
