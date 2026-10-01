//! Discrete Kalman Filter with continuous noise discretization and Joseph-form covariance updates.

use crate::core::controls::state_space::{discretize_ab, LinearSystem};
use nalgebra::{SMatrix, SVector};

/// Linear Kalman filter combining model prediction with sensor measurements.
#[derive(Debug, Clone)]
pub struct KalmanFilter<const STATES: usize, const INPUTS: usize, const OUTPUTS: usize> {
    xhat: SVector<f64, STATES>,
    p: SMatrix<f64, STATES, STATES>,
    q_continuous: SMatrix<f64, STATES, STATES>,
    r: SMatrix<f64, OUTPUTS, OUTPUTS>,
    a: SMatrix<f64, STATES, STATES>,
    b: SMatrix<f64, STATES, INPUTS>,
    c: SMatrix<f64, OUTPUTS, STATES>,
    d: SMatrix<f64, OUTPUTS, INPUTS>,
}

impl<const STATES: usize, const INPUTS: usize, const OUTPUTS: usize>
    KalmanFilter<STATES, INPUTS, OUTPUTS>
{
    /// Constructs a Kalman filter from continuous system matrices and noise standard deviations.
    pub fn new(
        a: SMatrix<f64, STATES, STATES>,
        b: SMatrix<f64, STATES, INPUTS>,
        c: SMatrix<f64, OUTPUTS, STATES>,
        d: SMatrix<f64, OUTPUTS, INPUTS>,
        state_stddevs: &SVector<f64, STATES>,
        measurement_stddevs: &SVector<f64, OUTPUTS>,
    ) -> Self {
        let mut q = SMatrix::<f64, STATES, STATES>::zeros();
        for i in 0..STATES {
            let s = state_stddevs[i];
            q[(i, i)] = s * s;
        }

        let mut r = SMatrix::<f64, OUTPUTS, OUTPUTS>::zeros();
        for i in 0..OUTPUTS {
            let s = measurement_stddevs[i];
            r[(i, i)] = s * s;
        }

        Self {
            xhat: SVector::<f64, STATES>::zeros(),
            p: SMatrix::<f64, STATES, STATES>::zeros(),
            q_continuous: q,
            r,
            a,
            b,
            c,
            d,
        }
    }

    /// Constructs a Kalman filter from a LinearSystem plant.
    pub fn from_plant(
        plant: &LinearSystem<STATES, INPUTS, OUTPUTS>,
        state_stddevs: &SVector<f64, STATES>,
        measurement_stddevs: &SVector<f64, OUTPUTS>,
    ) -> Self {
        Self::new(
            *plant.a(),
            *plant.b(),
            *plant.c(),
            *plant.d(),
            state_stddevs,
            measurement_stddevs,
        )
    }

    /// Returns current state estimate `x_hat`.
    pub fn xhat(&self) -> &SVector<f64, STATES> {
        &self.xhat
    }

    /// Sets the state estimate `x_hat`.
    pub fn set_xhat(&mut self, xhat: SVector<f64, STATES>) {
        self.xhat = xhat;
    }

    /// Returns the state error covariance matrix P.
    pub fn p(&self) -> &SMatrix<f64, STATES, STATES> {
        &self.p
    }

    /// Sets the state error covariance matrix P.
    pub fn set_p(&mut self, p: SMatrix<f64, STATES, STATES>) {
        self.p = p;
    }

    /// Resets the state and covariance to zero.
    pub fn reset(&mut self) {
        self.xhat = SVector::<f64, STATES>::zeros();
        self.p = SMatrix::<f64, STATES, STATES>::zeros();
    }

    /// Projects the state forward by `dt` with control input `u`.
    pub fn predict(&mut self, u: &SVector<f64, INPUTS>, dt: f64) {
        let (ad, bd) = discretize_ab(&self.a, &self.b, dt);
        let q_discrete = self.q_continuous * dt;

        self.xhat = ad * self.xhat + bd * u;
        self.p = ad * self.p * ad.transpose() + q_discrete;
    }

    /// Corrects the state estimate using measurement `y` and control input `u`.
    pub fn correct(&mut self, y: &SVector<f64, OUTPUTS>, u: &SVector<f64, INPUTS>) {
        // Innovation covariance: Py = C * P * C^T + R
        let py = self.c * self.p * self.c.transpose() + self.r;

        // Kalman gain: K = P * C^T * Py^-1
        let py_chol = py.cholesky().expect("Py must be positive definite");
        let k = self.p * self.c.transpose() * py_chol.inverse();

        // State update: x_hat = x_hat + K * (y - (C * x_hat + D * u))
        let expected_y = self.c * self.xhat + self.d * u;
        self.xhat += k * (y - expected_y);

        // Joseph form covariance update: P = (I - K*C)*P*(I - K*C)^T + K*R*K^T
        let ident = SMatrix::<f64, STATES, STATES>::identity();
        let i_kc = ident - k * self.c;
        self.p = i_kc * self.p * i_kc.transpose() + k * self.r * k.transpose();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kalman_filter() {
        let a = SMatrix::<f64, 1, 1>::new(0.0);
        let b = SMatrix::<f64, 1, 1>::new(1.0);
        let c = SMatrix::<f64, 1, 1>::new(1.0);
        let d = SMatrix::<f64, 1, 1>::new(0.0);

        let q_std = SVector::<f64, 1>::new(0.1);
        let r_std = SVector::<f64, 1>::new(0.5);

        let mut kf = KalmanFilter::new(a, b, c, d, &q_std, &r_std);
        kf.predict(&SVector::<f64, 1>::new(1.0), 1.0);
        kf.correct(&SVector::<f64, 1>::new(1.0), &SVector::<f64, 1>::new(1.0));

        assert!((kf.xhat()[0] - 1.0).abs() < 0.2);
    }
}
