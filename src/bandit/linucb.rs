use serde::{Deserialize, Serialize};

pub const CONTEXT_DIM: usize = 9;
pub const NUM_ARMS: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmState {
    pub arm_id: usize,
    pub a_matrix: Vec<Vec<f64>>, // 9x9 matrix
    pub b_vector: Vec<f64>,      // 9x1 vector
    pub rounds_played: u32,
}

impl ArmState {
    pub fn new(arm_id: usize) -> Self {
        // Identity matrix I_9
        let mut a = vec![vec![0.0; CONTEXT_DIM]; CONTEXT_DIM];
        for i in 0..CONTEXT_DIM {
            a[i][i] = 1.0;
        }
        let b = vec![0.0; CONTEXT_DIM];
        Self {
            arm_id,
            a_matrix: a,
            b_vector: b,
            rounds_played: 0,
        }
    }

    /// Reset learning back to prior (Identity and zero reward vector)
    pub fn reset(&mut self) {
        for i in 0..CONTEXT_DIM {
            for j in 0..CONTEXT_DIM {
                self.a_matrix[i][j] = if i == j { 1.0 } else { 0.0 };
            }
            self.b_vector[i] = 0.0;
        }
        self.rounds_played = 0;
    }

    /// Gauss-Jordan inversion of 9x9 matrix A
    pub fn invert_a(&self) -> Option<Vec<Vec<f64>>> {
        let n = CONTEXT_DIM;
        let mut aug = vec![vec![0.0; 2 * n]; n];
        for i in 0..n {
            for j in 0..n {
                aug[i][j] = self.a_matrix[i][j];
            }
            aug[i][i + n] = 1.0;
        }

        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in (i + 1)..n {
                if aug[k][i].abs() > aug[max_row][i].abs() {
                    max_row = k;
                }
            }
            aug.swap(i, max_row);

            let pivot = aug[i][i];
            if pivot.abs() < 1e-9 {
                return None; // Singular
            }

            for j in 0..(2 * n) {
                aug[i][j] /= pivot;
            }

            for k in 0..n {
                if k != i {
                    let factor = aug[k][i];
                    for j in 0..(2 * n) {
                        aug[k][j] -= factor * aug[i][j];
                    }
                }
            }
        }

