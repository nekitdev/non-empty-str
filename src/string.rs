//! Non-empty [`String`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "std")]
use std::{borrow::Cow, collections::TryReserveError};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::TryReserveError,
    string::{String, ToString},
};

use core::{
    borrow::{Borrow, BorrowMut},
    convert::Infallible,
    fmt,
    ops::{Add, AddAssign, Deref, DerefMut, RangeBounds},
    str::FromStr,
};

use non_empty_iter::{FromNonEmptyIterator, IntoNonEmptyIterator, NonEmptyIterator};
use non_empty_slice::{EmptyByteVec, EmptySlice, NonEmptyByteVec, NonEmptyBytes};
use non_zero_size::Size;

use thiserror::Error;

use crate::{
    boxed::{EmptyBoxedStr, NonEmptyBoxedStr},
    cow::NonEmptyCowStr,
    internal::{ByteVec, Bytes},
    str::{EmptyStr, FromNonEmptyStr, NonEmptyStr, NonEmptyUtf8Error},
};

/// The error message used when the string is empty.
pub const EMPTY_STRING: &str = "the string is empty";

/// Similar to [`EmptyStr`], but holds the empty string provided.
#[derive(Debug, Error)]
#[error("{EMPTY_STRING}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(code(non_empty_str::string), help("make sure the string is non-empty"))
)]
pub struct EmptyString {
    string: String,
}

impl EmptyString {
    // NOTE: this is private to prevent creating this error with non-empty strings
    pub(crate) const fn new(string: String) -> Self {
        Self { string }
    }

    /// Returns the contained empty string.
    #[must_use]
    pub fn get(self) -> String {
        self.string
    }

    /// Constructs [`Self`] from [`EmptyBoxedStr`].
    #[must_use]
    pub fn from_empty_boxed_str(empty: EmptyBoxedStr) -> Self {
        Self::new(empty.get().into_string())
    }

    /// Converts [`Self`] into [`EmptyBoxedStr`].
    #[must_use]
    pub fn into_empty_boxed_str(self) -> EmptyBoxedStr {
        EmptyBoxedStr::from_empty_string(self)
    }
}

/// Couples [`NonEmptyUtf8Error`] with the [`NonEmptyByteVec`] that is invalid UTF-8.
#[derive(Debug, Error)]
#[error("{error}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(
        code(non_empty_str::string::utf8),
        help("make sure the bytes are valid UTF-8")
    )
)]
pub struct FromNonEmptyUtf8Error {
    #[source]
    #[cfg_attr(feature = "diagnostics", diagnostic_source)]
    error: NonEmptyUtf8Error,
    bytes: NonEmptyByteVec,
}

impl FromNonEmptyUtf8Error {
    // NOTE: this is private to prevent creating this error with valid UTF-8 bytes
    pub(crate) const fn new(error: NonEmptyUtf8Error, bytes: NonEmptyByteVec) -> Self {
        Self { error, bytes }
    }

    /// Returns contained invalid UTF-8 bytes as [`NonEmptyBytes`].
    #[must_use]
    pub const fn as_non_empty_bytes(&self) -> &NonEmptyBytes {
        self.bytes.as_non_empty_slice()
    }

    /// Returns the contained non-empty bytes.
    #[must_use]
    pub fn into_non_empty_bytes(self) -> NonEmptyByteVec {
        self.bytes
    }

    /// Returns the underlying UTF-8 error.
    #[must_use]
    pub const fn non_empty_error(&self) -> NonEmptyUtf8Error {
        self.error
    }

    /// Recovers the underlying UTF-8 error and the non-empty bytes.
    ///
    /// This is the same as returning [`non_empty_error`] and [`into_non_empty_bytes`].
    ///
    /// [`non_empty_error`]: Self::non_empty_error
    /// [`into_non_empty_bytes`]: Self::into_non_empty_bytes
    #[must_use]
    pub fn recover(self) -> (NonEmptyUtf8Error, NonEmptyByteVec) {
        (self.non_empty_error(), self.into_non_empty_bytes())
    }
}

