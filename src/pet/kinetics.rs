use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WellnessDimensions {
    pub eye_comfort: f64,      // D_eye
    pub hydration: f64,        // D_hyd
    pub mobility: f64,         // D_mob
    pub mental_freshness: f64, // D_cog
    pub posture_comfort: f64,  // D_pos
}

impl Default for WellnessDimensions {
    fn default() -> Self {
        Self::new_full()
    }
}

impl WellnessDimensions {
    pub fn new_full() -> Self {
        Self {
            eye_comfort: 100.0,
            hydration: 100.0,
            mobility: 100.0,
            mental_freshness: 100.0,
            posture_comfort: 100.0,
        }
    }

    /// Composite Wellness Score:
    /// S = 0.25 D_mob + 0.25 D_cog + 0.20 D_eye + 0.15 D_hyd + 0.15 D_pos
    pub fn composite_score(&self) -> f64 {
        let s = 0.25 * self.mobility
            + 0.25 * self.mental_freshness
            + 0.20 * self.eye_comfort
            + 0.15 * self.hydration
            + 0.15 * self.posture_comfort;
        // Round to 3 decimal places to avoid standard IEEE-754 floating point accumulation noise
        ((s * 1000.0).round() / 1000.0).clamp(0.0, 100.0)
    }

    /// Decay kinetics per active minute:
    /// - Eye: 0.50 pts/min
    /// - Hydration: 0.25 pts/min
    /// - Mobility: 0.35 pts/min
    /// - Mental: 0.25 pts/min (normal) or 0.50 pts/min (high intensity)
    /// - Posture: 0.25 pts/min
    pub fn decay_minute(&mut self, is_high_intensity: bool) {
        let mental_decay = if is_high_intensity { 0.50 } else { 0.25 };

        self.eye_comfort = ((self.eye_comfort - 0.50) * 1000.0).round() / 1000.0;
        self.eye_comfort = self.eye_comfort.clamp(0.0, 100.0);

        self.hydration = ((self.hydration - 0.25) * 1000.0).round() / 1000.0;
        self.hydration = self.hydration.clamp(0.0, 100.0);

        self.mobility = ((self.mobility - 0.35) * 1000.0).round() / 1000.0;
        self.mobility = self.mobility.clamp(0.0, 100.0);

        self.mental_freshness = ((self.mental_freshness - mental_decay) * 1000.0).round() / 1000.0;
        self.mental_freshness = self.mental_freshness.clamp(0.0, 100.0);

        self.posture_comfort = ((self.posture_comfort - 0.25) * 1000.0).round() / 1000.0;
        self.posture_comfort = self.posture_comfort.clamp(0.0, 100.0);
    }

    /// Apply activity recovery quantum
    pub fn apply_recovery(&mut self, eye_delta: f64, hyd_delta: f64, mob_delta: f64, cog_delta: f64, pos_delta: f64) {
        self.eye_comfort = (self.eye_comfort + eye_delta).clamp(0.0, 100.0);
        self.hydration = (self.hydration + hyd_delta).clamp(0.0, 100.0);
        self.mobility = (self.mobility + mob_delta).clamp(0.0, 100.0);
        self.mental_freshness = (self.mental_freshness + cog_delta).clamp(0.0, 100.0);
        self.posture_comfort = (self.posture_comfort + pos_delta).clamp(0.0, 100.0);
    }

    /// Overnight reset (>= 6h idle)
    pub fn reset_overnight(&mut self) {
        self.eye_comfort = 100.0;
        self.hydration = 100.0;
        self.mobility = 100.0;
        self.mental_freshness = 100.0;
        self.posture_comfort = 100.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_and_composite_weights() {
        let dims = WellnessDimensions::new_full();
        assert_eq!(dims.composite_score(), 100.0);
    }

    #[test]
    fn test_mathematical_proof_timeline_without_breaks() {
        let mut dims = WellnessDimensions::new_full();

        // After 60 minutes
        for _ in 0..60 {
            dims.decay_minute(false);
        }
        assert_eq!(dims.composite_score(), 80.5);

        // After 120 minutes total (60 more min)
        for _ in 0..60 {
            dims.decay_minute(false);
        }
        assert_eq!(dims.composite_score(), 61.0);

        // After 180 minutes total (60 more min)
        for _ in 0..60 {
            dims.decay_minute(false);
        }
        assert_eq!(dims.composite_score(), 41.5);

        // At 200 min: Eye clamps at 0.0
        for _ in 0..20 {
            dims.decay_minute(false);
        }
        assert_eq!(dims.eye_comfort, 0.0);

        // After 240 minutes total (40 more min)
        for _ in 0..40 {
            dims.decay_minute(false);
        }
        assert_eq!(dims.composite_score(), 26.0);

        // At minute 266.7, score falls below 20.0
        for _ in 0..26 {
            dims.decay_minute(false);
        }
        // At 266 minutes: 240 + 26 = 266
        // S = 26.0 - 26 * 0.225 = 20.15
        assert!(dims.composite_score() > 20.0);

        dims.decay_minute(false);
        // At 267 minutes: S = 26.0 - 27 * 0.225 = 19.925 < 20.0 (enters Napping!)
        assert!(dims.composite_score() < 20.0);
    }
}
