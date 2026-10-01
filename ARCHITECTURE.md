# RIT VEX U 2027-15 Software Architecture & Developer Onboarding Guide

Welcome to the **RIT VEX U Robotics (2027-15)** software codebase! This guide is written for new and returning software team members to get up to speed quickly on how the robot's code is designed, how to write autonomous routines, how to add new robot subsystems, and how to verify everything locally before flashing onto the V5 Brain.

---

## 1. The 3-Tier Planetary Architecture

To keep the codebase maintainable, testable, and isolated from vendor-specific bugs, our software is split into **three concentric layers** plus an outer **config** layer:

```text
  [ CONFIG ]     Robot hardware mapping, physical specs, PID gains, port assignments
      |
      v
  [ CRUST ]      Direct VEX V5 hardware drivers, V5 brain LCD, competition hooks
      |
      v
  [ MANTLE ]     Hardware Abstraction Layer (HAL): generic traits, mock devices, subsystems
      |
      v
  [ CORE ]       Platform-agnostic math: SE(2) geometry, kinematics, LQR, DARE, Kalman/UKF
```

### Strict Inward Dependency Rule
1. **`src/core/` (Inner Engine)**:
   - Contains pure mathematics, geometry, path tracking, digital filters, and control algorithms.
   - **STRICT RULE**: **ZERO** imports of VEX APIs, PROS, or vendor SDKs. Must compile anywhere without hardware.
2. **`src/mantle/` (Hardware Abstraction Layer - HAL)**:
   - Defines hardware-agnostic traits (`Motor`, `Encoder`, `InertialSensor`, `Controller`, `Display`).
   - Implements high-level robot subsystems (`TankDrive`, `Lift`, `Flywheel`, `Odometry`).
   - Implements communication protocols: COBS packet framing and the VEX Database Protocol (VDP/VDB).
   - Implements mock devices (`MockMotor`, `MockEncoder`, `MockDisplay`) enabling 100% unit test coverage on laptops without physical hardware.
3. **`src/crust/` (Direct VEX Integration)**:
   - Implements Mantle traits using concrete VEX V5 SDK interfaces (`V5Motor`, `Encoder`, `Inertial`, `BrainScreen`).
   - Manages RTOS competition lifecycle tasks (`autonomous`, `opcontrol`, `disabled`).
4. **`src/config/` (Hardware Map & Configuration)**:
   - Declares physical specs: track width, wheel diameter, gear ratios ([`specs.rs`](src/config/specs.rs)).
   - Declares smart port numbers and 3-wire ADI ports ([`ports.rs`](src/config/ports.rs)).
   - Declares tuned PID and feedforward constants ([`gains.rs`](src/config/gains.rs)).
   - Bundles all components into the [`RobotHardware`](src/config/robot.rs) struct.

---

## 2. Coordinate System Conventions

Our odometry, pure pursuit, and kinematics adhere to standard robotics conventions ($SE(2)$ Lie group algebra):

```text
                  +Y (Left)
                     ^
                     |
                     |       +θ (Counter-Clockwise)
                     |      /
                     |     v
       --------------+--------------> +X (Forward)
                     |
                     |
                     |
```

- **$+X$**: Forward direction of the robot / field.
- **$+Y$**: Left direction of the robot / field.
- **$\theta$ (Heading)**:
  - $0^\circ$ or $0$ radians aligns with $+X$.
  - $+90^\circ$ aligns with $+Y$.
  - Positive rotation is **Counter-Clockwise (CCW)**.
- **Units**:
  - Distance: **Inches**
  - Angles: Degrees or Radians (wrap-around normalized to $[-180^\circ, +180^\circ]$ or $[-\pi, +\pi]$)
  - Time: Seconds / Milliseconds (`Timer` monotonic clock)
  - Motors: Volts ($-12.0\text{V}$ to $+12.0\text{V}$) or RPM

---

## 3. Developer Tutorials

### Tutorial 1: How to Add a New Subsystem (e.g. An Intake)

