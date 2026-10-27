use std::iter::FusedIterator;
use std::ops::Range;
use std::ptr::NonNull;

use super::{CFCharacterSet, CFLocale, CFString};
use crate::raw;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CFCharacterSetKind {
    DecimalDigit,
    NonBase,
}

impl CFCharacterSet {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn predefined(kind: CFCharacterSetKind) -> Self {
        let kind = match kind {
            CFCharacterSetKind::DecimalDigit => raw::kCFCharacterSetDecimalDigit,
            CFCharacterSetKind::NonBase => raw::kCFCharacterSetNonBase,
        };
        let set = unsafe { raw::CFCharacterSetGetPredefined(i64::from(kind)) };
        unsafe { Self::from_raw_borrowed(set.cast_mut().cast()) }.expect("predefined character set")
    }
}

impl CFString {
    #[must_use]
    pub fn composed_character_range(&self, index: usize) -> Option<Range<usize>> {
        let length = self.len();
        if index >= length {
            return None;
        }
        let range = unsafe {
            raw::CFStringGetRangeOfComposedCharactersAtIndex(
                self.as_ptr().cast(),
                raw::CFIndex::try_from(index).ok()?,
            )
        };
        checked_range(range, length)
    }

    pub fn composed_character_ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        let mut offset = 0;
        std::iter::from_fn(move || {
            let range = self.composed_character_range(offset)?;
            offset = range.end;
            Some(range)
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CFStringTokenUnit {
    Word,
    WordBoundary,
}

pub struct CFStringTokenizer<'a> {
    raw: NonNull<raw::__CFStringTokenizer>,
    string: &'a CFString,
    finished: bool,
}

impl<'a> CFStringTokenizer<'a> {
    #[must_use]
    pub fn new(
        string: &'a CFString,
        unit: CFStringTokenUnit,
        locale: Option<&CFLocale>,
    ) -> Option<Self> {
        let options = match unit {
            CFStringTokenUnit::Word => raw::kCFStringTokenizerUnitWord,
            CFStringTokenUnit::WordBoundary => raw::kCFStringTokenizerUnitWordBoundary,
        };
        let tokenizer = unsafe {
            raw::CFStringTokenizerCreate(
                std::ptr::null(),
                string.as_ptr().cast(),
                raw::CFRange {
                    location: 0,
                    length: raw::CFIndex::try_from(string.len()).ok()?,
                },
                u64::from(options),
                locale.map_or(std::ptr::null(), |locale| locale.as_ptr().cast()),
            )
        };
        Some(Self {
            raw: NonNull::new(tokenizer)?,
            string,
            finished: false,
        })
    }
}

impl Iterator for CFStringTokenizer<'_> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let kind = unsafe { raw::CFStringTokenizerAdvanceToNextToken(self.raw.as_ptr()) };
        if kind == u64::from(raw::kCFStringTokenizerTokenNone) {
            self.finished = true;
            return None;
        }
        let range = unsafe { raw::CFStringTokenizerGetCurrentTokenRange(self.raw.as_ptr()) };
        let range = checked_range(range, self.string.len());
        self.finished = range.is_none();
        range
    }
}

impl FusedIterator for CFStringTokenizer<'_> {}

impl Drop for CFStringTokenizer<'_> {
    fn drop(&mut self) {
        unsafe { raw::CFRelease(self.raw.as_ptr().cast()) };
    }
}

fn checked_range(range: raw::CFRange, length: usize) -> Option<Range<usize>> {
    let start = usize::try_from(range.location).ok()?;
    let count = usize::try_from(range.length).ok()?;
    let end = start.checked_add(count)?;
    (start < end && end <= length).then_some(start..end)
}
