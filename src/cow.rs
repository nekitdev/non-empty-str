//! Non-empty [`Cow<'_, str>`](Cow).

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::{fmt, ops::Deref};

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{borrow::Cow, string::String};

#[cfg(all(
    not(feature = "std"),
    feature = "alloc",
    feature = "serde",
    feature = "borrow"
))]
use alloc::borrow::ToOwned;

use const_macros::{const_early, const_ok, const_quick};

#[cfg(feature = "static")]
use into_static::IntoStatic;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[cfg(all(feature = "serde", feature = "borrow"))]
use serde::de::Visitor;

use crate::{empty::Empty, owned::OwnedStr, str::Str};

/// Represents non-empty clone-on-write strings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CowStr<'s> {
    value: Cow<'s, str>,
}

#[cfg(feature = "serde")]
impl Serialize for CowStr<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(all(feature = "serde", feature = "borrow"))]
struct CowStrVisitor;

#[cfg(all(feature = "serde", feature = "borrow"))]
impl<'de> Visitor<'de> for CowStrVisitor {
    type Value = CowStr<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("non-empty string")
    }

    fn visit_borrowed_str<E: Error>(self, value: &'de str) -> Result<Self::Value, E> {
        CowStr::borrowed(value).map_err(Error::custom)
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        self.visit_string(value.to_owned())
    }

    fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
        CowStr::owned(value).map_err(Error::custom)
    }
}

#[cfg(all(feature = "serde", feature = "borrow"))]
impl<'de: 'c, 'c> Deserialize<'de> for CowStr<'c> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(CowStrVisitor)
    }
}

#[cfg(all(feature = "serde", not(feature = "borrow")))]
impl<'de> Deserialize<'de> for CowStr<'_> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Cow::deserialize(deserializer)?;

        Self::new(value).map_err(Error::custom)
    }
}

impl fmt::Display for CowStr<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
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

impl<'s> TryFrom<Cow<'s, str>> for CowStr<'s> {
    type Error = Empty;

    fn try_from(value: Cow<'s, str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'s> TryFrom<&'s str> for CowStr<'s> {
    type Error = Empty;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Self::borrowed(value)
    }
}

impl TryFrom<String> for CowStr<'_> {
    type Error = Empty;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::owned(value)
    }
}

impl<'s> From<Str<'s>> for CowStr<'s> {
    fn from(string: Str<'s>) -> Self {
        Self::from_str(string)
    }
}

impl From<OwnedStr> for CowStr<'_> {
    fn from(string: OwnedStr) -> Self {
        Self::from_owned_str(string)
    }
}

impl<'s> From<CowStr<'s>> for Cow<'s, str> {
    fn from(value: CowStr<'s>) -> Self {
        value.take()
    }
}

impl<'s> CowStr<'s> {
    /// Constructs [`Self`], provided that the value is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    pub fn new(value: Cow<'s, str>) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    /// Similar to [`new`], except the error is discarded.
    ///
    /// [`new`]: Self::new
    pub fn new_ok(value: Cow<'s, str>) -> Option<Self> {
        const_ok!(Self::new(value))
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
    pub const fn borrowed(value: &'s str) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::borrowed_unchecked(value) })
    }

    /// Similar to [`borrowed`], but the error is discarded.
    ///
    /// [`borrowed`]: Self::borrowed
    pub const fn borrowed_ok(value: &'s str) -> Option<Self> {
        const_quick!(value.is_empty());

        // SAFETY: the value is non-empty at this point
        Some(unsafe { Self::borrowed_unchecked(value) })
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
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::owned_unchecked(value) })
    }

    /// Similar to [`owned`], except the error is discarded.
    ///
    /// [`owned`]: Self::owned
    pub fn owned_ok(value: String) -> Option<Self> {
        const_ok!(Self::owned(value))
    }

    /// Similar to [`new_unchecked`], but accepts owned strings.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is non-empty.
    ///
    /// [`new_unchecked`]: Self::new_unchecked
    pub const unsafe fn owned_unchecked(value: String) -> Self {
        // SAFETY: the caller must ensure that the value is non-empty
        unsafe { Self::new_unchecked(Cow::Owned(value)) }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_non_empty(&self) {
        // SAFETY: the value is non-empty by construction
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    /// Consumes [`Self`] and returns the wrapped string.
    pub fn take(self) -> Cow<'s, str> {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }

    /// Constructs [`Self`] from [`Str`].
    pub const fn from_str(string: Str<'s>) -> Self {
        // SAFETY: the contained string is non-empty
        unsafe { Self::borrowed_unchecked(string.take()) }
    }

    /// Constructs [`Self`] from [`OwnedStr`].
    pub fn from_owned_str(string: OwnedStr) -> Self {
        // SAFETY: the contained string is non-empty
        unsafe { Self::owned_unchecked(string.take()) }
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

/// An alias for [`CowStr<'static>`](CowStr).
#[cfg(feature = "static")]
pub type StaticCowStr = CowStr<'static>;

#[cfg(feature = "static")]
impl IntoStatic for CowStr<'_> {
    type Static = StaticCowStr;

    fn into_static(self) -> Self::Static {
        // SAFETY: the contained string is non-empty
        unsafe { Self::Static::new_unchecked(self.take().into_static()) }
    }
}
