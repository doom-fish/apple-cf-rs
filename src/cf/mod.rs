//! Safe Core Foundation wrappers.
//!
//! This module provides lightweight, dependency-free wrappers for the most
//! common Core Foundation value, collection, locale, formatter, and runtime
//! types used throughout the Apple media stack.

mod base;
mod collections;
mod primitives;
mod resources;
mod runtime;

pub use base::{AsCFType, CFType};
pub use collections::{CFArray, CFAttributedString, CFBag, CFDict, CFDictionary, CFTree};
pub use primitives::{CFData, CFDate, CFError, CFNumber, CFString, CFUUID};
pub use resources::{
    CFBundle, CFCalendar, CFCharacterSet, CFDateFormatter, CFDateFormatterStyle,
    CFFileSecurity, CFLocale, CFNumberFormatter, CFNumberFormatterStyle, CFPreferences,
    CFTimeZone, CFURL, CFXML,
};
pub use runtime::{
    CFFileDescriptor, CFMessagePort, CFNotificationCenter, CFReadStream, CFRunLoop,
    CFRunLoopRunResult, CFSocket, CFStreamPair, CFTimer, CFWriteStream,
};

/// Convenience alias for the bound read/write stream pair.
pub type CFStream = CFStreamPair;
