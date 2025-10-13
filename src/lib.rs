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

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod string;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub use string::{EmptyString, FromMaybeEmptyUtf8Error, FromNonEmptyUtf8Error, NonEmptyString};

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod boxed;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub use boxed::{EmptyBoxedStr, NonEmptyBoxedStr};

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod cow;

#[doc(inline)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub use cow::NonEmptyCowStr;

#[cfg(feature = "ownership")]
pub(crate) mod ownership;

#[cfg(feature = "serde")]
pub(crate) mod serde;

pub(crate) mod internal;
