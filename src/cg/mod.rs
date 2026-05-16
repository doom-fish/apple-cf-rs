//! Core Graphics types for screen coordinates, dimensions, and bitmap drawing.
//!
//! This module provides Rust equivalents of Core Graphics types used in
//! `ScreenCaptureKit` plus safe wrappers for the most common offscreen drawing
//! APIs (`CGColorSpace`, `CGImage`, and `CGContext`).

mod affine;
mod context;
mod drawing;
mod point;
mod rect;
mod size;

pub(crate) mod ffi {
    use core::ffi::c_void;

    use super::rect::CGRect;

    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        pub(crate) fn CGColorSpaceCreateDeviceRGB() -> *mut c_void;
        pub(crate) fn CGColorSpaceCreateDeviceGray() -> *mut c_void;
        pub(crate) fn CGColorSpaceCreateWithName(name: *const c_void) -> *mut c_void;
        pub(crate) fn CGColorSpaceRelease(cs: *mut c_void);
        pub(crate) fn CGColorSpaceRetain(cs: *mut c_void) -> *mut c_void;
        pub(crate) fn CGColorSpaceGetNumberOfComponents(cs: *mut c_void) -> usize;

        pub(crate) fn CGImageGetWidth(image: *mut c_void) -> usize;
        pub(crate) fn CGImageGetHeight(image: *mut c_void) -> usize;
        pub(crate) fn CGImageGetBitsPerComponent(image: *mut c_void) -> usize;
        pub(crate) fn CGImageGetBitsPerPixel(image: *mut c_void) -> usize;
        pub(crate) fn CGImageGetBytesPerRow(image: *mut c_void) -> usize;
        pub(crate) fn CGImageRelease(image: *mut c_void);
        pub(crate) fn CGImageRetain(image: *mut c_void) -> *mut c_void;

        pub(crate) fn CGBitmapContextCreate(
            data: *mut c_void,
            width: usize,
            height: usize,
            bits_per_component: usize,
            bytes_per_row: usize,
            space: *mut c_void,
            bitmap_info: u32,
        ) -> *mut c_void;
        pub(crate) fn CGBitmapContextGetData(context: *mut c_void) -> *mut c_void;
        pub(crate) fn CGBitmapContextGetWidth(context: *mut c_void) -> usize;
        pub(crate) fn CGBitmapContextGetHeight(context: *mut c_void) -> usize;
        pub(crate) fn CGBitmapContextGetBitsPerComponent(context: *mut c_void) -> usize;
        pub(crate) fn CGBitmapContextGetBitsPerPixel(context: *mut c_void) -> usize;
        pub(crate) fn CGBitmapContextGetBytesPerRow(context: *mut c_void) -> usize;
        pub(crate) fn CGBitmapContextGetColorSpace(context: *mut c_void) -> *mut c_void;
        pub(crate) fn CGBitmapContextGetAlphaInfo(context: *mut c_void) -> u32;
        pub(crate) fn CGBitmapContextCreateImage(context: *mut c_void) -> *mut c_void;

        pub(crate) fn CGContextRetain(context: *mut c_void) -> *mut c_void;
        pub(crate) fn CGContextRelease(context: *mut c_void);
        pub(crate) fn CGContextSetRGBFillColor(
            context: *mut c_void,
            red: f64,
            green: f64,
            blue: f64,
            alpha: f64,
        );
        pub(crate) fn CGContextSetRGBStrokeColor(
            context: *mut c_void,
            red: f64,
            green: f64,
            blue: f64,
            alpha: f64,
        );
        pub(crate) fn CGContextSetLineWidth(context: *mut c_void, width: f64);
        pub(crate) fn CGContextFillRect(context: *mut c_void, rect: CGRect);
        pub(crate) fn CGContextStrokeRect(context: *mut c_void, rect: CGRect);
        pub(crate) fn CGContextFillPath(context: *mut c_void);
        pub(crate) fn CGContextStrokePath(context: *mut c_void);
        pub(crate) fn CGContextClearRect(context: *mut c_void, rect: CGRect);
        pub(crate) fn CGContextMoveToPoint(context: *mut c_void, x: f64, y: f64);
        pub(crate) fn CGContextAddLineToPoint(context: *mut c_void, x: f64, y: f64);
        pub(crate) fn CGContextAddRect(context: *mut c_void, rect: CGRect);
        pub(crate) fn CGContextAddEllipseInRect(context: *mut c_void, rect: CGRect);
        pub(crate) fn CGContextBeginPath(context: *mut c_void);
        pub(crate) fn CGContextClosePath(context: *mut c_void);
        pub(crate) fn CGContextDrawImage(context: *mut c_void, rect: CGRect, image: *mut c_void);
        pub(crate) fn CGContextTranslateCTM(context: *mut c_void, tx: f64, ty: f64);
        pub(crate) fn CGContextScaleCTM(context: *mut c_void, sx: f64, sy: f64);
        pub(crate) fn CGContextRotateCTM(context: *mut c_void, radians: f64);
        pub(crate) fn CGContextSaveGState(context: *mut c_void);
        pub(crate) fn CGContextRestoreGState(context: *mut c_void);
    }
}

pub use affine::{CGAffineTransform, CGVector};
pub use context::CGContext;
pub use drawing::{CGColorSpace, CGImage};
pub use point::CGPoint;
pub use rect::CGRect;
pub use size::CGSize;

/// `CGDisplayID` type alias
pub type CGDisplayID = u32;
