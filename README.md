# RIT VEX U 2027-15 Autonomous & Driver Control System

A high-performance, safe, and modular autonomous robotics software stack built in modern Rust for VEX U competition robotics.

Structured around a **3-Tier Planetary Architecture** (`core`, `mantle`, and `crust`) with a clean top-level `config` layer, the system guarantees outward-to-inward dependency isolation, zero unsafe memory usage, compile-time thread safety, and mathematical parity across all estimation, control, and motion profiling algorithms.

---

## Architecture Overview

```text
                  +------------------------------------------------+
                  |                     config                     |
                  |  Physical specs, port mappings, PID gains,    |
                  |  and hardware assembly (RobotHardware bundle)  |
                  +-----------------------+------------------------+
                                          |
                                          v
                  +------------------------------------------------+
                  |                     crust                      |
                  |  Direct VEX V5 hardware bindings, brain        |
                  |  display driver, RTOS task hooks, competition  |
                  +-----------------------+------------------------+
                                          |
                                          v
                  +------------------------------------------------+
                  |                    mantle                      |
                  |  Hardware Abstraction Layer (HAL): generic     |
                  |  motors/sensors, subsystems (TankDrive, Lift,  |
                  |  Flywheel, Odometry), VDB telemetry, COBS bus  |
                  +-----------------------+------------------------+
                                          |
                                          v
                  +------------------------------------------------+
                  |                     core                       |
                  |  Pure mathematics: SE(2) Lie group geometry,   |
                  |  Kalman/UKF filtering, LQR, DARE, Pure Pursuit,|
                  |  motion profiles, unit types (ZERO VEX APIs)   |
                  +------------------------------------------------+
```

### 1. The Core (`src/core/`)
Platform-agnostic, pure mathematical and algorithmic engine. Has **zero** imports of VEX APIs, vendor SDKs, or hardware abstractions.
- **Controls**: PID with anti-windup clamping, Bang-Bang, Velocity Feedforward, Trapezoid Motion Profiling, Motion Controller, and modern state-space control (DARE solver, continuous-to-discrete system discretization, and Linear Quadratic Regulator).
- **Geometry**: $SE(2)$ Lie group spatial algebra (`Pose2d`, `Rotation2d`, `Translation2d`, `Transform2d`, `Twist2d`, exponential and logarithmic maps).
- **State Estimation**: Standard linear Kalman Filter, Scaled Spherical Simplex Unscented Kalman Filter (UKF), and 4th-order Runge-Kutta (RK4) numerical integration.
- **Path Tracking**: Pure pursuit trajectory tracking with circle-line intersection solving and adaptive lookahead distance.
- **Utilities**: Exponential and linear moving averages, thread-safe monotonic timers, physical unit conversions, and piecewise-linear interpolating maps.

### 2. The Mantle (`src/mantle/`)
Hardware Abstraction Layer (HAL) defining mockable interfaces and device-independent robot subsystems.
- **Traits & Mocks**: `Motor`, `MotorGroup`, `Encoder`, `InertialSensor`, `Controller`, and `Display`.
- **Subsystems**:
  - `TankDrive`: Closed-loop arcade and tank teleop, autonomous point-to-point and heading drives, slew rate limiters, curvature control, and braking modes.
  - `Flywheel`: High-speed velocity control with moving-average filtering.
  - `Lift`: Dual-motor PID position-holding lift with softstop bounds.
  - `Odometry`: 2-wheel tank odometry, 3-wheel tracking, arbitrary $N$-wheel $SE(2)$ tracking pods, and serial COBS odometry.
- **Bus & Telemetry**: Consistent Overhead Byte Stuffing (COBS) framing and the VEX Database Protocol (VDP/VDB) with 32-bit CRC verification.
- **Autonomous Framework**: Composable command pipeline with `AutoCommand`, conditions, sequential command queues, and `Initializer`.

