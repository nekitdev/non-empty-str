//! Non-empty [`Cow<'_, str>`].

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::{fmt, ops::Deref};

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::Cow, string::String};

#[cfg(all(not(feature = "std"), feature = "alloc", feature = "serde"))]
use alloc::borrow::ToOwned;

use const_macros::const_try;

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};

use crate::empty::{Empty, check_str};

/// Represents non-empty clone-on-write strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CowStr<'s> {
    value: Cow<'s, str>,
}

impl fmt::Display for CowStr<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

#[cfg(feature = "serde")]
struct CowStrVisitor;

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for CowStrVisitor {
    type Value = CowStr<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("non-empty string")
    }

    fn visit_borrowed_str<E: Error>(self, string: &'de str) -> Result<Self::Value, E> {
        Self::Value::borrowed(string).map_err(E::custom)
    }

    fn visit_str<E: Error>(self, string: &str) -> Result<Self::Value, E> {
        self.visit_string(string.to_owned())
    }

    fn visit_string<E: Error>(self, string: String) -> Result<Self::Value, E> {
        Self::Value::owned(string).map_err(E::custom)
    }
}

#[cfg(feature = "serde")]
impl Serialize for CowStr<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for CowStr<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(CowStrVisitor)
    }
}

impl AsRef<str> for CowStr<'_> {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Deref for CowStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'s> CowStr<'s> {
    /// Constructs [`Self`], provided that the value is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    pub fn new(value: Cow<'s, str>) -> Result<Self, Empty> {
        const_try!(check_str(value.as_ref()));

        Ok(unsafe { Self::new_unchecked(value) })
    }

    /// Constructs [`Self`] without checking if the value is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    pub const unsafe fn new_unchecked(value: Cow<'s, str>) -> Self {
        Self { value }
    }

    /// Similar to [`new`], but accepts borrowed strings.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    ///
    /// [`new`]: Self::new
    pub fn borrowed(value: &'s str) -> Result<Self, Empty> {
        Self::new(Cow::Borrowed(value))
    }

    /// Similar to [`new_unchecked`], but accepts borrowed strings.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    ///
    /// [`new_unchecked`]: Self::new_unchecked
    pub const unsafe fn borrowed_unchecked(value: &'s str) -> Self {
        // SAFETY: the caller must ensure that the value is non-empty
        unsafe { Self::new_unchecked(Cow::Borrowed(value)) }
    }

    /// Similar to [`new`], but accepts owned strings.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    ///
    /// [`new`]: Self::new
    pub fn owned(value: String) -> Result<Self, Empty> {
        Self::new(Cow::Owned(value))
    }

    /// Similar to [`new_unchecked`], but accepts owned strings.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    ///
    /// [`new_unchecked`]: Self::new_unchecked
    pub const unsafe fn owned_unchecked(value: String) -> Self {
        unsafe { Self::new_unchecked(Cow::Owned(value)) }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_non_empty(&self) {
        unsafe { assert_unchecked(!self.value.is_empty()) }
    }

    /// Consumes [`Self`] and returns the wrapped string.
    pub fn take(self) -> Cow<'s, str> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }
}

impl CowStr<'_> {
    /// Returns the wrapped string reference.
    pub fn get(&self) -> &str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value.as_ref()
    }
}
