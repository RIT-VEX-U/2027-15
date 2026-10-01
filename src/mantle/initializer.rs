//! Program initialization selector and configuration manager.
//!
//! Mirrors `core/utils/initializer.h` and `core/utils/initializer.cpp`.

use std::sync::Arc;

/// Represents a single selectable initialization routine.
pub struct Initialization {
    /// Human-readable label for this initialization routine.
    pub name: String,
    /// Function executed when this initialization is activated.
    pub init: Arc<dyn Fn() + Send + Sync>,
    /// Metadata tag (color, priority, weight, etc.)
    pub meta: u32,
}

impl Initialization {
    /// Creates a new initialization entry.
    pub fn new(name: impl Into<String>, init: impl Fn() + Send + Sync + 'static, meta: u32) -> Self {
        Self {
            name: name.into(),
            init: Arc::new(init),
            meta,
        }
    }

    /// Creates a purely functional initialization entry with a default weight/meta of 1.
    pub fn func(init: impl Fn() + Send + Sync + 'static) -> Self {
        Self::new("", init, 1)
    }
}

/// Common selection utilities and wrappers.
pub mod selector {
    

    /// Constant indicating that no valid initialization has been selected yet.
    pub const NO_SELECTION_INDEX: usize = usize::MAX;

    /// Type alias for a selector function.
    pub type SelectorFn = Box<dyn Fn() -> usize + Send + Sync>;

    /// Returns a selector function that always selects index `n`.
    pub fn select(n: usize) -> SelectorFn {
        Box::new(move || n)
    }

    /// Returns a selector function that selects an index uniformly in $[a, b)$.
    pub fn random(a: usize, b: usize) -> SelectorFn {
        Box::new(move || {
            if b <= a {
                return 0;
            }
            // Simple pseudo-random fallback without heavy dependencies
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos() as usize)
                .unwrap_or(0);
            a + (nanos % (b - a))
        })
    }

    /// Returns a selector function that selects an index uniformly in $[0, n)$.
    pub fn random_n(n: usize) -> SelectorFn {
        random(0, n)
    }

    /// Selector wrapper that executes after a delay.
    pub fn delayed(selector: SelectorFn, ms: u64) -> SelectorFn {
        Box::new(move || {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            selector()
        })
    }
}

/// Coordinates system pre-initialization, routine selection, and post-initialization.
pub struct Initializer {
    selector: Option<selector::SelectorFn>,
    initialization_list: Vec<Initialization>,
    pre_init: Option<Arc<dyn Fn() + Send + Sync>>,
    post_init: Option<Arc<dyn Fn() + Send + Sync>>,
    selection: usize,
    initialized: bool,
}

impl Initializer {
    /// Creates a simple initializer that unconditionally executes a single initialization routine.
    pub fn new_simple(init: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            selector: None,
            initialization_list: Vec::new(),
            pre_init: Some(Arc::new(init)),
            post_init: None,
            selection: selector::NO_SELECTION_INDEX,
            initialized: false,
        }
    }

    /// Creates an initializer with a list of selectable routines and a selector function.
    pub fn new(
        initializations: Vec<Initialization>,
        selector: Option<selector::SelectorFn>,
        pre_init: Option<Arc<dyn Fn() + Send + Sync>>,
        post_init: Option<Arc<dyn Fn() + Send + Sync>>,
    ) -> Self {
        Self {
            selector,
            initialization_list: initializations,
            pre_init,
            post_init,
            selection: selector::NO_SELECTION_INDEX,
            initialized: false,
        }
    }

    /// Runs initialization lifecycle: pre_init -> selection -> init -> post_init.
    pub fn initialize(&mut self) {
        if let Some(ref pre) = self.pre_init {
            pre();
        }

        if let Some(ref sel) = self.selector {
            self.selection = sel();
        }

        if self.selection < self.initialization_list.len() {
            let routine = &self.initialization_list[self.selection];
            (routine.init)();
        }

        if let Some(ref post) = self.post_init {
            post();
        }

        self.initialized = true;
    }

    /// Returns a slice of the registered initializations.
    pub fn initializations(&self) -> &[Initialization] {
        &self.initialization_list
    }

    /// Returns the number of selectable initializations.
    pub fn initialization_count(&self) -> usize {
        self.initialization_list.len()
    }

    /// Returns the name of the selected initialization, or an empty string.
    pub fn selected_name(&self) -> &str {
        if self.selection < self.initialization_list.len() {
            &self.initialization_list[self.selection].name
        } else {
            ""
        }
    }

    /// Returns the metadata tag of the selected initialization.
    pub fn selected_meta(&self) -> u32 {
        if self.selection < self.initialization_list.len() {
            self.initialization_list[self.selection].meta
        } else {
            u32::MAX
        }
    }

    /// Returns the selected index.
    pub fn selected_index(&self) -> usize {
        self.selection
    }

    /// Returns true if `initialize()` has not yet completed.
    pub fn uninitialized(&self) -> bool {
        !self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_initializer_simple() {
        let flag = Arc::new(AtomicU32::new(0));
        let flag_clone = Arc::clone(&flag);
        let mut init = Initializer::new_simple(move || {
            flag_clone.store(42, Ordering::SeqCst);
        });

        assert!(init.uninitialized());
        init.initialize();
        assert!(!init.uninitialized());
        assert_eq!(flag.load(Ordering::SeqCst), 42);
    }

    #[test]
    fn test_initializer_selection() {
        let choice = Arc::new(AtomicU32::new(0));
        let c1 = Arc::clone(&choice);
        let c2 = Arc::clone(&choice);

        let inits = vec![
            Initialization::new("ModeA", move || c1.store(1, Ordering::SeqCst), 100),
            Initialization::new("ModeB", move || c2.store(2, Ordering::SeqCst), 200),
        ];

        let mut init = Initializer::new(inits, Some(selector::select(1)), None, None);
        init.initialize();

        assert_eq!(init.selected_index(), 1);
        assert_eq!(init.selected_name(), "ModeB");
        assert_eq!(init.selected_meta(), 200);
        assert_eq!(choice.load(Ordering::SeqCst), 2);
    }
}
