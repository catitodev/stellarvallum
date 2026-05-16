# StellarVallum Governance

## Project Structure

### Roles

| Role | Description | How to Become |
|------|-------------|---------------|
| **User** | Uses StellarVallum for contract security | Install and use |
| **Contributor** | Submits PRs, issues, docs | Submit a merged PR |
| **Maintainer** | Reviews PRs, triages issues | Invite from Contributors |
| **Steering Committee** | Sets direction, approves ADRs | Vote by Maintainers |

### Current Steering Committee

- **@catitodev** - Project Lead, Original Vallum Creator
- **[VACANT]** - Security Expert
- **[VACANT]** - Stellar Ecosystem Representative
- **[VACANT]** - Community Manager

## Decision Making Process

### Types of Decisions

1. **Technical Decisions** (code, architecture)
   - Lazy consensus among Maintainers
   - Escalate to Steering if contentious

2. **Governance Decisions** (policies, roles)
   - Steering Committee vote
   - 2/3 majority required

3. **Security Decisions** (vulnerabilities, incidents)
   - Security team decides
   - Steering informed after resolution

### Architecture Decision Records (ADRs)

All major architectural decisions are recorded as ADRs in `docs/steering/DECISIONS/`.

Template: `NNN-title.md`

```markdown
# ADR-NNN: Title

## Status
- Proposed | Accepted | Deprecated | Superseded

## Context
What is the issue we're addressing?

## Decision
What did we decide?

## Consequences
Positive and negative consequences.

## Alternatives Considered
What else did we consider?
```

## Release Governance

### Versioning

- `v0.X.Y-testnet` - Beta releases (testnet only)
- `v1.X.Y` - Stable releases (mainnet ready)

### Release Process

1. Feature freeze announced 2 weeks before
2. RC (Release Candidate) published
3. 1 week testing period
4. Steering Committee approves
5. Release published with changelog

### Beta Exit Criteria

Before v1.0 (mainnet support):

- [ ] 1000+ contracts scanned on testnet
- [ ] 0 critical false positives reported
- [ ] External security audit completed
- [ ] Bug bounty program active
- [ ] 90 days without critical bugs
- [ ] Steering Committee unanimous approval

## Code of Conduct Enforcement

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

Violations reported to: steering@stellarvallum.dev

## Amendments

This governance document can be amended by:
1. Steering Committee proposal
2. 14-day public comment
3. 2/3 committee vote

---

*Adopted: 2026-05-15*
*Next review: 2026-08-15*
