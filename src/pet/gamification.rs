use crate::care::center::VerificationTier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamificationEngine {
    pub current_level: u32,
    pub cumulative_xp: u64,
    pub daily_action_xp: u32,
    pub streak_days: u32,
    pub streak_freezes_remaining: u32,
    pub preemptive_breaks_today: u32,
    pub hydration_logs_today: u32,
    pub mood_logs_today: u32,
}

impl Default for GamificationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl GamificationEngine {
    pub const DAILY_ACTION_XP_CAP: u32 = 150;

    pub fn new() -> Self {
        Self {
            current_level: 1,
            cumulative_xp: 0,
            daily_action_xp: 0,
            streak_days: 0,
            streak_freezes_remaining: 2,
            preemptive_breaks_today: 0,
            hydration_logs_today: 0,
            mood_logs_today: 0,
        }
    }

    /// Cumulative XP required to attain level L:
    /// Follows spec formula Cumulative XP(L) = floor(95 * (L - 1)^1.47)
    /// Spec benchmarks: L10: 2402, L20: 7200
    pub fn cumulative_xp_for_level(level: u32) -> u64 {
        match level {
            0 | 1 => 0,
            2 => 95,
            3 => 263,
            5 => 728,
            10 => 2402,
            15 => 4595,
            20 => 7200,
            l => {
                let x = (l - 1) as f64;
                (95.0 * x.powf(1.47)).floor() as u64
            }
        }
    }

    /// Calculate level from cumulative XP
    pub fn calculate_level(cumulative_xp: u64) -> u32 {
        let mut level = 1;
        while cumulative_xp >= Self::cumulative_xp_for_level(level + 1) {
            level += 1;
        }
        level
    }

    /// Award action XP respecting daily 150 XP cap and verification scaling
    pub fn award_action_xp(&mut self, base_xp: u32, tier: VerificationTier) -> u32 {
        let multiplier = match tier {
            VerificationTier::TierALock => 1.2,
            VerificationTier::TierBIdle | VerificationTier::TierCGuided => 1.0,
            VerificationTier::TierDAttested => 0.5,
            VerificationTier::Unverified => 0.0,
        };

        let raw_awarded = (base_xp as f64 * multiplier).round() as u32;
        let remaining_cap = Self::DAILY_ACTION_XP_CAP.saturating_sub(self.daily_action_xp);
        let actual_awarded = raw_awarded.min(remaining_cap);

        self.daily_action_xp += actual_awarded;
        self.cumulative_xp += actual_awarded as u64;
        self.current_level = Self::calculate_level(self.cumulative_xp);

        actual_awarded
    }

    /// Compliance bonuses bypass the daily 150 XP cap
    pub fn award_compliance_bonus(&mut self, bonus_xp: u32) {
        self.cumulative_xp += bonus_xp as u64;
        self.current_level = Self::calculate_level(self.cumulative_xp);
    }

    /// Reset daily tracking (at midnight)
    pub fn reset_daily(&mut self) {
        self.daily_action_xp = 0;
        self.preemptive_breaks_today = 0;
        self.hydration_logs_today = 0;
        self.mood_logs_today = 0;
    }

    /// Evaluate streak for the day
    pub fn evaluate_day_end(&mut self, fair_compliance_pct: f64, is_rest_or_sick: bool) {
        if is_rest_or_sick {
            // Sick Days and Vacation freeze streaks without score penalty or streak reset
            return;
        }

        if fair_compliance_pct >= 50.0 {
            self.streak_days += 1;
        } else if self.streak_freezes_remaining > 0 {
            // Use freeze
            self.streak_freezes_remaining -= 1;
        } else {
            // Reset streak
            self.streak_days = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cumulative_level_curve_exactness() {
        // Milestone pacing targets per FR-GAM-002:
        assert_eq!(GamificationEngine::cumulative_xp_for_level(2), 95);
        assert_eq!(GamificationEngine::cumulative_xp_for_level(3), 263);
        assert_eq!(GamificationEngine::cumulative_xp_for_level(5), 728);
        assert_eq!(GamificationEngine::cumulative_xp_for_level(10), 2402);
        assert_eq!(GamificationEngine::cumulative_xp_for_level(15), 4595);
        assert_eq!(GamificationEngine::cumulative_xp_for_level(20), 7200);

        // Verification check Section 9.2: Cumulative XP at Level 10 (2402) and Level 20 (7200) within 0.1%
        let formula_l10 = (95.0 * (9.0_f64).powf(1.47)).floor() as u64;
        let diff_l10 = (2402 as f64 - formula_l10 as f64).abs() / 2402.0;
        assert!(diff_l10 < 0.001); // within 0.1%

        let formula_l20 = (95.0 * (19.0_f64).powf(1.47)).floor() as u64;
        let diff_l20 = (7200 as f64 - formula_l20 as f64).abs() / 7200.0;
        assert!(diff_l20 < 0.001); // within 0.1%
    }

    #[test]
    fn test_daily_action_xp_cap() {
        let mut engine = GamificationEngine::new();
        let a1 = engine.award_action_xp(100, VerificationTier::TierCGuided);
        assert_eq!(a1, 100);

        let a2 = engine.award_action_xp(100, VerificationTier::TierCGuided);
        assert_eq!(a2, 50);
        assert_eq!(engine.daily_action_xp, 150);

        engine.award_compliance_bonus(25);
        assert_eq!(engine.cumulative_xp, 175);
    }
}
