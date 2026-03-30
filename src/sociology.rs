//! Sangha sociology math integration — validated computational sociology backing bhava's social systems.
//!
//! Provides bridge functions between sangha's empirically-validated sociology formulas
//! and bhava's personality/emotion types. Sangha provides the mathematical foundations
//! (Hatfield contagion, network topology, Asch conformity, Ringelmann effect, Shapley values);
//! bhava provides the personality engine that composes them.
//!
//! Requires the `sociology` feature.
//!
//! # Layer Model
//!
//! ```text
//! ┌──────────────────────────────────┐
//! │  Bhava (Personality Engine)      │
//! │  Mood, relationships, groups     │
//! ├──────────────────────────────────┤
//! │  This module (bridge)            │
//! │  Sociology math → Bhava types    │
//! ├──────────────────────────────────┤
//! │  Sangha (Sociology Math)         │
//! │  Networks, contagion, coalitions │
//! └──────────────────────────────────┘
//! ```
//!
//! # Bridge Functions
//!
//! ## Emotional Contagion
//! - [`hatfield_mood_delta`] — Hatfield emotional mimicry on a social network
//! - [`emotional_convergence`] — check if a group's emotions have converged
//!
//! ## Mood Propagation
//! - [`mood_propagation`] — linear mood diffusion with decay
//! - [`contagion_threshold`] — critical transmission rate for epidemic spread
//!
//! ## Social Network
//! - [`clustering_coefficient`] — local clustering for a network node
//! - [`dunbar_layer`] — which Dunbar intimacy circle a connection count falls into
//!
//! ## Social Influence
//! - [`conformity_pressure`] — Asch conformity model
//! - [`social_proof_weight`] — adoption fraction as social proof
//!
//! ## Group Dynamics
//! - [`social_loafing`] — Ringelmann effort loss in groups
//! - [`groupthink_risk`] — Janis groupthink risk assessment
//!
//! ## Collective Decision
//! - [`wisdom_of_crowds`] — aggregate estimates via trimmed mean / median
//! - [`shapley_values`] — fair allocation in cooperative games

// ── Emotional Contagion ────────────────────────────────────────────────

/// Run one Hatfield emotional contagion step on a network of agents.
///
/// Takes valence values [0.0, 1.0] per agent, a weighted adjacency list,
/// Hatfield config (mimicry_rate, feedback_strength), and a timestep.
/// Returns updated valence values after mimicry. Falls back to input on error.
///
/// ```
/// use bhava::sociology::hatfield_mood_delta;
///
/// let states = vec![0.8, 0.2, 0.5];
/// let adjacency = vec![
///     vec![(1, 0.5), (2, 0.3)],
///     vec![(0, 0.5), (2, 0.4)],
///     vec![(0, 0.3), (1, 0.4)],
/// ];
/// let updated = hatfield_mood_delta(&states, &adjacency, (1.0, 0.5), 0.1);
/// assert_eq!(updated.len(), 3);
/// // Agent 1 (low valence) should move toward neighbors
/// assert!(updated[1] > 0.2);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn hatfield_mood_delta(
    states: &[f32],
    adjacency: &[Vec<(usize, f64)>],
    config: (f64, f64),
    dt: f64,
) -> Vec<f32> {
    let fallback = || states.to_vec();
    let emotional_states: Vec<sangha::contagion::EmotionalState> = states
        .iter()
        .map(|&v| sangha::contagion::EmotionalState::new(f64::from(v).clamp(0.0, 1.0), 0.8))
        .collect::<core::result::Result<Vec<_>, _>>()
        .ok()
        .unwrap_or_default();
    if emotional_states.is_empty() && !states.is_empty() {
        return fallback();
    }
    let Ok(hatfield_config) =
        sangha::contagion::HatfieldConfig::new(config.0.max(0.0), config.1.max(0.0))
    else {
        return fallback();
    };
    sangha::contagion::hatfield_contagion_step(&emotional_states, adjacency, &hatfield_config, dt)
        .map(|result| {
            result
                .iter()
                .map(|s| (s.valence as f32).clamp(0.0, 1.0))
                .collect()
        })
        .unwrap_or_else(|_| states.to_vec())
}

