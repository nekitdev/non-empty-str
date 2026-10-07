pub mod import {
    pub use core::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        option::Option,
        result::Result,
    };
}

macro_rules! total {
    ($type: ty => $method: ident) => {
        impl $crate::internals::import::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.$method() == other.$method()
            }
        }

        impl $crate::internals::import::Eq for $type {}

        impl $crate::internals::import::PartialOrd for $type {
            fn partial_cmp(
                &self,
                other: &Self,
            ) -> $crate::internals::import::Option<$crate::internals::import::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl $crate::internals::import::Ord for $type {
            fn cmp(&self, other: &Self) -> $crate::internals::import::Ordering {
                self.$method().cmp(other.$method())
            }
        }
    };
}

pub(crate) use total;

macro_rules! partial {
    (
        $one_type: ty $(=> $one_method: ident)?,
        $two_type: ty $(=> $two_method: ident)? $(,)?
    ) => {
        impl $crate::internals::import::PartialEq<$two_type> for $one_type {
            fn eq(&self, other: &$two_type) -> bool {
                self$(.$one_method())? == other$(.$two_method())?
            }
        }

        impl $crate::internals::import::PartialEq<$one_type> for $two_type {
            fn eq(&self, other: &$one_type) -> bool {
                self$(.$two_method())? == other$(.$one_method())?
            }
        }

        impl $crate::internals::import::PartialOrd<$two_type> for $one_type {
            fn partial_cmp(
                &self, other: &$two_type
            ) -> $crate::internals::import::Option<$crate::internals::import::Ordering> {
                self$(.$one_method())?.partial_cmp(other$(.$two_method())?)
            }
        }

        impl $crate::internals::import::PartialOrd<$one_type> for $two_type {
            fn partial_cmp(
                &self, other: &$one_type
            ) -> $crate::internals::import::Option<$crate::internals::import::Ordering> {
                self$(.$two_method())?.partial_cmp(other$(.$one_method())?)
            }
        }
    };
}

pub(crate) use partial;

macro_rules! attempt {
    ($result: expr) => {
        match $result {
            $crate::internals::import::Result::Ok(value) => value,
            $crate::internals::import::Result::Err(err) => {
                return $crate::internals::import::Result::Err(err)
            }
        }
    };
}

pub(crate) use attempt;

macro_rules! map_error {
    ($result: expr => $map: expr) => {
        match $result {
            $crate::internals::import::Result::Ok(value) => {
                $crate::internals::import::Result::Ok(value)
            }
            $crate::internals::import::Result::Err(error) => {
                $crate::internals::import::Result::Err($map(error))
            }
        }
    };
}

pub(crate) use map_error;