/// Represents errors returned when the provided byte vector is empty or invalid UTF-8.
#[derive(Debug, Error)]
#[error(transparent)]
#[cfg_attr(
    feature = "diagnostics",
    derive(miette::Diagnostic),
    diagnostic(transparent)
)]
pub enum FromMaybeEmptyUtf8Error {
    /// The received byte vector is empty.
    Empty(#[from] EmptyByteVec),
    /// The received byte vector is non-empty, but invalid UTF-8.
    Utf8(#[from] FromNonEmptyUtf8Error),
}

/// Represents non-empty [`String`] values.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NonEmptyString {
    inner: String,
}

impl Clone for NonEmptyString {
    fn clone(&self) -> Self {
        // SAFETY: the string is non-empty by construction
        unsafe { Self::new_unchecked(self.as_string().clone()) }
    }

    fn clone_from(&mut self, source: &Self) {
        // SAFETY: cloning from non-empty string can not make the string empty
        unsafe {
            self.as_mut_string().clone_from(source.as_string());
        }
    }
}

impl fmt::Write for NonEmptyString {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        // SAFETY: writing to non-empty string can not make the string empty
        unsafe { self.as_mut_string().write_str(string) }
    }

    fn write_char(&mut self, character: char) -> fmt::Result {
        // SAFETY: writing to non-empty string can not make the string empty
        unsafe { self.as_mut_string().write_char(character) }
    }

    fn write_fmt(&mut self, arguments: fmt::Arguments<'_>) -> fmt::Result {
        // SAFETY: writing to non-empty string can not make the string empty
        unsafe { self.as_mut_string().write_fmt(arguments) }
    }
}

impl fmt::Display for NonEmptyString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_string().fmt(formatter)
    }
}

impl FromStr for NonEmptyString {
    type Err = EmptyStr;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        NonEmptyStr::try_from_str(string).map(Self::from_non_empty_str)
    }
}

impl FromNonEmptyStr for NonEmptyString {
    type Error = Infallible;

    fn from_non_empty_str(string: &NonEmptyStr) -> Result<Self, Self::Error> {
        Ok(Self::from_non_empty_str(string))
    }
}

impl Borrow<NonEmptyStr> for NonEmptyString {
    fn borrow(&self) -> &NonEmptyStr {
        self.as_non_empty_str()
    }
}

impl BorrowMut<NonEmptyStr> for NonEmptyString {
    fn borrow_mut(&mut self) -> &mut NonEmptyStr {
        self.as_non_empty_mut_str()
    }
}

impl Borrow<str> for NonEmptyString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl BorrowMut<str> for NonEmptyString {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = EmptyString;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::new(string)
    }
}

impl From<NonEmptyString> for String {
    fn from(non_empty: NonEmptyString) -> Self {
        non_empty.into_string()
    }
}

impl From<&NonEmptyStr> for NonEmptyString {
    fn from(non_empty: &NonEmptyStr) -> Self {
        non_empty.to_non_empty_string()
    }
}

impl From<&mut NonEmptyStr> for NonEmptyString {
    fn from(non_empty: &mut NonEmptyStr) -> Self {
        non_empty.to_non_empty_string()
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = EmptyStr;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let non_empty_string: &NonEmptyStr = string.try_into()?;

        Ok(non_empty_string.into())
    }
}

impl TryFrom<&mut str> for NonEmptyString {
    type Error = EmptyStr;

    fn try_from(string: &mut str) -> Result<Self, Self::Error> {
        let non_empty_string: &mut NonEmptyStr = string.try_into()?;

        Ok(non_empty_string.into())
    }
}

impl From<char> for NonEmptyString {
    fn from(character: char) -> Self {
        Self::single(character)
    }
}

impl AsRef<Self> for NonEmptyString {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for NonEmptyString {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsRef<String> for NonEmptyString {
    fn as_ref(&self) -> &String {
        self.as_string()
    }
}

impl AsRef<NonEmptyStr> for NonEmptyString {
    fn as_ref(&self) -> &NonEmptyStr {
        self.as_non_empty_str()
    }
}

impl AsMut<NonEmptyStr> for NonEmptyString {
    fn as_mut(&mut self) -> &mut NonEmptyStr {
        self.as_non_empty_mut_str()
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsMut<str> for NonEmptyString {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl AsRef<Bytes> for NonEmptyString {
    fn as_ref(&self) -> &Bytes {
        self.as_bytes()
    }
}

impl Deref for NonEmptyString {
    type Target = NonEmptyStr;

    fn deref(&self) -> &Self::Target {
        self.as_non_empty_str()
    }
}

impl DerefMut for NonEmptyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_non_empty_mut_str()
    }
}

impl Add<&str> for NonEmptyString {
    type Output = Self;

