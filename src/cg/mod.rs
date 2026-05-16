//! Core Graphics types for screen coordinates and dimensions
//!
//! This module provides Rust equivalents of Core Graphics types used in
//! `ScreenCaptureKit` for representing screen coordinates, sizes, and rectangles.

mod affine;
mod drawing;
mod point;
mod rect;
mod size;

pub use affine::{CGAffineTransform, CGVector};
pub use drawing::{CGColorSpace, CGImage};
pub use point::CGPoint;
pub use rect::CGRect;
pub use size::CGSize;

/// `CGDisplayID` type alias
pub type CGDisplayID = u32;
