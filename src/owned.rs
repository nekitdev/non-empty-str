//! Non-empty [`String`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::{fmt, ops::Deref};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::ToOwned, string::String};

use const_macros::{const_early, const_ok};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::{cow::CowStr, empty::Empty, str::Str};

/// Represents non-empty owned strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedStr {
    value: String,
}

#[cfg(feature = "serde")]
impl Serialize for OwnedStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for OwnedStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;

        Self::new(string).map_err(Error::custom)
    }
}

impl fmt::Display for OwnedStr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl TryFrom<String> for OwnedStr {
    type Error = Empty;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<OwnedStr> for String {
    fn from(owned: OwnedStr) -> Self {
        owned.take()
    }
}

impl From<Str<'_>> for OwnedStr {
    fn from(string: Str<'_>) -> Self {
        Self::from_str(string)
    }
}

impl From<CowStr<'_>> for OwnedStr {
    fn from(cow: CowStr<'_>) -> Self {
        Self::from_cow_str(cow)
    }
}

impl Deref for OwnedStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl OwnedStr {
    /// Constructs [`Self`], provided that the value is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    pub fn new(value: String) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    /// Similar to [`new`], but the error is discarded.
    ///
    /// [`new`]: Self::new
    pub fn new_ok(value: String) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    /// Constructs [`Self`] without checking if the value is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    pub const unsafe fn new_unchecked(value: String) -> Self {
        Self { value }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_non_empty(&self) {
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    /// Constructs [`Self`] from [`Str`] via cloning.
    pub fn from_str(value: Str<'_>) -> Self {
        // SAFETY: the contained string is non-empty
        unsafe { Self::new_unchecked(value.take().to_owned()) }
    }

    /// Constructs [`Self`] from [`CowStr`] via (optionally) cloning.
    pub fn from_cow_str(value: CowStr<'_>) -> Self {
        // SAFETY: the contained string is non-empty
        unsafe { Self::new_unchecked(value.take().into_owned()) }
    }

    /// Consumes [`Self`] and returns the contained [`String`].
    pub fn take(self) -> String {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }

    /// Returns the contained string reference.
    pub fn get(&self) -> &str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value.as_str()
    }
}