/// Check whether a group of agents has reached emotional convergence.
///
/// Returns true if all valence values are within `epsilon` of the group mean.
/// Falls back to false on error.
///
/// ```
/// use bhava::sociology::emotional_convergence;
///
/// assert!(emotional_convergence(&[0.5, 0.51, 0.49], 0.05));
/// assert!(!emotional_convergence(&[0.1, 0.9], 0.05));
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn emotional_convergence(states: &[f32], epsilon: f64) -> bool {
    let emotional_states: Vec<sangha::contagion::EmotionalState> = states
        .iter()
        .filter_map(|&v| {
            sangha::contagion::EmotionalState::new(f64::from(v).clamp(0.0, 1.0), 0.5).ok()
        })
        .collect();
    if emotional_states.len() != states.len() {
        return false;
    }
    sangha::contagion::emotional_convergence(&emotional_states, epsilon).unwrap_or(false)
}

// ── Mood Propagation ───────────────────────────────────────────────────

/// Run linear mood diffusion with decay toward neutral.
///
/// Takes mood values per agent, weighted adjacency list, decay rate, and timestep.
/// Moods diffuse toward neighbors and decay toward 0.5 (neutral).
/// Falls back to input on error.
///
/// ```
/// use bhava::sociology::mood_propagation;
///
/// let moods = vec![0.9, 0.1, 0.5];
/// let adjacency = vec![
///     vec![(1, 0.5)],
///     vec![(0, 0.5)],
///     vec![],
/// ];
/// let updated = mood_propagation(&moods, &adjacency, 0.1, 0.1);
/// assert_eq!(updated.len(), 3);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn mood_propagation(
    moods: &[f64],
    adjacency: &[Vec<(usize, f64)>],
    decay: f64,
    dt: f64,
) -> Vec<f64> {
    sangha::contagion::mood_propagation(moods, adjacency, decay, dt)
        .unwrap_or_else(|_| moods.to_vec())
}

/// Compute the epidemic threshold for emotional contagion on a network.
///
/// Returns the critical transmission rate below which contagion dies out.
/// Based on the largest eigenvalue of the adjacency matrix.
/// Falls back to 1.0 (conservative — high threshold, hard to spread) on error.
///
/// ```
/// use bhava::sociology::contagion_threshold;
///
/// let adjacency = vec![
///     vec![(1, 1.0), (2, 1.0)],
///     vec![(0, 1.0), (2, 1.0)],
///     vec![(0, 1.0), (1, 1.0)],
/// ];
/// let threshold = contagion_threshold(&adjacency);
/// assert!(threshold > 0.0 && threshold <= 1.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn contagion_threshold(adjacency: &[Vec<(usize, f64)>]) -> f64 {
    sangha::contagion::contagion_threshold(adjacency)
        .unwrap_or(1.0)
        .clamp(0.0, f64::MAX)
}

// ── Social Network ─────────────────────────────────────────────────────

/// Compute the local clustering coefficient for a node in a social network.
///
/// Measures how connected a node's neighbors are to each other (0.0 to 1.0).
/// Returns 0.0 on error or if the node has fewer than 2 neighbors.
///
/// ```
/// use bhava::sociology::clustering_coefficient;
///
/// // Triangle: all three nodes connected — perfect clustering
/// let edges = vec![
///     vec![(1, 1.0), (2, 1.0)],
///     vec![(0, 1.0), (2, 1.0)],
///     vec![(0, 1.0), (1, 1.0)],
/// ];
/// let cc = clustering_coefficient(&edges, 0);
/// assert!((cc - 1.0).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn clustering_coefficient(edges: &[Vec<(usize, f64)>], node: usize) -> f64 {
    let mut network = sangha::network::SocialNetwork::new(edges.len());
    for (i, neighbors) in edges.iter().enumerate() {
        for &(j, w) in neighbors {
            // add_edge is bidirectional, so only add once per pair
            if i <= j {
                let _ = network.add_edge(i, j, w);
            }
        }
    }
    sangha::network::clustering_coefficient(&network, node)
        .unwrap_or(0.0)
        .clamp(0.0, 1.0)
}

