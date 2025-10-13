#[cfg(not(feature = "serde"))]
compile_error!("expected `serde` to be enabled");

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::string::String;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::str::NonEmptyStr;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::{boxed::NonEmptyBoxedStr, string::NonEmptyString};

impl Serialize for NonEmptyStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de: 's, 's> Deserialize<'de> for &'s NonEmptyStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = <&str>::deserialize(deserializer)?;

        let non_empty = string.try_into().map_err(D::Error::custom)?;

        Ok(non_empty)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Serialize for NonEmptyString {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_string().serialize(serializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for NonEmptyString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;

        Self::new(string).map_err(D::Error::custom)
    }
}

// NOTE: `Serialize` is implemented for `Box<T>`, provided `T: Serialize`
// `NonEmptyStr` is `Serialize`, therefore `NonEmptyBoxedStr` is as well

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for NonEmptyBoxedStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let non_empty_string = NonEmptyString::deserialize(deserializer)?;

        Ok(non_empty_string.into_non_empty_boxed_str())
    }
}
