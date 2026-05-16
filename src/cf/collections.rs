//! Core Foundation collection wrappers.
//!
#![allow(clippy::missing_panics_doc)]

//! ```rust
//! use apple_cf::cf::{CFArray, CFAttributedString, CFBag, CFDictionary, CFString, CFTree};
//!
//! let first = CFString::new("first");
//! let second = CFString::new("second");
//! let array = CFArray::from_values(&[&first, &second]);
//! assert_eq!(array.len(), 2);
//!
//! let dict = CFDictionary::from_pairs(&[(&first, &second)]);
//! assert!(dict.contains_key(&first));
//!
//! let bag = CFBag::from_values(&[&first, &first, &second]);
//! assert_eq!(bag.count_of_value(&first), 2);
//!
//! let attributed = CFAttributedString::new(&first);
//! assert_eq!(attributed.string().to_string(), "first");
//!
//! let root = CFTree::new(Some(&first));
//! let child = CFTree::new(Some(&second));
//! root.append_child(&child);
//! assert_eq!(root.child_count(), 1);
//! ```

use super::base::{impl_cf_type_wrapper, AsCFType, CFType, SwiftObject};
use super::CFString;
use crate::ffi;
use std::fmt;

impl_cf_type_wrapper!(CFArray, cf_array_get_type_id);
impl_cf_type_wrapper!(CFDictionary, cf_dictionary_get_type_id);
pub type CFDict = CFDictionary;
impl_cf_type_wrapper!(CFBag, cf_bag_get_type_id);
impl_cf_type_wrapper!(CFAttributedString, cf_attributed_string_get_type_id);

impl CFArray {
    /// Create an array from borrowed Core Foundation values.
    #[must_use]
    pub fn from_values(values: &[&dyn AsCFType]) -> Self {
        let raw_values: Vec<*mut std::ffi::c_void> = values.iter().map(|value| value.as_ptr()).collect();
        let ptr = unsafe { ffi::cf_array_create(raw_values.as_ptr(), raw_values.len()) };
        Self::from_raw(ptr).expect("CFArrayCreate returned NULL")
    }

    /// Number of elements in the array.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { ffi::cf_array_get_count(self.as_ptr()) }
    }

    /// Whether the array is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copy the value at `index`.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<CFType> {
        let ptr = unsafe { ffi::cf_array_get_value_at_index(self.as_ptr(), index) };
        CFType::from_raw(ptr)
    }

    /// Copy all elements into a Rust vector.
    #[must_use]
    pub fn values(&self) -> Vec<CFType> {
        (0..self.len()).filter_map(|index| self.get(index)).collect()
    }
}

impl CFDictionary {
    /// Create a dictionary from borrowed key/value pairs.
    #[must_use]
    pub fn from_pairs(pairs: &[(&dyn AsCFType, &dyn AsCFType)]) -> Self {
        let keys: Vec<*mut std::ffi::c_void> = pairs.iter().map(|(key, _)| key.as_ptr()).collect();
        let values: Vec<*mut std::ffi::c_void> = pairs.iter().map(|(_, value)| value.as_ptr()).collect();
        let ptr = unsafe { ffi::cf_dictionary_create(keys.as_ptr(), values.as_ptr(), pairs.len()) };
        Self::from_raw(ptr).expect("CFDictionaryCreate returned NULL")
    }

    /// Number of key/value pairs.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { ffi::cf_dictionary_get_count(self.as_ptr()) }
    }

    /// Whether the dictionary is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether `key` exists in the dictionary.
    #[must_use]
    pub fn contains_key(&self, key: &dyn AsCFType) -> bool {
        unsafe { ffi::cf_dictionary_contains_key(self.as_ptr(), key.as_ptr()) }
    }

    /// Copy the value associated with `key`.
    #[must_use]
    pub fn get(&self, key: &dyn AsCFType) -> Option<CFType> {
        let ptr = unsafe { ffi::cf_dictionary_get_value(self.as_ptr(), key.as_ptr()) };
        CFType::from_raw(ptr)
    }

    /// Copy all keys into a `CFArray`.
    #[must_use]
    pub fn keys(&self) -> CFArray {
        let ptr = unsafe { ffi::cf_dictionary_copy_keys(self.as_ptr()) };
        CFArray::from_raw(ptr).expect("CFDictionary keys array should be non-null")
    }

    /// Copy all values into a `CFArray`.
    #[must_use]
    pub fn values(&self) -> CFArray {
        let ptr = unsafe { ffi::cf_dictionary_copy_values(self.as_ptr()) };
        CFArray::from_raw(ptr).expect("CFDictionary values array should be non-null")
    }
}

