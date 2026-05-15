//! Raw FFI declarations for the Swift bridge.
//!
//! These are intentionally low-level (`*mut c_void` + scalar arguments) and
//! `unsafe`; safe wrappers live in [`crate::iosurface`], [`crate::dispatch_queue`],
//! etc.
//!
//! Currently exposes:
//! * `acf_free_string` — heap-string deallocator used by every bridge fn that
//!   returns an owned C string back to Rust.
//! * `dispatch_queue_*` — Grand Central Dispatch queue lifetime + creation.
//! * `io_surface_*` — IOSurface accessors and lifetime control.
//!
//! Future framework additions (CoreMedia, CoreVideo, Metal, ...) extend this
//! module and gain matching `@_cdecl` exports under
//! `swift-bridge/Sources/<Framework>Bridge/`.

#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    /// Free a heap-allocated C string previously returned by any bridge fn.
    /// Centralised in `AppleCFBridge.swift`.
    pub fn acf_free_string(s: *mut c_char);

    // ---- dispatch ----
    pub fn dispatch_queue_create(label: *const c_char, qos: i32) -> *const c_void;
    pub fn dispatch_queue_release(queue: *const c_void);
    pub fn dispatch_queue_retain(queue: *const c_void) -> *const c_void;

    // ---- IOSurface ----
    pub fn io_surface_create(
        width: usize,
        height: usize,
        pixel_format: u32,
        bytes_per_element: usize,
        out_surface: *mut *mut c_void,
    ) -> i32;
    pub fn io_surface_create_with_properties(
        width: usize,
        height: usize,
        pixel_format: u32,
        bytes_per_element: usize,
        bytes_per_row: usize,
        alloc_size: usize,
        plane_count: usize,
        plane_widths: *const usize,
        plane_heights: *const usize,
        plane_bytes_per_row: *const usize,
        plane_bytes_per_element: *const usize,
        plane_offsets: *const usize,
        plane_sizes: *const usize,
        surface_out: *mut *mut c_void,
    ) -> i32;
    pub fn io_surface_release(surface: *mut c_void);
    pub fn io_surface_retain(surface: *mut c_void) -> *mut c_void;
    pub fn io_surface_hash(surface: *mut c_void) -> usize;
    pub fn io_surface_get_width(surface: *mut c_void) -> usize;
    pub fn io_surface_get_height(surface: *mut c_void) -> usize;
    pub fn io_surface_get_bytes_per_row(surface: *mut c_void) -> usize;
    pub fn io_surface_get_alloc_size(surface: *mut c_void) -> usize;
    pub fn io_surface_get_pixel_format(surface: *mut c_void) -> u32;
    pub fn io_surface_get_id(surface: *mut c_void) -> u32;
    pub fn io_surface_get_seed(surface: *mut c_void) -> u32;
    pub fn io_surface_get_plane_count(surface: *mut c_void) -> usize;
    pub fn io_surface_get_width_of_plane(surface: *mut c_void, plane_index: usize) -> usize;
    pub fn io_surface_get_height_of_plane(surface: *mut c_void, plane_index: usize) -> usize;
    pub fn io_surface_get_bytes_per_row_of_plane(surface: *mut c_void, plane_index: usize)
        -> usize;
    pub fn io_surface_get_base_address_of_plane(
        surface: *mut c_void,
        plane_index: usize,
    ) -> *mut c_void;
    pub fn io_surface_get_base_address(surface: *mut c_void) -> *mut c_void;
    pub fn io_surface_get_bytes_per_element(surface: *mut c_void) -> usize;
    pub fn io_surface_get_element_width(surface: *mut c_void) -> usize;
    pub fn io_surface_get_element_height(surface: *mut c_void) -> usize;
    pub fn io_surface_is_in_use(surface: *mut c_void) -> bool;
    pub fn io_surface_increment_use_count(surface: *mut c_void);
    pub fn io_surface_decrement_use_count(surface: *mut c_void);
    pub fn io_surface_lock(surface: *mut c_void, options: u32, seed: *mut u32) -> i32;
    pub fn io_surface_unlock(surface: *mut c_void, options: u32, seed: *mut u32) -> i32;
}
