//! Dispatch Queue wrapper for custom queue management
//!
//! This module provides a safe Rust wrapper around GCD (Grand Central Dispatch) queues
//! that can be used with `ScreenCaptureKit` streams.
//!
//! ## When to Use Custom Queues
//!
//! By default, stream output handlers are called on a system-managed queue. Use a custom
//! queue when you need:
//!
//! - **Priority control** - Use `UserInteractive` `QoS` for low-latency UI updates
//! - **Thread isolation** - Ensure handlers run on a specific queue
//! - **Performance tuning** - Adjust queue priority based on your app's needs
//!
//! ## Example
//!
#![allow(clippy::missing_panics_doc)]

//! ```rust,no_run
//! use apple_cf::dispatch_queue::{DispatchQueue, DispatchQoS};
//!
//! // Create a high-priority queue for frame processing
//! let queue = DispatchQueue::new("com.myapp.capture", DispatchQoS::UserInteractive);
//! // Pass `queue.as_ptr()` into any framework that accepts a `dispatch_queue_t`.
//! ```

use std::ffi::{c_void, CString};
use std::fmt;
use std::time::Duration;

/// Quality of Service levels for dispatch queues
///
/// These `QoS` levels help the system prioritize work appropriately.
///
/// # Examples
///
/// ```
/// use apple_cf::dispatch_queue::{DispatchQueue, DispatchQoS};
///
/// // High priority for UI-affecting work
/// let queue = DispatchQueue::new("com.myapp.ui", DispatchQoS::UserInteractive);
///
/// // Lower priority for background tasks
/// let bg_queue = DispatchQueue::new("com.myapp.background", DispatchQoS::Background);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DispatchQoS {
    /// Background `QoS` - for maintenance or cleanup tasks
    Background = 0,
    /// Utility `QoS` - for tasks that may take some time
    Utility = 1,
    /// Default `QoS` - standard priority
    #[default]
    Default = 2,
    /// User Initiated `QoS` - for tasks initiated by the user
    UserInitiated = 3,
    /// User Interactive `QoS` - for tasks that affect the UI
    UserInteractive = 4,
}

/// A wrapper around GCD `DispatchQueue`
///
/// This allows you to provide a custom dispatch queue for stream output handling
/// instead of using the default queue.
///
/// # Example
///
/// ```no_run
/// use apple_cf::dispatch_queue::{DispatchQueue, DispatchQoS};
///
/// let queue = DispatchQueue::new("com.myapp.capture", DispatchQoS::UserInteractive);
/// ```
pub struct DispatchQueue {
    ptr: *const c_void,
}

unsafe impl Send for DispatchQueue {}
unsafe impl Sync for DispatchQueue {}

impl DispatchQueue {
    /// Creates a new dispatch queue with the specified label and `QoS`
    ///
    /// # Arguments
    ///
    /// * `label` - A string label for the queue (e.g., "com.myapp.capture")
    /// * `qos` - The quality of service level for the queue
    ///
    /// # Examples
    ///
    /// ```
    /// use apple_cf::dispatch_queue::{DispatchQueue, DispatchQoS};
    ///
    /// let queue = DispatchQueue::new("com.myapp.capture", DispatchQoS::UserInteractive);
    /// // Use the queue with SCStream's add_output_handler_with_queue
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the label contains null bytes or if queue creation fails
    #[must_use]
    pub fn new(label: &str, qos: DispatchQoS) -> Self {
        let c_label = CString::new(label).expect("Label contains null byte");
        let ptr = unsafe { crate::ffi::dispatch_queue_create(c_label.as_ptr(), qos as i32) };
        assert!(!ptr.is_null(), "Failed to create dispatch queue");
        Self { ptr }
    }

    /// Returns the raw pointer to the dispatch queue
    ///
    /// This is used internally for FFI calls (and for testing)
    #[must_use]
    pub const fn as_ptr(&self) -> *const c_void {
        self.ptr
    }
}

impl Clone for DispatchQueue {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                ptr: crate::ffi::dispatch_queue_retain(self.ptr),
            }
        }
    }
}

impl Drop for DispatchQueue {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::dispatch_queue_release(self.ptr);
        }
    }
}

impl fmt::Debug for DispatchQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DispatchQueue")
            .field("ptr", &self.ptr)
            .finish()
    }
}

impl fmt::Display for DispatchQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DispatchQueue")
    }
}

fn timeout_ms(timeout: Option<Duration>) -> i64 {
    timeout.map_or(-1, |duration| i64::try_from(duration.as_millis()).unwrap_or(i64::MAX))
}

