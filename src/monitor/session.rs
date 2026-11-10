use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Paused,
    Completed,
}

#[derive(Debug, Clone)]
pub struct SessionLifecycleTracker {
    pub session_id: String,
    pub session_state: SessionState,
    pub idle_seconds: u64,
    pub continuous_idle_seconds: u64,
    pub continuous_ocular_exposure_sec: u64,
    pub continuous_sedentary_sec: u64,
    pub session_elapsed_sec: u64,
}

impl Default for SessionLifecycleTracker {
    fn default() -> Self {
        Self::new("default_session".to_string())
    }
}

impl SessionLifecycleTracker {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            session_state: SessionState::Active,
            idle_seconds: 0,
            continuous_idle_seconds: 0,
            continuous_ocular_exposure_sec: 0,
            continuous_sedentary_sec: 0,
            session_elapsed_sec: 0,
        }
    }

    /// Advance time by 1 second with input status
    pub fn tick_1s(&mut self, had_input: bool) {
        self.session_elapsed_sec += 1;

        if had_input {
            // Check if we were idle before this tick
            if self.idle_seconds >= 20 {
                // Visual rest pause occurred! Resets Continuous Ocular Exposure
                self.continuous_ocular_exposure_sec = 0;
            }
            if self.idle_seconds >= 120 {
                // Physical break occurred! Resets Continuous Sedentary Session
                self.continuous_sedentary_sec = 0;
            }

            self.idle_seconds = 0;
            self.continuous_idle_seconds = 0;
            self.session_state = SessionState::Active;

            self.continuous_ocular_exposure_sec += 1;
            self.continuous_sedentary_sec += 1;
        } else {
            self.idle_seconds += 1;
            self.continuous_idle_seconds += 1;

            if self.idle_seconds < 20 {
                self.continuous_ocular_exposure_sec += 1;
            }
            if self.idle_seconds < 120 {
                self.continuous_sedentary_sec += 1;
            }

            // Session state transitions:
            // Continuous idle 30m to 120m (1800s to 7200s): Paused Session
            // Continuous idle > 120m (7200s): Session Terminated / Completed
            if self.continuous_idle_seconds > 7200 {
                self.session_state = SessionState::Completed;
            } else if self.continuous_idle_seconds >= 1800 {
                self.session_state = SessionState::Paused;
            }
        }
    }

    /// Explicit visual rest completion (e.g. 20-20-20)
    pub fn reset_ocular_counter(&mut self) {
        self.continuous_ocular_exposure_sec = 0;
    }

    /// Explicit physical break completion (e.g. stretch, walk)
    pub fn reset_sedentary_counter(&mut self) {
        self.continuous_sedentary_sec = 0;
    }

    /// Check if overnight reset threshold has been reached (>= 6 hours continuous idle = 21600s)
    pub fn requires_overnight_reset(&self) -> bool {
        self.continuous_idle_seconds >= 21600
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_exposure_counters() {
        let mut tracker = SessionLifecycleTracker::new("s1".into());

        // Work for 30s
        for _ in 0..30 {
            tracker.tick_1s(true);
        }
        assert_eq!(tracker.continuous_ocular_exposure_sec, 30);
        assert_eq!(tracker.continuous_sedentary_sec, 30);

        // Idle for 25s (exceeds 20s visual rest)
        for _ in 0..25 {
            tracker.tick_1s(false);
        }
        // Then user inputs again
        tracker.tick_1s(true);
        // Ocular should have reset to 1
        assert_eq!(tracker.continuous_ocular_exposure_sec, 1);
        // Sedentary counter did NOT reset (< 120s)
        assert!(tracker.continuous_sedentary_sec > 30);
    }

    #[test]
    fn test_session_state_transitions() {
        let mut tracker = SessionLifecycleTracker::new("s2".into());
        assert_eq!(tracker.session_state, SessionState::Active);

        // Advance 1800s idle -> Paused
        for _ in 0..1800 {
            tracker.tick_1s(false);
        }
        assert_eq!(tracker.session_state, SessionState::Paused);

        // Advance past 7200s -> Completed
        for _ in 1800..7201 {
            tracker.tick_1s(false);
        }
        assert_eq!(tracker.session_state, SessionState::Completed);
    }
}
