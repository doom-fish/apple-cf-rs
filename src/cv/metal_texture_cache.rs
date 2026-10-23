//! `CVMetalTextureCache` wrapper.
//!
#![allow(clippy::missing_panics_doc)]

//! ```rust,no_run
//! use apple_cf::cv::CVMetalTextureCache;
//!
//! if let Some(cache) = CVMetalTextureCache::system_default() {
//!     cache.flush();
//! }
//! ```

use crate::cv::CVPixelBuffer;
use crate::ffi;
use std::ffi::c_void;
use std::fmt;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {
    fn CVMetalTextureCacheGetTypeID() -> usize;
    fn CVMetalTextureGetTypeID() -> usize;
    fn CVMetalTextureCacheCreate(
        allocator: *const c_void,
        cache_attributes: *const c_void,
        metal_device: *mut c_void,
        texture_attributes: *const c_void,
        cache_out: *mut *mut c_void,
    ) -> i32;
    fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: *const c_void,
        texture_cache: *mut c_void,
        source_image: *mut c_void,
        texture_attributes: *const c_void,
        pixel_format: usize,
        width: usize,
        height: usize,
        plane_index: usize,
        texture_out: *mut *mut c_void,
    ) -> i32;
    fn CVMetalTextureGetTexture(image: *mut c_void) -> *mut c_void;
}

const CV_RETURN_INVALID_ARGUMENT: i32 = -6661;
const CV_RETURN_ALLOCATION_FAILED: i32 = -6662;

/// Owned wrapper around `CVMetalTextureCacheRef`.
pub struct CVMetalTextureCache(*mut c_void);

impl CVMetalTextureCache {
    /// Create a texture cache using the system-default Metal device.
    #[must_use]
    pub fn system_default() -> Option<Self> {
        let ptr = unsafe { ffi::cv_metal_texture_cache_create_system_default() };
        unsafe { Self::from_raw(ptr) }
    }

    /// Adopts a +1 retained `CVMetalTextureCacheRef` and returns `None` for null.
    ///
    /// # Safety
    ///
    /// A non-null `ptr` must be a live `CVMetalTextureCacheRef` of the exact
    /// type carrying one retain transferred to this wrapper. The caller must
    /// not release or separately adopt that transferred retain.
    #[must_use]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    /// Retains a +0 borrowed cache pointer and returns an owned wrapper.
    ///
    /// # Safety
    ///
    /// A non-null `ptr` must be a live `CVMetalTextureCacheRef` of the exact
    /// type for the duration of the retain call.
    #[must_use]
    pub unsafe fn from_raw_borrowed(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let retained = unsafe { ffi::cf_type_retain(ptr) };
            unsafe { Self::from_raw(retained) }
        }
    }

    /// Borrow the raw +0 cache pointer while `self` remains alive.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.0
    }

    /// Core Video type identifier.
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { CVMetalTextureCacheGetTypeID() }
    }

    /// Flush pending cached textures.
    pub fn flush(&self) {
        unsafe { ffi::cv_metal_texture_cache_flush(self.0) };
    }

    #[allow(clippy::missing_safety_doc, clippy::missing_errors_doc)]
    pub unsafe fn with_device(device: *mut c_void) -> Result<Self, i32> {
        if device.is_null() {
            return Err(CV_RETURN_INVALID_ARGUMENT);
        }
        let mut cache = std::ptr::null_mut();
        let status = unsafe {
            CVMetalTextureCacheCreate(
                std::ptr::null(),
                std::ptr::null(),
                device,
                std::ptr::null(),
                &raw mut cache,
            )
        };
        if status != 0 {
            return Err(status);
        }
        unsafe { Self::from_raw(cache) }.ok_or(CV_RETURN_ALLOCATION_FAILED)
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn create_texture_from_image(
        &self,
        image: &CVPixelBuffer,
        pixel_format: usize,
        width: usize,
        height: usize,
        plane_index: usize,
    ) -> Result<CVMetalTexture, i32> {
        let mut texture = std::ptr::null_mut();
        let status = unsafe {
            CVMetalTextureCacheCreateTextureFromImage(
                std::ptr::null(),
                self.0,
                image.as_ptr(),
                std::ptr::null(),
                pixel_format,
                width,
                height,
                plane_index,
                &raw mut texture,
            )
        };
        if status != 0 {
            return Err(status);
        }
        unsafe { CVMetalTexture::from_raw(texture) }.ok_or(CV_RETURN_ALLOCATION_FAILED)
    }
}

pub struct CVMetalTexture(*mut c_void);

unsafe impl Send for CVMetalTexture {}
unsafe impl Sync for CVMetalTexture {}

impl CVMetalTexture {
    #[allow(clippy::missing_safety_doc)]
    #[must_use]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.0
    }

    #[must_use]
    pub fn type_id() -> usize {
        unsafe { CVMetalTextureGetTypeID() }
    }

    #[must_use]
    pub fn texture(&self) -> *mut c_void {
        unsafe { CVMetalTextureGetTexture(self.0) }
    }
}

crate::utils::retained::cf_retained!(
    CVMetalTexture,
    retain = ffi::cf_type_retain,
    release = ffi::cf_type_release,
);

impl fmt::Debug for CVMetalTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CVMetalTexture")
            .field("ptr", &self.0)
            .finish()
    }
}

crate::utils::retained::cf_retained!(
    CVMetalTextureCache,
    retain = ffi::cf_type_retain,
    release = ffi::cf_type_release,
);

impl PartialEq for CVMetalTextureCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CVMetalTextureCache {}

impl std::hash::Hash for CVMetalTextureCache {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { ffi::cf_type_hash(self.0) }.hash(state);
    }
}

impl fmt::Debug for CVMetalTextureCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CVMetalTextureCache")
            .field("ptr", &self.0)
            .field("type_id", &Self::type_id())
            .finish()
    }
}
