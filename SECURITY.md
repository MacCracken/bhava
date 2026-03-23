# Security Policy

## Scope

Bhava is a personality and emotional state library for AI agents and game NPCs. The core library (traits, mood, archetype, sentiment, spirit, relationship) performs no network I/O and no file I/O. The optional `ai` feature adds HTTP client dependencies for agnosai/hoosh integration.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Personality prompt composition | Prompt injection via trait/identity content | Consumer responsibility; bhava composes, caller validates |
| Mood vector values | Out-of-range f32 values | Clamped to [-1.0, 1.0] on set |
| Sentiment keyword matching | Adversarial input | No-alloc keyword scan; bounded static lexicons |
| Serde deserialization | Crafted JSON | Enum validation via serde derive |
| AI prompt composition (opt-in) | System prompt injection | Feature-gated; caller controls identity/spirit content |
| Sentiment feedback loop (opt-in) | Mood manipulation via crafted responses | Scale parameter (0.0-1.0) limits feedback strength |
| Relationship graph | Unbounded growth | Consumer responsibility; bhava provides in-memory graph |
| Markdown deserialization | Malformed input | Unknown traits/levels default to Balanced; missing name returns None |
| Decay computation | Time-based overflow | Chrono duration with safe arithmetic |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.23.x | Yes |
| < 0.23 | No |
| < 0.1 | No |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- Zero `unsafe` code
- No network I/O in core (ai feature opt-in)
- All public types `Send + Sync` compatible
- Minimal dependency surface
- Structured logging via `tracing` (no secrets in logs)
