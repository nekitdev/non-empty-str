//! Non-empty [`Box<str>`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{boxed::Box, string::String};

use non_empty_iter::{FromNonEmptyIterator, IntoNonEmptyIterator};
use non_empty_slice::{NonEmptyBoxedBytes, NonEmptyBytes};
use thiserror::Error;

use crate::{
    cow::NonEmptyCowStr,
    internal::Bytes,
    str::NonEmptyStr,
    string::{EmptyString, NonEmptyString},
};

/// The error message used when the boxed string is empty.
pub const EMPTY_BOXED_STR: &str = "the boxed string is empty";

/// Similar to [`EmptyString`], but contains the empty boxed string provided.
#[derive(Debug, Error)]
#[error("{EMPTY_BOXED_STR}")]
pub struct EmptyBoxedStr {
    boxed: Box<str>,
}

impl EmptyBoxedStr {
    // NOTE: this is private to prevent creating this error with non-empty boxed strings
    pub(crate) const fn new(boxed: Box<str>) -> Self {
        Self { boxed }
    }

    /// Returns the contained empty boxed string.
    #[must_use]
    pub fn get(self) -> Box<str> {
        self.boxed
    }

    /// Constructs [`Self`] from [`EmptyString`].
    #[must_use]
    pub fn from_empty_string(empty: EmptyString) -> Self {
        Self::new(empty.get().into_boxed_str())
    }

    /// Converts [`Self`] into [`EmptyString`].
    #[must_use]
    pub fn into_empty_string(self) -> EmptyString {
        EmptyString::from_empty_boxed_str(self)
    }
}

/// Represents non-empty boxed strings, [`Box<NonEmptyStr>`].
pub type NonEmptyBoxedStr = Box<NonEmptyStr>;

impl Clone for NonEmptyBoxedStr {
    fn clone(&self) -> Self {
        self.to_non_empty_string().into_non_empty_boxed_str()
    }
}

impl From<NonEmptyBoxedStr> for Box<str> {
    fn from(boxed: NonEmptyBoxedStr) -> Self {
        boxed.into_boxed_str()
    }
}

impl From<NonEmptyBoxedStr> for Box<Bytes> {
    fn from(boxed: NonEmptyBoxedStr) -> Self {
        boxed.into_boxed_bytes()
    }
}

impl TryFrom<Box<str>> for NonEmptyBoxedStr {
    type Error = EmptyBoxedStr;

    fn try_from(boxed: Box<str>) -> Result<Self, Self::Error> {
        NonEmptyStr::from_boxed_str(boxed)
    }
}

impl TryFrom<String> for NonEmptyBoxedStr {
    type Error = EmptyString;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let non_empty_string = NonEmptyString::new(string)?;

        Ok(non_empty_string.into())
    }
}

impl From<NonEmptyBoxedStr> for NonEmptyString {
    fn from(non_empty: NonEmptyBoxedStr) -> Self {
        non_empty.into_non_empty_string()
    }
}

impl From<NonEmptyString> for NonEmptyBoxedStr {
    fn from(non_empty: NonEmptyString) -> Self {
        non_empty.into_non_empty_boxed_str()
    }
}

impl From<NonEmptyBoxedStr> for String {
    fn from(non_empty: NonEmptyBoxedStr) -> Self {
        non_empty.into_boxed_str().into_string()
    }
}

impl From<NonEmptyBoxedStr> for NonEmptyBoxedBytes {
    fn from(non_empty: NonEmptyBoxedStr) -> Self {
        non_empty.into_non_empty_boxed_bytes()
    }
}

impl NonEmptyStr {
    /// Constructs [`Self`] from [`Box<str>`], provided the boxed string is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyBoxedStr`] if the boxed string is empty.
    pub fn from_boxed_str(boxed: Box<str>) -> Result<Box<Self>, EmptyBoxedStr> {
        if boxed.is_empty() {
            return Err(EmptyBoxedStr::new(boxed));
        }

        // SAFETY: the boxed string is non-empty at this point
        Ok(unsafe { Self::from_boxed_str_unchecked(boxed) })
    }