    fn add(mut self, string: &str) -> Self::Output {
        self.push_str(string);

        self
    }
}

impl Add<&NonEmptyStr> for NonEmptyString {
    type Output = Self;

    fn add(mut self, non_empty: &NonEmptyStr) -> Self::Output {
        self.extend_from(non_empty);

        self
    }
}

impl AddAssign<&str> for NonEmptyString {
    fn add_assign(&mut self, string: &str) {
        self.push_str(string);
    }
}

impl AddAssign<&NonEmptyStr> for NonEmptyString {
    fn add_assign(&mut self, non_empty: &NonEmptyStr) {
        self.extend_from(non_empty);
    }
}

impl<'s> Extend<&'s str> for NonEmptyString {
    fn extend<I: IntoIterator<Item = &'s str>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend(iterable);
        }
    }
}

impl<'s> Extend<&'s NonEmptyStr> for NonEmptyString {
    fn extend<I: IntoIterator<Item = &'s NonEmptyStr>>(&mut self, iterable: I) {
        self.extend(iterable.into_iter().map(NonEmptyStr::as_str));
    }
}

impl<'c> Extend<&'c char> for NonEmptyString {
    fn extend<I: IntoIterator<Item = &'c char>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend(iterable);
        }
    }
}

impl Extend<Box<str>> for NonEmptyString {
    fn extend<I: IntoIterator<Item = Box<str>>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend(iterable);
        }
    }
}

impl Extend<NonEmptyBoxedStr> for NonEmptyString {
    fn extend<I: IntoIterator<Item = NonEmptyBoxedStr>>(&mut self, iterable: I) {
        self.extend(iterable.into_iter().map(NonEmptyStr::into_boxed_str));
    }
}

impl<'s> Extend<Cow<'s, str>> for NonEmptyString {
    fn extend<I: IntoIterator<Item = Cow<'s, str>>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend(iterable);
        }
    }
}

impl<'s> Extend<NonEmptyCowStr<'s>> for NonEmptyString {
    fn extend<I: IntoIterator<Item = Cow<'s, NonEmptyStr>>>(&mut self, iterable: I) {
        self.extend(iterable.into_iter().map(|non_empty| match non_empty {
            Cow::Borrowed(string) => Cow::Borrowed(string.as_str()),
            Cow::Owned(string) => Cow::Owned(string.into_string()),
        }));
    }
}

impl Extend<String> for NonEmptyString {
    fn extend<I: IntoIterator<Item = String>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend(iterable);
        }
    }
}

impl Extend<Self> for NonEmptyString {
    fn extend<I: IntoIterator<Item = Self>>(&mut self, iterable: I) {
        self.extend(iterable.into_iter().map(Self::into_string));
    }
}

impl Extend<char> for NonEmptyString {
    fn extend<I: IntoIterator<Item = char>>(&mut self, iterable: I) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend(iterable);
        }
    }
}

impl NonEmptyString {
    /// Constructs [`Self`], provided that the [`String`] is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyString`] if the string is empty.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_str::NonEmptyString;
    ///
    /// let message = NonEmptyString::new("Hello, world!".to_owned()).unwrap();
    /// ```
    ///
    /// Handling possible errors and recovering empty strings:
    ///
    /// ```
    /// use non_empty_str::NonEmptyString;
    ///
    /// let empty_owned = NonEmptyString::new(String::new()).unwrap_err();
    ///
    /// let empty = empty_owned.get();
    /// ```
    pub const fn new(string: String) -> Result<Self, EmptyString> {
        if string.is_empty() {
            return Err(EmptyString::new(string));
        }

        // SAFETY: the string is non-empty at this point
        Ok(unsafe { Self::new_unchecked(string) })
    }

    /// Constructs [`Self`] without checking if the string is non-empty.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the string is non-empty.
    #[must_use]
    pub const unsafe fn new_unchecked(inner: String) -> Self {
        debug_assert!(!inner.is_empty());

        Self { inner }
    }

    #[cfg(feature = "unsafe-assert")]
    const fn assert_non_empty(&self) {
        use core::hint::assert_unchecked;

        // SAFETY: the string is non-empty by construction
        unsafe {
            assert_unchecked(!self.as_string_no_assert().is_empty());
        }
    }

