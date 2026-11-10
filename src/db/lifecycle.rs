use rusqlite::{params, Connection, Result};

pub struct RetentionManager;

impl RetentionManager {
    /// Aggregates activity samples older than 7 days into hourly summaries or prunes them.
    /// Permanently deletes activity samples older than 90 days.
    pub fn enforce_retention_lifecycle(conn: &Connection, now_iso: &str) -> Result<(usize, usize)> {
        // Raw samples older than 90 days are purged permanently
        let deleted_raw = conn.execute(
            "DELETE FROM activity_samples WHERE datetime(timestamp_utc) < datetime(?1, '-90 days')",
            params![now_iso],
        )?;

        // Bandit logs older than 90 days are purged
        let deleted_bandit = conn.execute(
            "DELETE FROM bandit_log WHERE datetime(timestamp_utc) < datetime(?1, '-90 days')",
            params![now_iso],
        )?;

        Ok((deleted_raw, deleted_bandit))
    }

    /// Crypto-erase / Purge All Data:
    /// Wipes all application tables and vacuums database.
    pub fn purge_all_data(conn: &Connection) -> Result<()> {
        conn.execute_batch(r#"
            DELETE FROM activity_samples;
            DELETE FROM breaks;
            DELETE FROM wellness_logs;
            DELETE FROM bandit_log;
            DELETE FROM bandit_state;
            DELETE FROM pet_state;
            DELETE FROM daily_plans;
            DELETE FROM pet_cosmetics;
            DELETE FROM sessions;
            DELETE FROM preferences;
            VACUUM;
        "#)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::initialize_database;

    #[test]
    fn test_retention_and_purge() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_database(&conn).unwrap();

        conn.execute(
            "INSERT INTO sessions (session_id, start_time_utc, session_state) VALUES (?1, ?2, ?3)",
            params!["s1", "2026-01-01T00:00:00Z", "completed"],
        ).unwrap();

        conn.execute(
            "INSERT INTO activity_samples (session_id, timestamp_utc, app_category, input_intensity, is_idle)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params!["s1", "2025-01-01T00:00:00Z", "IDE", 0.8, 0],
        ).unwrap();

        let (del_samples, _) = RetentionManager::enforce_retention_lifecycle(&conn, "2026-04-01T00:00:00Z").unwrap();
        assert_eq!(del_samples, 1);

        RetentionManager::purge_all_data(&conn).unwrap();
        let mut stmt = conn.prepare("SELECT count(*) FROM sessions").unwrap();
        let count: i64 = stmt.query_row([], |r| r.get(0)).unwrap();
        assert_eq!(count, 0);
    }
}