    /// Constructs [`Self`] from [`Box<str>`] without checking if the boxed string is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the boxed string is non-empty.
    #[must_use]
    pub unsafe fn from_boxed_str_unchecked(boxed: Box<str>) -> Box<Self> {
        // SAFETY: the caller must ensure that the boxed string is non-empty
        // moreover, `Self` is `repr(transparent)`, so it is safe to transmute
        // finally, `Box` is created from the raw pointer existing within this function only
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut Self) }
    }

    /// Converts [`Self`] into [`Box<str>`].
    #[must_use]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        // SAFETY: `Self` is `repr(transparent)`, so it is safe to transmute
        // moreover, `Box` is created from the raw pointer existing within this function only
        unsafe { Box::from_raw(Box::into_raw(self) as *mut str) }
    }

    /// Constructs [`Self`] from [`NonEmptyString`].
    #[must_use]
    pub fn from_non_empty_string(non_empty: NonEmptyString) -> Box<Self> {
        // SAFETY: the string is non-empty by construction, so is the underlying boxed string
        unsafe { Self::from_boxed_str_unchecked(non_empty.into_string().into_boxed_str()) }
    }

    /// Converts [`Self`] into [`NonEmptyString`].
    #[must_use]
    pub fn into_non_empty_string(self: Box<Self>) -> NonEmptyString {
        NonEmptyString::from_non_empty_boxed_str(self)
    }

    /// Converts [`Self`] into [`Box<[u8]>`](Box).
    #[must_use]
    pub fn into_boxed_bytes(self: Box<Self>) -> Box<Bytes> {
        self.into_boxed_str().into_boxed_bytes()
    }

    /// Converts [`Self`] into [`NonEmptyBoxedBytes`].
    #[must_use]
    pub fn into_non_empty_boxed_bytes(self: Box<Self>) -> NonEmptyBoxedBytes {
        // SAFETY: the string is non-empty by construction, so are its bytes
        unsafe { NonEmptyBytes::from_boxed_slice_unchecked(self.into_boxed_bytes()) }
    }
}

impl NonEmptyString {
    /// Constructs [`Self`] from [`NonEmptyBoxedStr`].
    #[must_use]
    pub fn from_non_empty_boxed_str(non_empty: NonEmptyBoxedStr) -> Self {
        // SAFETY: the boxed string is non-empty by construction, so is the resulting string
        unsafe { Self::new_unchecked(non_empty.into_boxed_str().into_string()) }
    }

    /// Converts [`Self`] into [`NonEmptyBoxedStr`].
    #[must_use]
    pub fn into_non_empty_boxed_str(self) -> NonEmptyBoxedStr {
        NonEmptyStr::from_non_empty_string(self)
    }

    /// Converts [`Self`] into [`Box<str>`].
    #[must_use]
    pub fn into_boxed_str(self) -> Box<str> {
        self.into_string().into_boxed_str()
    }

    /// Converts [`Self`] into [`Box<[u8]>`](Box).
    #[must_use]
    pub fn into_boxed_bytes(self) -> Box<Bytes> {
        self.into_non_empty_boxed_str().into_boxed_bytes()
    }

    /// Converts [`Self`] into [`NonEmptyBoxedBytes`].
    #[must_use]
    pub fn into_non_empty_boxed_bytes(self) -> NonEmptyBoxedBytes {
        self.into_non_empty_boxed_str().into_non_empty_boxed_bytes()
    }
}

impl FromNonEmptyIterator<char> for NonEmptyBoxedStr {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = char>>(iterable: I) -> Self {
        NonEmptyString::from_non_empty_iter(iterable).into_non_empty_boxed_str()
    }
}

impl<'c> FromNonEmptyIterator<&'c char> for NonEmptyBoxedStr {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = &'c char>>(iterable: I) -> Self {
        NonEmptyString::from_non_empty_iter(iterable).into_non_empty_boxed_str()
    }
}

impl<'s> FromNonEmptyIterator<&'s NonEmptyStr> for NonEmptyBoxedStr {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = &'s NonEmptyStr>>(iterable: I) -> Self {
        NonEmptyString::from_non_empty_iter(iterable).into_non_empty_boxed_str()
    }
}

impl FromNonEmptyIterator<NonEmptyString> for NonEmptyBoxedStr {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = NonEmptyString>>(iterable: I) -> Self {
        NonEmptyString::from_non_empty_iter(iterable).into_non_empty_boxed_str()
    }
}

impl FromNonEmptyIterator<Self> for NonEmptyBoxedStr {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = Self>>(iterable: I) -> Self {
        NonEmptyString::from_non_empty_iter(iterable).into_non_empty_boxed_str()
    }
}

impl<'s> FromNonEmptyIterator<NonEmptyCowStr<'s>> for NonEmptyBoxedStr {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = NonEmptyCowStr<'s>>>(
        iterable: I,
    ) -> Self {
        NonEmptyString::from_non_empty_iter(iterable).into_non_empty_boxed_str()
    }
}
