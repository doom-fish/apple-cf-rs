use std::ptr;

use super::CFURL;
use crate::raw;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum CFURLComponent {
    Scheme = 1,
    Authority = 2,
    Path = 3,
    ResourceSpecifier = 4,
    User = 5,
    Password = 6,
    Host = 8,
    Port = 9,
    Parameters = 10,
    Query = 11,
    Fragment = 12,
}

impl CFURL {
    #[must_use]
    pub fn parse(value: &str, base: Option<&Self>) -> Option<Self> {
        let length = i64::try_from(value.len()).ok()?;
        let reference = unsafe {
            raw::CFURLCreateWithBytes(
                ptr::null(),
                value.as_ptr(),
                length,
                0x0800_0100,
                base.map_or(ptr::null(), |base| base.as_ptr().cast()),
            )
        };
        let reference = unsafe { Self::from_raw(reference.cast_mut().cast()) }?;
        if base.is_none() {
            return Some(reference);
        }
        let absolute = unsafe { raw::CFURLCopyAbsoluteURL(reference.as_ptr().cast()) };
        unsafe { Self::from_raw(absolute.cast_mut().cast()) }
    }

    #[must_use]
    pub fn component(&self, component: CFURLComponent) -> Option<String> {
        let url = self.as_ptr().cast();
        let range =
            unsafe { raw::CFURLGetByteRangeForComponent(url, component as i64, ptr::null_mut()) };
        let start = usize::try_from(range.location).ok()?;
        let length = usize::try_from(range.length).ok()?;
        let byte_count = unsafe { raw::CFURLGetBytes(url, ptr::null_mut(), 0) };
        let mut bytes = vec![0; usize::try_from(byte_count).ok()?];
        let copied = unsafe { raw::CFURLGetBytes(url, bytes.as_mut_ptr(), byte_count) };
        if copied != byte_count {
            return None;
        }
        let end = start.checked_add(length)?;
        String::from_utf8(bytes.get(start..end)?.to_vec()).ok()
    }
}
