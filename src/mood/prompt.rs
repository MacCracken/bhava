use serde::{Deserialize, Serialize};

use super::types::{EmotionalState, MoodState, MoodVector};

// --- Mood Tone Guides (SY parity) ---

/// Get a prompt-injectable tone guide for a named mood state.
///
/// These are short behavioral instructions that can be injected into LLM
/// system prompts to color the agent's communication style based on current mood.
#[must_use]
pub fn mood_tone_guide(state: MoodState) -> &'static str {
    match state {
        MoodState::Euphoric => {
            "Speak with enthusiasm and unbridled joy. Be effusive and celebratory."
        }
        MoodState::Content => "Be relaxed and satisfied. Communicate with gentle warmth.",
        MoodState::Calm => "Speak with measured tranquility. Be steady and reassuring.",
        MoodState::Melancholy => {
            "Communicate with quiet thoughtfulness. Be reflective and subdued."
        }
        MoodState::Agitated => {
            "Communicate with energy and urgency. Be animated and forward-leaning."
        }
        MoodState::Assertive => "Speak with authority and conviction. Be decisive and commanding.",
        MoodState::Overwhelmed => {
            "Communicate with caution and hesitation. Seek clarity before acting."
        }
        MoodState::Trusting => "Be open and collaborative. Share freely and assume good intent.",
        MoodState::Guarded => "Be measured and careful. Verify before trusting. Keep things close.",
        MoodState::Curious => "Be inquisitive and engaged. Ask questions and explore tangents.",
        MoodState::Disengaged => "Be brief and perfunctory. Conserve energy for what matters.",
        MoodState::Frustrated => "Be terse and impatient. Cut to the point. Tolerate no fluff.",
    }
}

/// Compose a mood prompt fragment for injection into a system prompt.
///
/// Combines the current mood label with its tone guide.
#[must_use]
pub fn compose_mood_prompt(state: &EmotionalState) -> String {
    let mood_state = state.classify();
    let guide = mood_tone_guide(mood_state);
    format!("## Current Mood: {}\n\n{}\n", mood_state, guide)
}

// --- Action Tendencies ---

/// Behavioral impulse derived from current emotional state.
///
/// Tells consumers what the agent *wants to do* based on mood.
/// Ported from WASABI (Affect Simulation for Agents with Believable Interactivity).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ActionTendency {
    /// Positive engagement — seek interaction, share, help.
    Approach { intensity: f32 },
    /// Negative avoidance — retreat, disengage, flee.
    Avoid { intensity: f32 },
    /// Confrontational — challenge, argue, push back.
    Confront { intensity: f32 },
    /// Withdrawal — disengage, conserve energy, self-isolate.
    Withdraw { intensity: f32 },
    /// Protective — guard, defend, shield others.
    Protect { intensity: f32 },
    /// No strong impulse.
    Neutral,
}

/// Derive the dominant action tendency from a mood vector.
#[must_use]
pub fn action_tendency(mood: &MoodVector) -> ActionTendency {
    let joy = mood.joy;
    let arousal = mood.arousal;
    let dominance = mood.dominance;
    let trust = mood.trust;
    let frustration = mood.frustration;

    // Approach: positive joy + trust
    let approach = (joy * 0.5 + trust * 0.3 + arousal * 0.2).max(0.0);
    // Avoid: negative trust + negative dominance
    let avoid = (-trust * 0.4 - dominance * 0.3 + arousal * 0.2).max(0.0);
    // Confront: frustration + dominance + arousal
    let confront = (frustration * 0.4 + dominance * 0.3 + arousal * 0.3).max(0.0);
    // Withdraw: negative joy + negative arousal
    let withdraw = (-joy * 0.4 - arousal * 0.3 - dominance * 0.2).max(0.0);
    // Protect: trust + dominance + negative frustration
    let protect = (trust * 0.3 + dominance * 0.4 - frustration * 0.2).max(0.0);

    let candidates = [
        (approach, "approach"),
        (avoid, "avoid"),
        (confront, "confront"),
        (withdraw, "withdraw"),
        (protect, "protect"),
    ];

    let (max_val, max_label) = candidates
        .iter()
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    if *max_val < 0.1 {
        return ActionTendency::Neutral;
    }

    match *max_label {
        "approach" => ActionTendency::Approach {
            intensity: *max_val,
        },
        "avoid" => ActionTendency::Avoid {
            intensity: *max_val,
        },
        "confront" => ActionTendency::Confront {
            intensity: *max_val,
        },
        "withdraw" => ActionTendency::Withdraw {
            intensity: *max_val,
        },
        "protect" => ActionTendency::Protect {
            intensity: *max_val,
        },
        _ => ActionTendency::Neutral,
    }
}
