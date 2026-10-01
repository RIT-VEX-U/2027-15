//! Autonomous condition predicates and combinators (And, Or, Timeout, Counter).

use crate::core::time::Timer;

/// Abstract boolean condition evaluated dynamically during autonomous routines.
pub trait Condition: Send + Sync {
    /// Evaluates the condition.
    fn test(&mut self) -> bool;

    /// Returns a human-readable description of the condition.
    fn to_string_desc(&self) -> String {
        "Condition".to_string()
    }
}

/// A condition backed by an arbitrary closure or function.
pub struct FunctionCondition<F>
where
    F: FnMut() -> bool + Send + Sync,
{
    f: F,
    description: String,
}

impl<F> FunctionCondition<F>
where
    F: FnMut() -> bool + Send + Sync,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            description: "FunctionCondition".to_string(),
        }
    }

    pub fn with_description(f: F, desc: impl Into<String>) -> Self {
        Self {
            f,
            description: desc.into(),
        }
    }
}

impl<F> Condition for FunctionCondition<F>
where
    F: FnMut() -> bool + Send + Sync,
{
    fn test(&mut self) -> bool {
        (self.f)()
    }

    fn to_string_desc(&self) -> String {
        self.description.clone()
    }
}

/// Condition that returns false until evaluated `max` times, after which it returns true.
pub struct TimesTestedCondition {
    count: usize,
    max: usize,
}

/// Type alias for TimesTestedCondition.
pub type TimesTested = TimesTestedCondition;

impl TimesTestedCondition {
    pub fn new(max: usize) -> Self {
        Self { count: 0, max }
    }
}

impl Condition for TimesTestedCondition {
    fn test(&mut self) -> bool {
        self.count += 1;
        self.count >= self.max
    }

    fn to_string_desc(&self) -> String {
        format!("TimesTestedCondition({}/{})", self.count, self.max)
    }
}

/// Condition that becomes true after a given duration (in seconds) has elapsed.
pub struct IfTimePassed {
    time_s: f64,
    timer: Timer,
}

impl IfTimePassed {
    pub fn new(time_s: f64) -> Self {
        Self {
            time_s,
            timer: Timer::new(),
        }
    }
}

impl Condition for IfTimePassed {
    fn test(&mut self) -> bool {
        self.timer.time_sec() > self.time_s
    }

    fn to_string_desc(&self) -> String {
        format!("IfTimePassed({:.2}s)", self.time_s)
    }
}

/// Logical AND of two conditions.
pub struct AndCondition<C1: Condition, C2: Condition> {
    pub c1: C1,
    pub c2: C2,
}

impl<C1: Condition, C2: Condition> Condition for AndCondition<C1, C2> {
    fn test(&mut self) -> bool {
        self.c1.test() && self.c2.test()
    }

    fn to_string_desc(&self) -> String {
        format!(
            "({} AND {})",
            self.c1.to_string_desc(),
            self.c2.to_string_desc()
        )
    }
}

/// Logical OR of two conditions.
pub struct OrCondition<C1: Condition, C2: Condition> {
    pub c1: C1,
    pub c2: C2,
}

impl<C1: Condition, C2: Condition> Condition for OrCondition<C1, C2> {
    fn test(&mut self) -> bool {
        self.c1.test() || self.c2.test()
    }

    fn to_string_desc(&self) -> String {
        format!(
            "({} OR {})",
            self.c1.to_string_desc(),
            self.c2.to_string_desc()
        )
    }
}
