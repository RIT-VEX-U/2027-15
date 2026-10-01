//! CommandController managing the sequential execution of autonomous command queues.

use super::auto_command::AutoCommand;
use super::delay_command::DelayCommand;
use crate::core::time::{delay_ms, Timer};
use std::collections::VecDeque;

type CancelFunc = Box<dyn FnMut() -> bool + Send + Sync>;

/// Manages and executes an autonomous route composed of AutoCommands in FIFO order.
#[derive(Default)]
pub struct CommandController {
    command_queue: VecDeque<Box<dyn AutoCommand>>,
    command_timed_out: bool,
    should_cancel: Option<CancelFunc>,
    pub print_path_logs: bool,
}

impl CommandController {
    /// Creates an empty CommandController.
    pub fn new() -> Self {
        Self {
            command_queue: VecDeque::new(),
            command_timed_out: false,
            should_cancel: None,
            print_path_logs: true,
        }
    }

    /// Appends a command to the execution queue.
    pub fn add(&mut self, cmd: Box<dyn AutoCommand>) {
        self.command_queue.push_back(cmd);
    }

    /// Appends a delay command (in milliseconds) to the execution queue.
    pub fn add_delay(&mut self, ms: u64) {
        self.add(Box::new(DelayCommand::new(ms)));
    }

    /// Registers a cancellation predicate function.
    pub fn add_cancel_func<F>(&mut self, f: F)
    where
        F: FnMut() -> bool + Send + Sync + 'static,
    {
        self.should_cancel = Some(Box::new(f));
    }

    /// Executes all commands in the queue until completion or cancellation.
    pub fn run(&mut self) {
        let mut timer = Timer::new();

        while let Some(mut cmd) = self.command_queue.pop_front() {
            if let Some(ref mut cancel) = self.should_cancel {
                if cancel() {
                    break;
                }
            }

            timer.reset();
            self.command_timed_out = false;

            loop {
                if timer.time_sec() > cmd.timeout_seconds() {
                    cmd.on_timeout();
                    self.command_timed_out = true;
                    break;
                }

                if cmd.run() {
                    break;
                }

                delay_ms(5);
            }
        }
    }

    /// Returns whether the last executed command timed out.
    pub fn last_command_timed_out(&self) -> bool {
        self.command_timed_out
    }

    /// Returns the number of commands remaining in the queue.
    pub fn len(&self) -> usize {
        self.command_queue.len()
    }

    /// Returns whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.command_queue.is_empty()
    }
}
