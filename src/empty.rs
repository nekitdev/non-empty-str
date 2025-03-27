//! Emptiness errors.

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

use thiserror::Error;

/// The message for errors returned on empty strings.
pub const EMPTY: &str = "the string is empty";

/// Represents errors that occur when the input string is empty.
#[derive(Debug, Error)]
#[error("the string is empty")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(non_empty_str::empty), help("make sure the string is non-empty"))
)]
pub struct Empty;
