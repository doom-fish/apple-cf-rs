#![allow(warnings)]
#![allow(clippy::all)]

use super::generated::*;
use core::ptr;

#[must_use]
#[inline]
pub const fn CFRangeMake(loc: CFIndex, len: CFIndex) -> CFRange {
    CFRange {
        location: loc,
        length: len,
    }
}

pub const CFByteOrderUnknown: CFByteOrder = 0;
pub const CFByteOrderLittleEndian: CFByteOrder = 1;
pub const CFByteOrderBigEndian: CFByteOrder = 2;

#[must_use]
#[inline]
pub const fn CFByteOrderGetCurrent() -> CFByteOrder {
    if cfg!(target_endian = "little") {
        CFByteOrderLittleEndian
    } else if cfg!(target_endian = "big") {
        CFByteOrderBigEndian
    } else {
        CFByteOrderUnknown
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt16(arg: u16) -> u16 {
    arg.swap_bytes()
}

#[must_use]
#[inline]
pub const fn CFSwapInt32(arg: u32) -> u32 {
    arg.swap_bytes()
}

#[must_use]
#[inline]
pub const fn CFSwapInt64(arg: u64) -> u64 {
    arg.swap_bytes()
}

#[must_use]
#[inline]
pub const fn CFSwapInt16BigToHost(arg: u16) -> u16 {
    if cfg!(target_endian = "big") {
        arg
    } else {
        CFSwapInt16(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt32BigToHost(arg: u32) -> u32 {
    if cfg!(target_endian = "big") {
        arg
    } else {
        CFSwapInt32(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt64BigToHost(arg: u64) -> u64 {
    if cfg!(target_endian = "big") {
        arg
    } else {
        CFSwapInt64(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt16HostToBig(arg: u16) -> u16 {
    if cfg!(target_endian = "big") {
        arg
    } else {
        CFSwapInt16(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt32HostToBig(arg: u32) -> u32 {
    if cfg!(target_endian = "big") {
        arg
    } else {
        CFSwapInt32(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt64HostToBig(arg: u64) -> u64 {
    if cfg!(target_endian = "big") {
        arg
    } else {
        CFSwapInt64(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt16LittleToHost(arg: u16) -> u16 {
    if cfg!(target_endian = "little") {
        arg
    } else {
        CFSwapInt16(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt32LittleToHost(arg: u32) -> u32 {
    if cfg!(target_endian = "little") {
        arg
    } else {
        CFSwapInt32(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt64LittleToHost(arg: u64) -> u64 {
    if cfg!(target_endian = "little") {
        arg
    } else {
        CFSwapInt64(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt16HostToLittle(arg: u16) -> u16 {
    if cfg!(target_endian = "little") {
        arg
    } else {
        CFSwapInt16(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt32HostToLittle(arg: u32) -> u32 {
    if cfg!(target_endian = "little") {
        arg
    } else {
        CFSwapInt32(arg)
    }
}

#[must_use]
#[inline]
pub const fn CFSwapInt64HostToLittle(arg: u64) -> u64 {
    if cfg!(target_endian = "little") {
        arg
    } else {
        CFSwapInt64(arg)
    }
}

#[must_use]
#[inline]
pub fn CFConvertFloat32HostToSwapped(arg: Float32) -> CFSwappedFloat32 {
    let mut bits = arg.to_bits();
    if cfg!(target_endian = "little") {
        bits = bits.swap_bytes();
    }
    CFSwappedFloat32 { v: bits }
}

#[must_use]
#[inline]
pub fn CFConvertFloat32SwappedToHost(arg: CFSwappedFloat32) -> Float32 {
    let mut bits = arg.v;
    if cfg!(target_endian = "little") {
        bits = bits.swap_bytes();
    }
    Float32::from_bits(bits)
}

#[must_use]
#[inline]
pub fn CFConvertFloat64HostToSwapped(arg: Float64) -> CFSwappedFloat64 {
    let mut bits = arg.to_bits();
    if cfg!(target_endian = "little") {
        bits = bits.swap_bytes();
    }
    CFSwappedFloat64 { v: bits }
}

#[must_use]
#[inline]
pub fn CFConvertFloat64SwappedToHost(arg: CFSwappedFloat64) -> Float64 {
    let mut bits = arg.v;
    if cfg!(target_endian = "little") {
        bits = bits.swap_bytes();
    }
    Float64::from_bits(bits)
}

#[must_use]
#[inline]
pub fn CFConvertFloatHostToSwapped(arg: f32) -> CFSwappedFloat32 {
    CFConvertFloat32HostToSwapped(arg)
}

#[must_use]
#[inline]
pub fn CFConvertFloatSwappedToHost(arg: CFSwappedFloat32) -> f32 {
    CFConvertFloat32SwappedToHost(arg)
}

#[must_use]
#[inline]
pub fn CFConvertDoubleHostToSwapped(arg: f64) -> CFSwappedFloat64 {
    CFConvertFloat64HostToSwapped(arg)
}

#[must_use]
#[inline]
pub fn CFConvertDoubleSwappedToHost(arg: CFSwappedFloat64) -> f64 {
    CFConvertFloat64SwappedToHost(arg)
}

#[must_use]
#[inline]
pub const fn CFUserNotificationCheckBoxChecked(i: CFIndex) -> CFOptionFlags {
    (1_u64 << (8 + i as u32)) as CFOptionFlags
}

#[must_use]
#[inline]
pub const fn CFUserNotificationSecureTextField(i: CFIndex) -> CFOptionFlags {
    (1_u64 << (16 + i as u32)) as CFOptionFlags
}

#[must_use]
#[inline]
pub const fn CFUserNotificationPopUpSelection(n: CFIndex) -> CFOptionFlags {
    ((n as u64) << 24) as CFOptionFlags
}

#[inline]
pub unsafe fn CFStringInitInlineBuffer(
    str_: CFStringRef,
    buf: *mut CFStringInlineBuffer,
    range: CFRange,
) {
    if let Some(buf) = unsafe { buf.as_mut() } {
        buf.theString = str_;
        buf.rangeToBuffer = range;
        let direct_uni = unsafe { CFStringGetCharactersPtr(str_) };
        buf.directUniCharBuffer = direct_uni;
        buf.directCStringBuffer = if direct_uni.is_null() {
            unsafe { CFStringGetCStringPtr(str_, kCFStringEncodingASCII) }
        } else {
            ptr::null()
        };
        buf.bufferedRangeStart = 0;
        buf.bufferedRangeEnd = 0;
    }
}

#[must_use]
#[inline]
pub unsafe fn CFStringGetCharacterFromInlineBuffer(
    buf: *mut CFStringInlineBuffer,
    idx: CFIndex,
) -> UniChar {
    let Some(buf) = (unsafe { buf.as_mut() }) else {
        return 0;
    };
    if idx < 0 || idx >= buf.rangeToBuffer.length {
        return 0;
    }
    if !buf.directUniCharBuffer.is_null() {
        return unsafe {
            *buf.directUniCharBuffer
                .add((idx + buf.rangeToBuffer.location) as usize)
        };
    }
    if !buf.directCStringBuffer.is_null() {
        return unsafe {
            *buf.directCStringBuffer
                .add((idx + buf.rangeToBuffer.location) as usize) as u8
        } as UniChar;
    }
    if idx >= buf.bufferedRangeEnd || idx < buf.bufferedRangeStart {
        buf.bufferedRangeStart = (idx - 4).max(0);
        buf.bufferedRangeEnd = (buf.bufferedRangeStart + 64).min(buf.rangeToBuffer.length);
        unsafe {
            CFStringGetCharacters(
                buf.theString,
                CFRangeMake(
                    buf.rangeToBuffer.location + buf.bufferedRangeStart,
                    buf.bufferedRangeEnd - buf.bufferedRangeStart,
                ),
                buf.buffer.as_mut_ptr(),
            );
        }
    }
    buf.buffer[(idx - buf.bufferedRangeStart) as usize]
}

#[must_use]
#[inline]
pub const fn CFStringIsSurrogateHighCharacter(character: UniChar) -> Boolean {
    ((character >= 0xD800) && (character <= 0xDBFF)) as Boolean
}

#[must_use]
#[inline]
pub const fn CFStringIsSurrogateLowCharacter(character: UniChar) -> Boolean {
    ((character >= 0xDC00) && (character <= 0xDFFF)) as Boolean
}

#[must_use]
#[inline]
pub const fn CFStringGetLongCharacterForSurrogatePair(
    surrogateHigh: UniChar,
    surrogateLow: UniChar,
) -> UTF32Char {
    (((surrogateHigh as UTF32Char - 0xD800) << 10) + (surrogateLow as UTF32Char - 0xDC00) + 0x10000)
        as UTF32Char
}

#[must_use]
#[inline]
pub unsafe fn CFStringGetSurrogatePairForLongCharacter(
    character: UTF32Char,
    surrogates: *mut UniChar,
) -> Boolean {
    if !(0x10000..0x110000).contains(&character) {
        return 0;
    }
    let scalar = character - 0x10000;
    if !surrogates.is_null() {
        unsafe {
            *surrogates = (0xD800 + (scalar >> 10)) as UniChar;
            *surrogates.add(1) = (0xDC00 + (scalar & 0x3ff)) as UniChar;
        }
    }
    1
}

#[must_use]
#[inline]
pub fn CMTagGetCategory(tag: CMTag) -> CMTagCategory {
    tag.category
}

#[must_use]
#[inline]
pub fn CMTagGetValue(tag: CMTag) -> CMTagValue {
    tag.value
}

#[must_use]
#[inline]
pub fn CMTagHasCategory(tag: CMTag, category: CMTagCategory) -> Boolean {
    (CMTagGetCategory(tag) == category) as Boolean
}

#[must_use]
#[inline]
pub fn CMTagCategoryEqualToTagCategory(tag1: CMTag, tag2: CMTag) -> Boolean {
    (tag1.category == tag2.category) as Boolean
}

#[must_use]
#[inline]
pub fn CMTagIsValid(tag: CMTag) -> Boolean {
    (unsafe { CMTagGetValueDataType(tag) } != kCMTagDataType_Invalid as CMTagDataType) as Boolean
}

#[must_use]
#[inline]
pub fn CMTagCategoryValueEqualToValue(tag1: CMTag, tag2: CMTag) -> Boolean {
    ((tag1.category == tag2.category)
        && (unsafe { CMTagGetValueDataType(tag1) } == unsafe { CMTagGetValueDataType(tag2) })
        && (tag1.value == tag2.value)) as Boolean
}

#[must_use]
#[inline]
pub unsafe fn CMTimebaseCreateWithMasterTimebase(
    allocator: CFAllocatorRef,
    masterTimebase: CMTimebaseRef,
    timebaseOut: *mut CMTimebaseRef,
) -> OSStatus {
    unsafe { CMTimebaseCreateWithSourceTimebase(allocator, masterTimebase, timebaseOut) }
}

#[must_use]
#[inline]
pub unsafe fn CMTimebaseSetMasterClock(
    timebase: CMTimebaseRef,
    newMasterClock: CMClockRef,
) -> OSStatus {
    unsafe { CMTimebaseSetSourceClock(timebase, newMasterClock) }
}

#[must_use]
#[inline]
pub unsafe fn CMTimebaseSetMasterTimebase(
    timebase: CMTimebaseRef,
    newMasterTimebase: CMTimebaseRef,
) -> OSStatus {
    unsafe { CMTimebaseSetSourceTimebase(timebase, newMasterTimebase) }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVMetalTextureCache {
    _unused: [u8; 0],
}

pub type CVMetalTextureCacheRef = *mut __CVMetalTextureCache;
pub type CVMetalTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVMetalTextureGetTypeID() -> CFTypeID;
    pub fn CVMetalTextureGetTexture(image: CVMetalTextureRef) -> *mut core::ffi::c_void;
    pub fn CVMetalTextureIsFlipped(image: CVMetalTextureRef) -> Boolean;
    pub fn CVMetalTextureGetCleanTexCoords(
        image: CVMetalTextureRef,
        lowerLeft: *mut f32,
        lowerRight: *mut f32,
        upperRight: *mut f32,
        upperLeft: *mut f32,
    );
    pub fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVMetalTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        pixelFormat: usize,
        width: usize,
        height: usize,
        planeIndex: usize,
        textureOut: *mut CVMetalTextureRef,
    ) -> CVReturn;
    pub fn CVMetalBufferGetBuffer(buffer: CVMetalBufferRef) -> *mut core::ffi::c_void;
    pub fn CVMetalBufferCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: *mut core::ffi::c_void,
        metalBufferAttributes: CFDictionaryRef,
        cacheOut: *mut CVMetalBufferCacheRef,
    ) -> CVReturn;
}

#[must_use]
#[inline]
pub unsafe fn dispatch_get_main_queue() -> dispatch_queue_main_t {
    ptr::addr_of_mut!(_dispatch_main_q)
}
