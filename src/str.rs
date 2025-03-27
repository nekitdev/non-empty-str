//! Non-empty [`str`].

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

use core::{fmt, ops::Deref};

use const_macros::{const_early, const_ok};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::empty::Empty;

/// Represents non-empty [`str`] values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Str<'s> {
    value: &'s str,
}

#[cfg(feature = "serde")]
impl Serialize for Str<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
type Value<'s> = &'s str;

#[cfg(feature = "serde")]
impl<'de: 's, 's> Deserialize<'de> for Str<'s> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;

        Self::new(value).map_err(Error::custom)
    }
}

impl fmt::Display for Str<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<'s> TryFrom<&'s str> for Str<'s> {
    type Error = Empty;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'s> From<Str<'s>> for &'s str {
    fn from(value: Str<'s>) -> Self {
        value.take()
    }
}

impl AsRef<str> for Str<'_> {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Deref for Str<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'s> Str<'s> {
    /// Constructs [`Self`], provided the given value is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the given string is empty.
    pub const fn new(value: &'s str) -> Result<Self, Empty> {
        const_early!(value.is_empty() => Empty);

        // SAFETY: the value is non-empty at this point
        Ok(unsafe { Self::new_unchecked(value) })
    }

    /// Similar to [`new`], but the error is discarded.
    ///
    /// [`new`]: Self::new
    pub const fn new_ok(value: &'s str) -> Option<Self> {
        const_ok!(Self::new(value))
    }

    /// Constructs [`Self`] without checking if the given value is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given value is non-empty.
    pub const unsafe fn new_unchecked(value: &'s str) -> Self {
        Self { value }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        // SAFETY: the value is non-empty by construction
        unsafe {
            assert_unchecked(!self.value.is_empty());
        }
    }

    /// Consumes [`Self`], returning the wrapped string.
    pub const fn take(self) -> &'s str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }
}

/// Type alias for [`Str`] with `'static` lifetime.
pub type StaticStr = Str<'static>;

impl Str<'_> {
    /// Returns the wrapped string reference.
    pub const fn get(&self) -> &str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.value
    }
}
