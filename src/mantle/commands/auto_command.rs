//! Autonomous Command abstractions and composition (Sequential, Parallel, Branching, Repeating).

use super::condition::Condition;
use crate::core::time::Timer;
use std::collections::VecDeque;

/// Base trait for executable autonomous robot commands.
pub trait AutoCommand: Send + Sync {
    /// Executes a single iteration of the command.
    /// Returns `true` when the command has fully completed, or `false` if it should continue running.
    fn run(&mut self) -> bool;

    /// Called if the command times out before completion.
    fn on_timeout(&mut self) {}

    /// Returns the configured timeout in seconds.
    fn timeout_seconds(&self) -> f64 {
        10.0
    }

    /// Human-readable representation.
    fn to_string_desc(&self) -> String {
        "AutoCommand".to_string()
    }

    /// String description alias.
    fn to_string(&self) -> String {
        self.to_string_desc()
    }
}

/// A command that wraps a generic closure.
pub struct FunctionCommand<F>
where
    F: FnMut() -> bool + Send + Sync,
{
    f: F,
    description: String,
    timeout: f64,
}

impl<F> FunctionCommand<F>
where
    F: FnMut() -> bool + Send + Sync,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            description: "FunctionCommand".to_string(),
            timeout: 10.0,
        }
    }

    pub fn with_description(f: F, desc: impl Into<String>) -> Self {
        Self {
            f,
            description: desc.into(),
            timeout: 10.0,
        }
    }
}

impl<F> AutoCommand for FunctionCommand<F>
where
    F: FnMut() -> bool + Send + Sync,
{
    fn run(&mut self) -> bool {
        (self.f)()
    }

    fn timeout_seconds(&self) -> f64 {
        self.timeout
    }

    fn to_string_desc(&self) -> String {
        self.description.clone()
    }
}

/// Command that pauses execution until a condition evaluates to true.
pub struct WaitUntilCondition<C: Condition> {
    pub cond: C,
    timeout: f64,
}

/// Type alias for WaitUntilCondition.
pub type WaitUntil<C> = WaitUntilCondition<C>;

impl<C: Condition> WaitUntilCondition<C> {
    pub fn new(cond: C) -> Self {
        Self { cond, timeout: 10.0 }
    }
}

impl<C: Condition> AutoCommand for WaitUntilCondition<C> {
    fn run(&mut self) -> bool {
        self.cond.test()
    }

    fn timeout_seconds(&self) -> f64 {
        self.timeout
    }

    fn to_string_desc(&self) -> String {
        format!("WaitUntil({})", self.cond.to_string_desc())
    }
}

/// Executes a queue of commands sequentially in FIFO order.
pub struct InOrder {
    cmds: VecDeque<Box<dyn AutoCommand>>,
    current_cmd: Option<Box<dyn AutoCommand>>,
    timer: Timer,
    total_timeout: f64,
}

impl InOrder {
    pub fn new(cmds: Vec<Box<dyn AutoCommand>>) -> Self {
        let total_timeout: f64 = cmds.iter().map(|c| c.timeout_seconds()).sum();
        Self {
            cmds: VecDeque::from(cmds),
            current_cmd: None,
            timer: Timer::new(),
            total_timeout,
        }
    }
}

impl AutoCommand for InOrder {
    fn run(&mut self) -> bool {
        loop {
            if self.current_cmd.is_none() {
                if let Some(cmd) = self.cmds.pop_front() {
                    self.current_cmd = Some(cmd);
                    self.timer.reset();
                } else {
                    return true;
                }
            }

            if let Some(ref mut cmd) = self.current_cmd {
                if self.timer.time_sec() > cmd.timeout_seconds() {
                    cmd.on_timeout();
                    self.current_cmd = None;
                    continue;
                }

                if cmd.run() {
                    self.current_cmd = None;
                } else {
                    return false;
                }
            }
        }
    }

    fn on_timeout(&mut self) {
        if let Some(ref mut cmd) = self.current_cmd {
            cmd.on_timeout();
        }
    }

    fn timeout_seconds(&self) -> f64 {
        self.total_timeout
    }

    fn to_string_desc(&self) -> String {
        format!("InOrder({} commands)", self.cmds.len())
    }
}

/// Executes multiple commands in parallel until all have completed.
pub struct Parallel {
    cmds: Vec<Option<Box<dyn AutoCommand>>>,
    timer: Timer,
    timeout: f64,
}

impl Parallel {
    pub fn new(cmds: Vec<Box<dyn AutoCommand>>) -> Self {
        let timeout = cmds.iter().map(|c| c.timeout_seconds()).fold(0.0, f64::max);
        Self {
            cmds: cmds.into_iter().map(Some).collect(),
            timer: Timer::new(),
            timeout,
        }
    }
}

impl AutoCommand for Parallel {
    fn run(&mut self) -> bool {
        let mut all_done = true;

        for slot in self.cmds.iter_mut() {
            if let Some(ref mut cmd) = slot {
                if cmd.run() {
                    *slot = None;
                } else {
                    all_done = false;
                }
            }
        }

        all_done
    }

    fn on_timeout(&mut self) {
        for ref mut cmd in self.cmds.iter_mut().flatten() {
            cmd.on_timeout();
        }
    }

    fn timeout_seconds(&self) -> f64 {
        self.timeout
    }

    fn to_string_desc(&self) -> String {
        "Parallel".to_string()
    }
}

/// Conditionally branches to one of two commands based on a runtime Condition.
pub struct Branch<C: Condition> {
    pub cond: C,
    pub true_choice: Box<dyn AutoCommand>,
    pub false_choice: Box<dyn AutoCommand>,
    chosen: bool,
    choice: bool,
}

impl<C: Condition> Branch<C> {
    pub fn new(cond: C, false_choice: Box<dyn AutoCommand>, true_choice: Box<dyn AutoCommand>) -> Self {
        Self {
            cond,
            true_choice,
            false_choice,
            chosen: false,
            choice: false,
        }
    }
}

impl<C: Condition> AutoCommand for Branch<C> {
    fn run(&mut self) -> bool {
        if !self.chosen {
            self.choice = self.cond.test();
            self.chosen = true;
        }

        if self.choice {
            self.true_choice.run()
        } else {
            self.false_choice.run()
        }
    }

    fn on_timeout(&mut self) {
        if self.chosen {
            if self.choice {
                self.true_choice.on_timeout();
            } else {
                self.false_choice.on_timeout();
            }
        }
    }

    fn to_string_desc(&self) -> String {
        format!("Branch({})", self.cond.to_string_desc())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_command() {
        let mut count = 0;
        let mut cmd = FunctionCommand::new(|| {
            count += 1;
            count >= 3
        });

        assert!(!cmd.run());
        assert!(!cmd.run());
        assert!(cmd.run());
    }
}
