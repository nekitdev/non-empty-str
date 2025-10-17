//! Non-empty [`str`].

#[cfg(feature = "std")]
use std::{ffi::OsStr, path::Path};

use core::{
    fmt,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr,
    slice::SliceIndex,
    str::Utf8Error,
};

use non_empty_slice::{EmptySlice, NonEmptyBytes};
use non_zero_size::Size;
use thiserror::Error;

use crate::{
    internal::{Bytes, MutBytes, RawBytes, attempt, map_error},
    iter::{
        Bytes as BytesIter, CharIndices, Chars, EncodeUtf16, EscapeDebug, EscapeDefault,
        EscapeUnicode, Lines, SplitAsciiWhitespace, SplitWhitespace,
    },
};

/// The error message used when the string is empty.
pub const EMPTY_STR: &str = "the string is empty";

/// Represents errors returned when received strings are empty.
#[derive(Debug, Error)]
#[error("{EMPTY_STR}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(code(non_empty_str::str), help("make sure the string is non-empty"))
)]
pub struct EmptyStr;

/// Represents errors returned when the received non-empty bytes are not valid UTF-8.
///
/// This is returned from [`from_non_empty_utf8`] and [`from_non_empty_utf8_mut`] methods
/// on [`NonEmptyStr`].
///
/// [`from_non_empty_utf8`]: NonEmptyStr::from_non_empty_utf8
/// [`from_non_empty_utf8_mut`]: NonEmptyStr::from_non_empty_utf8_mut
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("{error}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(
        code(non_empty_str::str::utf8),
        help("make sure the bytes are valid UTF-8")
    )
)]
pub struct NonEmptyUtf8Error {
    #[from]
    #[source]
    error: Utf8Error,
}

impl NonEmptyUtf8Error {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(error: Utf8Error) -> Self {
        Self { error }
    }

    /// Returns the contained [`Utf8Error`].
    #[must_use]
    pub const fn get(self) -> Utf8Error {
        self.error
    }
}

/// Represents errors returned when the received bytes are either empty or not valid UTF-8.
///
/// This is returned from [`from_utf8`] and [`from_utf8_mut`] methods on [`NonEmptyStr`].
///
/// [`from_utf8`]: NonEmptyStr::from_utf8
/// [`from_utf8_mut`]: NonEmptyStr::from_utf8_mut
#[derive(Debug, Error)]
#[error(transparent)]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(transparent)
)]
pub enum MaybeEmptyUtf8Error {
    /// The received bytes are empty.
    Empty(#[from] EmptySlice),
    /// The received bytes are non-empty, but not valid UTF-8.
    Utf8(#[from] NonEmptyUtf8Error),
}

/// Parsing values from non-empty strings.
pub trait FromNonEmptyStr: Sized {
    /// The associated error type returned when parsing fails.
    type Error;

    /// Parses [`Self`] from the given non-empty string.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if parsing fails.
    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error>;
}

/// Represents non-empty [`str`] values.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NonEmptyStr {
    inner: str,
}

impl fmt::Display for NonEmptyStr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl<'s> TryFrom<&'s str> for &'s NonEmptyStr {
    type Error = EmptyStr;

    fn try_from(string: &'s str) -> Result<Self, Self::Error> {
        NonEmptyStr::try_from_str(string)
    }
}

impl<'s> TryFrom<&'s mut str> for &'s mut NonEmptyStr {
    type Error = EmptyStr;

    fn try_from(string: &'s mut str) -> Result<Self, Self::Error> {
        NonEmptyStr::try_from_mut_str(string)
    }
}

impl<'s> From<&'s NonEmptyStr> for &'s str {
    fn from(string: &'s NonEmptyStr) -> Self {
        string.as_str()
    }
}

impl<'b> TryFrom<&'b NonEmptyBytes> for &'b NonEmptyStr {
    type Error = NonEmptyUtf8Error;

    fn try_from(non_empty: &'b NonEmptyBytes) -> Result<Self, Self::Error> {
        NonEmptyStr::from_non_empty_utf8(non_empty)
    }
}

