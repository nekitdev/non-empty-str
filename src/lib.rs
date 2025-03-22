//! Non-empty strings.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(any(feature = "alloc", feature = "std"))]
pub mod cow;

pub mod empty;
pub mod str;

#[macro_use]
pub mod macros;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use cow::{CowStr, StaticCowStr};

pub use empty::Empty;
pub use str::{StaticStr, Str};
