//! Controller input abstraction: axes, buttons, debouncing, and callbacks.

use std::sync::{Arc, Mutex};

/// A single joystick axis on the controller.
#[derive(Debug, Clone, Default)]
pub struct ControllerAxis {
    value: Arc<Mutex<i32>>,
}

impl ControllerAxis {
    /// Creates a new controller axis.
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    /// Returns the current axis value (-100 to 100).
    pub fn value(&self) -> i32 {
        *self.value.lock().unwrap()
    }

    /// Sets the axis value (-100 to 100).
    pub fn set_value(&self, val: i32) {
        *self.value.lock().unwrap() = val.clamp(-100, 100);
    }
}

type ButtonCallback = Box<dyn Fn() + Send + Sync + 'static>;

/// A single button on the controller.
#[derive(Default)]
pub struct ControllerButton {
    pressed_state: Arc<Mutex<bool>>,
    on_pressed: Arc<Mutex<Vec<ButtonCallback>>>,
    on_released: Arc<Mutex<Vec<ButtonCallback>>>,
}

impl ControllerButton {
    /// Creates a new controller button.
    pub fn new() -> Self {
        Self {
            pressed_state: Arc::new(Mutex::new(false)),
            on_pressed: Arc::new(Mutex::new(Vec::new())),
            on_released: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Returns whether the button is currently being held down.
    pub fn pressing(&self) -> bool {
        *self.pressed_state.lock().unwrap()
    }

    /// Alias for pressing().
    pub fn is_pressed(&self) -> bool {
        self.pressing()
    }

    /// Registers a callback to run when the button transitions from released to pressed.
    pub fn pressed(&self, callback: impl Fn() + Send + Sync + 'static) {
        self.on_pressed.lock().unwrap().push(Box::new(callback));
    }

    /// Registers a callback to run when the button transitions from pressed to released.
    pub fn released(&self, callback: impl Fn() + Send + Sync + 'static) {
        self.on_released.lock().unwrap().push(Box::new(callback));
    }

    /// Updates the button state and fires event callbacks if the state has changed.
    pub fn set_pressed(&self, pressed: bool) {
        let mut state = self.pressed_state.lock().unwrap();
        if *state != pressed {
            *state = pressed;
            if pressed {
                let callbacks = self.on_pressed.lock().unwrap();
                for cb in callbacks.iter() {
                    cb();
                }
            } else {
                let callbacks = self.on_released.lock().unwrap();
                for cb in callbacks.iter() {
                    cb();
                }
            }
        }
    }
}

/// Abstract handheld controller with standard dual joysticks and 12 buttons.
#[derive(Default)]
pub struct Controller {
    pub axis1: ControllerAxis,
    pub axis2: ControllerAxis,
    pub axis3: ControllerAxis,
    pub axis4: ControllerAxis,

    pub button_a: ControllerButton,
    pub button_b: ControllerButton,
    pub button_x: ControllerButton,
    pub button_y: ControllerButton,

    pub button_up: ControllerButton,
    pub button_down: ControllerButton,
    pub button_left: ControllerButton,
    pub button_right: ControllerButton,

    pub button_l1: ControllerButton,
    pub button_l2: ControllerButton,
    pub button_r1: ControllerButton,
    pub button_r2: ControllerButton,
}

impl Controller {
    /// Creates a new Controller instance.
    pub fn new() -> Self {
        Self::default()
    }
}
