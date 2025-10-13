#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

pub type RawBytes = *const u8;
pub type MutBytes = *mut u8;

pub type Byte = u8;
pub type Bytes = [u8];

#[cfg(any(feature = "std", feature = "alloc"))]
pub type ByteVec = Vec<u8>;

pub mod import {
    pub use core::result::Result;
}

macro_rules! attempt {
    ($result: expr) => {
        match $result {
            $crate::internal::import::Result::Ok(value) => value,
            $crate::internal::import::Result::Err(err) => {
                return $crate::internal::import::Result::Err(err)
            }
        }
    };
}

pub(crate) use attempt;

macro_rules! map_error {
    ($result: expr => $map: expr) => {
        match $result {
            $crate::internal::import::Result::Ok(value) => {
                $crate::internal::import::Result::Ok(value)
            }
            $crate::internal::import::Result::Err(error) => {
                $crate::internal::import::Result::Err($map(error))
            }
        }
    };
}

pub(crate) use map_error;