impl<'b> TryFrom<&'b mut NonEmptyBytes> for &'b mut NonEmptyStr {
    type Error = NonEmptyUtf8Error;

    fn try_from(non_empty: &'b mut NonEmptyBytes) -> Result<Self, Self::Error> {
        NonEmptyStr::from_non_empty_utf8_mut(non_empty)
    }
}

impl<'b> TryFrom<&'b Bytes> for &'b NonEmptyStr {
    type Error = MaybeEmptyUtf8Error;

    fn try_from(bytes: &'b Bytes) -> Result<Self, Self::Error> {
        NonEmptyStr::from_utf8(bytes)
    }
}

impl<'b> TryFrom<&'b mut Bytes> for &'b mut NonEmptyStr {
    type Error = MaybeEmptyUtf8Error;

    fn try_from(bytes: &'b mut Bytes) -> Result<Self, Self::Error> {
        NonEmptyStr::from_utf8_mut(bytes)
    }
}

impl<'s> From<&'s mut NonEmptyStr> for &'s mut str {
    fn from(string: &'s mut NonEmptyStr) -> Self {
        string.as_mut_str()
    }
}

impl AsRef<Self> for NonEmptyStr {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for NonEmptyStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsMut<Self> for NonEmptyStr {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsMut<str> for NonEmptyStr {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl AsRef<NonEmptyBytes> for NonEmptyStr {
    fn as_ref(&self) -> &NonEmptyBytes {
        self.as_non_empty_bytes()
    }
}

impl AsRef<Bytes> for NonEmptyStr {
    fn as_ref(&self) -> &Bytes {
        self.as_bytes()
    }
}

#[cfg(feature = "std")]
impl AsRef<OsStr> for NonEmptyStr {
    fn as_ref(&self) -> &OsStr {
        self.as_str().as_ref()
    }
}

#[cfg(feature = "std")]
impl AsRef<Path> for NonEmptyStr {
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}

impl Deref for NonEmptyStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl DerefMut for NonEmptyStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

impl<I: SliceIndex<str>> Index<I> for NonEmptyStr {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl<I: SliceIndex<str>> IndexMut<I> for NonEmptyStr {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_mut_str().index_mut(index)
    }
}

impl NonEmptyStr {
    /// Constructs [`Self`] from anything that can be converted to string, provided it is non-empty.
    ///
    /// Prefer [`try_from_str`] if only [`str`] is used, as this allows for `const` evaluation.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyStr`] if the string is empty.
    ///
    /// [`try_from_str`]: Self::try_from_str
    pub fn try_new<S: AsRef<str> + ?Sized>(string: &S) -> Result<&Self, EmptyStr> {
        Self::try_from_str(string.as_ref())
    }

    /// Constructs [`Self`] from anything that can be mutably converted to string,
    /// provided it is non-empty.
    ///
    /// Prefer [`try_from_mut_str`] if only [`str`] is used, as this allows for `const` evaluation.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyStr`] if the string is empty.
    ///
    /// [`try_from_mut_str`]: Self::try_from_mut_str
    pub fn try_new_mut<S: AsMut<str> + ?Sized>(string: &mut S) -> Result<&mut Self, EmptyStr> {
        Self::try_from_mut_str(string.as_mut())
    }

    /// Similar to [`try_new`], but the error is discarded.
    ///
    /// Prefer [`from_str`] if only [`str`] is used, as this allows for `const` evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_str::NonEmptyStr;
    ///
    /// let non_empty = NonEmptyStr::new("Hello, world!").unwrap();
    ///
    /// // `NonEmptyStr` is `AsRef<str>`, so it can also be used!
    /// let from_non_empty = NonEmptyStr::new(non_empty).unwrap();
    /// ```
    ///
    /// [`try_new`]: Self::try_new
    /// [`from_str`]: Self::from_str
    pub fn new<S: AsRef<str> + ?Sized>(string: &S) -> Option<&Self> {
        Self::from_str(string.as_ref())
    }

