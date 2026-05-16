# StellarVallum Steering Committee

## Purpose

This directory contains official steering documents that guide the development,
governance, and direction of StellarVallum.

## Structure

```
docs/steering/
├── README.md              # This file
├── GOVERNANCE.md          # Project governance model
├── ROADMAP.md             # Official development roadmap
├── SECURITY_POLICY.md     # Security disclosure policy
├── CONTRIBUTING.md        # Contribution guidelines
├── CODE_OF_CONDUCT.md     # Community standards
├── DECISIONS/             # Architecture Decision Records (ADRs)
│   ├── 001-testnet-first.md
│   ├── 002-ai-agnostic.md
│   ├── 003-no-ai-default.md
│   └── 004-openrouter-binding.md
└── MEETINGS/              # Steering committee meeting notes
    └── 2026-05-15-initial.md
```

## Governance Model

### Steering Committee

The Steering Committee oversees the technical direction of StellarVallum.

**Current Members:**
- @catitodev (Project Lead)
- [Open for community members]

**Responsibilities:**
- Approve major architectural changes
- Set release priorities
- Review security-critical code
- Manage vendor relationships (OpenRouter, etc.)

### Decision Making

1. **RFC Process**: Major changes require RFC (Request for Comments)
2. **Lazy Consensus**: 7-day comment period, silence = consent
3. **Voting**: For contentious issues, simple majority of steering committee

## Official Content

### What belongs here?

✅ Architecture decisions and rationale
✅ Security policies and procedures
✅ Governance and contribution rules
✅ Roadmap and milestone definitions
✅ Meeting notes and decisions

### What does NOT belong here?

❌ User documentation (use docs/guides/)
❌ API documentation (use src/docs/)
❌ Marketing content (use website/)
❌ Unofficial opinions or speculation

## Contributing to Steering

To propose changes to steering documents:

1. Open an issue with `[STEERING]` prefix
2. Describe the change and rationale
3. Allow 14 days for community feedback
4. Steering committee votes if needed
5. Merge with committee approval

---

*Last updated: 2026-05-15*
*Next review: 2026-06-15*
