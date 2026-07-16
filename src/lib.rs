//! Non-empty strings.

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub mod macros;

pub mod str;

pub mod iter;

#[doc(inline)]
pub use str::{EmptyStr, FromNonEmptyStr, MaybeEmptyUtf8Error, NonEmptyStr, NonEmptyUtf8Error};

cfg_select! {
    any(feature = "std", feature = "alloc") => {
        pub mod boxed;
        pub mod cow;
        pub mod string;

        #[doc(inline)]
        pub use boxed::{EmptyBoxedStr, NonEmptyBoxedStr};

        #[doc(inline)]
        pub use cow::NonEmptyCowStr;

        #[doc(inline)]
        pub use string::{
            EmptyString, FromMaybeEmptyUtf8Error, FromNonEmptyUtf8Error, NonEmptyString
        };
    }
    _ => {}
}

pub(crate) mod cmp;

#[cfg(feature = "ownership")]
pub(crate) mod ownership;

#[cfg(feature = "serde")]
pub(crate) mod serde;

pub(crate) mod internals;