/// Determine which Dunbar intimacy layer a connection count falls into.
///
/// Dunbar layers: [5, 15, 50, 150]. Returns the layer index (0-3)
/// or 4 if the count exceeds 150.
///
/// - Layer 0 (≤5): intimate support group
/// - Layer 1 (≤15): sympathy group
/// - Layer 2 (≤50): close friends
/// - Layer 3 (≤150): casual friends (Dunbar's number)
/// - Layer 4 (>150): acquaintances
///
/// ```
/// use bhava::sociology::dunbar_layer;
///
/// assert_eq!(dunbar_layer(3), 0);
/// assert_eq!(dunbar_layer(10), 1);
/// assert_eq!(dunbar_layer(100), 3);
/// assert_eq!(dunbar_layer(200), 4);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn dunbar_layer(connections: usize) -> usize {
    sangha::network::DUNBAR_LAYERS
        .iter()
        .position(|&limit| connections <= limit)
        .unwrap_or(sangha::network::DUNBAR_LAYERS.len())
}

// ── Social Influence ───────────────────────────────────────────────────

/// Determine whether an individual conforms under group pressure (Asch model).
///
/// Returns true if the individual conforms. Higher group pressure and larger
/// group size increase conformity; higher individual conviction resists it.
/// Falls back to false (no conformity) on error.
///
/// ```
/// use bhava::sociology::conformity_pressure;
///
/// // Low conviction + high pressure + large group → conform
/// assert!(conformity_pressure(0.2, 0.9, 5));
///
/// // High conviction → resist
/// assert!(!conformity_pressure(0.9, 0.5, 3));
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn conformity_pressure(conviction: f64, group_pressure: f64, group_size: usize) -> bool {
    sangha::influence::conformity_threshold(
        conviction.clamp(0.0, 1.0),
        group_pressure.clamp(0.0, 1.0),
        group_size,
    )
    .unwrap_or(false)
}

/// Compute social proof weight as the fraction of adopters in a population.
///
/// Returns adopters / population as a value in [0.0, 1.0].
/// Falls back to 0.0 on error (e.g., zero population).
///
/// ```
/// use bhava::sociology::social_proof_weight;
///
/// let weight = social_proof_weight(30, 100);
/// assert!((weight - 0.3).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn social_proof_weight(adopters: usize, population: usize) -> f64 {
    sangha::influence::social_proof_weight(adopters, population)
        .unwrap_or(0.0)
        .clamp(0.0, 1.0)
}

// ── Group Dynamics ─────────────────────────────────────────────────────

/// Compute per-person effort under social loafing (Ringelmann effect).
///
/// Larger groups produce less effort per member. `loss_factor` controls
/// the rate of loss (typically 0.1). Returns effort ≥ 10% of individual.
/// Falls back to `individual_effort` on error.
///
/// ```
/// use bhava::sociology::social_loafing;
///
/// let solo = social_loafing(1, 1.0, 0.1);
/// let group = social_loafing(5, 1.0, 0.1);
/// assert!(group < solo);
/// assert!(group > 0.0);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn social_loafing(group_size: usize, individual_effort: f64, loss_factor: f64) -> f64 {
    sangha::group::social_loafing(group_size, individual_effort, loss_factor)
        .unwrap_or(individual_effort)
}

/// Assess groupthink risk using Janis's model.
///
/// Inputs are cohesion, insulation, and leader bias (each 0.0 to 1.0).
/// Returns a risk score in [0.0, 1.0]. Falls back to 0.0 on error.
///
/// ```
/// use bhava::sociology::groupthink_risk;
///
/// // High cohesion + high insulation + strong leader bias → high risk
/// let risk = groupthink_risk(0.9, 0.8, 0.9);
/// assert!(risk > 0.7);
///
/// // Low everything → low risk
/// let low = groupthink_risk(0.1, 0.1, 0.1);
/// assert!(low < 0.3);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
#[inline]
pub fn groupthink_risk(cohesion: f64, insulation: f64, leader_bias: f64) -> f64 {
    sangha::group::groupthink_risk(
        cohesion.clamp(0.0, 1.0),
        insulation.clamp(0.0, 1.0),
        leader_bias.clamp(0.0, 1.0),
    )
    .unwrap_or(0.0)
    .clamp(0.0, 1.0)
}

// ── Collective Decision ────────────────────────────────────────────────

