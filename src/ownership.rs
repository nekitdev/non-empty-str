#[cfg(not(feature = "ownership"))]
compile_error!("expected `ownership` to be enabled");

#[cfg(any(feature = "std", feature = "alloc"))]
mod std_or_alloc {
    use ownership::impl_identity;

    use crate::{boxed::NonEmptyBoxedStr, string::NonEmptyString};

    impl_identity!(NonEmptyBoxedStr);
    impl_identity!(NonEmptyString);
}