    const fn as_string_no_assert(&self) -> &String {
        &self.inner
    }

    const unsafe fn as_mut_string_no_assert(&mut self) -> &mut String {
        &mut self.inner
    }

    fn into_string_no_assert(self) -> String {
        self.inner
    }

    /// Constructs [`Self`] from [`NonEmptyStr`] via cloning.
    ///
    /// # Examples
    ///
    /// Basic snippet:
    ///
    /// ```
    /// use non_empty_str::{NonEmptyString, NonEmptyStr};
    ///
    /// let nekit = NonEmptyStr::from_str("nekit").unwrap();
    ///
    /// let owned = NonEmptyString::from_non_empty_str(nekit);
    /// ```
    #[must_use]
    pub fn from_non_empty_str(string: &NonEmptyStr) -> Self {
        // SAFETY: the string is non-empty by construction
        unsafe { Self::new_unchecked(string.as_str().to_owned()) }
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
        let len = self.as_string().len();

        // SAFETY: the string is non-empty by construction, so its length is non-zero
        unsafe { Size::new_unchecked(len) }
    }

    /// Returns the capacity of the string in bytes as [`Size`].
    #[must_use]
    pub const fn capacity(&self) -> Size {
        let capacity = self.as_string().capacity();

        // SAFETY: capacity is always non-zero for non-empty strings
        unsafe { Size::new_unchecked(capacity) }
    }

