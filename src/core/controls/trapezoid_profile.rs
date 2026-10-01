//! Trapezoidal and triangular 1D motion profile generator.

/// State along a 1D motion profile containing position, velocity, and acceleration.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MotionState {
    pub pos: f64,
    pub vel: f64,
    pub acc: f64,
}

/// Trapezoidal / Triangular motion profile generator with asymmetric acceleration and deceleration.
#[derive(Debug, Clone, Copy)]
pub struct TrapezoidProfile {
    x_initial: f64,
    x_target: f64,
    v_max: f64,
    v_peak: f64,
    accel: f64,
    decel: f64,
    distance: f64,
    dist_accel: f64,
    dist_decel: f64,
    dist_cruise: f64,
    dist_full: f64,

    time_accel: f64,
    time_decel: f64,
    time_cruise: f64,
    time_total: f64,

    triangular: bool,
    direction: f64,
}

impl TrapezoidProfile {
    /// Constructs a motion profile from initial position to target position with max velocity, acceleration, and deceleration.
    pub fn new(x_initial: f64, x_target: f64, v_max: f64, accel: f64, decel: f64) -> Self {
        let distance = x_target - x_initial;
        let direction = if distance >= 0.0 { 1.0 } else { -1.0 };
        let abs_dist = distance.abs();

        let dist_accel = 0.5 * (v_max * v_max) / accel;
        let dist_decel = 0.5 * (v_max * v_max) / decel;
        let dist_full = dist_accel + dist_decel;

        let (triangular, v_peak, time_accel, time_decel, dist_cruise, time_cruise, time_total) =
            if abs_dist > dist_full {
                let time_accel = v_max / accel;
                let time_decel = v_max / decel;
                let dist_cruise = abs_dist - dist_full;
                let time_cruise = dist_cruise / v_max;
                let time_total = time_accel + time_cruise + time_decel;
                (false, v_max, time_accel, time_decel, dist_cruise, time_cruise, time_total)
            } else {
                let v_peak = ((2.0 * abs_dist * accel * decel) / (accel + decel)).sqrt();
                let time_accel = v_peak / accel;
                let time_decel = v_peak / decel;
                let time_cruise = 0.0;
                let time_total = time_accel + time_decel;
                (true, v_peak, time_accel, time_decel, 0.0, time_cruise, time_total)
            };

        Self {
            x_initial,
            x_target,
            v_max,
            v_peak,
            accel,
            decel,
            distance,
            dist_accel,
            dist_decel,
            dist_cruise,
            dist_full,
            time_accel,
            time_decel,
            time_cruise,
            time_total,
            triangular,
            direction,
        }
    }

    /// Evaluates the motion profile at elapsed time `t` (in seconds).
    pub fn calculate(&self, mut t: f64) -> MotionState {
        if t < 0.0 {
            t = 0.0;
        } else if t > self.time_total {
            t = self.time_total;
        }

        let (pos_local, vel_local, acc_local) = if t < self.time_accel {
            (
                0.5 * self.accel * (t * t),
                self.accel * t,
                self.accel,
            )
        } else if !self.triangular && t < self.time_accel + self.time_cruise {
            (
                self.dist_accel + self.v_max * (t - self.time_accel),
                self.v_max,
                0.0,
            )
        } else {
            let time_deceled = if self.triangular {
                t - self.time_accel
            } else {
                t - (self.time_accel + self.time_cruise)
            };

            let pos = if self.triangular {
                (0.5 * self.accel * (self.time_accel * self.time_accel))
                    + (self.v_peak * time_deceled)
                    - (0.5 * self.decel * (time_deceled * time_deceled))
            } else {
                self.dist_accel
                    + (self.v_max * (self.time_cruise + time_deceled))
                    - (0.5 * self.decel * (time_deceled * time_deceled))
            };

            let vel = if self.triangular {
                self.v_peak - (self.decel * time_deceled)
            } else {
                self.v_max - (self.decel * time_deceled)
            };

            (pos, vel, -self.decel)
        };

        MotionState {
            pos: self.x_initial + (self.direction * pos_local),
            vel: self.direction * vel_local,
            acc: self.direction * acc_local,
        }
    }

    /// Returns the total execution time of the motion profile.
    pub fn total_time(&self) -> f64 {
        self.time_total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapezoid_profile() {
        let profile = TrapezoidProfile::new(0.0, 10.0, 2.0, 1.0, 1.0);
        let start = profile.calculate(0.0);
        assert_eq!(start.pos, 0.0);
        assert_eq!(start.vel, 0.0);

        let end = profile.calculate(profile.total_time());
        assert!((end.pos - 10.0).abs() < 1e-6);
        assert!(end.vel.abs() < 1e-6);
    }
}
