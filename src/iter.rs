//! Various iterators over non-empty strings.

use core::{iter::Map, str};

use non_empty_iter::NonEmptyIterator;

use crate::{internal::Byte, str::NonEmptyStr};

/// Represents functions mapping non-empty [`prim@str`] to [`NonEmptyStr`].
///
/// This is mostly an implementation detail, though it can be useful in case
/// one needs to name the type of the iterator explicitly.
pub type NonEmptyStrFn<'s> = fn(&'s str) -> &'s NonEmptyStr;

/// Represents non-empty iterators over the bytes in non-empty strings.
///
/// This `struct` is created by the [`bytes`] method on [`NonEmptyStr`].
///
/// [`bytes`]: NonEmptyStr::bytes
#[derive(Debug)]
pub struct Bytes<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> Bytes<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for Bytes<'s> {
    type Item = Byte;
    type IntoIter = str::Bytes<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().bytes()
    }
}

unsafe impl NonEmptyIterator for Bytes<'_> {}

/// Represents non-empty iterators over the characters in non-empty strings.
///
/// This `struct` is created by the [`chars`] method on [`NonEmptyStr`].
///
/// [`chars`]: NonEmptyStr::chars
#[derive(Debug)]
pub struct Chars<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> Chars<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for Chars<'s> {
    type Item = char;
    type IntoIter = str::Chars<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().chars()
    }
}

unsafe impl NonEmptyIterator for Chars<'_> {}

/// Represents non-empty iterators over the characters and their positions in non-empty strings.
///
/// This `struct` is created by the [`char_indices`] method on [`NonEmptyStr`].
///
/// [`char_indices`]: NonEmptyStr::char_indices
#[derive(Debug)]
pub struct CharIndices<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> CharIndices<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for CharIndices<'s> {
    type Item = (usize, char);
    type IntoIter = str::CharIndices<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().char_indices()
    }
}

unsafe impl NonEmptyIterator for CharIndices<'_> {}

/// Represents iterators over the non-whitespace non-empty substrings of non-empty strings.
///
/// Note that this `struct` does not implement [`NonEmptyIterator`] as the iterator can be empty,
/// specifically, if the input string consists of whitespace only.
///
/// This `struct` is created by the [`split_whitespace`] method on [`NonEmptyStr`].
///
/// [`split_whitespace`]: NonEmptyStr::split_whitespace
#[derive(Debug)]
pub struct SplitWhitespace<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> SplitWhitespace<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for SplitWhitespace<'s> {
    type Item = &'s NonEmptyStr;
    type IntoIter = Map<str::SplitWhitespace<'s>, NonEmptyStrFn<'s>>;

    fn into_iter(self) -> Self::IntoIter {
        self.string
            .as_str()
            .split_whitespace()
            // SAFETY: `split_whitespace` never yields empty substrings
            .map(|string| unsafe { NonEmptyStr::from_str_unchecked(string) })
    }
}

// NOTE: `SplitWhitespace<'_>` does not implement `NonEmptyIterator` as it can be empty,
// specifically, if the input string consists of whitespace only

/// Represents iterators over the non-ASCII-whitespace non-empty substrings of non-empty strings.
///
/// Note that this `struct` does not implement [`NonEmptyIterator`] as the iterator can be empty,
/// specifically, if the input string consists of ASCII whitespace only.
///
/// This `struct` is created by the [`split_ascii_whitespace`] method on [`NonEmptyStr`].
///
/// [`split_ascii_whitespace`]: NonEmptyStr::split_ascii_whitespace
#[derive(Debug)]
pub struct SplitAsciiWhitespace<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> SplitAsciiWhitespace<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for SplitAsciiWhitespace<'s> {
    type Item = &'s NonEmptyStr;
    type IntoIter = Map<str::SplitAsciiWhitespace<'s>, NonEmptyStrFn<'s>>;

    fn into_iter(self) -> Self::IntoIter {
        self.string
            .as_str()
            .split_ascii_whitespace()
            // SAFETY: `split_ascii_whitespace` never yields empty substrings
            .map(|string| unsafe { NonEmptyStr::from_str_unchecked(string) })
    }
}

// NOTE: `SplitAsciiWhitespace<'_>` does not implement `NonEmptyIterator` as it can be empty,
// specifically, if the input string consists of ASCII whitespace only

/// Represents non-empty iterators over the UTF-16 encoding of non-empty strings.
///
/// This `struct` is created by the [`encode_utf16`] method on [`NonEmptyStr`].
///
/// [`encode_utf16`]: NonEmptyStr::encode_utf16
#[derive(Debug)]
pub struct EncodeUtf16<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> EncodeUtf16<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for EncodeUtf16<'s> {
    type Item = u16;
    type IntoIter = str::EncodeUtf16<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().encode_utf16()
    }
}

unsafe impl NonEmptyIterator for EncodeUtf16<'_> {}

/// Represents non-empty iterators over the debug-escaped characters in non-empty strings.
///
/// This `struct` is created by the [`escape_debug`] method on [`NonEmptyStr`].
///
/// [`escape_debug`]: NonEmptyStr::escape_debug
#[derive(Debug)]
pub struct EscapeDebug<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> EscapeDebug<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for EscapeDebug<'s> {
    type Item = char;
    type IntoIter = str::EscapeDebug<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().escape_debug()
    }
}

unsafe impl NonEmptyIterator for EscapeDebug<'_> {}

/// Represents non-empty iterators over the default-escaped characters in non-empty strings.
///
/// This `struct` is created by the [`escape_default`] method on [`NonEmptyStr`].
///
/// [`escape_default`]: NonEmptyStr::escape_default
#[derive(Debug)]
pub struct EscapeDefault<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> EscapeDefault<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for EscapeDefault<'s> {
    type Item = char;
    type IntoIter = str::EscapeDefault<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().escape_default()
    }
}

unsafe impl NonEmptyIterator for EscapeDefault<'_> {}

/// Represents non-empty iterators over the Unicode-escaped characters in non-empty strings.
///
/// This `struct` is created by the [`escape_unicode`] method on [`NonEmptyStr`].
///
/// [`escape_unicode`]: NonEmptyStr::escape_unicode
#[derive(Debug)]
pub struct EscapeUnicode<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> EscapeUnicode<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for EscapeUnicode<'s> {
    type Item = char;
    type IntoIter = str::EscapeUnicode<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().escape_unicode()
    }
}

unsafe impl NonEmptyIterator for EscapeUnicode<'_> {}

/// Represents non-empty iterators over the lines in non-empty strings.
///
/// This `struct` is created by the [`lines`] method on [`NonEmptyStr`].
///
/// [`lines`]: NonEmptyStr::lines
#[derive(Debug)]
pub struct Lines<'s> {
    string: &'s NonEmptyStr,
}

impl<'s> Lines<'s> {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(string: &'s NonEmptyStr) -> Self {
        Self { string }
    }
}

impl<'s> IntoIterator for Lines<'s> {
    type Item = &'s str;

    type IntoIter = str::Lines<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.string.as_str().lines()
    }
}

unsafe impl NonEmptyIterator for Lines<'_> {}