    /// Extracts the string slice containing the entire string.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.as_string().as_str()
    }

    /// Returns the mutable string slice containing the entire string.
    pub const fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: getting mutable slice can not make the string empty
        unsafe { self.as_mut_string().as_mut_str() }
    }

    /// Returns contained string reference as [`NonEmptyStr`].
    #[must_use]
    pub const fn as_non_empty_str(&self) -> &NonEmptyStr {
        // SAFETY: the string is non-empty by construction
        unsafe { NonEmptyStr::from_str_unchecked(self.as_str()) }
    }

    /// Returns contained mutable string reference as [`NonEmptyStr`].
    pub const fn as_non_empty_mut_str(&mut self) -> &mut NonEmptyStr {
        // SAFETY: the string is non-empty by construction
        unsafe { NonEmptyStr::from_mut_str_unchecked(self.as_mut_str()) }
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
        // SAFETY: getting mutable bytes can not make the string empty
        // moreover, the caller must ensure that the bytes remain valid UTF-8
        unsafe { self.as_mut_str().as_bytes_mut() }
    }

    /// Returns the underlying bytes of the string as [`NonEmptyBytes`].
    #[must_use]
    pub const fn as_non_empty_bytes(&self) -> &NonEmptyBytes {
        self.as_non_empty_str().as_non_empty_bytes()
    }

    /// Returns the underlying mutable bytes of the string as [`NonEmptyBytes`].
    ///
    /// # Safety
    ///
    /// The caller must ensure that the bytes remain valid UTF-8.
    pub const unsafe fn as_non_empty_bytes_mut(&mut self) -> &mut NonEmptyBytes {
        // SAFETY: the caller must ensure that the bytes remain valid UTF-8
        unsafe { self.as_non_empty_mut_str().as_non_empty_bytes_mut() }
    }

    /// Returns the contained string reference.
    #[must_use]
    pub const fn as_string(&self) -> &String {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.as_string_no_assert()
    }

    /// Similar to [`from_non_empty_utf8_lossy`], but accepts the possibility of empty bytes.
    ///
    /// # Errors
    ///
    /// Returns [`EmptySlice`] if the given bytes are empty.
    ///
    /// [`from_non_empty_utf8_lossy`]: Self::from_non_empty_utf8_lossy
    pub fn from_utf8_lossy(bytes: &Bytes) -> Result<NonEmptyCowStr<'_>, EmptySlice> {
        NonEmptyBytes::try_from_slice(bytes).map(Self::from_non_empty_utf8_lossy)
    }

    /// Similar to [`from_non_empty_utf8_lossy_owned`], but
    /// accepts the possibility of empty byte vectors.
    ///
    /// # Errors
    ///
    /// Returns [`EmptyByteVec`] if the given byte vector is empty.
    ///
    /// [`from_non_empty_utf8_lossy_owned`]: Self::from_non_empty_utf8_lossy_owned
    pub fn from_utf8_lossy_owned(bytes: ByteVec) -> Result<Self, EmptyByteVec> {
        NonEmptyByteVec::new(bytes).map(Self::from_non_empty_utf8_lossy_owned)
    }

    /// Converts the given [`NonEmptyBytes`] to non-empty string, including invalid characters.
    ///
    /// Any invalid UTF-8 sequences will be replaced with [`char::REPLACEMENT_CHARACTER`].
    ///
    /// This function returns [`NonEmptyCowStr<'_>`], since it may borrow the input bytes
    /// in case they are valid UTF-8, or allocate new non-empty string otherwise.
    #[must_use]
    pub fn from_non_empty_utf8_lossy(non_empty: &NonEmptyBytes) -> NonEmptyCowStr<'_> {
        match String::from_utf8_lossy(non_empty.as_slice()) {
            // SAFETY: passing non-empty bytes results in non-empty lossy string
            Cow::Owned(string) => Cow::Owned(unsafe { Self::new_unchecked(string) }),
            Cow::Borrowed(string) => {
                // SAFETY: bytes are valid and non-empty, so this is safe
                Cow::Borrowed(unsafe { NonEmptyStr::from_str_unchecked(string) })
            }
        }
    }

    /// Converts the given [`NonEmptyByteVec`] to non-empty string, including invalid characters.
    ///
    /// Any invalid UTF-8 sequences will be replaced with [`char::REPLACEMENT_CHARACTER`].
    ///
    /// This function does not guarantee reuse of the original byte vector allocation.
    #[must_use]
    pub fn from_non_empty_utf8_lossy_owned(non_empty: NonEmptyByteVec) -> Self {
        let cow = Self::from_non_empty_utf8_lossy(non_empty.as_non_empty_slice());

        if let Cow::Owned(string) = cow {
            string
        } else {
            // SAFETY: if `from_non_empty_utf8_lossy` returns `Cow::Borrowed`, it is valid UTF-8
            // moreover, the bytes are non-empty by construction, so this is safe
            unsafe { Self::from_non_empty_utf8_unchecked(non_empty) }
        }
    }

    /// Converts the given byte vector to non-empty string if it is non-empty and valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`FromMaybeEmptyUtf8Error`] if the byte vector is empty or invalid UTF-8.
    pub fn from_utf8(bytes: ByteVec) -> Result<Self, FromMaybeEmptyUtf8Error> {
        let non_empty = NonEmptyByteVec::new(bytes)?;

        let string = Self::from_non_empty_utf8(non_empty)?;

        Ok(string)
    }

    /// Converts the given [`NonEmptyByteVec`] to non-empty string if it is valid UTF-8.
    ///
    /// # Errors
    ///
    /// Returns [`FromNonEmptyUtf8Error`] if the byte vector is invalid UTF-8.
    pub fn from_non_empty_utf8(non_empty: NonEmptyByteVec) -> Result<Self, FromNonEmptyUtf8Error> {
        let string = String::from_utf8(non_empty.into_vec()).map_err(|error| {
            let non_empty_error = error.utf8_error().into();

            // SAFETY: reclaiming ownership of previously passed non-empty bytes is safe
            let non_empty = unsafe { NonEmptyByteVec::new_unchecked(error.into_bytes()) };

            FromNonEmptyUtf8Error::new(non_empty_error, non_empty)
        })?;

        // SAFETY: the bytes are non-empty by construction, so is the resulting string
        Ok(unsafe { Self::new_unchecked(string) })
    }

    /// Constructs [`Self`] from the given [`NonEmptyByteVec`] without checking for UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the non-empty byte vector is valid UTF-8.
    #[must_use]
    pub unsafe fn from_non_empty_utf8_unchecked(non_empty: NonEmptyByteVec) -> Self {
        // SAFETY: the caller must ensure that the bytes are valid UTF-8
        // moreover, the bytes are non-empty by construction, so is the resulting string
        unsafe { Self::from_utf8_unchecked(non_empty.into_vec()) }
    }

    /// Constructs [`Self`] from the given byte vector without
    /// checking for emptiness or UTF-8 validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the byte vector is non-empty and valid UTF-8.
    #[must_use]
    pub unsafe fn from_utf8_unchecked(bytes: ByteVec) -> Self {
        // SAFETY: the caller must ensure that the bytes are non-empty and valid UTF-8
        unsafe { Self::new_unchecked(String::from_utf8_unchecked(bytes)) }
    }

    /// Returns the contained mutable string reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the string remains non-empty.
    #[must_use]
    pub const unsafe fn as_mut_string(&mut self) -> &mut String {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        // SAFETY: the caller must ensure that the string remains non-empty
        unsafe { self.as_mut_string_no_assert() }
    }

    /// Returns the contained [`String`].
    #[must_use]
    pub fn into_string(self) -> String {
        #[cfg(feature = "unsafe-assert")]
        self.assert_non_empty();

        self.into_string_no_assert()
    }

    /// Converts [`Self`] into the underlying byte vector.
    #[must_use]
    pub fn into_bytes(self) -> ByteVec {
        self.into_string().into_bytes()
    }

    /// Converts [`Self`] into the underlying byte vector as [`NonEmptyByteVec`].
    #[must_use]
    pub fn into_non_empty_bytes(self) -> NonEmptyByteVec {
        // SAFETY: the string is non-empty by construction, so are its bytes
        unsafe { NonEmptyByteVec::new_unchecked(self.into_bytes()) }
    }

    /// Appends the given [`char`] to the end of this string.
    pub fn push(&mut self, character: char) {
        // SAFETY: pushing can not make the string empty
        unsafe {
            self.as_mut_string().push(character);
        }
    }

    /// Appends the given [`str`] onto the end of this string.
    pub fn push_str(&mut self, string: &str) {
        // SAFETY: pushing can not make the string empty
        unsafe {
            self.as_mut_string().push_str(string);
        }
    }

    /// Copies bytes from the given range to the end of the string.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds or not on character boundaries.
    pub fn extend_from_within<R: RangeBounds<usize>>(&mut self, source: R) {
        // SAFETY: extending can not make the string empty
        unsafe {
            self.as_mut_string().extend_from_within(source);
        }
    }

    /// Appends anything that can be converted to string onto the end of this string.
    pub fn extend_from<S: AsRef<str>>(&mut self, string: S) {
        self.push_str(string.as_ref());
    }

    /// Reserves capacity for at least `additional` more bytes to be added.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// This method can over-allocate to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    pub fn reserve(&mut self, additional: Size) {
        // SAFETY: reserving can not make the string empty
        unsafe {
            self.as_mut_string().reserve(additional.get());
        }
    }

    /// Reserves the minimum capacity for exactly `additional` more values to be added.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// Unlike [`reserve`], this method will not deliberately over-allocate
    /// to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    ///
    /// [`reserve`]: Self::reserve
    pub fn reserve_exact(&mut self, additional: Size) {
        // SAFETY: reserving can not make the string empty
        unsafe {
            self.as_mut_string().reserve_exact(additional.get());
        }
    }

    /// Tries to reserve capacity for at least `additional` more bytes to be added.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// This method can over-allocate to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// Returns [`TryReserveError`] if the allocation fails or capacity overflows.
    pub fn try_reserve(&mut self, additional: Size) -> Result<(), TryReserveError> {
        // SAFETY: reserving can not make the string empty
        unsafe { self.as_mut_string().try_reserve(additional.get()) }
    }

    /// Tries to reserve the minimum capacity for exactly `additional` more bytes to be added.
    ///
    /// Note that the additional capacity is required to be non-zero via [`Size`].
    ///
    /// Unlike [`try_reserve`], this method will not deliberately over-allocate
    /// to speculatively avoid frequent reallocations.
    ///
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Errors
    ///
    /// Returns [`TryReserveError`] if the allocation fails or capacity overflows.
    ///
    /// [`try_reserve`]: Self::try_reserve
    pub fn try_reserve_exact(&mut self, additional: Size) -> Result<(), TryReserveError> {
        // SAFETY: reserving can not make the string empty
        unsafe { self.as_mut_string().try_reserve_exact(additional.get()) }
    }

    /// Shrinks the capacity of the string as much as possible.
    pub fn shrink_to_fit(&mut self) {
        // SAFETY: shrinking can not make the string empty
        unsafe {
            self.as_mut_string().shrink_to_fit();
        }
    }

    /// Shrinks the capacity of the string to the specified amount.
    ///
    /// The capacity will remain at least as large as both the length and the supplied amount.
    ///
    /// Does nothing if the current capacity is less than or equal to the specified amount.
    pub fn shrink_to(&mut self, capacity: Size) {
        // SAFETY: shrinking can not make the string empty
        unsafe {
            self.as_mut_string().shrink_to(capacity.get());
        }
    }

    /// Shortens this string to the specified non-zero length.
    ///
    /// Does nothing if `new` is greater than or equal to the string's [`len`].
    ///
    /// [`len`]: Self::len
    pub fn truncate(&mut self, new: Size) {
        // SAFETY: truncating to non-zero length can not make the string empty
        unsafe {
            self.as_mut_string().truncate(new.get());
        }
    }

    /// Checks whether the string is almost empty, meaning it only contains one character.
    #[must_use]
    pub fn next_empty(&self) -> bool {
        let (character, _) = self.chars().consume();

        self.len().get() - character.len_utf8() == 0
    }

    /// The negated version of [`next_empty`].
    ///
    /// [`next_empty`]: Self::next_empty
    #[must_use]
    pub fn next_non_empty(&self) -> bool {
        !self.next_empty()
    }

    /// Removes the last character from the string and returns it,
    /// or [`None`] if the string would become empty.
    pub fn pop(&mut self) -> Option<char> {
        self.next_non_empty()
            // SAFETY: popping only when the string would remain non-empty
            .then(|| unsafe { self.as_mut_string().pop() })
            .flatten()
    }

    /// Consumes and leaks the string, returning the mutable reference of its contents.
    #[must_use]
    pub fn leak<'a>(self) -> &'a mut str {
        self.into_string().leak()
    }

    /// Similar to [`leak`], but returns [`NonEmptyStr`].
    ///
    /// [`leak`]: Self::leak
    #[must_use]
    pub fn leak_non_empty<'a>(self) -> &'a mut NonEmptyStr {
        // SAFETY: the string is non-empty by construction, so is the leaked string
        unsafe { NonEmptyStr::from_mut_str_unchecked(self.leak()) }
    }

    /// Inserts the given character at the specified index,
    /// shifting all bytes after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or not on character boundary.
    pub fn insert(&mut self, index: usize, character: char) {
        // SAFETY: inserting can not make the string empty
        unsafe {
            self.as_mut_string().insert(index, character);
        }
    }

    /// Inserts the given string at the specified index, shifting all bytes after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or not on character boundary.
    pub fn insert_str(&mut self, index: usize, string: &str) {
        // SAFETY: inserting can not make the string empty
        unsafe {
            self.as_mut_string().insert_str(index, string);
        }
    }

    /// Inserts anything that can be converted to string at the specified index,
    /// shifting all bytes after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or not on character boundary.
    pub fn insert_from<S: AsRef<str>>(&mut self, index: usize, string: S) {
        self.insert_str(index, string.as_ref());
    }

    /// Removes and returns the character at the given index within the string,
    /// shifting all bytes after it to the left.
    ///
    /// Returns [`None`] if the string would become empty.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds or not on character boundary.
    pub fn remove(&mut self, index: usize) -> Option<char> {
        self.next_non_empty()
            // SAFETY: removing only when the string would remain non-empty
            .then(|| unsafe { self.as_mut_string().remove(index) })
    }

    /// Splits the string into two at the given non-zero index.
    ///
    /// The index has to be non-zero to guaratee that the string would remain non-empty.
    ///
    /// # Panics
    ///
    /// Panics if the provided index is out of bounds or not on character boundary.
    pub fn split_off(&mut self, at: Size) -> String {
        // SAFETY: splitting at non-zero index can not make the string empty
        unsafe { self.as_mut_string().split_off(at.get()) }
    }
}