impl CFBag {
    /// Create a bag from borrowed Core Foundation values.
    #[must_use]
    pub fn from_values(values: &[&dyn AsCFType]) -> Self {
        let raw_values: Vec<*mut std::ffi::c_void> = values.iter().map(|value| value.as_ptr()).collect();
        let ptr = unsafe { ffi::cf_bag_create(raw_values.as_ptr(), raw_values.len()) };
        Self::from_raw(ptr).expect("CFBagCreate returned NULL")
    }

    /// Number of values in the bag.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { ffi::cf_bag_get_count(self.as_ptr()) }
    }

    /// Whether the bag is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the bag contains `candidate`.
    #[must_use]
    pub fn contains(&self, candidate: &dyn AsCFType) -> bool {
        unsafe { ffi::cf_bag_contains_value(self.as_ptr(), candidate.as_ptr()) }
    }

    /// Number of times `candidate` appears in the bag.
    #[must_use]
    pub fn count_of_value(&self, candidate: &dyn AsCFType) -> usize {
        unsafe { ffi::cf_bag_get_count_of_value(self.as_ptr(), candidate.as_ptr()) }
    }
}

impl CFAttributedString {
    /// Create an attributed string with no attributes.
    #[must_use]
    pub fn new(string: &CFString) -> Self {
        let ptr = unsafe { ffi::cf_attributed_string_create(string.as_ptr()) };
        Self::from_raw(ptr).expect("CFAttributedStringCreate returned NULL")
    }

    /// Underlying plain string.
    #[must_use]
    pub fn string(&self) -> CFString {
        let ptr = unsafe { ffi::cf_attributed_string_get_string(self.as_ptr()) };
        CFString::from_raw(ptr).expect("CFAttributedStringGetString returned NULL")
    }

    /// Character length.
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { ffi::cf_attributed_string_get_length(self.as_ptr()) }
    }

    /// Whether the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Safe wrapper around a Swift-backed `CFTree` helper.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CFTree(SwiftObject);

impl CFTree {
    /// Create a tree node with an optional Core Foundation payload.
    #[must_use]
    pub fn new(value: Option<&dyn AsCFType>) -> Self {
        let ptr = unsafe { ffi::cf_tree_create(value.map_or(std::ptr::null_mut(), AsCFType::as_ptr)) };
        Self(SwiftObject::from_raw(ptr).expect("tree bridge returned NULL"))
    }

    #[must_use]
    pub(crate) fn from_raw(ptr: *mut std::ffi::c_void) -> Option<Self> {
        SwiftObject::from_raw(ptr).map(Self)
    }

    /// Borrow the raw tree handle.
    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.0.as_ptr()
    }

    /// Append `child` to this node.
    pub fn append_child(&self, child: &Self) {
        unsafe { ffi::cf_tree_append_child(self.as_ptr(), child.as_ptr()) };
    }

    /// Number of direct children.
    #[must_use]
    pub fn child_count(&self) -> usize {
        unsafe { ffi::cf_tree_get_child_count(self.as_ptr()) }
    }

    /// Copy the child at `index`.
    #[must_use]
    pub fn child_at(&self, index: usize) -> Option<Self> {
        let ptr = unsafe { ffi::cf_tree_get_child_at_index(self.as_ptr(), index) };
        Self::from_raw(ptr)
    }

    /// Copy the payload value if present.
    #[must_use]
    pub fn value(&self) -> Option<CFType> {
        let ptr = unsafe { ffi::cf_tree_copy_value(self.as_ptr()) };
        CFType::from_raw(ptr)
    }
}

impl fmt::Debug for CFTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CFTree")
            .field("ptr", &self.as_ptr())
            .field("child_count", &self.child_count())
            .finish()
    }
}
