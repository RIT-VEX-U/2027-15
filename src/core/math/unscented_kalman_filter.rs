//! Non-linear Square-Root Unscented Kalman Filter (SR-UKF) using Scaled Spherical Simplex Sigma Points.

use nalgebra::{DMatrix, DVector};

/// Generates Scaled Spherical Simplex Sigma Points (N + 2 points instead of standard 2N + 1).
#[derive(Debug, Clone)]
pub struct ScaledSphericalSimplexSigmaPoints {
    states: usize,
    num_sigmas: usize,
    wm: DVector<f64>,
    wc: DVector<f64>,
    c_mat: DMatrix<f64>,
}

impl ScaledSphericalSimplexSigmaPoints {
    /// Creates a new sigma point generator.
    pub fn new(states: usize, alpha: f64, beta: f64) -> Self {
        let num_sigmas = states + 2;
        let c = 1.0 / (alpha * alpha * ((states + 1) as f64));
        let mut wm = DVector::from_element(num_sigmas, c);
        let mut wc = DVector::from_element(num_sigmas, c);

        wm[0] = 1.0 - (1.0 / (alpha * alpha));
        wc[0] = 1.0 - (1.0 / (alpha * alpha)) + (1.0 - alpha * alpha + beta);

        let mut q = DVector::<f64>::zeros(states);
        for i in 0..states {
            let t = (i + 1) as f64;
            q[i] = alpha * ((t * ((states + 1) as f64)) / (t + 1.0)).sqrt();
        }

        let mut c_mat = DMatrix::<f64>::zeros(states, num_sigmas);
        for row in 0..states {
            for col in 1..=(row + 1) {
                c_mat[(row, col)] = -q[row] / ((row + 1) as f64);
            }
            if row + 2 < num_sigmas {
                c_mat[(row, row + 2)] = q[row];
            }
        }

        Self {
            states,
            num_sigmas,
            wm,
            wc,
            c_mat,
        }
    }

    /// Number of sigma points.
    pub fn num_sigmas(&self) -> usize {
        self.num_sigmas
    }

    /// Computes sigma points from state mean and square-root covariance S.
    pub fn square_root_sigma_points(&self, x: &DVector<f64>, s: &DMatrix<f64>) -> DMatrix<f64> {
        let mut sigmas = s * &self.c_mat;
        for mut col in sigmas.column_iter_mut() {
            col += x;
        }
        sigmas
    }

    pub fn wm(&self) -> &DVector<f64> {
        &self.wm
    }

    pub fn wc(&self) -> &DVector<f64> {
        &self.wc
    }
}

/// Square-Root Unscented Kalman Filter.
pub struct UnscentedKalmanFilter<F, H> {
    states: usize,
    inputs: usize,
    outputs: usize,
    f: F,
    h: H,
    xhat: DVector<f64>,
    s: DMatrix<f64>,
    sqrt_q: DMatrix<f64>,
    sqrt_r: DMatrix<f64>,
    pts: ScaledSphericalSimplexSigmaPoints,
}

impl<F, H> UnscentedKalmanFilter<F, H>
where
    F: Fn(&DVector<f64>, &DVector<f64>) -> DVector<f64>,
    H: Fn(&DVector<f64>, &DVector<f64>) -> DVector<f64>,
{
    /// Constructs a new Unscented Kalman Filter.
    pub fn new(
        states: usize,
        inputs: usize,
        outputs: usize,
        f: F,
        h: H,
        state_stddevs: &DVector<f64>,
        measurement_stddevs: &DVector<f64>,
    ) -> Self {
        let mut sqrt_q = DMatrix::zeros(states, states);
        for i in 0..states {
            sqrt_q[(i, i)] = state_stddevs[i];
        }

        let mut sqrt_r = DMatrix::zeros(outputs, outputs);
        for i in 0..outputs {
            sqrt_r[(i, i)] = measurement_stddevs[i];
        }

        let pts = ScaledSphericalSimplexSigmaPoints::new(states, 0.001, 2.0);

        Self {
            states,
            inputs,
            outputs,
            f,
            h,
            xhat: DVector::zeros(states),
            s: DMatrix::zeros(states, states),
            sqrt_q,
            sqrt_r,
            pts,
        }
    }

    /// Returns current state estimate `x_hat`.
    pub fn xhat(&self) -> &DVector<f64> {
        &self.xhat
    }

    /// Sets state estimate `x_hat`.
    pub fn set_xhat(&mut self, xhat: DVector<f64>) {
        self.xhat = xhat;
    }

    /// Resets the filter state and covariance.
    pub fn reset(&mut self) {
        self.xhat.fill(0.0);
        self.s.fill(0.0);
    }

    /// Predict step advancing state by `dt` with control input `u`.
    pub fn predict(&mut self, u: &DVector<f64>, dt: f64) {
        let sigmas = self.pts.square_root_sigma_points(&self.xhat, &self.s);
        let num_pts = self.pts.num_sigmas();

        let mut sigmas_f = DMatrix::zeros(self.states, num_pts);
        for i in 0..num_pts {
            let col = sigmas.column(i).into_owned();
            let deriv = (self.f)(&col, u);
            let next_pt = col + deriv * dt;
            sigmas_f.set_column(i, &next_pt);
        }

        // Mean computation: x_hat = sum(Wm_i * X_i)
        let mut x_new = DVector::zeros(self.states);
        for i in 0..num_pts {
            x_new += sigmas_f.column(i) * self.pts.wm()[i];
        }
        self.xhat = x_new;

        // Covariance update: S = qr( [sqrt(Wc_1)*(X_1..L - x_hat), sqrt(Q)*sqrt(dt)] )
        let mut s_cov = &self.sqrt_q * dt.sqrt();
        for i in 1..num_pts {
            let diff = sigmas_f.column(i) - &self.xhat;
            s_cov += &diff * diff.transpose() * self.pts.wc()[i];
        }
        self.s = s_cov;
    }

    /// Correct step incorporating measurement `y` with control input `u`.
    pub fn correct(&mut self, u: &DVector<f64>, y: &DVector<f64>) {
        let sigmas = self.pts.square_root_sigma_points(&self.xhat, &self.s);
        let num_pts = self.pts.num_sigmas();

        let mut sigmas_h = DMatrix::zeros(self.outputs, num_pts);
        for i in 0..num_pts {
            let col = sigmas.column(i).into_owned();
            let meas = (self.h)(&col, u);
            sigmas_h.set_column(i, &meas);
        }

        let mut yhat = DVector::zeros(self.outputs);
        for i in 0..num_pts {
            yhat += sigmas_h.column(i) * self.pts.wm()[i];
        }

        let mut py = &self.sqrt_r * &self.sqrt_r;
        let mut pxy = DMatrix::zeros(self.states, self.outputs);

        for i in 0..num_pts {
            let y_diff = sigmas_h.column(i) - &yhat;
            let x_diff = sigmas.column(i) - &self.xhat;
            let y_diff_t = y_diff.transpose();
            let w = self.pts.wc()[i];
            py += &y_diff * &y_diff_t * w;
            pxy += &x_diff * &y_diff_t * w;
        }

        if let Some(py_inv) = py.try_inverse() {
            let k = pxy * py_inv;
            self.xhat += &k * (y - &yhat);
        }
    }
}
