# ADR-004: OpenRouter Binding

## Status
Accepted

## Context
OpenRouter provides unified access to multiple AI models (Claude, GPT, Gemini, etc.) through a single API.

## Decision
OpenRouter is the **recommended external AI provider** for StellarVallum.

### Binding Requirements

1. **Transparency**: Model name always visible to user
2. **Cost visibility**: Show estimated cost before operation
3. **API key isolation**: Never log or expose keys
4. **Fallback**: Auto-switch to No-AI if OpenRouter fails

### Configuration

```toml
[ai]
provider = "openrouter"
model = "anthropic/claude-3.5-sonnet"
api_key = "${OPENROUTER_API_KEY}"

[ai.fallback]
enabled = true
mode = "no-ai"
```

## Consequences

### Positive
- Single API for multiple models
- Competitive pricing
- Easy model switching
- Good Rust ecosystem support

### Negative
- Additional dependency
- Requires API key management
- Network dependency

## Alternatives Considered

1. **Direct OpenAI API**: Rejected - vendor lock-in
2. **Direct Anthropic API**: Rejected - vendor lock-in
3. **Multiple direct APIs**: Rejected - too complex
4. **OpenRouter**: Accepted - unified, agnostic

## References
- [OpenRouter Docs](https://openrouter.ai/docs)
- [OpenRouter Models](https://openrouter.ai/models)
