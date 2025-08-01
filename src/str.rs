//! Non-empty [`str`].

use core::{fmt, ops::Deref};

use thiserror::Error;

/// The error message used when the string is empty.
pub const EMPTY: &str = "the string is empty";

/// Represents errors returned when received strings are empty.
#[derive(Debug, Error)]
#[error("{EMPTY}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(code(non_empty_str::str), help("make sure the string is non-empty"))
)]
pub struct Empty;

/// Represents non-empty [`str`] values.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Str {
    inner: str,
}

impl fmt::Display for Str {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod owned {
    use crate::owned::OwnedStr;

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    use alloc::borrow::ToOwned;

    use super::Str;

    impl ToOwned for Str {
        type Owned = OwnedStr;

        fn to_owned(&self) -> Self::Owned {
            Self::Owned::from_str(self)
        }
    }
}

impl<'s> TryFrom<&'s str> for &'s Str {
    type Error = Empty;

    fn try_from(string: &'s str) -> Result<Self, Self::Error> {
        Str::try_from_str(string)
    }
}

impl<'s> From<&'s Str> for &'s str {
    fn from(string: &'s Str) -> Self {
        string.get()
    }
}

impl AsRef<Self> for Str {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl Str {
    /// Constructs [`Self`] from anything that can be converted to string, provided it is non-empty.
    ///
    /// Prefer [`try_from_str`] if only [`str`] is used, as this allows for `const` evaluation.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    ///
    /// [`try_from_str`]: Self::try_from_str
    pub fn try_new<S: AsRef<str> + ?Sized>(string: &S) -> Result<&Self, Empty> {
        Self::try_from_str(string.as_ref())
    }

    /// Similar to [`try_new`], but the error is discarded.
    ///
    /// Prefer [`from_str`] if only [`str`] is used, as this allows for `const` evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_str::Str;
    ///
    /// let non_empty = Str::new("Hello, world!").unwrap();
    ///
    /// // `Str` is `AsRef<str>`, so it can also be used!
    /// let from_non_empty = Str::new(non_empty).unwrap();
    /// ```
    ///
    /// [`try_new`]: Self::try_new
    /// [`from_str`]: Self::from_str
    pub fn new<S: AsRef<str> + ?Sized>(string: &S) -> Option<&Self> {
        Self::from_str(string.as_ref())
    }

    /// Constructs [`Self`] from anything that can be converted to string, without doing any checks.
    ///
    /// Prefer [`from_str_unchecked`] if only [`str`] is used; this allows for `const` evaluation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the string is non-empty.
    ///
    /// [`from_str_unchecked`]: Self::from_str_unchecked
    #[must_use]
    pub unsafe fn new_unchecked<S: AsRef<str> + ?Sized>(string: &S) -> &Self {
        // SAFETY: the caller must ensure that the string is non-empty
        unsafe { Self::from_str_unchecked(string.as_ref()) }
    }

    /// Constructs [`Self`] from [`str`], provided the string is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] if the string is empty.
    pub const fn try_from_str(string: &str) -> Result<&Self, Empty> {
        if string.is_empty() {
            return Err(Empty);
        }

        // SAFETY: the string is non-empty at this point
        Ok(unsafe { Self::from_str_unchecked(string) })
    }

    /// Similar to [`try_from_str`], but the error is discarded.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_str::Str;
    ///
    /// let message = Str::from_str("Hello, world!").unwrap();
    /// ```
    ///
    /// [`None`] is returned if the string is empty, therefore the following snippet panics:
    ///
    /// ```should_panic
    /// use non_empty_str::Str;
    ///
    /// let never = Str::from_str("").unwrap();
    /// ```
    ///
    /// [`try_from_str`]: Self::try_from_str
    #[must_use]
    pub const fn from_str(string: &str) -> Option<&Self> {
        if string.is_empty() {
            return None;
        }

        // SAFETY: the string is non-empty at this point
        Some(unsafe { Self::from_str_unchecked(string) })
    }

    /// Constructs [`Self`] from [`str`], without checking if the string is empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the string is non-empty.
    #[must_use]
    pub const unsafe fn from_str_unchecked(string: &str) -> &Self {
        debug_assert!(!string.is_empty());

        // SAFETY: the caller must ensure that the string is non-empty
        // `Str` is `#[repr(transparent)]`, so it is safe to transmute
        #[allow(clippy::ref_as_ptr)]
        unsafe {
            &*(string as *const str as *const Self)
        }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the string is non-empty by construction
        unsafe {
            assert_unchecked(!self.inner.is_empty());
        }
    }

    /// Returns the contained string.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_str::Str;
    ///
    /// let string = "Hello, world!";
    ///
    /// let non_empty = Str::from_str(string).unwrap();
    ///
    /// assert_eq!(non_empty.get(), string);
    /// ```
    #[must_use]
    pub const fn get(&self) -> &str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        &self.inner
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Str;

    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    impl Serialize for Str {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.get().serialize(serializer)
        }
    }

    impl<'de: 's, 's> Deserialize<'de> for &'s Str {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let string = <&str>::deserialize(deserializer)?;

            let non_empty = string.try_into().map_err(D::Error::custom)?;

            Ok(non_empty)
        }
    }
}
