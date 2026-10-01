//! Linear Quadratic Regulator (LQR) optimal state-feedback controller.

use super::dare_solver::solve_dare;
use super::discretization::discretize_ab;
use super::linear_system::LinearSystem;
use nalgebra::{SMatrix, SVector};

/// Forms a diagonal cost matrix from a set of tolerances using Bryson's Rule:
///
/// `Q_ii = 1.0 / (tol_i^2)`
pub fn cost_matrix<const DIM: usize>(tolerances: &SVector<f64, DIM>) -> SMatrix<f64, DIM, DIM> {
    let mut q = SMatrix::<f64, DIM, DIM>::zeros();
    for i in 0..DIM {
        let tol = tolerances[i];
        q[(i, i)] = 1.0 / (tol * tol);
    }
    q
}

/// Linear Quadratic Regulator (LQR) controller optimizing gain matrix K for:
///
/// `u = K * (r - x)`
#[derive(Debug, Clone)]
pub struct LinearQuadraticRegulator<const STATES: usize, const INPUTS: usize> {
    k: SMatrix<f64, INPUTS, STATES>,
}

impl<const STATES: usize, const INPUTS: usize> LinearQuadraticRegulator<STATES, INPUTS> {
    /// Constructs an LQR controller from continuous system matrices, cost matrices, and timestep.
    pub fn new(
        a: &SMatrix<f64, STATES, STATES>,
        b: &SMatrix<f64, STATES, INPUTS>,
        q: &SMatrix<f64, STATES, STATES>,
        r: &SMatrix<f64, INPUTS, INPUTS>,
        dt: f64,
    ) -> Self {
        let (ad, bd) = discretize_ab(a, b, dt);
        let s = solve_dare(&ad, &bd, q, r);

        // K = (B_d^T * S * B_d + R)^-1 * (B_d^T * S * A_d)
        let inner = bd.transpose() * s * bd + r;
        let inner_chol = inner
            .cholesky()
            .expect("LQR: inner matrix B^T S B + R must be positive definite");
        let k = inner_chol.solve(&(bd.transpose() * s * ad));

        Self { k }
    }

    /// Constructs an LQR controller from state and input tolerance vectors using Bryson's rule.
    pub fn from_tolerances(
        a: &SMatrix<f64, STATES, STATES>,
        b: &SMatrix<f64, STATES, INPUTS>,
        q_tolerances: &SVector<f64, STATES>,
        r_tolerances: &SVector<f64, INPUTS>,
        dt: f64,
    ) -> Self {
        let q = cost_matrix(q_tolerances);
        let r = cost_matrix(r_tolerances);
        Self::new(a, b, &q, &r, dt)
    }

    /// Constructs an LQR controller from a LinearSystem plant.
    pub fn from_plant<const OUTPUTS: usize>(
        plant: &LinearSystem<STATES, INPUTS, OUTPUTS>,
        q_tolerances: &SVector<f64, STATES>,
        r_tolerances: &SVector<f64, INPUTS>,
        dt: f64,
    ) -> Self {
        Self::from_tolerances(plant.a(), plant.b(), q_tolerances, r_tolerances, dt)
    }

    /// Returns the optimal state feedback gain matrix K.
    pub fn k(&self) -> &SMatrix<f64, INPUTS, STATES> {
        &self.k
    }

    /// Computes optimal control input `u = K * (r - x)`.
    pub fn calculate(
        &self,
        x: &SVector<f64, STATES>,
        r: &SVector<f64, STATES>,
    ) -> SVector<f64, INPUTS> {
        self.k * (r - x)
    }

    /// Recomputes K to compensate for input/sensor latency:
    ///
    /// `K_delay = K * (A_d - B_d * K)^(input_delay / dt)`
    pub fn latency_compensate<const OUTPUTS: usize>(
        &mut self,
        plant: &LinearSystem<STATES, INPUTS, OUTPUTS>,
        dt: f64,
        input_delay: f64,
    ) {
        let (ad, bd) = plant.disc_ab(dt);
        let closed_loop = ad - bd * self.k;
        // Matrix power via log/exp or integer steps
        let steps = (input_delay / dt).round() as usize;
        let mut factor = SMatrix::<f64, STATES, STATES>::identity();
        for _ in 0..steps {
            factor *= closed_loop;
        }
        self.k *= factor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lqr_scalar() {
        let a = SMatrix::<f64, 1, 1>::new(0.0);
        let b = SMatrix::<f64, 1, 1>::new(1.0);
        let q = SMatrix::<f64, 1, 1>::new(1.0);
        let r = SMatrix::<f64, 1, 1>::new(1.0);

        let lqr = LinearQuadraticRegulator::new(&a, &b, &q, &r, 0.01);
        let x = SVector::<f64, 1>::new(1.0);
        let setpt = SVector::<f64, 1>::new(0.0);
        let u = lqr.calculate(&x, &setpt);
        // Should drive state toward zero (negative control output)
        assert!(u[0] < 0.0);
    }
}
