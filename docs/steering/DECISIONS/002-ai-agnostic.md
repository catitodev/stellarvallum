# ADR-002: AI-Agnostic Architecture

## Status
Accepted

## Context
AI services for code analysis are evolving rapidly. Locking into a single provider creates vendor risk.

## Decision
StellarVallum will use a **provider-agnostic AI system** with:

1. **Trait-based interface**: All providers implement `AIProvider`
2. **Factory pattern**: Runtime provider selection
3. **No-AI default**: Pure Rust heuristics as baseline
4. **Fallback mechanism**: Auto-switch to No-AI if provider fails

## Consequences

### Positive
- No vendor lock-in
- Users choose their AI (or none)
- Cost control
- Privacy options (local LLM)
- Resilient to API outages

### Negative
- More complex architecture
- Need to maintain multiple providers
- Feature parity challenges

## Supported Providers

| Provider | Status | Cost | Privacy |
|----------|--------|------|---------|
| No-AI | ✅ Default | Free | 100% |
| OpenRouter | ✅ Implemented | Variable | API-dependent |
| Local LLM | ✅ Implemented | Free (hardware) | 100% |
| OpenAI | 🚧 Planned | Pay-per-use | API-dependent |

## References
- [OpenRouter](https://openrouter.ai)
- [Ollama](https://ollama.ai)