    /// Similar to [`try_new_mut`], but the error is discarded.
    ///
    /// Prefer [`from_mut_str`] if only [`str`] is used, as this allows for `const` evaluation.
    ///
    /// [`try_new_mut`]: Self::try_new_mut
    /// [`from_mut_str`]: Self::from_mut_str
    pub fn new_mut<S: AsMut<str> + ?Sized>(string: &mut S) -> Option<&mut Self> {
        Self::from_mut_str(string.as_mut())
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

    /// Constructs [`Self`] from anything that can be mutably converted to string,
    /// without doing any checks.
    ///
    /// Prefer [`from_mut_str_unchecked`] if only [`str`] is used;
    /// this allows for `const` evaluation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the string is non-empty.
    ///
    /// [`from_mut_str_unchecked`]: Self::from_mut_str_unchecked
    pub unsafe fn new_unchecked_mut<S: AsMut<str> + ?Sized>(string: &mut S) -> &mut Self {
        // SAFETY: the caller must ensure that the string is non-empty
        unsafe { Self::from_mut_str_unchecked(string.as_mut()) }
    }

    /// Constructs [`Self`] from [`str`], provided the string is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyStr`] if the string is empty.
    pub const fn try_from_str(string: &str) -> Result<&Self, EmptyStr> {
        if string.is_empty() {
            return Err(EmptyStr);
        }

        // SAFETY: the string is non-empty at this point
        Ok(unsafe { Self::from_str_unchecked(string) })
    }

    /// Constructs [`Self`] from mutable [`str`], provided the string is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyStr`] if the string is empty.
    pub const fn try_from_mut_str(string: &mut str) -> Result<&mut Self, EmptyStr> {
        if string.is_empty() {
            return Err(EmptyStr);
        }

        // SAFETY: the string is non-empty at this point
        Ok(unsafe { Self::from_mut_str_unchecked(string) })
    }

    /// Similar to [`try_from_str`], but the error is discarded.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_str::NonEmptyStr;
    ///
    /// let message = NonEmptyStr::from_str("Hello, world!").unwrap();
    /// ```
    ///
    /// [`None`] is returned if the string is empty, therefore the following snippet panics:
    ///
    /// ```should_panic
    /// use non_empty_str::NonEmptyStr;
    ///
    /// let never = NonEmptyStr::from_str("").unwrap();
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

    /// Similar to [`try_from_mut_str`], but the error is discarded.
    ///
    /// [`try_from_mut_str`]: Self::try_from_mut_str
    pub const fn from_mut_str(string: &mut str) -> Option<&mut Self> {
        if string.is_empty() {
            return None;
        }

        // SAFETY: the string is non-empty at this point
        Some(unsafe { Self::from_mut_str_unchecked(string) })
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
        // `Self` is `repr(transparent)`, so it is safe to transmute
        unsafe { &*(ptr::from_ref(string) as *const Self) }
    }

    /// Constructs [`Self`] from mutable [`str`], without checking if the string is empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the string is non-empty.
    pub const unsafe fn from_mut_str_unchecked(string: &mut str) -> &mut Self {
        debug_assert!(!string.is_empty());

        // SAFETY: the caller must ensure that the string is non-empty
        // `Self` is `repr(transparent)`, so it is safe to transmute
        unsafe { &mut *(ptr::from_mut(string) as *mut Self) }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the string is non-empty by construction
        unsafe {
            assert_unchecked(!self.as_str_no_assert().is_empty());
        }
    }

    const fn as_str_no_assert(&self) -> &str {
        &self.inner
    }

    const fn as_mut_str_no_assert(&mut self) -> &mut str {
        &mut self.inner
    }

    /// Returns the contained string.
    ///
    /// # Examples
    ///
    /// ```
    /// use non_empty_str::NonEmptyStr;
    ///
    /// let string = "Hello, world!";
    ///
    /// let non_empty = NonEmptyStr::from_str(string).unwrap();
    ///
    /// assert_eq!(non_empty.as_str(), string);
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.as_str_no_assert()
    }

    /// Returns the contained mutable string.
    #[must_use]
    pub const fn as_mut_str(&mut self) -> &mut str {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.as_mut_str_no_assert()
    }

