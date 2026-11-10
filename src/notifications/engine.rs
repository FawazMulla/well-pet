use crate::monitor::process::AppCategory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationLevel {
    None,
    Level1, // Gentle ambient sprite animation
    Level2, // Ambient gesture and tray badge
    Level3, // Interactive prompt (Care Center micro-card)
    Level4, // Persistent banner
}

#[derive(Debug, Clone)]
pub struct HardGateState {
    pub screen_sharing: bool,
    pub presentation_mode: bool,
    pub active_video_call: bool,
    pub full_screen_media: bool,
    pub dnd_mode: bool,
    pub hyperfocus_timer_active: bool,
    pub rest_or_vacation: bool,
    pub manual_suppress: bool,
}

impl HardGateState {
    pub fn is_gated(&self) -> bool {
        self.screen_sharing
            || self.presentation_mode
            || self.active_video_call
            || self.full_screen_media
            || self.dnd_mode
            || self.hyperfocus_timer_active
            || self.rest_or_vacation
            || self.manual_suppress
    }
}

pub struct NotificationEngine;

impl NotificationEngine {
    /// Calculate Interruptibility Score I in [0, 100]:
    /// I = 100 - (0.45 * C_input + 0.20 * C_switch + 0.35 * C_app)
    /// C_input = 100 * V (where V is the 60-second average input ratio)
    /// C_switch = min(100.0, 20.0 * window_switches_last_60s)
    /// C_app: IDE/Document = 80; Media = 50; Browser = 30; Other = 20; Unknown = 40
    /// If app category is None, weights rescale to sum to 1.0 (I = 100 - C_input)
    pub fn calculate_interruptibility(
        v_60s: f64,
        window_switches_last_60s: u32,
        app_category: Option<AppCategory>,
    ) -> f64 {
        let c_input = 100.0 * v_60s.clamp(0.0, 1.0);
        let c_switch = (20.0 * window_switches_last_60s as f64).min(100.0);

        match app_category {
            Some(cat) => {
                let c_app = match cat {
                    AppCategory::IDE | AppCategory::Document => 80.0,
                    AppCategory::Media => 50.0,
                    AppCategory::Browser => 30.0,
                    AppCategory::Other => 20.0,
                    AppCategory::Meeting => 90.0, // Hard gate normally handles this, but keep high
                };
                let cost = 0.45 * c_input + 0.20 * c_switch + 0.35 * c_app;
                (100.0 - cost).clamp(0.0, 100.0)
            }
            None => {
                // When app info is unavailable, rescale: with input ratio alone, I = 100 - C_input
                (100.0 - c_input).clamp(0.0, 100.0)
            }
        }
    }

    /// Exhaustive Delivery Decision Table evaluation:
    /// Returns (Maximum Allowed Level, Action Description)
    pub fn evaluate_delivery(
        offset_minutes_from_anchor: i32, // negative = early, positive = past anchor
        interruptibility_score: f64,
        has_natural_pause: bool, // >= 15s idle
        hard_gates: &HardGateState,
    ) -> (EscalationLevel, &'static str) {
        if hard_gates.is_gated() {
            return (EscalationLevel::None, "Suppressed by active Hard Gate");
        }

        // Before Window: > 15m early
        if offset_minutes_from_anchor < -15 {
            return (EscalationLevel::None, "Wait.");
        }

        // In Window: [-15, +15]
        if offset_minutes_from_anchor >= -15 && offset_minutes_from_anchor <= 15 {
            if interruptibility_score >= 70.0 {
                if has_natural_pause {
                    return (EscalationLevel::Level3, "Deliver now, then escalate every 5 min.");
                } else {
                    return (EscalationLevel::Level2, "Ambient gesture and tray badge only.");
                }
            } else if interruptibility_score >= 40.0 {
                if has_natural_pause {
                    return (EscalationLevel::Level3, "Deliver now.");
                } else {
                    return (EscalationLevel::None, "Queue in memory until pause or window ends.");
                }
            } else {
                // I < 40
                if has_natural_pause {
                    return (EscalationLevel::Level1, "Gentle ambient gesture only.");
                } else {
                    return (EscalationLevel::None, "Defer; never interrupt deep focus.");
                }
            }
        }

        // Overdue: > 15m past anchor
        if interruptibility_score >= 40.0 && has_natural_pause {
            (EscalationLevel::Level3, "Deliver now.")
        } else {
            (EscalationLevel::Level1, "Ambient gesture only.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interruptibility_formula() {
        // Deep focus: IDE, V = 0.9, 1 switch
        // C_input = 90, C_switch = 20, C_app = 80
        // cost = 0.45*90 + 0.20*20 + 0.35*80 = 40.5 + 4 + 28 = 72.5
        // I = 100 - 72.5 = 27.5
        let i_deep = NotificationEngine::calculate_interruptibility(0.9, 1, Some(AppCategory::IDE));
        assert_eq!(i_deep, 27.5);

        // Reading in browser: V = 0.2, 1 switch
        // C_input = 20, C_switch = 20, C_app = 30
        // cost = 0.45*20 + 0.20*20 + 0.35*30 = 9 + 4 + 10.5 = 23.5
        // I = 100 - 23.5 = 76.5
        let i_read = NotificationEngine::calculate_interruptibility(0.2, 1, Some(AppCategory::Browser));
        assert_eq!(i_read, 76.5);

        // App unavailable fallback: V = 0.4 -> I = 100 - 40 = 60
        let i_fallback = NotificationEngine::calculate_interruptibility(0.4, 0, None);
        assert_eq!(i_fallback, 60.0);
    }

    #[test]
    fn test_exhaustive_decision_table() {
        let no_gates = HardGateState {
            screen_sharing: false,
            presentation_mode: false,
            active_video_call: false,
            full_screen_media: false,
            dnd_mode: false,
            hyperfocus_timer_active: false,
            rest_or_vacation: false,
            manual_suppress: false,
        };

        // Before window
        let (lvl, _) = NotificationEngine::evaluate_delivery(-20, 80.0, true, &no_gates);
        assert_eq!(lvl, EscalationLevel::None);

        // In window, I >= 70, pause = true -> Level 3
        let (lvl, _) = NotificationEngine::evaluate_delivery(0, 75.0, true, &no_gates);
        assert_eq!(lvl, EscalationLevel::Level3);

        // In window, I >= 70, pause = false -> Level 2
        let (lvl, _) = NotificationEngine::evaluate_delivery(0, 75.0, false, &no_gates);
        assert_eq!(lvl, EscalationLevel::Level2);

        // In window, I < 40 (deep focus), pause = false -> None (defer)
        let (lvl, _) = NotificationEngine::evaluate_delivery(0, 30.0, false, &no_gates);
        assert_eq!(lvl, EscalationLevel::None);

        // Hard gate active -> Level None
        let mut gated = no_gates.clone();
        gated.screen_sharing = true;
        let (lvl, _) = NotificationEngine::evaluate_delivery(0, 90.0, true, &gated);
        assert_eq!(lvl, EscalationLevel::None);
    }
}