impl ToOwned for NonEmptyStr {
    type Owned = NonEmptyString;

    fn to_owned(&self) -> Self::Owned {
        self.to_non_empty_string()
    }
}

impl NonEmptyStr {
    /// Converts [`Self`] to [`NonEmptyString`] via cloning.
    #[must_use]
    pub fn to_non_empty_string(&self) -> NonEmptyString {
        NonEmptyString::from_non_empty_str(self)
    }

    /// Converts [`Self`] to [`NonEmptyByteVec`] via cloning.
    #[must_use]
    pub fn to_non_empty_bytes(&self) -> NonEmptyByteVec {
        self.to_non_empty_string().into_non_empty_bytes()
    }

    /// Converts this string to its lowercase equivalent.
    #[must_use]
    pub fn to_lowercase(&self) -> String {
        self.as_str().to_lowercase()
    }

    /// Converts this string to its uppercase equivalent.
    #[must_use]
    pub fn to_uppercase(&self) -> String {
        self.as_str().to_uppercase()
    }

    /// Creates [`NonEmptyString`] by repeating this string certain number of times.
    ///
    /// Note that the count is non-zero in order to guarantee that
    /// the resulting string is non-empty.
    ///
    /// # Panics
    ///
    /// Panics on capacity overflow.
    #[must_use]
    pub fn repeat(&self, count: Size) -> NonEmptyString {
        let non_empty = self.as_str().repeat(count.get());

        // SAFETY: repeating non-empty string non-zero times results in non-empty string
        unsafe { NonEmptyString::new_unchecked(non_empty) }
    }

