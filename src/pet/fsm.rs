use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalState {
    Thriving,
    Happy,
    Tired,
    Sleepy,
    Napping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionTier {
    Juvenile = 0, // Tier 0
    Adult = 1,    // Tier 1
    Luminary = 2, // Tier 2
}

#[derive(Debug, Clone)]
pub struct PetStateMachine {
    pub current_state: EmotionalState,
    pub candidate_state: EmotionalState,
    pub candidate_streak: u32,
    pub evolution_tier: EvolutionTier,
    pub current_level: u32,
    pub cumulative_xp: u64,
    pub active_habit_days: u32,
}

impl Default for PetStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl PetStateMachine {
    pub fn new() -> Self {
        Self {
            current_state: EmotionalState::Thriving,
            candidate_state: EmotionalState::Thriving,
            candidate_streak: 0,
            evolution_tier: EvolutionTier::Juvenile,
            current_level: 1,
            cumulative_xp: 0,
            active_habit_days: 0,
        }
    }

    /// Map composite score S to nominal emotional state:
    /// - Thriving: S >= 80.0
    /// - Happy: 60.0 <= S < 80.0
    /// - Tired: 40.0 <= S < 60.0
    /// - Sleepy: 20.0 <= S < 40.0
    /// - Napping: S < 20.0
    pub fn nominal_state_for_score(score: f64) -> EmotionalState {
        if score >= 80.0 {
            EmotionalState::Thriving
        } else if score >= 60.0 {
            EmotionalState::Happy
        } else if score >= 40.0 {
            EmotionalState::Tired
        } else if score >= 20.0 {
            EmotionalState::Sleepy
        } else {
            EmotionalState::Napping
        }
    }

    /// Evaluate 1-minute tick with hysteresis smoothing:
    /// Score must remain across a threshold boundary for two consecutive
    /// 1-minute evaluations before changing state.
    pub fn evaluate_score_tick(&mut self, score: f64) -> EmotionalState {
        let nominal = Self::nominal_state_for_score(score);

        if nominal == self.current_state {
            self.candidate_state = nominal;
            self.candidate_streak = 0;
        } else if nominal == self.candidate_state {
            self.candidate_streak += 1;
            // 2 consecutive evaluations required to transition
            if self.candidate_streak >= 2 {
                self.current_state = nominal;
                self.candidate_streak = 0;
            }
        } else {
            self.candidate_state = nominal;
            self.candidate_streak = 1;
        }

        self.current_state
    }

    /// Evaluate evolutionary tier:
    /// - Tier 0 -> Tier 1: Level 10 (>= 2402 cumulative XP) and >= 25 active habit days.
    /// - Tier 1 -> Tier 2: Level 20 (>= 7200 cumulative XP) and >= 75 active habit days.
    pub fn check_evolution(&mut self) -> Option<EvolutionTier> {
        let prev_tier = self.evolution_tier;

        if self.evolution_tier == EvolutionTier::Juvenile
            && self.current_level >= 10
            && self.cumulative_xp >= 2402
            && self.active_habit_days >= 25
        {
            self.evolution_tier = EvolutionTier::Adult;
        } else if self.evolution_tier == EvolutionTier::Adult
            && self.current_level >= 20
            && self.cumulative_xp >= 7200
            && self.active_habit_days >= 75
        {
            self.evolution_tier = EvolutionTier::Luminary;
        }

        if self.evolution_tier != prev_tier {
            Some(self.evolution_tier)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hysteresis_smoothing() {
        let mut fsm = PetStateMachine::new();
        assert_eq!(fsm.current_state, EmotionalState::Thriving);

        // Score drops to 75 (Happy territory)
        assert_eq!(fsm.evaluate_score_tick(75.0), EmotionalState::Thriving);
        // Second consecutive evaluation at 75 -> transitions to Happy
        assert_eq!(fsm.evaluate_score_tick(75.0), EmotionalState::Happy);

        // Fluctuation to 85 for just 1 evaluation should not transition back to Thriving
        assert_eq!(fsm.evaluate_score_tick(85.0), EmotionalState::Happy);
        // Second evaluation at 85 -> transitions back to Thriving
        assert_eq!(fsm.evaluate_score_tick(85.0), EmotionalState::Thriving);
    }

    #[test]
    fn test_evolution_progression() {
        let mut fsm = PetStateMachine::new();
        assert_eq!(fsm.evolution_tier, EvolutionTier::Juvenile);

        fsm.current_level = 10;
        fsm.cumulative_xp = 2402;
        fsm.active_habit_days = 24;
        assert_eq!(fsm.check_evolution(), None);

        fsm.active_habit_days = 25;
        assert_eq!(fsm.check_evolution(), Some(EvolutionTier::Adult));

        fsm.current_level = 20;
        fsm.cumulative_xp = 7200;
        fsm.active_habit_days = 75;
        assert_eq!(fsm.check_evolution(), Some(EvolutionTier::Luminary));
    }
}
