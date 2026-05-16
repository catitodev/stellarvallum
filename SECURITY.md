# Security Policy

## Supported Versions

| Version | Supported | Network |
|---------|-----------|---------|
| v0.1.x  | ✅ Active | Testnet only |
| < v0.1  | ❌ No     | - |

## Reporting a Vulnerability

**Please do NOT open public issues for security vulnerabilities.**

Instead, report privately:

1. **Email**: security@stellarvallum.dev
2. **Encrypted**: PGP key available at [security.stellarvallum.dev](https://security.stellarvallum.dev)
3. **Response time**: Within 48 hours

### What to Include

- Description of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Security Measures

### Code Security
- No hardcoded secrets
- All API keys via environment variables
- Pre-commit hooks for secret detection
- Dependency scanning with cargo-audit

### Testnet Safety
- Mainnet operations blocked in Beta
- Network passphrase validation
- Resource limit enforcement
- No real value at risk

### AI Privacy
- No-AI mode: 100% private (default)
- Local LLM: 100% private (optional)
- OpenRouter: Code sent to API only with explicit consent
- No training data collection

## Bug Bounty

Planned for v0.2.0-community phase.

Scope will include:
- False positive reduction
- New vulnerability detection
- Performance improvements
- Documentation improvements

## Security Audit

External security audit planned for v0.3.0-audit phase.

Auditor: TBD (Certora, Veridise, or OtterSec)

## Acknowledgments

We thank security researchers who responsibly disclose vulnerabilities.

*Last updated: 2026-05-15*
