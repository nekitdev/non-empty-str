//! Macros used for constructing non-empty strings.

/// Constructs [`Str`] from the given string, panicking if it is empty.
///
/// [`Str`]: crate::str::Str
#[macro_export]
macro_rules! const_str {
    ($string: expr) => {
        $crate::str::Str::new_ok($string).expect($crate::empty::EMPTY)
    };
}

/// Similar to [`const_str`], but constructs borrowed [`CowStr`].
///
/// [`CowStr`]: crate::cow::CowStr
#[cfg(any(feature = "alloc", feature = "std"))]
#[macro_export]
macro_rules! const_borrowed_str {
    ($string: expr) => {
        $crate::cow::CowStr::borrowed_ok($string).expect($crate::empty::EMPTY)
    };
}