    /// Checks if the string is empty. Always returns [`false`].
    ///
    /// This method is deprecated since the string is never empty.
    #[deprecated = "this string is never empty"]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns the length of the string in bytes as [`Size`].
    #[must_use]
    pub const fn len(&self) -> Size {
        let len = self.as_str().len();

        // SAFETY: the string is non-empty by construction, so its length is non-zero
        unsafe { Size::new_unchecked(len) }
    }

    /// Returns the underlying bytes of the string.
    #[must_use]
    pub const fn as_bytes(&self) -> &Bytes {
        self.as_str().as_bytes()
    }

    /// Returns the underlying mutable bytes of the string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes remain valid UTF-8.
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut Bytes {
        // SAFETY: the caller must ensure that the bytes remain valid UTF-8
        unsafe { self.as_mut_str().as_bytes_mut() }
    }

    /// Returns the underlying bytes of the string as [`NonEmptyBytes`].
    #[must_use]
    pub const fn as_non_empty_bytes(&self) -> &NonEmptyBytes {
        // SAFETY: the string is non-empty by construction, so are its bytes
        unsafe { NonEmptyBytes::from_slice_unchecked(self.as_bytes()) }
    }

    /// Returns the underlying mutable bytes of the string as [`NonEmptyBytes`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes remain valid UTF-8.
    pub const unsafe fn as_non_empty_bytes_mut(&mut self) -> &mut NonEmptyBytes {
        // SAFETY: the caller must ensure that the bytes remain valid UTF-8
        // moreover, the string is non-empty by construction, so are its bytes
        unsafe { NonEmptyBytes::from_mut_slice_unchecked(self.as_bytes_mut()) }
    }

    /// Converts given bytes to non-empty string, provided the bytes are non-empty and valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`MaybeEmptyUtf8Error`] if the bytes are either empty or not valid UTF-8.
    pub const fn from_utf8(bytes: &Bytes) -> Result<&Self, MaybeEmptyUtf8Error> {
        let non_empty = attempt!(
            map_error!(NonEmptyBytes::try_from_slice(bytes) => MaybeEmptyUtf8Error::Empty)
        );

        map_error!(Self::from_non_empty_utf8(non_empty) => MaybeEmptyUtf8Error::Utf8)
    }

    /// Converts given mutable bytes to non-empty string, provided the bytes
    /// are non-empty and valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`MaybeEmptyUtf8Error`] if the bytes are either empty or not valid UTF-8.
    pub const fn from_utf8_mut(bytes: &mut Bytes) -> Result<&mut Self, MaybeEmptyUtf8Error> {
        let non_empty = attempt!(
            map_error!(NonEmptyBytes::try_from_mut_slice(bytes) => MaybeEmptyUtf8Error::Empty)
        );

        map_error!(Self::from_non_empty_utf8_mut(non_empty) => MaybeEmptyUtf8Error::Utf8)
    }

    /// Converts given non-empty bytes to non-empty string, provided the bytes are valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyUtf8Error`] if the bytes are not valid UTF-8.
    pub const fn from_non_empty_utf8(
        non_empty: &NonEmptyBytes,
    ) -> Result<&Self, NonEmptyUtf8Error> {
        let string =
            attempt!(map_error!(str::from_utf8(non_empty.as_slice()) => NonEmptyUtf8Error::new));

        // SAFETY: the bytes are non-empty by construction, so is the resulting string
        Ok(unsafe { Self::from_str_unchecked(string) })
    }

    /// Converts given mutable non-empty bytes to non-empty string,
    /// provided the bytes are valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`NonEmptyUtf8Error`] if the bytes are not valid UTF-8.
    pub const fn from_non_empty_utf8_mut(
        non_empty: &mut NonEmptyBytes,
    ) -> Result<&mut Self, NonEmptyUtf8Error> {
        let string = attempt!(
            map_error!(str::from_utf8_mut(non_empty.as_mut_slice()) => NonEmptyUtf8Error::new)
        );

        // SAFETY: the bytes are non-empty by construction, so is the resulting string
        Ok(unsafe { Self::from_mut_str_unchecked(string) })
    }