    /// Converts this string to its lowercase equivalent as [`NonEmptyString`].
    #[must_use]
    pub fn to_non_empty_lowercase(&self) -> NonEmptyString {
        // SAFETY: converting non-empty string to lowercase gives non-empty output
        unsafe { NonEmptyString::new_unchecked(self.to_lowercase()) }
    }

    /// Converts this string to its uppercase equivalent as [`NonEmptyString`].
    #[must_use]
    pub fn to_non_empty_uppercase(&self) -> NonEmptyString {
        // SAFETY: converting non-empty string to uppercase gives non-empty output
        unsafe { NonEmptyString::new_unchecked(self.to_uppercase()) }
    }
}

impl NonEmptyString {
    /// Constructs [`Self`] containing single provided character.
    #[must_use]
    pub fn single(character: char) -> Self {
        // SAFETY: non-empty construction
        unsafe { Self::new_unchecked(character.to_string()) }
    }

    /// Constructs [`Self`] with the specified capacity in bytes, pushing the provided character.
    #[must_use]
    pub fn with_capacity_and_char(capacity: Size, character: char) -> Self {
        let mut string = String::with_capacity(capacity.get());

        string.push(character);

        // SAFETY: non-empty construction
        unsafe { Self::new_unchecked(string) }
    }
}

