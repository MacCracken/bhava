# Roadmap

## Scope

Bhava owns personality modeling, emotional state, and sentiment analysis for AGNOS agents and game NPCs.

**Bhava does NOT own:** natural language processing (hoosh), agent orchestration (daimon), game logic (joshua), desktop integration (aethersafha).

## Current Status

All planned 0.22.3 features are implemented. See [CHANGELOG.md](../../CHANGELOG.md) for details.

## Next Phase

### Medium Priority

- Plutchik compound emotions — 8 primary emotions + combination table (Joy+Trust=Love, Fear+Surprise=Awe, etc.)
- Second-order damping — oscillatory emotional response for neurotic agents (underdamped/overdamped decay)
- Emotional memory bank — somatic markers: agents remember how entities/events made them feel
- Emotion amplifier — personality modulates incoming emotion stimulus intensity (ALMA layer interaction)

### Low Priority

- Emotional volatility — variance of deviation over mood history
- Sentiment momentum — linear regression slope over recent valence
- Relationship reciprocity — symmetry metrics for A→B vs B→A
- Cause-tagged decay — active causes suppress decay; resolution accelerates it (FAtiMA-style)
