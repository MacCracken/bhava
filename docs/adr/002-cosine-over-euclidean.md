# ADR-002: Cosine Similarity for Compatibility Scoring

**Status:** Accepted
**Date:** 2026-03-23

## Context

Compatibility between two personality profiles was originally computed using inverse normalized Euclidean distance. This measures "how far apart" two profiles are in trait space.

Problem: two profiles with the same *pattern* but different *intensities* (e.g., mildly warm vs very warm) scored as incompatible despite having the same behavioral direction.

## Decision

Replace Euclidean-based compatibility with cosine similarity, which measures the angle between two trait vectors in 15-dimensional space.

```
cos(A, B) = (A · B) / (|A| × |B|)
mapped: similarity = (cos + 1) / 2
```

Zero vectors (all-Balanced profiles) return 1.0 by convention.

## Consequences

- **Behavioral correctness**: Same-direction profiles score >0.9 regardless of intensity
- **Orthogonal detection**: Unrelated profiles score ~0.5 (instead of high via Euclidean)
- **Opposite detection**: Opposing patterns score ~0.0
- **Performance**: +0.9ns per call (one extra sqrt) — negligible at 14ns total
- **Euclidean preserved**: `distance()` still available for raw geometric distance
