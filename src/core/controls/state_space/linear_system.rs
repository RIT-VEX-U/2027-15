//! Continuous state-space linear dynamic system model.

use super::discretization::discretize_ab;
use nalgebra::{SMatrix, SVector};

/// Represents a continuous state-space linear time-invariant system:
///
/// `dx/dt = A * x + B * u`
///
/// `y = C * x + D * u`
#[derive(Debug, Clone)]
pub struct LinearSystem<const STATES: usize, const INPUTS: usize, const OUTPUTS: usize> {
    ac: SMatrix<f64, STATES, STATES>,
    bc: SMatrix<f64, STATES, INPUTS>,
    c: SMatrix<f64, OUTPUTS, STATES>,
    d: SMatrix<f64, OUTPUTS, INPUTS>,
}

impl<const STATES: usize, const INPUTS: usize, const OUTPUTS: usize>
    LinearSystem<STATES, INPUTS, OUTPUTS>
{
    /// Constructs a continuous linear system.
    pub fn new(
        a: SMatrix<f64, STATES, STATES>,
        b: SMatrix<f64, STATES, INPUTS>,
        c: SMatrix<f64, OUTPUTS, STATES>,
        d: SMatrix<f64, OUTPUTS, INPUTS>,
    ) -> Self {
        Self { ac: a, bc: b, c, d }
    }

    /// Returns the continuous system matrix A.
    pub fn a(&self) -> &SMatrix<f64, STATES, STATES> {
        &self.ac
    }

    /// Returns the continuous input matrix B.
    pub fn b(&self) -> &SMatrix<f64, STATES, INPUTS> {
        &self.bc
    }

    /// Returns the output matrix C.
    pub fn c(&self) -> &SMatrix<f64, OUTPUTS, STATES> {
        &self.c
    }

    /// Returns the feedthrough matrix D.
    pub fn d(&self) -> &SMatrix<f64, OUTPUTS, INPUTS> {
        &self.d
    }

    /// Discretizes the system over timestep `dt`.
    pub fn disc_ab(&self, dt: f64) -> (SMatrix<f64, STATES, STATES>, SMatrix<f64, STATES, INPUTS>) {
        discretize_ab(&self.ac, &self.bc, dt)
    }

    /// Propagates the state forward by `dt` with control input `u`.
    pub fn compute_x(
        &self,
        x: &SVector<f64, STATES>,
        u: &SVector<f64, INPUTS>,
        dt: f64,
    ) -> SVector<f64, STATES> {
        let (ad, bd) = self.disc_ab(dt);
        ad * x + bd * u
    }

    /// Computes the system output `y = C * x + D * u`.
    pub fn compute_y(
        &self,
        x: &SVector<f64, STATES>,
        u: &SVector<f64, INPUTS>,
    ) -> SVector<f64, OUTPUTS> {
        self.c * x + self.d * u
    }
}
