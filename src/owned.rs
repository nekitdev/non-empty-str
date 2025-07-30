//! Non-empty [`String`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::ToOwned, string::String};

use core::{borrow::Borrow, fmt, ops::Deref};

use thiserror::Error;

use crate::str::Str;

/// The error message used when the owned string is empty.
pub const EMPTY_OWNED: &str = "the owned string is empty";

/// Similar to [`Empty`], but holds the empty string provided.
///
/// [`Empty`]: crate::str::Empty
#[derive(Debug, Error)]
#[error("{EMPTY_OWNED}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(
        code(non_empty_str::owned),
        help("make sure the owned string is non-empty")
    )
)]
pub struct EmptyOwned {
    string: String,
}

impl EmptyOwned {
    const fn new(string: String) -> Self {
        Self { string }
    }

    /// Returns the contained empty string.
    #[must_use]
    pub fn get(self) -> String {
        self.string
    }
}

/// Represents non-empty [`String`] values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct OwnedStr {
    inner: String,
}

impl Borrow<Str> for OwnedStr {
    fn borrow(&self) -> &Str {
        self.as_str()
    }
}

impl fmt::Display for OwnedStr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl TryFrom<String> for OwnedStr {
    type Error = EmptyOwned;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OwnedStr> for String {
    fn from(string: OwnedStr) -> Self {
        string.get()
    }
}

impl From<&Str> for OwnedStr {
    fn from(string: &Str) -> Self {
        Self::from_str(string)
    }
}

impl Deref for OwnedStr {
    type Target = Str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl OwnedStr {
    /// Constructs [`Self`], provided that the [`String`] is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyOwned`] if the string is empty.
    pub const fn new(string: String) -> Result<Self, EmptyOwned> {
        if string.is_empty() {
            return Err(EmptyOwned::new(string));
        }

        // SAFETY: the string is non-empty at this point
        Ok(unsafe { Self::new_unchecked(string) })
    }

    /// Constructs [`Self`] without checking if the value is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    #[must_use]
    pub const unsafe fn new_unchecked(inner: String) -> Self {
        debug_assert!(!inner.is_empty());

        Self { inner }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        unsafe {
            assert_unchecked(!self.inner.is_empty());
        }
    }

    /// Constructs [`Self`] from [`Str`] via cloning.
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn from_str(string: &Str) -> Self {
        unsafe { Self::new_unchecked(string.get().to_owned()) }
    }

    /// Returns contained string reference as [`Str`].
    #[must_use]
    pub const fn as_str(&self) -> &Str {
        // SAFETY: the string is non-empty by construction
        unsafe { Str::from_str_unchecked(self.inner.as_str()) }
    }

    /// Returns the contained [`String`].
    #[must_use]
    pub fn get(self) -> String {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.inner
    }
}

#[cfg(feature = "serde")]
mod serde {
    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    use alloc::string::String;

    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    use super::OwnedStr;

    impl Serialize for OwnedStr {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_str().serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for OwnedStr {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let string = String::deserialize(deserializer)?;

            let non_empty = string.try_into().map_err(D::Error::custom)?;

            Ok(non_empty)
        }
    }
}
