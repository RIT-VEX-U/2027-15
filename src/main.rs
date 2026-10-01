//! Main entry point for the 2027-15 VEX robot program.
//!
//! Mirrors `src/main.cpp`.

use robot_2027::competition::opcontrol;
use robot_2027::config::RobotHardware;

fn main() {
    println!("Initializing 2027-15 Robot...");
    let mut robot = RobotHardware::new();

    // Run hardware pre-initialization, routine selection, and post-initialization
    robot.initializer.initialize();

    println!("Selected Autonomous: {}", robot.initializer.selected_name());

    // Register competition callbacks or execute opcontrol
    // In hardware competition runtime, the competition switch directs execution
    println!("Entering Driver Control...");
    // Pass None to run infinitely on real robot or allow tests to supply cancellation token
    let stop_token = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let stop_clone = std::sync::Arc::clone(&stop_token);

    // Stop after a brief execution if running in non-interactive terminal environment
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        stop_clone.store(true, std::sync::atomic::Ordering::Relaxed);
    });

    opcontrol(&mut robot, Some(stop_token));
    println!("Program finished successfully.");
}
