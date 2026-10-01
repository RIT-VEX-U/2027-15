//! FeedForward model implementation (`kS`, `kV`, `kA`, `kG`).

use crate::core::math::math_util::sign;

/// Feedforward configuration parameters.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FeedForwardConfig {
    /// Static friction voltage offset to overcome friction.
    pub ks: f64,
    /// Velocity coefficient: voltage per unit velocity.
    pub kv: f64,
    /// Acceleration coefficient: voltage per unit acceleration.
    pub ka: f64,
    /// Gravity offset: voltage required to counteract gravity (e.g. on vertical lifts).
    pub kg: f64,
}

/// FeedForward model calculator.
#[derive(Debug, Clone, Copy)]
pub struct FeedForward {
    pub cfg: FeedForwardConfig,
}

impl FeedForward {
    /// Creates a new FeedForward instance with the given configuration.
    pub fn new(cfg: FeedForwardConfig) -> Self {
        Self { cfg }
    }

    /// Computes the feedforward control output.
    ///
    /// Formula: `F = kG + kS * sgn(v) + kV * v + kA * a`
    pub fn calculate(&self, v: f64, a: f64, pid_ref: f64) -> f64 {
        let mut ks_sign = 0.0;
        if v != 0.0 {
            ks_sign = sign(v);
        } else if pid_ref != 0.0 {
            ks_sign = sign(pid_ref);
        }

        (self.cfg.ks * ks_sign) + (self.cfg.kv * v) + (self.cfg.ka * a) + self.cfg.kg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedforward() {
        let ff = FeedForward::new(FeedForwardConfig {
            ks: 0.5,
            kv: 2.0,
            ka: 0.1,
            kg: 1.0,
        });

        // Steady forward motion
        let out = ff.calculate(1.0, 0.0, 0.0);
        // kg (1.0) + ks * 1.0 (0.5) + kv * 1.0 (2.0) = 3.5
        assert_eq!(out, 3.5);

        // At rest, but with pid_ref > 0
        let out_rest = ff.calculate(0.0, 0.0, 1.0);
        // kg (1.0) + ks * 1.0 (0.5) = 1.5
        assert_eq!(out_rest, 1.5);
    }
}