    /// Converts given non-empty bytes to non-empty string without checking for UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid UTF-8.
    #[must_use]
    pub const unsafe fn from_non_empty_utf8_unchecked(non_empty: &NonEmptyBytes) -> &Self {
        // SAFETY: the caller must ensure that the bytes are valid UTF-8
        // moreover, the bytes are non-empty by construction
        unsafe { Self::from_utf8_unchecked(non_empty.as_slice()) }
    }

    /// Converts given mutable non-empty bytes to non-empty string
    /// without checking for UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid UTF-8.
    pub const unsafe fn from_non_empty_utf8_unchecked_mut(
        non_empty: &mut NonEmptyBytes,
    ) -> &mut Self {
        // SAFETY: the caller must ensure that the bytes are valid UTF-8
        // moreover, the bytes are non-empty by construction
        unsafe { Self::from_utf8_unchecked_mut(non_empty.as_mut_slice()) }
    }

    /// Converts given bytes to non-empty string without checking for emptiness or UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid UTF-8 and non-empty.
    #[must_use]
    pub const unsafe fn from_utf8_unchecked(bytes: &Bytes) -> &Self {
        // SAFETY: the caller must ensure that the bytes are valid UTF-8 and non-empty
        unsafe { Self::from_str_unchecked(str::from_utf8_unchecked(bytes)) }
    }

    /// Converts given mutable bytes to non-empty string
    /// without checking for emptiness or UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes are valid UTF-8 and non-empty.
    pub const unsafe fn from_utf8_unchecked_mut(bytes: &mut Bytes) -> &mut Self {
        // SAFETY: the caller must ensure that the bytes are valid UTF-8 and non-empty
        unsafe { Self::from_mut_str_unchecked(str::from_utf8_unchecked_mut(bytes)) }
    }

    /// Returns non-empty iterators over the bytes in this string.
    #[must_use]
    pub const fn bytes(&self) -> BytesIter<'_> {
        BytesIter::new(self)
    }

    /// Returns non-empty iterators over the characters in this string.
    #[must_use]
    pub const fn chars(&self) -> Chars<'_> {
        Chars::new(self)
    }

    /// Returns non-empty iterators over the characters and their positions in this string.
    #[must_use]
    pub const fn char_indices(&self) -> CharIndices<'_> {
        CharIndices::new(self)
    }