        let mut inv = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                inv[i][j] = aug[i][j + n];
            }
        }
        Some(inv)
    }

    /// Calculate ridge regression weights: theta = A^-1 * b
    pub fn compute_theta(&self) -> Vec<f64> {
        if let Some(inv) = self.invert_a() {
            let mut theta = vec![0.0; CONTEXT_DIM];
            for i in 0..CONTEXT_DIM {
                for j in 0..CONTEXT_DIM {
                    theta[i] += inv[i][j] * self.b_vector[j];
                }
            }
            theta
        } else {
            vec![0.0; CONTEXT_DIM]
        }
    }

    /// Update with discount factor gamma:
    /// A <- gamma * A + x * x^T
    /// b <- gamma * b + r * x
    pub fn update(&mut self, x: &[f64; CONTEXT_DIM], reward: f64, gamma: f64) {
        self.rounds_played += 1;
        for i in 0..CONTEXT_DIM {
            self.b_vector[i] = gamma * self.b_vector[i] + reward * x[i];
            for j in 0..CONTEXT_DIM {
                self.a_matrix[i][j] = gamma * self.a_matrix[i][j] + x[i] * x[j];
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinUCBBandit {
    pub arms: [ArmState; NUM_ARMS],
    pub total_rounds: u32,
    pub exploration_count: u32,
    pub reward_history: Vec<f64>,
}

impl Default for LinUCBBandit {
    fn default() -> Self {
        Self::new()
    }
}

impl LinUCBBandit {
    pub const ALPHA: f64 = 0.5;
    pub const GAMMA: f64 = 0.99;

    pub fn new() -> Self {
        Self {
            arms: [ArmState::new(0), ArmState::new(1), ArmState::new(2)],
            total_rounds: 0,
            exploration_count: 0,
            reward_history: Vec::new(),
        }
    }

    /// Reset learning
    pub fn reset_learning(&mut self) {
        for arm in self.arms.iter_mut() {
            arm.reset();
        }
        self.total_rounds = 0;
        self.exploration_count = 0;
        self.reward_history.clear();
    }

    /// Select arm given 9-dim context vector x
    /// Cold start: during first 7 days or < 20 played rounds, strictly play Arm 1 (On-Time)
    pub fn select_arm(&mut self, x: &[f64; CONTEXT_DIM]) -> usize {
        // Cold start rule
        if self.total_rounds < 20 {
            return 1; // Arm 1: Nominal On-Time
        }

        let mut best_arm = 1;
        let mut best_score = f64::NEG_INFINITY;

        for arm_id in 0..NUM_ARMS {
            let arm = &self.arms[arm_id];
            let theta = arm.compute_theta();

            // predicted reward: theta^T * x
            let mut mean_r = 0.0;
            for i in 0..CONTEXT_DIM {
                mean_r += theta[i] * x[i];
            }

            // exploration bonus: alpha * sqrt(x^T * A^-1 * x)
            let mut variance = 0.0;
            if let Some(inv) = arm.invert_a() {
                for i in 0..CONTEXT_DIM {
                    for j in 0..CONTEXT_DIM {
                        variance += x[i] * inv[i][j] * x[j];
                    }
                }
            }
            let ucb = mean_r + Self::ALPHA * variance.max(0.0).sqrt();

            // Ties break toward Arm 1 (On-Time)
            if ucb > best_score || ( (ucb - best_score).abs() < 1e-6 && arm_id == 1) {
                best_score = ucb;
                best_arm = arm_id;
            }
        }

        best_arm
    }

    /// Update chosen arm with reward
    pub fn record_outcome(&mut self, chosen_arm: usize, x: &[f64; CONTEXT_DIM], reward: f64) {
        self.total_rounds += 1;
        self.reward_history.push(reward);
        self.arms[chosen_arm].update(x, reward, Self::GAMMA);

        // Drift detection:
        // If rolling average reward of last 20 rounds drops >= 0.4 below previous 60 rounds,
        // shrink weights halfway toward initial prior
        if self.reward_history.len() >= 80 {
            let len = self.reward_history.len();
            let last_20_avg: f64 = self.reward_history[len - 20..].iter().sum::<f64>() / 20.0;
            let prev_60_avg: f64 = self.reward_history[len - 80..len - 20].iter().sum::<f64>() / 60.0;

            if prev_60_avg - last_20_avg >= 0.4 {
                // Drift detected! Shrink halfway toward initial prior
                for arm in self.arms.iter_mut() {
                    for i in 0..CONTEXT_DIM {
                        arm.b_vector[i] *= 0.5;
                        for j in 0..CONTEXT_DIM {
                            let prior = if i == j { 1.0 } else { 0.0 };
                            arm.a_matrix[i][j] = 0.5 * arm.a_matrix[i][j] + 0.5 * prior;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cold_start_and_reset() {
        let mut bandit = LinUCBBandit::new();
        let context = [1.0, 0.5, 0.5, 1.0, 0.0, 0.5, 0.0, 1.0, 0.0];

        // First 19 rounds must be Arm 1 (Cold Start)
        for _ in 0..19 {
            assert_eq!(bandit.select_arm(&context), 1);
            bandit.record_outcome(1, &context, 1.0);
        }

        // Reset learning
        bandit.reset_learning();
        assert_eq!(bandit.total_rounds, 0);
        assert_eq!(bandit.arms[1].rounds_played, 0);
        assert_eq!(bandit.arms[1].b_vector[0], 0.0);
    }

    #[test]
    fn test_matrix_inversion() {
        let arm = ArmState::new(0);
        let inv = arm.invert_a().unwrap();
        // Inverse of identity is identity
        assert_eq!(inv[0][0], 1.0);
        assert_eq!(inv[0][1], 0.0);
    }
}
