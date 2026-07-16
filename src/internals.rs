#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

pub type Byte = u8;

pub type RawBytes = *const Byte;
pub type MutBytes = *mut Byte;

pub type Bytes = [Byte];

#[cfg(any(feature = "std", feature = "alloc"))]
pub type ByteVec = Vec<Byte>;

pub mod import {
    pub use core::result::Result;
}

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
