use serde::{Deserialize, Serialize};

// --- Second-Order Damping ---

/// Damped emotional response — models oscillatory or smooth return to baseline.
///
/// - `zeta < 1.0` → underdamped (oscillatory, neurotic agents)
/// - `zeta = 1.0` → critically damped (fastest smooth return)
/// - `zeta > 1.0` → overdamped (sluggish, stoic agents)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DampedResponse {
    /// Current position (deviation from baseline).
    pub position: f32,
    /// Current velocity (rate of change).
    pub velocity: f32,
    /// Damping ratio (0.3 = oscillatory, 1.0 = smooth, 2.0 = sluggish).
    pub zeta: f32,
    /// Natural frequency (related to decay half-life).
    pub omega: f32,
}

impl DampedResponse {
    /// Create a new damped response.
    #[must_use]
    pub fn new(zeta: f32, omega: f32) -> Self {
        Self {
            position: 0.0,
            velocity: 0.0,
            zeta: zeta.max(0.01),
            omega: omega.max(0.01),
        }
    }

    /// Apply an impulse (e.g., emotional stimulus).
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn impulse(&mut self, force: f32) {
        self.velocity += force;
    }

    /// Step the simulation forward by `dt` seconds.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all))]
    pub fn step(&mut self, dt: f32) {
        let accel =
            -2.0 * self.zeta * self.omega * self.velocity - self.omega * self.omega * self.position;
        self.velocity += accel * dt;
        self.position += self.velocity * dt;
    }

    /// Whether the response has settled (position and velocity near zero).
    #[must_use]
    pub fn is_settled(&self, threshold: f32) -> bool {
        self.position.abs() < threshold && self.velocity.abs() < threshold
    }
}
