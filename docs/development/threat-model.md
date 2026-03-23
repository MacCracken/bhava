# Threat Model

## Trust Boundaries

- **Trusts**: calling application for authorization and input validation
- **Does not trust**: deserialized JSON, user-supplied text for sentiment analysis

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Prompt composition | Injection via trait/identity content | Consumer responsibility; bhava composes text, caller validates before sending to LLM |
| Mood vector values | Out-of-range f32 values | Clamped to [-1.0, 1.0] on every `set()` and `nudge()` |
| Decay computation | Time-based overflow | Chrono duration with safe arithmetic; negative elapsed is no-op |
| Sentiment analysis | Adversarial input | No-alloc keyword scan against bounded static lexicons; no regex, no recursion |
| Serde deserialization | Crafted JSON | Enum validation via serde derive; invalid variants rejected |
| AI client (opt-in) | Network I/O, endpoint injection | Feature-gated; not compiled by default; endpoint is caller-configured |
| Personality distance | NaN from degenerate inputs | All trait levels map to finite f32 values; sqrt of sum of squares is always finite |

## Unsafe Code

None. Zero `unsafe` blocks in the crate.

## Privilege Model

Bhava requires no elevated privileges. It performs no I/O in core modules. The `ai` feature adds outbound HTTP only.

## Design Principles

- Zero `unsafe` code
- No network I/O in core (ai feature opt-in)
- All public types `Send + Sync` compatible
- Minimal dependency surface (3 core deps: serde, thiserror, chrono)
- No secrets in logs or error messages
- `#[non_exhaustive]` on all public enums for forward compatibility
