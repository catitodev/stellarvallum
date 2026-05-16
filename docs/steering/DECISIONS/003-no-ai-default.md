# ADR-003: No-AI as Default

## Status
Accepted

## Context
While AI can enhance analysis, it introduces costs, privacy concerns, and external dependencies.

## Decision
**No-AI mode is the default** for all StellarVallum operations.

Users must explicitly opt-in to AI providers via configuration.

## Rationale

1. **Accessibility**: Free for all users
2. **Privacy**: Code never leaves user's machine
3. **Reliability**: No network dependency
4. **Speed**: Instant results
5. **Determinism**: Same input = same output

## When to Use AI

| Scenario | Recommendation |
|----------|---------------|
| Quick scan | No-AI (default) |
| Deep analysis | OpenRouter/Local |
| Complex contract | OpenRouter (Claude/GPT-4) |
| Sensitive code | Local LLM |
| Budget constraint | No-AI |

## Implementation

```toml
[ai]
# Default
provider = "none"

# Opt-in required
# provider = "openrouter"
# provider = "local"
```

## References
- [ADR-002: AI-Agnostic Architecture](002-ai-agnostic.md)
