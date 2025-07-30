//! Macros for creating non-empty strings in `const` contexts.

/// Constantly constructs [`Str`] from the given string, failing compilation if the string is empty.
///
/// # Examples
///
/// Simple usage:
///
/// ```
/// use non_empty_str::const_str;
///
/// let nekit = const_str!("nekit");
/// ```
///
/// Compilation failure if the string is empty:
///
/// ```compile_fail
/// use non_empty_str::const_str;
///
/// let empty = const_str!("");
/// ```
///
/// [`Str`]: crate::str::Str
#[macro_export]
macro_rules! const_str {
    ($string: expr) => {
        const { $crate::str::Str::from_str($string).expect($crate::str::EMPTY) }
    };
}