/// Aggregate estimates using wisdom of crowds.
///
/// Method can be `"mean"`, `"median"`, or `"trimmed"` (trimmed mean, removes
/// top/bottom 10%). Falls back to simple mean on unrecognized method.
/// Returns 0.0 on error or empty input.
///
/// ```
/// use bhava::sociology::wisdom_of_crowds;
///
/// let estimates = vec![42.0, 45.0, 38.0, 50.0, 40.0];
/// let consensus = wisdom_of_crowds(&estimates, "mean");
/// assert!((consensus - 43.0).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn wisdom_of_crowds(estimates: &[f64], method: &str) -> f64 {
    let agg = match method {
        "median" => sangha::collective::AggregationMethod::Median,
        "trimmed" => sangha::collective::AggregationMethod::TrimmedMean,
        _ => sangha::collective::AggregationMethod::Mean,
    };
    sangha::collective::wisdom_of_crowds(estimates, agg).unwrap_or(0.0)
}

/// Compute Shapley values for a cooperative game.
///
/// Takes the number of players and a coalition value vector (indexed by bitmask,
/// length 2^player_count). Returns per-player fair allocation.
/// Falls back to equal split on error. Maximum 20 players.
///
/// ```
/// use bhava::sociology::shapley_values;
///
/// // 2-player game: each alone = 1, together = 3
/// let values = vec![0.0, 1.0, 1.0, 3.0];
/// let shapley = shapley_values(2, &values);
/// assert_eq!(shapley.len(), 2);
/// // Each player gets 1 (alone) + 0.5 * (3 - 1) = 2.0 — wait, actually
/// // Shapley for symmetric: each gets 1.5
/// assert!((shapley[0] - 1.5).abs() < 0.01);
/// ```
#[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
#[must_use]
pub fn shapley_values(player_count: usize, values: &[f64]) -> Vec<f64> {
    let equal_fallback = || {
        if player_count == 0 {
            vec![]
        } else {
            let total = values.last().copied().unwrap_or(0.0);
            vec![total / player_count as f64; player_count]
        }
    };
    let Ok(game) = sangha::coalition::CoalitionGame::new(player_count, values.to_vec()) else {
        return equal_fallback();
    };
    sangha::coalition::shapley_value(&game)
        .map(|sv| sv.values)
        .unwrap_or_else(|_| equal_fallback())
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Emotional Contagion ────────────────────────────────────────────

    #[test]
    fn hatfield_converges_moods() {
        let states = vec![0.9_f32, 0.1, 0.5];
        let adjacency = vec![
            vec![(1, 0.5), (2, 0.3)],
            vec![(0, 0.5), (2, 0.4)],
            vec![(0, 0.3), (1, 0.4)],
        ];
        let updated = hatfield_mood_delta(&states, &adjacency, (1.0, 0.5), 0.1);
        assert_eq!(updated.len(), 3);
        // Low-valence agent should move up toward neighbors
        assert!(updated[1] > 0.1);
    }

    #[test]
    fn hatfield_empty_network() {
        let result = hatfield_mood_delta(&[], &[], (1.0, 0.5), 0.1);
        assert!(result.is_empty());
    }

    #[test]
    fn convergence_true_when_close() {
        assert!(emotional_convergence(&[0.5, 0.51, 0.49], 0.05));
    }

    #[test]
    fn convergence_false_when_spread() {
        assert!(!emotional_convergence(&[0.1, 0.9], 0.05));
    }

    // ── Mood Propagation ───────────────────────────────────────────────

    #[test]
    fn propagation_returns_correct_length() {
        let moods = vec![0.9, 0.1, 0.5];
        let adjacency = vec![vec![(1, 0.5)], vec![(0, 0.5)], vec![]];
        let updated = mood_propagation(&moods, &adjacency, 0.1, 0.1);
        assert_eq!(updated.len(), 3);
    }

    #[test]
    fn propagation_empty() {
        let result = mood_propagation(&[], &[], 0.1, 0.1);
        assert!(result.is_empty());
    }

    #[test]
    fn threshold_positive_for_connected() {
        let adjacency = vec![
            vec![(1, 1.0), (2, 1.0)],
            vec![(0, 1.0), (2, 1.0)],
            vec![(0, 1.0), (1, 1.0)],
        ];
        let t = contagion_threshold(&adjacency);
        assert!(t > 0.0, "threshold should be positive: {t}");
    }

    // ── Social Network ─────────────────────────────────────────────────

    #[test]
    fn clustering_triangle_is_one() {
        let edges = vec![
            vec![(1, 1.0), (2, 1.0)],
            vec![(0, 1.0), (2, 1.0)],
            vec![(0, 1.0), (1, 1.0)],
        ];
        let cc = clustering_coefficient(&edges, 0);
        assert!((cc - 1.0).abs() < 0.01);
    }

    #[test]
    fn clustering_star_is_zero() {
        // Star: center (0) connected to 1,2,3 — but 1,2,3 not connected to each other
        let edges = vec![
            vec![(1, 1.0), (2, 1.0), (3, 1.0)],
            vec![(0, 1.0)],
            vec![(0, 1.0)],
            vec![(0, 1.0)],
        ];
        let cc = clustering_coefficient(&edges, 0);
        assert!((cc - 0.0).abs() < 0.01);
    }

    #[test]
    fn dunbar_layer_intimate() {
        assert_eq!(dunbar_layer(3), 0);
        assert_eq!(dunbar_layer(5), 0);
    }

    #[test]
    fn dunbar_layer_sympathy() {
        assert_eq!(dunbar_layer(10), 1);
        assert_eq!(dunbar_layer(15), 1);
    }

    #[test]
    fn dunbar_layer_casual() {
        assert_eq!(dunbar_layer(100), 3);
        assert_eq!(dunbar_layer(150), 3);
    }

    #[test]
    fn dunbar_layer_acquaintance() {
        assert_eq!(dunbar_layer(200), 4);
    }

    // ── Social Influence ───────────────────────────────────────────────

    #[test]
    fn conformity_low_conviction_conforms() {
        assert!(conformity_pressure(0.2, 0.9, 5));
    }

    #[test]
    fn conformity_high_conviction_resists() {
        assert!(!conformity_pressure(0.9, 0.5, 3));
    }

    #[test]
    fn social_proof_fraction() {
        let weight = social_proof_weight(30, 100);
        assert!((weight - 0.3).abs() < 0.01);
    }

    #[test]
    fn social_proof_zero_population() {
        let weight = social_proof_weight(0, 0);
        assert!((weight - 0.0).abs() < 0.01);
    }

    // ── Group Dynamics ─────────────────────────────────────────────────

    #[test]
    fn loafing_group_less_than_solo() {
        let solo = social_loafing(1, 1.0, 0.1);
        let group = social_loafing(5, 1.0, 0.1);
        assert!(
            group < solo,
            "group effort {group} should be less than solo {solo}"
        );
    }

    #[test]
    fn loafing_never_zero() {
        let effort = social_loafing(100, 1.0, 0.1);
        assert!(effort > 0.0);
    }

    #[test]
    fn groupthink_high_risk() {
        let risk = groupthink_risk(0.9, 0.8, 0.9);
        assert!(risk > 0.7, "high inputs should give high risk: {risk}");
    }

    #[test]
    fn groupthink_low_risk() {
        let risk = groupthink_risk(0.1, 0.1, 0.1);
        assert!(risk < 0.3, "low inputs should give low risk: {risk}");
    }

    // ── Collective Decision ────────────────────────────────────────────

    #[test]
    fn wisdom_mean() {
        let estimates = vec![42.0, 45.0, 38.0, 50.0, 40.0];
        let result = wisdom_of_crowds(&estimates, "mean");
        assert!((result - 43.0).abs() < 0.01);
    }

    #[test]
    fn wisdom_empty() {
        let result = wisdom_of_crowds(&[], "mean");
        assert!((result - 0.0).abs() < 0.01);
    }

    #[test]
    fn shapley_symmetric_game() {
        // 2 players: alone=1, together=3
        let values = vec![0.0, 1.0, 1.0, 3.0];
        let sv = shapley_values(2, &values);
        assert_eq!(sv.len(), 2);
        assert!((sv[0] - 1.5).abs() < 0.01, "player 0 shapley: {}", sv[0]);
        assert!((sv[1] - 1.5).abs() < 0.01, "player 1 shapley: {}", sv[1]);
    }

    #[test]
    fn shapley_empty_game() {
        let sv = shapley_values(0, &[]);
        assert!(sv.is_empty());
    }
}