impl FromNonEmptyIterator<char> for NonEmptyString {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = char>>(iterable: I) -> Self {
        let (character, iterator) = iterable.into_non_empty_iter().consume();

        let mut output = Self::single(character);

        output.extend(iterator);

        output
    }
}

impl<'c> FromNonEmptyIterator<&'c char> for NonEmptyString {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = &'c char>>(iterable: I) -> Self {
        let (&character, iterator) = iterable.into_non_empty_iter().consume();

        let mut output = Self::single(character);

        output.extend(iterator);

        output
    }
}

impl<'s> FromNonEmptyIterator<&'s NonEmptyStr> for NonEmptyString {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = &'s NonEmptyStr>>(iterable: I) -> Self {
        let (non_empty, iterator) = iterable.into_non_empty_iter().consume();

        let mut output = Self::from_non_empty_str(non_empty);

        output.extend(iterator);

        output
    }
}

impl FromNonEmptyIterator<Self> for NonEmptyString {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = Self>>(iterable: I) -> Self {
        let (mut output, iterator) = iterable.into_non_empty_iter().consume();

        output.extend(iterator);

        output
    }
}

impl FromNonEmptyIterator<NonEmptyBoxedStr> for NonEmptyString {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = NonEmptyBoxedStr>>(iterable: I) -> Self {
        let (non_empty, iterator) = iterable.into_non_empty_iter().consume();

        let mut output = Self::from_non_empty_boxed_str(non_empty);

        output.extend(iterator);

        output
    }
}

impl<'s> FromNonEmptyIterator<NonEmptyCowStr<'s>> for NonEmptyString {
    fn from_non_empty_iter<I: IntoNonEmptyIterator<Item = NonEmptyCowStr<'s>>>(
        iterable: I,
    ) -> Self {
        let (non_empty, iterator) = iterable.into_non_empty_iter().consume();

        let mut output = non_empty.into_owned();

        output.extend(iterator);

        output
    }
}
