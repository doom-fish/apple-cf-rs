use std::ffi::c_void;

use crate::cf::{CFType, CFUUID};

use super::{CGDisplayID, CGRect};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGGetActiveDisplayList(
        max_displays: u32,
        active_displays: *mut CGDisplayID,
        display_count: *mut u32,
    ) -> i32;
    fn CGMainDisplayID() -> CGDisplayID;
    fn CGDisplayBounds(display_id: CGDisplayID) -> CGRect;
    fn CGDisplayCopyDisplayMode(display_id: CGDisplayID) -> *mut c_void;
    fn CGDisplayModeGetWidth(mode: *mut c_void) -> usize;
    fn CGDisplayModeGetHeight(mode: *mut c_void) -> usize;
    fn CGDisplayModeGetPixelWidth(mode: *mut c_void) -> usize;
    fn CGDisplayModeGetPixelHeight(mode: *mut c_void) -> usize;
    fn CGDisplayModeGetRefreshRate(mode: *mut c_void) -> f64;
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn CGDisplayCreateUUIDFromDisplayID(display_id: CGDisplayID) -> *mut c_void;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Display(CGDisplayID);

impl Display {
    #[must_use]
    pub const fn new(id: CGDisplayID) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn id(self) -> CGDisplayID {
        self.0
    }

    #[must_use]
    pub fn active() -> Vec<Self> {
        let mut count = 0;
        if unsafe { CGGetActiveDisplayList(0, std::ptr::null_mut(), &raw mut count) } != 0 {
            return Vec::new();
        }
        if count == 0 {
            return Vec::new();
        }

        let mut displays = vec![0; count as usize];
        if unsafe { CGGetActiveDisplayList(count, displays.as_mut_ptr(), &raw mut count) } != 0 {
            return Vec::new();
        }
        displays.truncate(count as usize);
        displays.into_iter().map(Self).collect()
    }

    #[must_use]
    pub fn main() -> Self {
        Self(unsafe { CGMainDisplayID() })
    }

    #[must_use]
    pub fn bounds(self) -> CGRect {
        unsafe { CGDisplayBounds(self.0) }
    }

    #[must_use]
    pub fn uuid(self) -> Option<CFUUID> {
        let ptr = unsafe { CGDisplayCreateUUIDFromDisplayID(self.0) };
        unsafe { CFUUID::from_raw(ptr) }
    }

    #[must_use]
    pub fn mode(self) -> Option<DisplayMode> {
        let ptr = unsafe { CGDisplayCopyDisplayMode(self.0) };
        unsafe { CFType::from_raw(ptr) }.map(DisplayMode)
    }
}

#[derive(Debug, Clone)]
pub struct DisplayMode(CFType);

impl DisplayMode {
    #[must_use]
    pub fn width(&self) -> usize {
        unsafe { CGDisplayModeGetWidth(self.0.as_ptr()) }
    }

    #[must_use]
    pub fn height(&self) -> usize {
        unsafe { CGDisplayModeGetHeight(self.0.as_ptr()) }
    }

    #[must_use]
    pub fn pixel_width(&self) -> usize {
        unsafe { CGDisplayModeGetPixelWidth(self.0.as_ptr()) }
    }

    #[must_use]
    pub fn pixel_height(&self) -> usize {
        unsafe { CGDisplayModeGetPixelHeight(self.0.as_ptr()) }
    }

    #[must_use]
    pub fn refresh_rate(&self) -> f64 {
        unsafe { CGDisplayModeGetRefreshRate(self.0.as_ptr()) }
    }
}
