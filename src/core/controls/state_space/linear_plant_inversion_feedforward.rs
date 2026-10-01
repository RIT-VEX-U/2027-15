//! Linear plant inversion feedforward controller.

use super::discretization::discretize_ab;
use super::linear_system::LinearSystem;
use nalgebra::{SMatrix, SVector};

/// Computes feedforward control inputs by inverting discrete plant dynamics:
///
/// `B_d * u_ff = next_r - A_d * r`
#[derive(Debug, Clone)]
pub struct LinearPlantInversionFeedforward<const STATES: usize, const INPUTS: usize> {
    ac: SMatrix<f64, STATES, STATES>,
    bc: SMatrix<f64, STATES, INPUTS>,
    ad: SMatrix<f64, STATES, STATES>,
    bd: SMatrix<f64, STATES, INPUTS>,
    uff: SVector<f64, INPUTS>,
    r: SVector<f64, STATES>,
    dt: f64,
}

impl<const STATES: usize, const INPUTS: usize> LinearPlantInversionFeedforward<STATES, INPUTS> {
    /// Constructs a feedforward controller from continuous system matrices and nominal timestep.
    pub fn new(
        a: SMatrix<f64, STATES, STATES>,
        b: SMatrix<f64, STATES, INPUTS>,
        dt: f64,
    ) -> Self {
        let (ad, bd) = discretize_ab(&a, &b, dt);
        Self {
            ac: a,
            bc: b,
            ad,
            bd,
            uff: SVector::<f64, INPUTS>::zeros(),
            r: SVector::<f64, STATES>::zeros(),
            dt,
        }
    }

    /// Constructs a feedforward controller from a LinearSystem.
    pub fn from_plant<const OUTPUTS: usize>(
        plant: &LinearSystem<STATES, INPUTS, OUTPUTS>,
        dt: f64,
    ) -> Self {
        Self::new(*plant.a(), *plant.b(), dt)
    }

    /// Computes feedforward control input from current reference state `r` and next reference state `next_r`.
    pub fn calculate(
        &mut self,
        r: &SVector<f64, STATES>,
        next_r: &SVector<f64, STATES>,
    ) -> SVector<f64, INPUTS> {
        let rhs = next_r - self.ad * r;
        let b_t = self.bd.transpose();
        let btb = b_t * self.bd;
        self.uff = btb
            .try_inverse()
            .map(|inv| inv * b_t * rhs)
            .unwrap_or_else(SVector::<f64, INPUTS>::zeros);
        self.r = *next_r;
        self.uff
    }

    /// Computes feedforward control input using the stored reference and `next_r`.
    pub fn calculate_next(&mut self, next_r: &SVector<f64, STATES>) -> SVector<f64, INPUTS> {
        let current_r = self.r;
        self.calculate(&current_r, next_r)
    }

    /// Resets the reference state and feedforward output.
    pub fn reset(&mut self, initial_state: SVector<f64, STATES>) {
        self.r = initial_state;
        self.uff = SVector::<f64, INPUTS>::zeros();
    }
}
