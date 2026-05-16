//! `CGColorSpace`, `CGColor`, and `CGImage` — the most-used pieces
//! of CoreGraphics' drawing surface.
//!
//! These are RAII wrappers around `CFType`-style references with
//! retain/release. Use them with [`CGContext`] for offscreen
//! rasterisation, or for converting between formats via `ImageIO`.

use core::ffi::c_void;
use core::ptr;

/// Reference-counted `CGColorSpaceRef`.
pub struct CGColorSpace {
    ptr: *mut c_void,
}

unsafe impl Send for CGColorSpace {}
unsafe impl Sync for CGColorSpace {}

impl Drop for CGColorSpace {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { CGColorSpaceRelease(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CGColorSpace {
    fn clone(&self) -> Self {
        let p = unsafe { CGColorSpaceRetain(self.ptr) };
        Self { ptr: p }
    }
}

impl CGColorSpace {
    /// Device RGB.
    #[must_use]
    pub fn device_rgb() -> Self {
        Self {
            ptr: unsafe { CGColorSpaceCreateDeviceRGB() },
        }
    }

    /// Device gray.
    #[must_use]
    pub fn device_gray() -> Self {
        Self {
            ptr: unsafe { CGColorSpaceCreateDeviceGray() },
        }
    }

    /// sRGB.
    #[must_use]
    pub fn srgb() -> Self {
        unsafe {
            let n = CFStringCreateWithCStringLite(b"kCGColorSpaceSRGB\0".as_ptr());
            let p = CGColorSpaceCreateWithName(n);
            CFReleaseLite(n);
            Self { ptr: p }
        }
    }

    /// Display P3.
    #[must_use]
    pub fn display_p3() -> Self {
        unsafe {
            let n = CFStringCreateWithCStringLite(b"kCGColorSpaceDisplayP3\0".as_ptr());
            let p = CGColorSpaceCreateWithName(n);
            CFReleaseLite(n);
            Self { ptr: p }
        }
    }

    /// Number of color components (`3` for RGB, `1` for gray, …).
    #[must_use]
    pub fn number_of_components(&self) -> usize {
        unsafe { CGColorSpaceGetNumberOfComponents(self.ptr) }
    }

    /// Raw `CGColorSpaceRef` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

/// Reference-counted `CGImageRef` — an immutable bitmap.
pub struct CGImage {
    ptr: *mut c_void,
}

unsafe impl Send for CGImage {}
unsafe impl Sync for CGImage {}

impl Drop for CGImage {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { CGImageRelease(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for CGImage {
    fn clone(&self) -> Self {
        let p = unsafe { CGImageRetain(self.ptr) };
        Self { ptr: p }
    }
}

impl CGImage {
    /// Wrap a raw `CGImageRef` pointer — takes ownership without
    /// retaining.
    ///
    /// # Safety
    ///
    /// `ptr` must be a non-null `CGImageRef` whose ownership the
    /// caller is transferring to the returned [`CGImage`].
    #[must_use]
    pub const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    /// Width in pixels.
    #[must_use]
    pub fn width(&self) -> usize {
        unsafe { CGImageGetWidth(self.ptr) }
    }

    /// Height in pixels.
    #[must_use]
    pub fn height(&self) -> usize {
        unsafe { CGImageGetHeight(self.ptr) }
    }

    /// Bits per component (`8`, `16`, `32`).
    #[must_use]
    pub fn bits_per_component(&self) -> usize {
        unsafe { CGImageGetBitsPerComponent(self.ptr) }
    }

    /// Bits per pixel.
    #[must_use]
    pub fn bits_per_pixel(&self) -> usize {
        unsafe { CGImageGetBitsPerPixel(self.ptr) }
    }

    /// Bytes per row.
    #[must_use]
    pub fn bytes_per_row(&self) -> usize {
        unsafe { CGImageGetBytesPerRow(self.ptr) }
    }

    /// Raw `CGImageRef` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGColorSpaceCreateDeviceRGB() -> *mut c_void;
    fn CGColorSpaceCreateDeviceGray() -> *mut c_void;
    fn CGColorSpaceCreateWithName(name: *const c_void) -> *mut c_void;
    fn CGColorSpaceRelease(cs: *mut c_void);
    fn CGColorSpaceRetain(cs: *mut c_void) -> *mut c_void;
    fn CGColorSpaceGetNumberOfComponents(cs: *mut c_void) -> usize;

    fn CGImageGetWidth(image: *mut c_void) -> usize;
    fn CGImageGetHeight(image: *mut c_void) -> usize;
    fn CGImageGetBitsPerComponent(image: *mut c_void) -> usize;
    fn CGImageGetBitsPerPixel(image: *mut c_void) -> usize;
    fn CGImageGetBytesPerRow(image: *mut c_void) -> usize;
    fn CGImageRelease(image: *mut c_void);
    fn CGImageRetain(image: *mut c_void) -> *mut c_void;

    fn CFStringCreateWithCString(
        allocator: *const c_void,
        bytes: *const u8,
        encoding: u32,
    ) -> *mut c_void;
    fn CFRelease(cf: *const c_void);
}

#[allow(non_snake_case)]
unsafe fn CFStringCreateWithCStringLite(bytes: *const u8) -> *const c_void {
    CFStringCreateWithCString(ptr::null(), bytes, 0x0800_0100).cast_const()
}

#[allow(non_snake_case)]
unsafe fn CFReleaseLite(p: *const c_void) {
    CFRelease(p);
}
