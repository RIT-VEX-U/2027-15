//! Generic asynchronous hierarchical finite state machine.
//!
//! Provides a state machine runtime for controlling complex robot subsystems
//! (such as catapults, multi-stage intakes, and articulated arms).

use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Trait defining a state in the state machine.
pub trait State<S, ID, M>: Send + 'static
where
    ID: Copy + PartialEq + Debug + Send + 'static,
    M: Clone + Debug + Send + 'static,
{
    /// Unique identifier for this state.
    fn id(&self) -> ID;

    /// Called once upon entering this state.
    fn entry(&mut self, _sys: &mut S) {}

    /// Continuously executed while active. Can return an internal message trigger.
    fn work(&mut self, _sys: &mut S) -> Option<M> {
        None
    }

    /// Called once upon transitioning out of this state.
    fn exit(&mut self, _sys: &mut S) {}

    /// Handles an incoming message and returns the next state (or retains the current state).
    fn respond(&mut self, sys: &mut S, message: M) -> Box<dyn State<S, ID, M>>;
}

/// Thread-safe controller for a state machine running in the background.
pub struct StateMachine<S, ID, M>
where
    S: Send + 'static,
    ID: Copy + PartialEq + Debug + Send + 'static,
    M: Clone + Debug + Send + 'static,
{
    current_id: Arc<Mutex<ID>>,
    incoming_message: Arc<Mutex<Option<M>>>,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    _marker: std::marker::PhantomData<(S, M)>,
}

impl<S, ID, M> StateMachine<S, ID, M>
where
    S: Send + 'static,
    ID: Copy + PartialEq + Debug + Send + 'static,
    M: Clone + Debug + Send + 'static,
{
    /// Starts the state machine in a background thread.
    ///
    /// # Parameters
    /// - `system`: Shared reference or owned system state wrapped in Arc/Mutex
    /// - `initial_state`: The state to begin execution in
    /// - `delay_ms`: Loop delay between state iterations in milliseconds
    /// - `do_log`: Whether to log state transitions and messages
    pub fn spawn(
        system: Arc<Mutex<S>>,
        initial_state: Box<dyn State<S, ID, M>>,
        delay_ms: u64,
        do_log: bool,
    ) -> Self {
        let initial_id = initial_state.id();
        let current_id = Arc::new(Mutex::new(initial_id));
        let incoming_message: Arc<Mutex<Option<M>>> = Arc::new(Mutex::new(None));
        let running = Arc::new(AtomicBool::new(true));

        let current_id_clone = Arc::clone(&current_id);
        let incoming_msg_clone = Arc::clone(&incoming_message);
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            let mut cur_state = initial_state;
            {
                let mut sys = system.lock().unwrap();
                cur_state.entry(&mut *sys);
                *current_id_clone.lock().unwrap() = cur_state.id();
            }

            while running_clone.load(Ordering::Relaxed) {
                // Internal message from work
                let internal_msg = {
                    let mut sys = system.lock().unwrap();
                    cur_state.work(&mut *sys)
                };

                let mut msg_to_process = internal_msg;

                // External message check
                if msg_to_process.is_none() {
                    let mut incoming_lock = incoming_msg_clone.lock().unwrap();
                    msg_to_process = incoming_lock.take();
                }

                if let Some(msg) = msg_to_process {
                    if do_log {
                        println!("StateMachine responding to message: {:?}", msg);
                    }
                    let mut sys = system.lock().unwrap();
                    let next_state = cur_state.respond(&mut *sys, msg);
                    if next_state.id() != cur_state.id() {
                        if do_log {
                            println!(
                                "StateMachine transition: {:?} -> {:?}",
                                cur_state.id(),
                                next_state.id()
                            );
                        }
                        cur_state.exit(&mut *sys);
                        cur_state = next_state;
                        cur_state.entry(&mut *sys);
                        *current_id_clone.lock().unwrap() = cur_state.id();
                    } else {
                        cur_state = next_state;
                    }
                }

                thread::sleep(Duration::from_millis(delay_ms));
            }

            let mut sys = system.lock().unwrap();
            cur_state.exit(&mut *sys);
        });

        Self {
            current_id,
            incoming_message,
            running,
            handle: Some(handle),
            _marker: std::marker::PhantomData,
        }
    }

    /// Queries the current state ID in a thread-safe manner.
    pub fn current_state(&self) -> ID {
        *self.current_id.lock().unwrap()
    }

    /// Sends a message to the state machine.
    pub fn send_message(&self, message: M) {
        let mut msg_lock = self.incoming_message.lock().unwrap();
        *msg_lock = Some(message);
    }

    /// Stops the background worker thread.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl<S, ID, M> Drop for StateMachine<S, ID, M>
where
    S: Send + 'static,
    ID: Copy + PartialEq + Debug + Send + 'static,
    M: Clone + Debug + Send + 'static,
{
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestStateId {
        Idle,
        Active,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestMessage {
        Start,
        Stop,
    }

    struct TestSys {
        count: i32,
    }

    struct IdleState;
    impl State<TestSys, TestStateId, TestMessage> for IdleState {
        fn id(&self) -> TestStateId {
            TestStateId::Idle
        }
        fn respond(
            &mut self,
            _sys: &mut TestSys,
            msg: TestMessage,
        ) -> Box<dyn State<TestSys, TestStateId, TestMessage>> {
            match msg {
                TestMessage::Start => Box::new(ActiveState),
                TestMessage::Stop => Box::new(IdleState),
            }
        }
    }

    struct ActiveState;
    impl State<TestSys, TestStateId, TestMessage> for ActiveState {
        fn id(&self) -> TestStateId {
            TestStateId::Active
        }
        fn work(&mut self, sys: &mut TestSys) -> Option<TestMessage> {
            sys.count += 1;
            if sys.count >= 3 {
                Some(TestMessage::Stop)
            } else {
                None
            }
        }
        fn respond(
            &mut self,
            _sys: &mut TestSys,
            msg: TestMessage,
        ) -> Box<dyn State<TestSys, TestStateId, TestMessage>> {
            match msg {
                TestMessage::Stop => Box::new(IdleState),
                TestMessage::Start => Box::new(ActiveState),
            }
        }
    }

    #[test]
    fn test_state_machine_flow() {
        let sys = Arc::new(Mutex::new(TestSys { count: 0 }));
        let sm = StateMachine::spawn(Arc::clone(&sys), Box::new(IdleState), 10, false);

        assert_eq!(sm.current_state(), TestStateId::Idle);
        sm.send_message(TestMessage::Start);

        thread::sleep(Duration::from_millis(150));
        assert_eq!(sm.current_state(), TestStateId::Idle);
        assert!(sys.lock().unwrap().count >= 3);
    }
}
