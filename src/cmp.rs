use core::ops::Deref;

use crate::{
    internals::{partial, total},
    str::NonEmptyStr,
};

// NonEmptyStr

total!(NonEmptyStr => as_str);

partial!(NonEmptyStr => as_str, str);
partial!(NonEmptyStr, &str => deref);

partial!(NonEmptyStr, &NonEmptyStr => deref);

#[cfg(any(feature = "std", feature = "alloc"))]
mod std_or_alloc {
    cfg_select! {
        feature = "std" => {
            use std::borrow::Cow;
        }
        feature = "alloc" => {
            use alloc::{borrow::Cow, boxed::Box, string::String};
        }
        _ => {
            compile_error!("expected either `std` or `alloc` to be enabled");
        }
    }

    use core::ops::Deref;

    use crate::{
        boxed::NonEmptyBoxedStr,
        cow::NonEmptyCowStr,
        internals::{partial, total},
        str::NonEmptyStr,
        string::NonEmptyString,
    };

    // NonEmptyStr (continued)

    partial!(NonEmptyStr, Box<str> => as_ref);
    partial!(NonEmptyStr, Cow<'_, str> => as_ref);
    partial!(NonEmptyStr, String => as_str);

    partial!(NonEmptyStr, NonEmptyCowStr<'_> => as_ref);

    // NonEmptyString

    total!(NonEmptyString => as_string);

    partial!(NonEmptyString => as_string, String);
    partial!(NonEmptyString => as_non_empty_str, NonEmptyStr);
    partial!(NonEmptyString, &NonEmptyStr => deref);
    partial!(NonEmptyString => as_str, str);
    partial!(NonEmptyString, &str => deref);
    partial!(NonEmptyString, NonEmptyCowStr<'_> => as_ref);
    partial!(NonEmptyString, Cow<'_, str> => as_ref);
    partial!(NonEmptyString, NonEmptyBoxedStr => as_ref);
    partial!(NonEmptyString, Box<str> => as_ref);

    // NonEmptyBoxedStr

    partial!(NonEmptyBoxedStr => as_ref, NonEmptyStr);
    partial!(NonEmptyBoxedStr, &NonEmptyStr => deref);
    partial!(NonEmptyBoxedStr => as_ref, str);
    partial!(NonEmptyBoxedStr, &str => deref);
    partial!(NonEmptyBoxedStr => as_ref, Box<str>);
    partial!(NonEmptyBoxedStr, NonEmptyCowStr<'_> => as_ref);
    partial!(NonEmptyBoxedStr, Cow<'_, str> => as_ref);
    partial!(NonEmptyBoxedStr => as_ref, String);
}
