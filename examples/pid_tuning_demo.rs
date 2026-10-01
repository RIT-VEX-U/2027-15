//! PID Controller & Plant Simulation Demo
//!
//! Demonstrates simulated closed-loop feedback control using `PID` and `FeedForward`.
//!
//! Run with:
//! ```bash
//! cargo run --example pid_tuning_demo
//! ```

use robot_2027::core::controls::feedback_base::Feedback;
use robot_2027::core::controls::feedforward::{FeedForward, FeedForwardConfig};
use robot_2027::core::controls::pid::{PIDConfig, PID};

fn main() {
    println!("=== PID Closed-Loop Tuning & Step Response Demo ===");

    let target_rpm = 3000.0;

    // Configure PID feedback gains
    let pid_cfg = PIDConfig {
        p: 0.004,
        i: 0.0002,
        d: 0.0001,
        deadband: 10.0,
        on_target_time: 0.1,
        ..Default::default()
    };

    // Configure velocity feedforward (kV to reach target RPM with voltage)
    let ff_cfg = FeedForwardConfig {
        ks: 0.2,           // static friction voltage
        kv: 12.0 / 3600.0, // velocity gain: ~3.33 mV/RPM
        ka: 0.0005,
        kg: 0.0,
    };

    let mut pid = PID::new(pid_cfg);
    let ff = FeedForward::new(ff_cfg);

    pid.set_target(target_rpm);
    let ff_voltage = ff.calculate(target_rpm, 0.0, target_rpm);

    println!(
        "Target: {:.0} RPM | Feedforward Voltage: {:.2} V\n",
        target_rpm, ff_voltage
    );
    println!(
        "{:>5} | {:>10} | {:>10} | {:>8} | {:<25}",
        "Step", "RPM", "Error", "Volts", "Visual Output (Bar)"
    );
    println!("{:-<65}", "");

    // Simulated 1st-order velocity plant: RPM[t+1] = RPM[t] * 0.85 + Volts * 40.0
    let mut current_rpm = 0.0;
    let dt = 0.02; // 20ms update rate

    for step in 0..40 {
        let error = target_rpm - current_rpm;
        let pid_out = pid.update_dt(current_rpm, dt);
        let total_volts = (ff_voltage + pid_out).clamp(-12.0, 12.0);

        // Plant dynamic simulation (motor torque vs back-EMF and friction)
        let accel = (total_volts * 300.0) - (current_rpm * 1.0);
        current_rpm += accel * dt;
        if current_rpm < 0.0 {
            current_rpm = 0.0;
        }

        // Draw an ASCII bar indicating percentage of target RPM reached
        let ratio = (current_rpm / target_rpm).clamp(0.0, 1.2);
        let bar_len = (ratio * 20.0).round() as usize;
        let bar = format!(
            "[{:=>width$}{:<space$}]",
            "",
            "",
            width = bar_len.min(20),
            space = 20usize.saturating_sub(bar_len)
        );

        if step % 2 == 0 || (current_rpm - target_rpm).abs() < 15.0 {
            println!(
                "{:5} | {:9.1} | {:9.1} | {:7.2}V | {}",
                step, current_rpm, error, total_volts, bar
            );
        }

        if pid.is_on_target() {
            println!(
                "\nController converged on target within deadband in {} steps!",
                step
            );
            break;
        }
    }

    println!("\nPID simulation finished successfully.");
}