1. **Define the subsystem in `src/mantle/subsystems/`**:
   Create `src/mantle/subsystems/intake.rs`:
   ```rust
   use std::sync::{Arc, Mutex};
   use crate::mantle::motor::{DirectionType, Motor, VoltageUnits};

   pub struct Intake {
       motor: Arc<Mutex<Box<dyn Motor>>>,
       intake_voltage: f64,
   }

   impl Intake {
       pub fn new(motor: Arc<Mutex<Box<dyn Motor>>>, intake_voltage: f64) -> Self {
           Self { motor, intake_voltage }
       }

       pub fn run_in(&mut self) {
           let mut m = self.motor.lock().unwrap();
           m.spin(DirectionType::Forward, self.intake_voltage, VoltageUnits::Volt);
       }

       pub fn run_out(&mut self) {
           let mut m = self.motor.lock().unwrap();
           m.spin(DirectionType::Reverse, self.intake_voltage, VoltageUnits::Volt);
       }

       pub fn stop(&mut self) {
           let mut m = self.motor.lock().unwrap();
           m.stop();
       }
   }
   ```
2. **Assign a smart port in `src/config/ports.rs`**:
   ```rust
   pub const PORT_INTAKE: u8 = 15;
   ```
3. **Instantiate and wire it in `src/config/robot.rs`**:
   Add `pub intake: Intake` to `RobotHardware`.

---

### Tutorial 2: How to Write an Autonomous Routine

Autonomous routines live in [`src/competition/autonomous.rs`](src/competition/autonomous.rs):

```rust
use crate::config::RobotHardware;
use crate::mantle::motor::DirectionType;

pub fn autonomous(robot: &mut RobotHardware) {
    println!("Executing 15-Second Match Autonomous...");

    // 1. Move forward 24 inches
    robot.drive.lock().unwrap().drive_forward(24.0, 8.0, 1.0);

    // 2. Turn 90 degrees counter-clockwise
    robot.drive.lock().unwrap().turn_to_heading(90.0, 6.0, 1.0);

    // 3. Score with lift
    robot.lift.lock().unwrap().set_target(500.0);
}
```

---

### Tutorial 3: How to Tune PID & Feedforward

All PID tuning constants are cleanly isolated in [`src/config/gains.rs`](src/config/gains.rs):
- `DRIVE_P`: Start small ($0.05$). Increase until the robot reaches the target without violent oscillation.
- `DRIVE_D`: Dampens oscillations and prevents overshoot.
- `DRIVE_I`: Eliminates steady-state error (keep low or zero unless needed).
- `DRIVE_DEADBAND`: Minimum error tolerance in inches before considering on-target.

You can simulate PID tuning on your laptop without touching hardware:
```bash
cargo run --example pid_tuning_demo
```

---

## 4. Useful Runnable Simulation Examples

We provide interactive terminal simulations in [`examples/`](examples/):

| Example | Command | Description |
|---|---|---|
| **Pure Pursuit Pathing** | `cargo run --example pure_pursuit_simulation` | Generates waypoints, smooths with gradient descent, and simulates robot trajectory tracking. |
| **Odometry Kinematics** | `cargo run --example odometry_demo` | Translates wheel encoder ticks into $(X, Y, \theta)$ positions across forward drives and turns. |
| **PID Step Response** | `cargo run --example pid_tuning_demo` | Visualizes velocity feedback control, anti-windup clamping, and convergence. |
| **State-Space & LQR** | `cargo run --example state_space_lqr` | Solves DARE, computes optimal LQR gain matrix $K$, and simulates optimal state response. |

---

## 5. Development & Contribution Workflow

Before submitting a PR or merging to `main`:

```bash
# 1. Format code according to project style
cargo fmt

# 2. Check for formatting compliance
cargo fmt --check

# 3. Run Clippy static analysis with warnings denied
cargo clippy --all-targets -- -D warnings

# 4. Run all unit and integration tests
cargo test

# 5. Build optimized release binary
cargo build --release

# 6. Generate and view HTML documentation
cargo doc --no-deps --open
```
