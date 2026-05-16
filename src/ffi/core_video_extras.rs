#![allow(missing_docs)]

use core::ffi::c_void;

extern "C" {
    pub fn cv_metal_texture_cache_create_system_default() -> *mut c_void;
    pub fn cv_metal_texture_cache_flush(cache: *mut c_void);
}
