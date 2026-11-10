use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct InputMonitor {
    // Stores 1-second activity flags: true = active input occurred in this 1s slice
    second_buffer: VecDeque<bool>,
    // Rolling 10-second slice ratios (up to 6 per 60s)
    slice_ratios_60s: VecDeque<f64>,
    // 5-minute average ratios for percentile calculation
    window_5min_ratios: Vec<f64>,
    battery_level: f64,
}

impl Default for InputMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMonitor {
    pub fn new() -> Self {
        Self {
            second_buffer: VecDeque::with_capacity(10),
            slice_ratios_60s: VecDeque::with_capacity(6),
            window_5min_ratios: Vec::new(),
            battery_level: 1.0,
        }
    }

    pub fn set_battery_level(&mut self, level: f64) {
        self.battery_level = level;
    }

    pub fn sampling_interval_seconds(&self) -> u64 {
        // Battery saver mode (<= 25% battery): sampling interval throttles to 2s
        if self.battery_level <= 0.25 {
            2
        } else {
            1
        }
    }

    /// Record a 1-second sample slice: true if physical input occurred
    pub fn record_sample(&mut self, had_input: bool) -> Option<f64> {
        self.second_buffer.push_back(had_input);
        if self.second_buffer.len() >= 10 {
            let active_count = self.second_buffer.iter().take(10).filter(|&&x| x).count();
            let slice_ratio = active_count as f64 / 10.0;
            self.second_buffer.drain(..10);

            self.slice_ratios_60s.push_back(slice_ratio);
            if self.slice_ratios_60s.len() > 6 {
                self.slice_ratios_60s.pop_front();
            }
            Some(slice_ratio)
        } else {
            None
        }
    }

    /// Average input ratio V over the last 60 seconds (in [0.0, 1.0])
    pub fn get_v_60s(&self) -> f64 {
        if self.slice_ratios_60s.is_empty() {
            0.0
        } else {
            let sum: f64 = self.slice_ratios_60s.iter().sum();
            sum / self.slice_ratios_60s.len() as f64
        }
    }

    pub fn record_5min_ratio(&mut self, ratio: f64) {
        self.window_5min_ratios.push(ratio);
    }

    /// High-Intensity Work Detection (80th Percentile Rule):
    /// Average input ratio V over 5 min is >= user's 80th percentile of past 5-min averages
    /// AND at least 0.60.
    pub fn is_high_intensity(&self, current_5min_ratio: f64) -> bool {
        if current_5min_ratio < 0.60 {
            return false;
        }
        if self.window_5min_ratios.is_empty() {
            return current_5min_ratio >= 0.60;
        }

        let mut sorted = self.window_5min_ratios.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let idx = ((sorted.len() as f64 * 0.80).ceil() as usize).saturating_sub(1);
        let p80 = sorted[idx.min(sorted.len() - 1)];

        current_5min_ratio >= p80
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_ratio_and_battery_throttle() {
        let mut monitor = InputMonitor::new();
        assert_eq!(monitor.sampling_interval_seconds(), 1);

        monitor.set_battery_level(0.20);
        assert_eq!(monitor.sampling_interval_seconds(), 2);

        // Feed 10 samples (8 true, 2 false)
        for _ in 0..8 {
            monitor.record_sample(true);
        }
        monitor.record_sample(false);
        let slice = monitor.record_sample(false);
        assert_eq!(slice, Some(0.8));
        assert_eq!(monitor.get_v_60s(), 0.8);
    }

    #[test]
    fn test_high_intensity_detection() {
        let mut monitor = InputMonitor::new();
        // Seed some history
        for r in [0.2, 0.3, 0.4, 0.5, 0.7, 0.8] {
            monitor.record_5min_ratio(r);
        }
        // Below 0.60 is never high intensity
        assert!(!monitor.is_high_intensity(0.55));
        // >= 0.60 and >= 80th percentile (which is ~0.7-0.8)
        assert!(monitor.is_high_intensity(0.85));
    }
}
