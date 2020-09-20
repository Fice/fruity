//! [Core Foundation](https://developer.apple.com/documentation/corefoundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_foundation`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_foundation")]

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}

mod cf_type;
mod cmp;

pub use cf_type::*;
pub use cmp::*;

/// A constant that indicates that a search operation did not succeed in
/// locating the target value.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfnotfound).
#[allow(non_upper_case_globals)]
pub const kCFNotFound: CFIndex = -1;

/// Type for hash codes returned by
/// [`CFType::hash`](struct.CFType.html#method.hash).
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfhashcode).
pub type CFHashCode = usize;

/// An integer type used as an array index, count, size, or length.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfindex).
pub type CFIndex = isize;

pub(crate) type Boolean = std::os::raw::c_uchar;
