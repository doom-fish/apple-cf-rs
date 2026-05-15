//! Core Video types and wrappers.
//!
//! Provides safe Rust wrappers for `CVPixelBuffer` and `CVPixelBufferPool`,
//! the CoreVideo primitives that pair an IOSurface with format metadata
//! (pixel format, dimensions, planes). Required for any pipeline that
//! talks to VideoToolbox decoders, Vision, or AVFoundation capture.

mod pixel_buffer;

pub use pixel_buffer::{
    CVPixelBuffer, CVPixelBufferLockFlags, CVPixelBufferLockGuard, CVPixelBufferPool,
    PixelBufferCursorExt,
};