/// Wrapper around `DispatchGroup`.
#[derive(PartialEq, Eq, Hash)]
pub struct DispatchGroup {
    ptr: *mut c_void,
}

impl DispatchGroup {
    /// Create a new empty group.
    #[must_use]
    pub fn new() -> Self {
        let ptr = unsafe { crate::ffi::acf_dispatch_group_create() };
        assert!(!ptr.is_null(), "failed to create DispatchGroup");
        Self { ptr }
    }

    /// Enter the group.
    pub fn enter(&self) {
        unsafe { crate::ffi::acf_dispatch_group_enter(self.ptr) };
    }

    /// Leave the group.
    pub fn leave(&self) {
        unsafe { crate::ffi::acf_dispatch_group_leave(self.ptr) };
    }

    /// Wait for the group to finish.
    #[must_use]
    pub fn wait(&self, timeout: Option<Duration>) -> bool {
        unsafe { crate::ffi::acf_dispatch_group_wait(self.ptr, timeout_ms(timeout)) }
    }
}

impl Default for DispatchGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for DispatchGroup {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { crate::ffi::acf_object_retain(self.ptr) },
        }
    }
}

impl Drop for DispatchGroup {
    fn drop(&mut self) {
        unsafe { crate::ffi::acf_object_release(self.ptr) };
    }
}

impl fmt::Debug for DispatchGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DispatchGroup").field("ptr", &self.ptr).finish()
    }
}

/// Wrapper around `DispatchSemaphore`.
#[derive(PartialEq, Eq, Hash)]
pub struct DispatchSemaphore {
    ptr: *mut c_void,
}

impl DispatchSemaphore {
    /// Create a semaphore with an initial signal count.
    #[must_use]
    pub fn new(value: i64) -> Self {
        let ptr = unsafe { crate::ffi::acf_dispatch_semaphore_create(value) };
        assert!(!ptr.is_null(), "failed to create DispatchSemaphore");
        Self { ptr }
    }

    /// Signal the semaphore.
    #[must_use]
    pub fn signal(&self) -> i64 {
        unsafe { crate::ffi::acf_dispatch_semaphore_signal(self.ptr) }
    }

    /// Wait for the semaphore.
    #[must_use]
    pub fn wait(&self, timeout: Option<Duration>) -> bool {
        unsafe { crate::ffi::acf_dispatch_semaphore_wait(self.ptr, timeout_ms(timeout)) }
    }
}

impl Clone for DispatchSemaphore {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { crate::ffi::acf_object_retain(self.ptr) },
        }
    }
}

impl Drop for DispatchSemaphore {
    fn drop(&mut self) {
        unsafe { crate::ffi::acf_object_release(self.ptr) };
    }
}

impl fmt::Debug for DispatchSemaphore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DispatchSemaphore")
            .field("ptr", &self.ptr)
            .finish()
    }
}

/// Minimal timer-backed `DispatchSource` wrapper.
#[derive(PartialEq, Eq, Hash)]
pub struct DispatchSource {
    ptr: *mut c_void,
}

impl DispatchSource {
    /// Create a repeating timer source.
    #[must_use]
    pub fn timer(interval: Duration, leeway: Duration) -> Self {
        let interval_ms = u64::try_from(interval.as_millis()).unwrap_or(u64::MAX);
        let leeway_ms = u64::try_from(leeway.as_millis()).unwrap_or(u64::MAX);
        let ptr = unsafe { crate::ffi::acf_dispatch_source_timer_create(interval_ms, leeway_ms) };
        assert!(!ptr.is_null(), "failed to create DispatchSource timer");
        Self { ptr }
    }

    /// Resume the timer source after creation.
    pub fn resume(&self) {
        unsafe { crate::ffi::acf_dispatch_source_timer_resume(self.ptr) };
    }

    /// Cancel the source.
    pub fn cancel(&self) {
        unsafe { crate::ffi::acf_dispatch_source_timer_cancel(self.ptr) };
    }

    /// Number of timer firings observed by the bridge.
    #[must_use]
    pub fn fire_count(&self) -> u64 {
        unsafe { crate::ffi::acf_dispatch_source_timer_fire_count(self.ptr) }
    }
}

impl Clone for DispatchSource {
    fn clone(&self) -> Self {
        Self {
            ptr: unsafe { crate::ffi::acf_object_retain(self.ptr) },
        }
    }
}

impl Drop for DispatchSource {
    fn drop(&mut self) {
        unsafe { crate::ffi::acf_object_release(self.ptr) };
    }
}

impl fmt::Debug for DispatchSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DispatchSource")
            .field("ptr", &self.ptr)
            .field("fire_count", &self.fire_count())
            .finish()
    }
}
