//! Checking for emptiness.

use const_macros::const_early;

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

use thiserror::Error;

/// Represents errors that occur when the input string is empty.
#[derive(Debug, Error)]
#[error("received an empty string")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(non_empty_str::empty), help("make sure the string is non-empty"))
)]
pub struct Empty;

/// Checks whether the given string is non-empty.
///
/// # Errors
///
/// Returns [`Empty`] if the string is empty.
pub const fn check_str(string: &str) -> Result<(), Empty> {
    const_early!(string.is_empty() => Empty);

    Ok(())
}

/// Similar to [`check_str`], but is generic over the input type.
///
/// # Errors
///
/// Returns [`Empty`] if the input is empty.
pub fn check<S: AsRef<str>>(string: S) -> Result<(), Empty> {
    check_str(string.as_ref())
}
