# Threat Model

## Trust Boundaries

- **Trusts**: calling application for authorization and input validation
- **Does not trust**: deserialized JSON, user-supplied text for sentiment analysis, markdown input for personality import

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Prompt composition | Injection via trait/identity/spirit content | Consumer responsibility; bhava composes text, caller validates before sending to LLM |
| Mood vector values | Out-of-range f32 values | Clamped to [-1.0, 1.0] on every `set()` and `nudge()` |
| Decay computation | Time-based overflow | Chrono duration with safe arithmetic; negative elapsed is no-op |
| Sentiment analysis | Adversarial input | Single-pass keyword scan against bounded static lexicons; no regex, no recursion |
| Sentiment feedback | Mood manipulation via crafted AI responses | `scale` parameter (0.0–1.0) limits feedback strength; caller controls |
| Serde deserialization | Crafted JSON | Enum validation via serde derive; invalid variants rejected; missing traits default to Balanced |
| Markdown import | Malformed personality files | Unknown traits/levels silently default to Balanced; missing name returns `None` |
| AI prompt composition | System prompt injection via identity layers | Feature-gated; caller controls all content passed to `compose_system_prompt()` |
| Relationship graph | Unbounded growth | In-memory `Vec`-backed; consumer responsible for lifecycle management |
| Spirit content | Arbitrary text in passions/inspirations/pains | Consumer responsibility; bhava composes into markdown, caller validates |
| SQLite storage | SQL injection via IDs | Parameterized queries (`?1`) throughout; no string interpolation in SQL |
| SQLite storage | Data corruption | JSON serialization via serde; schema migrations via `CREATE TABLE IF NOT EXISTS` |
| Sentiment monitor | Unbounded buffer growth | Consumer calls `flush()` at stream end; `reset()` available for cleanup |
| Cosine similarity | Division by zero on zero vectors | Returns 1.0 by convention (two zero vectors are identical) |
| Personality distance | NaN from degenerate inputs | All trait levels map to finite f32 values; sqrt of sum of squares is always finite |
| Bridge modules (compat/psychology/sociology) | Upstream crate API changes | Bridge layer isolates bhava from dep changes; all bridges are infallible (clamp/fallback) |
| Bridge f32↔f64 conversion | Precision loss at type boundary | All conversions clamped at both ends; bhava uses f32, bodh/sangha use f64 |
| Bridge dep errors | Upstream computation failures | All bodh/sangha Result types absorbed with `.unwrap_or(fallback)`; bridge never propagates errors |

## Unsafe Code

None. Zero `unsafe` blocks in the crate.

## Privilege Model

Bhava requires no elevated privileges. It performs no I/O in core modules. The `ai` feature adds outbound HTTP only (via reqwest).

## Design Principles

- Zero `unsafe` code
- No network I/O in core (ai feature opt-in)
- All public types `Send + Sync` compatible
- Minimal dependency surface (3 core deps: serde, thiserror, chrono; bridge deps are optional and feature-gated)
- No secrets in logs or error messages
- `#[non_exhaustive]` on all public enums for forward compatibility
- `#[must_use]` on 37 pure functions to prevent accidental value drops
- All mood/affinity/trust values clamped to safe ranges
