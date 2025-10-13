//! Macros for creating non-empty strings.

/// Constructs [`NonEmptyStr`] from the given string, panicking if the it is empty.
///
/// # Examples
///
/// Simple usage:
///
/// ```
/// use non_empty_str::non_empty_str;
///
/// let nekit = non_empty_str!("nekit");
/// ```
///
/// Panicking if the string is empty:
///
/// ```should_panic
/// use non_empty_str::non_empty_str;
///
/// let never = non_empty_str!("");
/// ```
///
/// Compilation failure when in `const` contexts:
///
/// ```compile_fail
/// use non_empty_str::non_empty_str;
///
/// let never = const { non_empty_str!("") };
/// ```
///
/// [`NonEmptyStr`]: crate::str::NonEmptyStr
#[macro_export]
macro_rules! non_empty_str {
    ($string: expr) => {
        $crate::str::NonEmptyStr::from_str($string).expect($crate::str::EMPTY_STR)
    };
}

/// Similar to [`non_empty_str!`] but for `const` contexts.
///
/// Note that the provided expression must be const-evaluatable, else the compilation will fail.
///
/// # Examples
///
/// ```
/// use non_empty_str::const_non_empty_str;
///
/// let message = const_non_empty_str!("Hello, world!");
/// ```
///
/// Failing compilation on empty strings:
///
/// ```compile_fail
/// use non_empty_str::const_non_empty_str;
///
/// let never = const_non_empty_str!("");
/// ```
#[macro_export]
macro_rules! const_non_empty_str {
    ($string: expr) => {
        const { $crate::non_empty_str!($string) }
    };
}
