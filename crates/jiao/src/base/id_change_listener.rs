// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Used to be notified when a gen/unique ID is invalidated, typically to preemptively purge
/// associated items from a cache that are no longer reachable.
///
/// The listener can be marked for deregistration if the cached item is remove
/// before the listener is triggered.
/// This prevents unbounded listener growth when cache items are routinely
/// removed before the gen ID/unique ID is invalidated.
pub struct IdChangeListener {
    should_deregister: AtomicBool,
    obj: Box<dyn IdChangeListenerTrait>,
}

impl IdChangeListener {
    pub fn new(obj: Box<dyn IdChangeListenerTrait>) -> Self {
        Self {
            should_deregister: AtomicBool::new(false),
            obj,
        }
    }

    /// Mark the listener is no longer needed.
    ///
    /// It should be removed and changed() should not be called.
    pub fn mark_should_deregister(&mut self) {
        self.should_deregister.store(true, Ordering::Relaxed);
    }

    /// Indicates whether `mark_should_deregister()` was called.
    #[must_use]
    pub fn should_deregister(&self) -> bool {
        self.should_deregister.load(Ordering::Acquire)
    }
}

pub trait IdChangeListenerTrait {
    fn changed(&mut self);
}

/// Manages a list of `IdChangeListeners`.
pub struct IdChangeListenerList {
    mutex: Arc<Mutex<i32>>,
    listeners: Vec<IdChangeListener>,
}

impl IdChangeListenerList {
    pub fn new() -> Self {
        unimplemented!()
    }

    /// Add a new listener to the list.
    ///
    /// It must not already be deregistered.
    /// Also clears out previously deregistered listeners.
    pub fn push(&mut self, listener: IdChangeListener) {
        //unimplemented!();
        self.listeners.push(listener);
    }

    /// The number of registered listeners (including deregisterd listeners that are yet-to-be
    /// removed.
    pub fn len(&self) -> usize {
        // unimplemented!()
        self.listeners.len()
    }

    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }

    /// Calls changed() on all listeners that haven't been deregistered and resets the list.
    pub fn changed() {
        unimplemented!()
    }

    /// Resets without calling changed() on the listeners.
    pub fn reset() {
        unimplemented!()
    }
}
