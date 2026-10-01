//! Discrete Algebraic Riccati Equation (DARE) solver using Structure-Preserving Doubling Algorithm (SDA).

use nalgebra::SMatrix;

/// Computes the unique stabilizing positive semi-definite solution X to the discrete algebraic Riccati equation:
///
/// `A^T X A - X - A^T X B (B^T X B + R)^-1 B^T X A + Q = 0`
pub fn solve_dare<const STATES: usize, const INPUTS: usize>(
    a: &SMatrix<f64, STATES, STATES>,
    b: &SMatrix<f64, STATES, INPUTS>,
    q: &SMatrix<f64, STATES, STATES>,
    r: &SMatrix<f64, INPUTS, INPUTS>,
) -> SMatrix<f64, STATES, STATES> {
    let r_chol = r
        .cholesky()
        .expect("DARE solver requires R to be positive definite");

    let mut a_k = *a;
    // G_0 = B * R^-1 * B^T
    let mut g_k = *b * r_chol.solve(&b.transpose());
    let mut h_k1 = *q;

    let ident = SMatrix::<f64, STATES, STATES>::identity();

    for _iteration in 0..100 {
        let h_k = h_k1;

        // W = I + G_k * H_k
        let w = ident + g_k * h_k;

        let w_inv = match w.try_inverse() {
            Some(inv) => inv,
            None => break,
        };

        // Solve W * V_1 = A_k => V_1 = W^-1 * A_k
        let v_1 = w_inv * a_k;

        // Solve W * V_2^T = G_k^T => V_2 = (W^-1 * G_k^T)^T
        let v_2 = (w_inv * g_k.transpose()).transpose();

        // Update iterates
        g_k += a_k * v_2 * a_k.transpose();
        h_k1 = h_k + v_1.transpose() * h_k * a_k;
        a_k *= v_1;

        let diff_norm = (h_k1 - h_k).norm();
        let h_norm = h_k1.norm();
        if diff_norm <= 1e-10 * h_norm || diff_norm < 1e-12 {
            break;
        }
    }

    h_k1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dare_scalar() {
        // Scalar system: x_{k+1} = a*x_k + b*u_k, Q = 1, R = 1, a = 1, b = 1
        // Discrete Riccati equation: p = a^2 p - a^2 b^2 p^2 / (r + b^2 p) + q
        // p = p - p^2 / (1 + p) + 1 => p^2 - p - 1 = 0 => p = (1 + sqrt(5))/2 = 1.6180339887...
        let a = SMatrix::<f64, 1, 1>::new(1.0);
        let b = SMatrix::<f64, 1, 1>::new(1.0);
        let q = SMatrix::<f64, 1, 1>::new(1.0);
        let r = SMatrix::<f64, 1, 1>::new(1.0);

        let p = solve_dare(&a, &b, &q, &r);
        let golden = (1.0 + 5.0f64.sqrt()) / 2.0;
        assert!((p[(0, 0)] - golden).abs() < 1e-6);
    }
}