### 3. The Crust (`src/crust/`)
Direct hardware integration layer consuming raw VEX V5 hardware interfaces:
- Concrete `V5Motor`, `Encoder`, and `Inertial` bindings implementing Mantle traits.
- `BrainScreen` LCD graphics driver implementing `mantle::display::Display`.
- V5 controller bindings with configurable deadbands.
- Competition lifecycle dispatchers (`autonomous`, `opcontrol`, `disabled`).
- Interactive video playback engine on the V5 Brain LCD.

### 4. Configuration & Hardware Map (`src/config/`)
Robot physical specifications, tuning gains, and wiring definitions:
- **`specs.rs`**: Track width ($12.0''$), wheel diameter ($2.75''$), gear ratios, bounding radius ($9.0''$), and heading correction cutoff.
- **`ports.rs`**: V5 Smart Port assignments for 8 drivetrain motors, lift, flywheel, inertial sensors, and serial radios.
- **`gains.rs`**: Tuned PID and feedforward constants for motion controllers.
- **`robot.rs`**: `RobotHardware` bundle wiring motors, sensors, subsystems, and the screen UI manager together.

---

## Project Structure

```text
src/
├── core/                # Platform-agnostic pure math, controls, and geometry
│   ├── controls/        # PID, PIDFF, Feedforward, Trapezoid Profile, LQR, DARE
│   ├── filter/          # Moving Average and Exponential Moving Average
│   ├── geometry/        # Pose2d, Rotation2d, Translation2d, Transform2d, Twist2d
│   ├── math/            # Math utils, RK4 integration, Kalman Filter, UKF
│   ├── pathing/         # Pure Pursuit tracking, path generation
│   ├── interpolating_map.rs # Piecewise-linear interpolation
│   ├── state_machine.rs # Thread-safe concurrent state machine
│   ├── time.rs          # Monotonic timer and delay abstractions
│   └── units.rs         # Physical unit definitions and conversions
├── mantle/              # VEX Hardware Abstraction Layer (HAL)
│   ├── comm/            # COBS framing, wrapper devices, and VDB protocol
│   ├── commands/        # AutoCommand, conditions, command dispatchers
│   ├── subsystems/      # TankDrive, Flywheel, Lift, and Odometry models
│   ├── controller.rs    # Mockable controller trait
│   ├── display.rs       # Mockable screen display trait and UI widgets
│   ├── motor.rs         # Motor and MotorGroup traits
│   └── sensor.rs        # Encoder and InertialSensor traits
├── crust/               # Direct VEX V5 hardware driver layer
│   ├── fun/             # Brain screen video playback widget
│   ├── brain.rs         # Brain screen driver
│   ├── competition.rs   # Competition state hooks
│   ├── controller.rs    # V5 controller hardware binding
│   ├── motor.rs         # V5 smart motor driver
│   └── sensor.rs        # V5 smart sensors (IMU, optical encoders)
├── config/              # Physical robot parameters and hardware wiring
│   ├── gains.rs         # PID tuning constants
│   ├── ports.rs         # V5 Smart Port mappings
│   ├── specs.rs         # Robot mechanical specifications
│   └── robot.rs         # Unified RobotHardware bundle
├── competition/         # Autonomous routines and driver control loops
│   ├── autonomous.rs    # Autonomous routine routines
│   └── opcontrol.rs     # Driver control loop
├── lib.rs               # Library root exposing core, mantle, crust, config
└── main.rs              # Robot executable entry point
```

---

## Quickstart & Development

### Prerequisites
- [Rust](https://rustup.rs/) (edition 2021, stable toolchain)

### Building
```bash
# Build the release binary
cargo build --release

# Check formatting
cargo fmt --check

# Run static analysis
cargo clippy --all-targets -- -D warnings
```

### Running Tests
The test suite validates $SE(2)$ Lie algebra, matrix operations, digital filters, odometry kinematics, and telemetry frame encoding:
```bash
cargo test
```

### Generating Documentation
Comprehensive Rustdoc documentation is available for all modules and traits:
```bash
cargo doc --no-deps --open
```

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
