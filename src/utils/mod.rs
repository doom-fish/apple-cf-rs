//! Shared utilities used by other modules in this crate and by downstream
//! crates that build on top of the apple-cf bridge.
//!
//! * [`completion`] — sync/async completion-handler bridges
//! * [`four_char_code`] — `FourCharCode` (used by pixel-format / `OSType` codes)
//! * [`ffi_string`] — owned-string helpers around bridge fns that return
//!   heap-allocated C strings
//! * [`panic_safe`] — wrappers that catch panics inside `extern "C"` callbacks
//!   so they don't unwind across the FFI boundary

pub mod completion;
pub mod ffi_string;
pub mod four_char_code;
pub mod panic_safe;

pub use four_char_code::FourCharCode;