    /// Returns non-empty iterators over the UTF-16 encoding of this string.
    #[must_use]
    pub const fn encode_utf16(&self) -> EncodeUtf16<'_> {
        EncodeUtf16::new(self)
    }

    /// Returns non-empty iterators over the debug-escaped characters in this string.
    #[must_use]
    pub const fn escape_debug(&self) -> EscapeDebug<'_> {
        EscapeDebug::new(self)
    }

    /// Returns non-empty iterators over the default-escaped characters in this string.
    #[must_use]
    pub const fn escape_default(&self) -> EscapeDefault<'_> {
        EscapeDefault::new(self)
    }

    /// Returns non-empty iterators over the Unicode-escaped characters in this string.
    #[must_use]
    pub const fn escape_unicode(&self) -> EscapeUnicode<'_> {
        EscapeUnicode::new(self)
    }

    /// Represents iterators over the non-ASCII-whitespace non-empty substrings of this string.
    #[must_use]
    pub const fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_> {
        SplitAsciiWhitespace::new(self)
    }

    /// Represents iterators over the non-whitespace non-empty substrings of this string.
    #[must_use]
    pub const fn split_whitespace(&self) -> SplitWhitespace<'_> {
        SplitWhitespace::new(self)
    }

    /// Returns the raw pointer to the underlying bytes of the string.
    ///
    /// The caller must ensure that the pointer is never written to.
    #[must_use]
    pub const fn as_ptr(&self) -> RawBytes {
        self.as_str().as_ptr()
    }

    /// Returns the mutable pointer to the underlying bytes of the string.
    ///
    /// The caller must ensure that the string remains valid UTF-8.
    pub const fn as_mut_ptr(&mut self) -> MutBytes {
        self.as_mut_str().as_mut_ptr()
    }

    /// Checks that the provided index lies on the character boundary.
    ///
    /// The start and end of the string are considered to be boundaries.
    ///
    /// Returns [`false`] if the index is out of bounds.
    #[must_use]
    pub const fn is_char_boundary(&self, index: usize) -> bool {
        self.as_str().is_char_boundary(index)
    }

    /// Splits the string into two at the given non-zero index.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left string.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or not on character boundary.
    #[must_use]
    pub const fn split_at(&self, index: Size) -> (&Self, &str) {
        let (left, right) = self.as_str().split_at(index.get());

        // SAFETY: splitting at non-zero index guarantees non-emptiness of the left string
        let left_non_empty = unsafe { Self::from_str_unchecked(left) };

        (left_non_empty, right)
    }

    /// Splits the mutable string into two at the given non-zero index.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left string.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or not on character boundary.
    pub const fn split_at_mut(&mut self, index: Size) -> (&mut Self, &mut str) {
        let (left, right) = self.as_mut_str().split_at_mut(index.get());

        // SAFETY: splitting at non-zero index guarantees non-emptiness of the left string
        let left_non_empty = unsafe { Self::from_mut_str_unchecked(left) };

        (left_non_empty, right)
    }

    /// Splits the string into two at the given non-zero index, returning [`None`] if out of bounds
    /// or not on character boundary.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left string.
    #[must_use]
    pub const fn split_at_checked(&self, index: Size) -> Option<(&Self, &str)> {
        let Some((left, right)) = self.as_str().split_at_checked(index.get()) else {
            return None;
        };

        // SAFETY: splitting at non-zero index guarantees non-emptiness of the left string
        let left_non_empty = unsafe { Self::from_str_unchecked(left) };

        Some((left_non_empty, right))
    }

    /// Splits the mutable string into two at the given non-zero index,
    /// returning [`None`] if out of bounds or not on character boundary.
    ///
    /// The index has to be non-zero in order to guarantee non-emptiness of the left string.
    pub const fn split_at_mut_checked(&mut self, index: Size) -> Option<(&mut Self, &mut str)> {
        let Some((left, right)) = self.as_mut_str().split_at_mut_checked(index.get()) else {
            return None;
        };

        // SAFETY: splitting at non-zero index guarantees non-emptiness of the left string
        let left_non_empty = unsafe { Self::from_mut_str_unchecked(left) };

        Some((left_non_empty, right))
    }

    /// Parses this non-empty string into another type.
    ///
    /// [`parse_non_empty`] can be used with any type that implements the [`FromNonEmptyStr`] trait.
    ///
    /// # Errors
    ///
    /// Returns [`F::Error`] if parsing fails.
    ///
    /// [`parse_non_empty`]: Self::parse_non_empty
    /// [`F::Error`]: FromNonEmptyStr::Error
    pub fn parse_non_empty<F: FromNonEmptyStr>(&self) -> Result<F, F::Error> {
        F::from_non_empty_str(self)
    }

    /// Returns non-empty iterators over the lines of this string.
    #[must_use]
    pub const fn lines(&self) -> Lines<'_> {
        Lines::new(self)
    }

    /// Checks if all characters of the string are in the ASCII range.
    #[must_use]
    pub const fn is_ascii(&self) -> bool {
        self.as_str().is_ascii()
    }

    /// Checks that the two strings are ASCII case-insensitively equal.
    #[must_use]
    pub const fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }

    /// Converts the string to its ASCII uppercase equivalent in-place.
    pub const fn make_ascii_uppercase(&mut self) {
        self.as_mut_str().make_ascii_uppercase();
    }

    /// Converts the string to its ASCII lowercase equivalent in-place.
    pub const fn make_ascii_lowercase(&mut self) {
        self.as_mut_str().make_ascii_lowercase();
    }

    /// Returns new string with leading ASCII whitespace removed.
    #[must_use]
    pub const fn trim_ascii_start(&self) -> &str {
        self.as_str().trim_ascii_start()
    }

    /// Returns new string with trailing ASCII whitespace removed.
    #[must_use]
    pub const fn trim_ascii_end(&self) -> &str {
        self.as_str().trim_ascii_end()
    }

    /// Returns new string with leading and trailing ASCII whitespace removed.
    #[must_use]
    pub const fn trim_ascii(&self) -> &str {
        self.as_str().trim_ascii()
    }
}
