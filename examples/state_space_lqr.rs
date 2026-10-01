//! State-Space & LQR Optimal Control Demo
//!
//! Demonstrates continuous-to-discrete system discretization, DARE solution,
//! and infinite-horizon Linear Quadratic Regulator (LQR) control.
//!
//! Run with:
//! ```bash
//! cargo run --example state_space_lqr
//! ```

use nalgebra::{SMatrix, SVector};
use robot_2027::core::controls::state_space::linear_quadratic_regulator::LinearQuadraticRegulator;

fn main() {
    println!("=== State-Space & LQR Optimal Control Demo ===");

    // System: 2-State Linear Model (Position and Velocity)
    // d/dt [position] = [0   1] [position] + [0  ] [voltage]
    // d/dt [velocity] = [0  -5] [velocity] + [40 ] [voltage]
    let a = SMatrix::<f64, 2, 2>::new(0.0, 1.0, 0.0, -5.0);
    let b = SMatrix::<f64, 2, 1>::new(0.0, 40.0);

    // Q: State penalty matrix (penalize position error heavily, velocity error moderately)
    // Q = diag([1.0 / max_pos_err^2, 1.0 / max_vel_err^2])
    let q = SMatrix::<f64, 2, 2>::new(10.0, 0.0, 0.0, 0.5);

    // R: Control effort penalty matrix (penalize high motor voltage: 1.0 / 12V^2)
    let r = SMatrix::<f64, 1, 1>::new(1.0 / 144.0);

    let dt = 0.02; // 20ms control loop (50 Hz)

    println!("Continuous A matrix:\n{:.2}", a);
    println!("Continuous B matrix:\n{:.2}", b);

    // Compute optimal LQR gain matrix K
    let lqr = LinearQuadraticRegulator::<2, 1>::new(&a, &b, &q, &r, dt);
    println!(
        "Optimal LQR Gain K: [{:.4}, {:.4}]\n",
        lqr.k()[(0, 0)],
        lqr.k()[(0, 1)]
    );

    // Simulate step response: Target Position = 24.0 inches, Target Velocity = 0.0
    let mut state = SVector::<f64, 2>::new(0.0, 0.0);
    let reference = SVector::<f64, 2>::new(24.0, 0.0);

    println!(
        "{:>5} | {:>10} | {:>10} | {:>8} | {:<20}",
        "Step", "Pos (in)", "Vel (in/s)", "Volts", "Position Progress"
    );
    println!("{:-<65}", "");

    for step in 0..30 {
        // u = K * (r - x)
        let control_u = lqr.calculate(&state, &reference);
        let voltage = control_u[0].clamp(-12.0, 12.0);

        // Discrete plant simulation step: x[k+1] = x[k] + (A*x + B*u)*dt
        let x_dot = a * state + b * SVector::<f64, 1>::new(voltage);
        state += x_dot * dt;

        let ratio = (state[0] / reference[0]).clamp(0.0, 1.2);
        let bar_len = (ratio * 15.0).round() as usize;
        let bar = format!(
            "[{:=>width$}{:<space$}]",
            "",
            "",
            width = bar_len.min(15),
            space = 15usize.saturating_sub(bar_len)
        );

        if step % 2 == 0 || (state[0] - reference[0]).abs() < 0.2 {
            println!(
                "{:5} | {:9.2} | {:9.2} | {:7.2}V | {}",
                step, state[0], state[1], voltage, bar
            );
        }

        if (state[0] - reference[0]).abs() < 0.1 && state[1].abs() < 0.2 {
            println!("\nLQR controller settled at setpoint in {} steps!", step);
            break;
        }
    }

    println!("\nState-space LQR demo completed successfully.");
}
