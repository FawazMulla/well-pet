use std::time::Instant;
use wellpet_core::{
    bandit::linucb::LinUCBBandit,
    monitor::session::SessionLifecycleTracker,
    pet::{fsm::PetStateMachine, kinetics::WellnessDimensions},
};

#[test]
fn test_8_hour_workday_simulation_speed_and_kinetics() {
    let start = Instant::now();
    let mut dims = WellnessDimensions::new_full();
    let mut fsm = PetStateMachine::new();
    let mut session = SessionLifecycleTracker::new("sim_8h".into());

    // 8 hours = 480 minutes = 28,800 seconds
    for m in 1..=480 {
        dims.decay_minute(false);
        let s = dims.composite_score();
        fsm.evaluate_score_tick(s);

        for _ in 0..60 {
            session.tick_1s(true);
        }

        if m == 60 {
            assert_eq!(s, 80.5);
        } else if m == 120 {
            assert_eq!(s, 61.0);
        } else if m == 180 {
            assert_eq!(s, 41.5);
        } else if m == 240 {
            assert_eq!(s, 26.0);
        } else if m == 267 {
            assert!(s < 20.0);
        }
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 2.0,
        "8-hour simulation must execute in < 2.0 seconds, took {:?}",
        elapsed
    );
}

#[test]
fn test_derived_flat_cadence_maintenance() {
    let mut dims = WellnessDimensions::new_full();

    for minute in 1..=480 {
        dims.decay_minute(false);

        if minute % 30 == 0 {
            dims.apply_recovery(15.0, 0.0, 0.0, 0.0, 0.0);
        }
        if minute % 100 == 0 {
            dims.apply_recovery(0.0, 25.0, 0.0, 0.0, 0.0);
            dims.apply_recovery(0.0, 0.0, 0.0, 25.0, 0.0);
        }
        if minute % 71 == 0 {
            dims.apply_recovery(0.0, 0.0, 25.0, 0.0, 0.0);
        }
        if minute % 80 == 0 {
            dims.apply_recovery(0.0, 0.0, 0.0, 0.0, 20.0);
        }

        let s = dims.composite_score();
        assert!(
            s >= 80.0,
            "Score dropped below 80.0 at minute {}: score was {}",
            minute,
            s
        );
    }
}

#[test]
fn test_bandit_vs_fixed_timer_14_day_benchmark() {
    let mut morning_bandit = LinUCBBandit::new();
    let mut bandit_accepted = 0;

    for day in 1..=14 {
        for round in 0..8 {
            let hour = 9.0 + (round as f64);
            let sin_circ = (2.0 * std::f64::consts::PI * hour / 24.0).sin();
            let cos_circ = (2.0 * std::f64::consts::PI * hour / 24.0).cos();

            let is_morning = hour < 13.0;
            let ctx_morning = [1.0, (round as f64) / 8.0, 0.7, 1.0, 0.0, 0.5, sin_circ, cos_circ, 0.0];

            let chosen_arm = morning_bandit.select_arm(&ctx_morning);

            let user_accepts = if is_morning {
                chosen_arm == 0 || chosen_arm == 1
            } else {
                chosen_arm == 2
            };

            let reward = if user_accepts { 1.0 } else { -0.5 };
            morning_bandit.record_outcome(chosen_arm, &ctx_morning, reward);

            if day >= 10 && user_accepts {
                bandit_accepted += 1;
            }
        }
    }

    let bandit_rate = (bandit_accepted as f64) / (5.0 * 8.0);
    let fixed_rate_recent = 0.50; // morning accepts 4 out of 8 = 50%

    assert!(
        bandit_rate >= fixed_rate_recent + 0.10,
        "Bandit acceptance rate ({:.2}) must be >= 10 percentage points higher than fixed timer ({:.2})",
        bandit_rate,
        fixed_rate_recent
    );
}
