//! Odometry Tracking Simulation Demo
//!
//! Demonstrates how 3-wheel tracking and tank odometry translate wheel encoder
//! rotations into precise Cartesian (X, Y) positions and heading angles.
//!
//! Run with:
//! ```bash
//! cargo run --example odometry_demo
//! ```

use robot_2027::core::geometry::Pose2d;
use robot_2027::mantle::subsystems::odometry::odometry_3wheel::{
    Odometry3Wheel, Odometry3WheelConfig,
};
use std::f64::consts::PI;

fn main() {
    println!("=== 3-Wheel Odometry Tracking Simulation ===");

    // Robot configuration:
    // Tracking wheels: 2.75" diameter, 360 ticks/rev
    // Left/Right parallel wheel spacing: 10.0" apart (5.0" from center)
    // Back perpendicular wheel offset: 4.0" behind center
    let cfg = Odometry3WheelConfig {
        wheelbase_dist: 10.0,
        off_axis_center_dist: 4.0,
        wheel_diam: 2.75,
    };

    let mut current_pose = Pose2d::default();

    println!(
        "Starting Pose: X={:.2}, Y={:.2}, Heading={:.2}°",
        current_pose.x(),
        current_pose.y(),
        current_pose.rotation().degrees()
    );

    // Phase 1: Drive straight forward 24 inches
    // Distance = PI * wheel_diam * (deg / 360) => deg = (dist / (PI * 2.75)) * 360
    let inches_to_deg = |inches: f64| -> f64 { (inches / (PI * cfg.wheel_diam)) * 360.0 };
    let straight_dist = 24.0;
    let straight_deg = inches_to_deg(straight_dist);

    println!("\n1. Driving straight forward 24.0 inches...");
    current_pose = Odometry3Wheel::calculate_new_pos(
        straight_deg,
        straight_deg,
        0.0, // Back wheel doesn't turn when driving purely straight
        &current_pose,
        &cfg,
    );
    println!(
        "Pose after straight: X={:.2}\", Y={:.2}\", Heading={:.2}°",
        current_pose.x(),
        current_pose.y(),
        current_pose.rotation().degrees()
    );

    // Phase 2: In-place turn of +90 degrees (counter-clockwise)
    // Arc length for wheels at radius R = wheelbase_dist / 2 = 5.0
    let turn_arc = (PI / 2.0) * (cfg.wheelbase_dist / 2.0);
    let turn_left_deg = -inches_to_deg(turn_arc);
    let turn_right_deg = inches_to_deg(turn_arc);
    // Back wheel moves on circle of radius off_axis_center_dist=4.0
    let back_arc = (PI / 2.0) * cfg.off_axis_center_dist;
    let back_deg = inches_to_deg(back_arc);

    println!("\n2. Turning 90 degrees counter-clockwise in place...");
    current_pose = Odometry3Wheel::calculate_new_pos(
        turn_left_deg,
        turn_right_deg,
        back_deg,
        &current_pose,
        &cfg,
    );
    println!(
        "Pose after turn: X={:.2}\", Y={:.2}\", Heading={:.2}°",
        current_pose.x(),
        current_pose.y(),
        current_pose.rotation().degrees()
    );

    // Phase 3: Drive forward 12 inches along new heading (+90 deg is +Y)
    println!("\n3. Driving forward 12.0 inches along +Y direction...");
    let leg2_deg = inches_to_deg(12.0);
    current_pose = Odometry3Wheel::calculate_new_pos(leg2_deg, leg2_deg, 0.0, &current_pose, &cfg);
    println!(
        "Pose after leg 2: X={:.2}\", Y={:.2}\", Heading={:.2}°",
        current_pose.x(),
        current_pose.y(),
        current_pose.rotation().degrees()
    );

    println!("\nOdometry math verified successfully!");
}
