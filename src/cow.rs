//! Non-empty [`Cow<'_, str>`](Cow).

cfg_select! {
    feature = "std" => {
        use std::borrow::Cow;
    }
    feature = "alloc" => {
        use alloc::borrow::Cow;
    }
    _ => {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::{boxed::NonEmptyBoxedStr, str::NonEmptyStr, string::NonEmptyString};

/// Represents non-empty clone-on-write strings, [`Cow<'s, NonEmptyStr>`](Cow).
pub type NonEmptyCowStr<'s> = Cow<'s, NonEmptyStr>;

impl From<NonEmptyCowStr<'_>> for NonEmptyString {
    fn from(non_empty: NonEmptyCowStr<'_>) -> Self {
        non_empty.into_owned()
    }
}

impl From<NonEmptyCowStr<'_>> for NonEmptyBoxedStr {
    fn from(non_empty: NonEmptyCowStr<'_>) -> Self {
        non_empty.into_owned().into_non_empty_boxed_str()
    }
}

impl<'s> From<&'s NonEmptyStr> for NonEmptyCowStr<'s> {
    fn from(non_empty: &'s NonEmptyStr) -> Self {
        Self::Borrowed(non_empty)
    }
}

impl From<NonEmptyString> for NonEmptyCowStr<'_> {
    fn from(non_empty: NonEmptyString) -> Self {
        Self::Owned(non_empty)
    }
}

impl<'s> From<&'s NonEmptyString> for NonEmptyCowStr<'s> {
    fn from(non_empty: &'s NonEmptyString) -> Self {
        Self::Borrowed(non_empty.as_non_empty_str())
    }
}
