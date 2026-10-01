//! Continuous to discrete state-space system matrix discretization via matrix exponential.

use nalgebra::SMatrix;

/// Computes the matrix exponential using scaling and squaring with Padé approximants.
pub fn matrix_exp<const D: usize>(m: &SMatrix<f64, D, D>) -> SMatrix<f64, D, D> {
    // 1-norm computation
    let mut norm1: f64 = 0.0;
    for c in 0..D {
        let col_sum: f64 = (0..D).map(|r| m[(r, c)].abs()).sum();
        if col_sum > norm1 {
            norm1 = col_sum;
        }
    }

    // Scaling parameter
    let mut s: i32 = 0;
    if norm1 > 0.5 {
        s = (norm1.log2().ceil() as i32) + 1;
        if s < 0 {
            s = 0;
        }
    }

    let scale = 1.0 / (2.0f64).powi(s);
    let a_scaled = m * scale;

    // Padé [6/6] approximant coefficients
    // c_j = (2p - j)! p! / ((2p)! j! (p - j)!)
    let c = [
        1.0,
        0.5,
        0.12,
        0.01818181818181818,
        0.001929314050212727,
        0.0001368940866579899,
        0.000004563136221932997,
    ];

    let ident = SMatrix::<f64, D, D>::identity();
    let mut a_pow = ident;
    let mut p = ident;
    let mut q = ident;

    for (j, &cj) in c.iter().enumerate().skip(1) {
        a_pow *= a_scaled;
        let term = a_pow * cj;
        p += term;
        if j % 2 == 1 {
            q -= term;
        } else {
            q += term;
        }
    }

    // Solve Q * R = P => R = Q^-1 * P
    let mut r = q.try_inverse().map(|inv| inv * p).unwrap_or(p);

    // Squaring step
    for _ in 0..s {
        r = r * r;
    }

    r
}

/// Discretizes continuous system matrices `(Ac, Bc)` over timestep `dt`.
///
/// Form:
/// ```text
///       [Ac  Bc]
///   M = [ 0   0]
///
///  e^(M * dt) = [Ad  Bd]
///               [ 0   I]
/// ```
pub fn discretize_ab<const STATES: usize, const INPUTS: usize>(
    ac: &SMatrix<f64, STATES, STATES>,
    bc: &SMatrix<f64, STATES, INPUTS>,
    dt: f64,
) -> (SMatrix<f64, STATES, STATES>, SMatrix<f64, STATES, INPUTS>) {
    // For general static dimension combination:
    let mut ad = SMatrix::<f64, STATES, STATES>::identity();
    let _bd = *bc * dt;

    // Series expansion:
    // Ad = I + Ac*dt + (Ac*dt)^2 / 2! + (Ac*dt)^3 / 3! + ...
    // Bd = (I*dt + Ac*dt^2 / 2! + Ac^2*dt^3 / 3! + ...) * Bc
    let mut term_a = SMatrix::<f64, STATES, STATES>::identity();
    let mut term_b = SMatrix::<f64, STATES, STATES>::identity() * dt;
    let mut factorial: f64 = 1.0;

    for k in 1..25 {
        factorial *= k as f64;
        term_a *= ac * dt;
        ad += term_a / factorial;

        term_b += (term_a * dt) / (k + 1) as f64;
    }

    let bd_result = term_b * bc;
    (ad, bd_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discretize_ab_scalar() {
        let a = SMatrix::<f64, 1, 1>::new(0.0);
        let b = SMatrix::<f64, 1, 1>::new(1.0);
        let (ad, bd) = discretize_ab(&a, &b, 0.1);

        assert!((ad[(0, 0)] - 1.0).abs() < 1e-6);
        assert!((bd[(0, 0)] - 0.1).abs() < 1e-6);
    }
}
