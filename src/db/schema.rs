use rusqlite::{Connection, Result};

pub const SCHEMA_DDL: &str = r#"
PRAGMA foreign_keys = ON;

-- Operational Work Sessions
CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY NOT NULL,
    start_time_utc TEXT NOT NULL,
    end_time_utc TEXT,
    session_state TEXT NOT NULL CHECK(session_state IN ('active', 'paused', 'completed'))
);

-- Granular Activity Samples (7-Day Rolling Retention)
CREATE TABLE IF NOT EXISTS activity_samples (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    timestamp_utc TEXT NOT NULL,
    app_category TEXT NOT NULL CHECK(app_category IN ('IDE', 'Browser', 'Document', 'Meeting', 'Media', 'Other')),
    input_intensity REAL NOT NULL CHECK(input_intensity >= 0.0 AND input_intensity <= 1.0),
    is_idle INTEGER NOT NULL CHECK(is_idle IN (0, 1))
);
CREATE INDEX IF NOT EXISTS idx_activity_timestamp ON activity_samples(timestamp_utc);

-- Intervention and Break Ledger (90-Day Granular Retention)
CREATE TABLE IF NOT EXISTS breaks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    plan_id INTEGER REFERENCES daily_plans(plan_id) ON DELETE SET NULL,
    scheduled_time_utc TEXT NOT NULL,
    actual_time_utc TEXT,
    break_type TEXT NOT NULL CHECK(break_type IN ('eye_20_20_20', 'eye_rotation', 'hydration', 'stretch_seated', 'stretch_standing', 'walk', 'box_breathing', 'relaxation_478', 'posture_check', 'mindful_pause')),
    scheduled_duration_sec INTEGER NOT NULL,
    actual_duration_sec INTEGER,
    outcome TEXT NOT NULL CHECK(outcome IN ('completed', 'snoozed', 'dismissed', 'ignored', 'preemptive')),
    verification_tier TEXT NOT NULL CHECK(verification_tier IN ('tier_a_lock', 'tier_b_idle', 'tier_c_guided', 'tier_d_attested', 'unverified')),
    xp_awarded INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_breaks_session ON breaks(session_id);

-- Discrete Wellness Logs (Water, Mood Check-Ins)
CREATE TABLE IF NOT EXISTS wellness_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE,
    timestamp_utc TEXT NOT NULL,
    log_type TEXT NOT NULL CHECK(log_type IN ('water_intake', 'mood_checkin', 'posture_nudge')),
    value_payload TEXT NOT NULL
);

-- Daily Synthesized Plans
CREATE TABLE IF NOT EXISTS daily_plans (
    plan_id INTEGER PRIMARY KEY AUTOINCREMENT,
    calendar_date TEXT NOT NULL UNIQUE,
    day_type TEXT NOT NULL CHECK(day_type IN ('deadline', 'normal', 'learning', 'creative', 'meeting_heavy', 'recovery', 'low_sleep', 'custom')),
    planned_duration_min INTEGER NOT NULL,
    schedule_payload_json TEXT NOT NULL,
    compliance_pct REAL DEFAULT 0.0,
    is_auto_generated INTEGER NOT NULL DEFAULT 1
);

-- Companion State Ledger (5 Renamed Dimensions)
CREATE TABLE IF NOT EXISTS pet_state (
    state_id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp_utc TEXT NOT NULL,
    eye_score REAL NOT NULL CHECK(eye_score >= 0.0 AND eye_score <= 100.0),
    hydration_score REAL NOT NULL CHECK(hydration_score >= 0.0 AND hydration_score <= 100.0),
    movement_score REAL NOT NULL CHECK(movement_score >= 0.0 AND movement_score <= 100.0),
    mental_score REAL NOT NULL CHECK(mental_score >= 0.0 AND mental_score <= 100.0),
    posture_score REAL NOT NULL CHECK(posture_score >= 0.0 AND posture_score <= 100.0),
    composite_score REAL NOT NULL CHECK(composite_score >= 0.0 AND composite_score <= 100.0),
    emotional_state TEXT NOT NULL CHECK(emotional_state IN ('thriving', 'happy', 'tired', 'sleepy', 'napping')),
    evolution_tier INTEGER NOT NULL CHECK(evolution_tier BETWEEN 0 AND 2),
    current_level INTEGER NOT NULL DEFAULT 1,
    cumulative_xp INTEGER NOT NULL DEFAULT 0,
    personality_vector_json TEXT NOT NULL
);

-- LinUCB Contextual Bandit Model State Persistence
CREATE TABLE IF NOT EXISTS bandit_state (
    arm_id INTEGER PRIMARY KEY NOT NULL, -- 0: Early (-10m), 1: On-Time (0m), 2: Late (+10m)
    a_matrix_json TEXT NOT NULL,         -- 9x9 covariance matrix JSON
    b_vector_json TEXT NOT NULL,         -- 9x1 reward vector JSON
    rounds_played INTEGER NOT NULL DEFAULT 0,
    last_updated_utc TEXT NOT NULL
);

-- LinUCB Contextual Bandit Execution Log (90-Day Retention)
CREATE TABLE IF NOT EXISTS bandit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp_utc TEXT NOT NULL,
    context_vector_json TEXT NOT NULL,   -- 9-dim context vector
    arm_chosen INTEGER NOT NULL,
    reward REAL,                         -- Null if sample discarded due to hard gate
    session_id TEXT NOT NULL REFERENCES sessions(session_id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_bandit_log_timestamp ON bandit_log(timestamp_utc);

-- Unlocked Cosmetics & Customizations
CREATE TABLE IF NOT EXISTS pet_cosmetics (
    item_id TEXT PRIMARY KEY NOT NULL,
    item_type TEXT NOT NULL CHECK(item_type IN ('headwear', 'eyewear', 'outfit', 'habitat_tile', 'particle_aura')),
    equipped INTEGER NOT NULL DEFAULT 0 CHECK(equipped IN (0, 1)),
    unlocked_at_utc TEXT NOT NULL
);

-- Application Preferences (Database is already encrypted via SQLCipher)
CREATE TABLE IF NOT EXISTS preferences (
    pref_key TEXT PRIMARY KEY NOT NULL,
    pref_value TEXT NOT NULL
);
"#;

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute_batch(SCHEMA_DDL)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_initialization() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_database(&conn).unwrap();

        // Verify tables exist
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='pet_state'")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        assert!(rows.next().unwrap().is_some());
    }
}
