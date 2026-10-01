//! State-space control and estimation methods.

pub mod dare_solver;
pub mod discretization;
pub mod linear_plant_inversion_feedforward;
pub mod linear_quadratic_regulator;
pub mod linear_system;

pub use dare_solver::solve_dare;
pub use discretization::{discretize_ab, matrix_exp};
pub use linear_plant_inversion_feedforward::LinearPlantInversionFeedforward;
pub use linear_quadratic_regulator::{cost_matrix, LinearQuadraticRegulator};
pub use linear_system::LinearSystem;
