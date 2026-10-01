//! Bang-Bang (hysteresis/threshold) controller.

use super::feedback_base::Feedback;

/// Bang-bang controller with threshold deadband.
#[derive(Debug, Clone, Copy)]
pub struct BangBang {
    setpt: f64,
    sensor_val: f64,
    lower_bound: f64,
    upper_bound: f64,
    last_output: f64,
    threshold: f64,
}

impl BangBang {
    /// Constructs a new BangBang controller.
    pub fn new(threshold: f64, low: f64, high: f64) -> Self {
        Self {
            setpt: low,
            sensor_val: low,
            lower_bound: low,
            upper_bound: high,
            last_output: 0.0,
            threshold,
        }
    }
}

impl Feedback for BangBang {
    fn init(&mut self, start_pt: f64, set_pt: f64) {
        self.sensor_val = start_pt;
        self.setpt = set_pt;
    }

    fn update(&mut self, val: f64) -> f64 {
        self.sensor_val = val;
        if (val - self.setpt).abs() < self.threshold {
            self.last_output = 0.0;
        } else if val > self.setpt {
            self.last_output = self.lower_bound;
        } else {
            self.last_output = self.upper_bound;
        }
        self.last_output
    }

    fn get(&self) -> f64 {
        self.last_output
    }

    fn set_limits(&mut self, lower: f64, upper: f64) {
        self.lower_bound = lower;
        self.upper_bound = upper;
    }

    fn is_on_target(&self) -> bool {
        (self.sensor_val - self.setpt).abs() < self.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bang_bang() {
        let mut bb = BangBang::new(1.0, -12.0, 12.0);
        bb.init(0.0, 10.0);

        assert_eq!(bb.update(5.0), 12.0);
        assert!(!bb.is_on_target());

        assert_eq!(bb.update(9.5), 0.0);
        assert!(bb.is_on_target());

        assert_eq!(bb.update(15.0), -12.0);
    }
}
