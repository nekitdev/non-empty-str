#[cfg(not(feature = "ownership"))]
compile_error!("expected `ownership` to be enabled");

#[cfg(any(feature = "std", feature = "alloc"))]
use ownership::impl_identity;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::{boxed::NonEmptyBoxedStr, string::NonEmptyString};

#[cfg(any(feature = "std", feature = "alloc"))]
impl_identity!(NonEmptyBoxedStr);

#[cfg(any(feature = "std", feature = "alloc"))]
impl_identity!(NonEmptyString);
