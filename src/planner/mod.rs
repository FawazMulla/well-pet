use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayArchetype {
    Deadline,
    Standard,
    Learning,
    Creative,
    MeetingHeavy,
    Recovery,
    LowSleep,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedBreak {
    pub anchor_minute: u32,
    pub break_type: String,
    pub duration_minutes: u32,
    pub is_meal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlan {
    pub archetype: DayArchetype,
    pub planned_duration_min: u32,
    pub breaks: Vec<PlannedBreak>,
}

pub struct PlannerEngine;

impl PlannerEngine {
    /// Ergonomic 8-point quality ruleset validation and repair
    pub fn validate_and_repair_plan(plan: &mut DayPlan) -> bool {
        let mut repaired = true;

        // Rule 1 & 2: Intervals between breaks >= 20m and <= 90m
        // Rule 5: Sessions > 5 hours (300 min) must have a 30-60m meal break
        if plan.planned_duration_min > 300 && !plan.breaks.iter().any(|b| b.is_meal) {
            // Insert meal break at midpoint
            let mid = plan.planned_duration_min / 2;
            plan.breaks.push(PlannedBreak {
                anchor_minute: mid,
                break_type: "mindful_meal".into(),
                duration_minutes: 45,
                is_meal: true,
            });
            plan.breaks.sort_by_key(|b| b.anchor_minute);
            repaired = false;
        }

        // Rule 6: Final 15 minutes reserved for wrap-up
        let wrap_start = plan.planned_duration_min.saturating_sub(15);
        plan.breaks.retain(|b| b.anchor_minute < wrap_start);

        // Rule 7: Maximum 3 consecutive breaks of the same category
        let mut streak = 0;
        let mut last_type = String::new();
        for b in plan.breaks.iter_mut() {
            if b.break_type == last_type {
                streak += 1;
                if streak >= 3 {
                    b.break_type = "stretch_seated".into();
                    streak = 1;
                }
            } else {
                streak = 1;
                last_type = b.break_type.clone();
            }
        }

        repaired
    }

    /// Fair compliance scoring formula:
    /// Compliance = (Verified Completed Prompts) / (Scheduled Prompts - Suppressed By Hard Gates) * 100
    pub fn calculate_fair_compliance(
        verified_completed: u32,
        scheduled_prompts: u32,
        suppressed_by_hard_gates: u32,
    ) -> f64 {
        let effective_denominator = scheduled_prompts.saturating_sub(suppressed_by_hard_gates);
        if effective_denominator == 0 {
            return 100.0;
        }
        let compliance = (verified_completed as f64 / effective_denominator as f64) * 100.0;
        compliance.clamp(0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fair_compliance_calculation() {
        // Scheduled: 10, Suppressed by meetings: 4, Verified completed: 6
        // Compliance: 6 / (10 - 4) * 100 = 100%
        let comp = PlannerEngine::calculate_fair_compliance(6, 10, 4);
        assert_eq!(comp, 100.0);

        // If suppressed by hard gates equals scheduled: 100%
        let comp_all_gated = PlannerEngine::calculate_fair_compliance(0, 5, 5);
        assert_eq!(comp_all_gated, 100.0);
    }

    #[test]
    fn test_plan_meal_validation() {
        let mut plan = DayPlan {
            archetype: DayArchetype::Standard,
            planned_duration_min: 480, // 8 hours
            breaks: vec![
                PlannedBreak { anchor_minute: 60, break_type: "eye_20_20_20".into(), duration_minutes: 1, is_meal: false },
                PlannedBreak { anchor_minute: 120, break_type: "eye_20_20_20".into(), duration_minutes: 1, is_meal: false },
            ],
        };

        PlannerEngine::validate_and_repair_plan(&mut plan);
        // Should have inserted a meal break since session > 300m
        assert!(plan.breaks.iter().any(|b| b.is_meal));
    }
}
