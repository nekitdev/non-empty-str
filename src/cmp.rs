use core::cmp::Ordering;

use crate::str::NonEmptyStr;

// NonEmptyStr <-> NonEmptyStr

impl PartialEq for NonEmptyStr {
    fn eq(&self, other: &Self) -> bool {
        self == other.as_str()
    }
}

impl Eq for NonEmptyStr {}

impl PartialOrd for NonEmptyStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NonEmptyStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

// NonEmptyStr <-> str

impl PartialEq<str> for NonEmptyStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialOrd<str> for NonEmptyStr {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl PartialEq<NonEmptyStr> for str {
    fn eq(&self, other: &NonEmptyStr) -> bool {
        self == other.as_str()
    }
}

impl PartialOrd<NonEmptyStr> for str {
    fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
        self.partial_cmp(other.as_str())
    }
}

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

    use core::cmp::Ordering;

    use crate::{
        boxed::NonEmptyBoxedStr, cow::NonEmptyCowStr, str::NonEmptyStr, string::NonEmptyString,
    };

    // NonEmptyStr <-> BoxedStr

    impl PartialEq<Box<str>> for NonEmptyStr {
        fn eq(&self, other: &Box<str>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<Box<str>> for NonEmptyStr {
        fn partial_cmp(&self, other: &Box<str>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyStr> for Box<str> {
        fn eq(&self, other: &NonEmptyStr) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyStr> for Box<str> {
        fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyStr <-> CowStr

    impl PartialEq<Cow<'_, str>> for NonEmptyStr {
        fn eq(&self, other: &Cow<'_, str>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<Cow<'_, str>> for NonEmptyStr {
        fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyStr> for Cow<'_, str> {
        fn eq(&self, other: &NonEmptyStr) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyStr> for Cow<'_, str> {
        fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyStr <-> String

    impl PartialEq<String> for NonEmptyStr {
        fn eq(&self, other: &String) -> bool {
            self == other.as_str()
        }
    }

    impl PartialOrd<String> for NonEmptyStr {
        fn partial_cmp(&self, other: &String) -> Option<Ordering> {
            self.partial_cmp(other.as_str())
        }
    }

    impl PartialEq<NonEmptyStr> for String {
        fn eq(&self, other: &NonEmptyStr) -> bool {
            self.as_str() == other
        }
    }

    impl PartialOrd<NonEmptyStr> for String {
        fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
            self.as_str().partial_cmp(other)
        }
    }

    // NonEmptyStr <-> NonEmptyCowStr

    impl PartialEq<NonEmptyCowStr<'_>> for NonEmptyStr {
        fn eq(&self, other: &NonEmptyCowStr<'_>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyCowStr<'_>> for NonEmptyStr {
        fn partial_cmp(&self, other: &NonEmptyCowStr<'_>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyStr> for NonEmptyCowStr<'_> {
        fn eq(&self, other: &NonEmptyStr) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyStr> for NonEmptyCowStr<'_> {
        fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyString <-> NonEmptyString

    impl PartialEq for NonEmptyString {
        fn eq(&self, other: &Self) -> bool {
            self == other.as_string()
        }
    }

    impl Eq for NonEmptyString {}

    impl PartialOrd for NonEmptyString {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for NonEmptyString {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_string().cmp(other.as_string())
        }
    }

    // NonEmptyString <-> String

    impl PartialEq<String> for NonEmptyString {
        fn eq(&self, other: &String) -> bool {
            self.as_string() == other
        }
    }

    impl PartialOrd<String> for NonEmptyString {
        fn partial_cmp(&self, other: &String) -> Option<Ordering> {
            self.as_string().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyString> for String {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self == other.as_string()
        }
    }

    impl PartialOrd<NonEmptyString> for String {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.partial_cmp(other.as_string())
        }
    }

    // NonEmptyString <-> NonEmptyStr

    impl PartialEq<NonEmptyStr> for NonEmptyString {
        fn eq(&self, other: &NonEmptyStr) -> bool {
            self.as_non_empty_str() == other
        }
    }

    impl PartialOrd<NonEmptyStr> for NonEmptyString {
        fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
            self.as_non_empty_str().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyString> for NonEmptyStr {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self == other.as_non_empty_str()
        }
    }

    impl PartialOrd<NonEmptyString> for NonEmptyStr {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.partial_cmp(other.as_non_empty_str())
        }
    }

    // NonEmptyString <-> str

    impl PartialEq<str> for NonEmptyString {
        fn eq(&self, other: &str) -> bool {
            self.as_str() == other
        }
    }

    impl PartialOrd<str> for NonEmptyString {
        fn partial_cmp(&self, other: &str) -> Option<Ordering> {
            self.as_str().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyString> for str {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self == other.as_str()
        }
    }

    impl PartialOrd<NonEmptyString> for str {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.partial_cmp(other.as_str())
        }
    }

    // NonEmptyString <-> NonEmptyCowStr

    impl PartialEq<NonEmptyCowStr<'_>> for NonEmptyString {
        fn eq(&self, other: &NonEmptyCowStr<'_>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyCowStr<'_>> for NonEmptyString {
        fn partial_cmp(&self, other: &NonEmptyCowStr<'_>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyString> for NonEmptyCowStr<'_> {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyString> for NonEmptyCowStr<'_> {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyString <-> CowStr

    impl PartialEq<Cow<'_, str>> for NonEmptyString {
        fn eq(&self, other: &Cow<'_, str>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<Cow<'_, str>> for NonEmptyString {
        fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyString> for Cow<'_, str> {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyString> for Cow<'_, str> {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyString <-> NonEmptyBoxedStr

    impl PartialEq<NonEmptyBoxedStr> for NonEmptyString {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for NonEmptyString {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyString> for NonEmptyBoxedStr {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyString> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyString <-> BoxedStr

    impl PartialEq<Box<str>> for NonEmptyString {
        fn eq(&self, other: &Box<str>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<Box<str>> for NonEmptyString {
        fn partial_cmp(&self, other: &Box<str>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyString> for Box<str> {
        fn eq(&self, other: &NonEmptyString) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyString> for Box<str> {
        fn partial_cmp(&self, other: &NonEmptyString) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyBoxedStr <-> NonEmptyStr

    impl PartialEq<NonEmptyStr> for NonEmptyBoxedStr {
        fn eq(&self, other: &NonEmptyStr) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyStr> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &NonEmptyStr) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyBoxedStr> for NonEmptyStr {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for NonEmptyStr {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    // NonEmptyBoxedStr <-> str

    impl PartialEq<str> for NonEmptyBoxedStr {
        fn eq(&self, other: &str) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<str> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &str) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyBoxedStr> for str {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for str {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    // NonEmptyBoxedStr <-> BoxedStr

    impl PartialEq<Box<str>> for NonEmptyBoxedStr {
        fn eq(&self, other: &Box<str>) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<Box<str>> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &Box<str>) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyBoxedStr> for Box<str> {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for Box<str> {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    // NonEmptyBoxedStr <-> NonEmptyCowStr

    impl PartialEq<NonEmptyCowStr<'_>> for NonEmptyBoxedStr {
        fn eq(&self, other: &NonEmptyCowStr<'_>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyCowStr<'_>> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &NonEmptyCowStr<'_>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyBoxedStr> for NonEmptyCowStr<'_> {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for NonEmptyCowStr<'_> {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyBoxedStr <-> CowStr

    impl PartialEq<Cow<'_, str>> for NonEmptyBoxedStr {
        fn eq(&self, other: &Cow<'_, str>) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<Cow<'_, str>> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &Cow<'_, str>) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }

    impl PartialEq<NonEmptyBoxedStr> for Cow<'_, str> {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for Cow<'_, str> {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    // NonEmptyBoxedStr <-> String

    impl PartialEq<String> for NonEmptyBoxedStr {
        fn eq(&self, other: &String) -> bool {
            self.as_ref() == other
        }
    }

    impl PartialOrd<String> for NonEmptyBoxedStr {
        fn partial_cmp(&self, other: &String) -> Option<Ordering> {
            self.as_ref().partial_cmp(other)
        }
    }

    impl PartialEq<NonEmptyBoxedStr> for String {
        fn eq(&self, other: &NonEmptyBoxedStr) -> bool {
            self == other.as_ref()
        }
    }

    impl PartialOrd<NonEmptyBoxedStr> for String {
        fn partial_cmp(&self, other: &NonEmptyBoxedStr) -> Option<Ordering> {
            self.partial_cmp(other.as_ref())
        }
    }
}
