use crate::pet::kinetics::WellnessDimensions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationTier {
    TierALock,     // User Locked the Screen: 100% all
    TierBIdle,     // Input Idle for Full Duration: 100% Eye/Mental, 50% Mobility/Posture
    TierCGuided,   // Guided Steps Completed in Care Center: 100% all
    TierDAttested, // Self-Attested Without Timer: 50% all
    Unverified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CareActivityType {
    Eye20_20_20,
    EyeRotation,
    Hydration,
    StretchSeated,
    StretchStanding,
    Walk,
    BoxBreathing,
    Relaxation478,
    PostureCheck,
    MindfulPause,
}

#[derive(Debug, Clone)]
pub struct CareCenter {
    pub last_hydration_timestamp_sec: Option<u64>,
    pub ambient_eye_window_active: bool,
    pub ambient_eye_window_start_sec: u64,
}

impl Default for CareCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl CareCenter {
    pub fn new() -> Self {
        Self {
            last_hydration_timestamp_sec: None,
            ambient_eye_window_active: false,
            ambient_eye_window_start_sec: 0,
        }
    }

    /// Check and log hydration (rate limited to at most one intake per 20 minutes / 1200s)
    pub fn log_hydration(&mut self, current_time_sec: u64, dims: &mut WellnessDimensions) -> bool {
        if let Some(last) = self.last_hydration_timestamp_sec {
            if current_time_sec < last + 1200 {
                return false; // Rate-limited
            }
        }
        self.last_hydration_timestamp_sec = Some(current_time_sec);
        // Hydration logs 250ml water (+25 pts)
        dims.apply_recovery(0.0, 25.0, 0.0, 0.0, 0.0);
        true
    }

    /// Trigger 20-20-20 ambient micro-prompt eligibility window (2 minutes / 120s)
    pub fn trigger_ambient_eye_window(&mut self, current_time_sec: u64) {
        self.ambient_eye_window_active = true;
        self.ambient_eye_window_start_sec = current_time_sec;
    }

    /// Complete ambient 20-20-20 through pet tap or idle cessation
    pub fn complete_ambient_eye_window(
        &mut self,
        current_time_sec: u64,
        tapped_pet: bool,
        idle_duration_sec: u64,
        dims: &mut WellnessDimensions,
    ) -> Option<VerificationTier> {
        if !self.ambient_eye_window_active {
            return None;
        }

        if current_time_sec > self.ambient_eye_window_start_sec + 120 {
            // Window expired
            self.ambient_eye_window_active = false;
            return None;
        }

        if tapped_pet {
            // Tier C (100% credit: +15 pts)
            self.ambient_eye_window_active = false;
            Self::apply_intervention_recovery(CareActivityType::Eye20_20_20, VerificationTier::TierCGuided, dims);
            Some(VerificationTier::TierCGuided)
        } else if idle_duration_sec >= 20 {
            // Tier B (100% credit: +15 pts)
            self.ambient_eye_window_active = false;
            Self::apply_intervention_recovery(CareActivityType::Eye20_20_20, VerificationTier::TierBIdle, dims);
            Some(VerificationTier::TierBIdle)
        } else {
            None
        }
    }

    /// Apply intervention recovery scaled by verification tier:
    /// - Tier A: 100% all
    /// - Tier B: 100% Eye & Mental, 50% Mobility & Posture
    /// - Tier C: 100% all
    /// - Tier D: 50% all
    pub fn apply_intervention_recovery(
        activity: CareActivityType,
        tier: VerificationTier,
        dims: &mut WellnessDimensions,
    ) {
        let (base_eye, base_hyd, base_mob, base_cog, base_pos) = match activity {
            CareActivityType::Eye20_20_20 => (15.0, 0.0, 0.0, 0.0, 0.0),
            CareActivityType::EyeRotation => (35.0, 0.0, 0.0, 0.0, 0.0),
            CareActivityType::Hydration => (0.0, 25.0, 0.0, 0.0, 0.0),
            CareActivityType::StretchSeated => (0.0, 0.0, 25.0, 0.0, 20.0),
            CareActivityType::StretchStanding => (0.0, 0.0, 25.0, 0.0, 20.0),
            CareActivityType::Walk => (0.0, 0.0, 40.0, 10.0, 20.0),
            CareActivityType::BoxBreathing => (0.0, 0.0, 0.0, 25.0, 0.0),
            CareActivityType::Relaxation478 => (0.0, 0.0, 0.0, 40.0, 0.0),
            CareActivityType::PostureCheck => (0.0, 0.0, 0.0, 0.0, 20.0),
            CareActivityType::MindfulPause => (0.0, 0.0, 0.0, 25.0, 0.0),
        };

        let (scale_eye, scale_hyd, scale_mob, scale_cog, scale_pos) = match tier {
            VerificationTier::TierALock => (1.0, 1.0, 1.0, 1.0, 1.0),
            VerificationTier::TierBIdle => (1.0, 1.0, 0.5, 1.0, 0.5),
            VerificationTier::TierCGuided => (1.0, 1.0, 1.0, 1.0, 1.0),
            VerificationTier::TierDAttested => (0.5, 0.5, 0.5, 0.5, 0.5),
            VerificationTier::Unverified => (0.0, 0.0, 0.0, 0.0, 0.0),
        };

        dims.apply_recovery(
            base_eye * scale_eye,
            base_hyd * scale_hyd,
            base_mob * scale_mob,
            base_cog * scale_cog,
            base_pos * scale_pos,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_credit_scaling() {
        let mut dims = WellnessDimensions {
            eye_comfort: 50.0,
            hydration: 50.0,
            mobility: 50.0,
            mental_freshness: 50.0,
            posture_comfort: 50.0,
        };

        // Tier B for StretchSeated: Mobility base 25 * 0.5 = 12.5, Posture base 20 * 0.5 = 10
        CareCenter::apply_intervention_recovery(
            CareActivityType::StretchSeated,
            VerificationTier::TierBIdle,
            &mut dims,
        );
        assert_eq!(dims.mobility, 62.5);
        assert_eq!(dims.posture_comfort, 60.0);

        // Tier D for BoxBreathing: Mental base 25 * 0.5 = 12.5
        CareCenter::apply_intervention_recovery(
            CareActivityType::BoxBreathing,
            VerificationTier::TierDAttested,
            &mut dims,
        );
        assert_eq!(dims.mental_freshness, 62.5);
    }

    #[test]
    fn test_hydration_rate_limit() {
        let mut care = CareCenter::new();
        let mut dims = WellnessDimensions::default();
        dims.hydration = 50.0;

        assert!(care.log_hydration(100, &mut dims));
        assert_eq!(dims.hydration, 75.0);

        // Within 20 minutes (e.g. at 500s) -> should be rejected
        assert!(!care.log_hydration(500, &mut dims));
        assert_eq!(dims.hydration, 75.0);

        // After 20 minutes (at 1301s) -> accepted
        assert!(care.log_hydration(1301, &mut dims));
        assert_eq!(dims.hydration, 100.0);
    }
}
