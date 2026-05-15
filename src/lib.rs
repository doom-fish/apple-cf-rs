//! `apple-cf` — safe, dependency-free Rust bindings for Apple's shared
//! Core* frameworks.
//!
//! This crate is the foundation of the doom-fish macOS Rust suite. It exists
//! so framework-agnostic types like [`cg::CGRect`], [`iosurface::IOSurface`],
//! and [`DispatchQueue`] don't have to be re-vendored by every crate that
//! builds on top of `CMSampleBuffer`/`CVPixelBuffer`/etc.
//!
//! # Modules
//!
//! | Module | Framework | Feature flag |
//! |---|---|---|
//! | [`cg`] | CoreGraphics value types | `cg` |
//! | [`iosurface`] | `IOSurface` (zero-copy GPU buffers) | `iosurface` |
//! | [`dispatch_queue`] | Grand Central Dispatch | `dispatch` |
//! | [`utils`] | shared FFI helpers (always on) | — |
//!
//! Future frameworks (`CoreMedia`, `CoreVideo`, Metal) will be added as separate
//! features so that downstream crates only pull in what they need.
//!
//! # Architecture
//!
//! ```text
//! Safe Rust API (CGRect, IOSurface, DispatchQueue, ...)
//!     └── extern "C" FFI declarations (src/ffi/mod.rs)
//!             └── Swift @_cdecl bridge (swift-bridge/Sources/...)
//!                     └── Apple Core* frameworks
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod ffi;
pub mod utils;

#[cfg(feature = "cg")]
#[cfg_attr(docsrs, doc(cfg(feature = "cg")))]
pub mod cg;

#[cfg(feature = "iosurface")]
#[cfg_attr(docsrs, doc(cfg(feature = "iosurface")))]
pub mod iosurface;

#[cfg(feature = "dispatch")]
#[cfg_attr(docsrs, doc(cfg(feature = "dispatch")))]
pub mod dispatch_queue;

#[cfg(feature = "cm")]
#[cfg_attr(docsrs, doc(cfg(feature = "cm")))]
pub mod cm;

#[cfg(feature = "cv")]
#[cfg_attr(docsrs, doc(cfg(feature = "cv")))]
pub mod cv;

pub use utils::FourCharCode;

/// Common imports for users of this crate.
pub mod prelude {
    #[cfg(feature = "cg")]
    pub use crate::cg::{CGPoint, CGRect, CGSize};
    #[cfg(feature = "cm")]
    pub use crate::cm::{CMBlockBuffer, CMFormatDescription, CMSampleBuffer, CMTime};
    #[cfg(feature = "cv")]
    pub use crate::cv::{CVPixelBuffer, CVPixelBufferLockFlags};
    #[cfg(feature = "dispatch")]
    pub use crate::dispatch_queue::{DispatchQoS, DispatchQueue};
    #[cfg(feature = "iosurface")]
    pub use crate::iosurface::{IOSurface, IOSurfaceLockOptions};
    pub use crate::utils::FourCharCode;
}
