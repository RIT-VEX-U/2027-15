//! Pure Pursuit Path Tracking Simulation Example
//!
//! Demonstrates how to generate, smooth, and track a trajectory using pure pursuit.
//!
//! Run with:
//! ```bash
//! cargo run --example pure_pursuit_simulation
//! ```

use robot_2027::core::geometry::{Pose2d, Rotation2d, Translation2d};
use robot_2027::core::pathing::pure_pursuit::{get_lookahead, inject_path, smooth_path, Path};

fn main() {
    println!("=== Pure Pursuit Simulation Demo ===");

    // 1. Define coarse autonomous waypoints (inches on a 144x144 VEX field)
    let waypoints = vec![
        Translation2d::new(0.0, 0.0),
        Translation2d::new(24.0, 12.0),
        Translation2d::new(48.0, 48.0),
        Translation2d::new(72.0, 48.0),
        Translation2d::new(96.0, 24.0),
    ];

    println!("Coarse Waypoints:");
    for (i, pt) in waypoints.iter().enumerate() {
        println!("  [{}] ({:.1}, {:.1})", i, pt.x(), pt.y());
    }

    // 2. Inject intermediate points every 3 inches
    let injected = inject_path(&waypoints, 3.0);
    println!("\nInjected points count: {}", injected.len());

    // 3. Smooth the path with gradient descent relaxation
    let smoothed = smooth_path(&injected, 0.2, 0.8, 0.001);
    println!("Smoothed path points: {}", smoothed.len());

    // 4. Construct Pure Pursuit Path with a 10-inch lookahead radius
    let lookahead_radius = 10.0;
    let path = Path::new(smoothed.clone(), lookahead_radius);
    println!(
        "Path valid (no self-intersecting loops within lookahead): {}",
        path.is_valid()
    );

    // 5. Simulate robot starting at (0, 0) tracking the path
    println!("\n--- Simulating Robot Motion ---");
    let mut current_pose = Pose2d::new(0.0, 0.0, Rotation2d::from_degrees(0.0));
    let dt = 0.1; // 100ms control loop
    let speed = 20.0; // inches per second

    let target_end = *smoothed.last().unwrap();

    for step in 0..60 {
        let dist_to_goal = current_pose.translation().distance(target_end);
        if dist_to_goal < 2.0 {
            println!(
                "Step {:02}: Reached goal! Final Pose: X={:.2}, Y={:.2}, Angle={:.1} deg",
                step,
                current_pose.x(),
                current_pose.y(),
                current_pose.rotation().degrees()
            );
            break;
        }

        // Compute pure pursuit lookahead point
        let lookahead = get_lookahead(&smoothed, current_pose, lookahead_radius);
        let target_x = lookahead.x();
        let target_y = lookahead.y();

        // Calculate heading to lookahead point
        let desired_heading = (target_y - current_pose.y())
            .atan2(target_x - current_pose.x())
            .to_degrees();

        // Move robot towards lookahead
        let move_dist = speed * dt;
        let rad = desired_heading.to_radians();
        let new_x = current_pose.x() + move_dist * rad.cos();
        let new_y = current_pose.y() + move_dist * rad.sin();

        current_pose = Pose2d::new(new_x, new_y, Rotation2d::from_degrees(desired_heading));

        if step % 5 == 0 || step < 5 {
            println!(
                "Step {:02}: Pose=({:5.1}, {:5.1}) | Heading={:5.1}° | Lookahead=({:5.1}, {:5.1}) | Dist to goal={:5.1}\"",
                step,
                current_pose.x(),
                current_pose.y(),
                current_pose.rotation().degrees(),
                target_x,
                target_y,
                dist_to_goal
            );
        }
    }

    println!("\nSimulation completed successfully.");
}
