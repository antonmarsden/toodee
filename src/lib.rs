/*!

A lightweight two-dimensional wrapper around a `Vec`.

*/

#![cfg_attr(not(any(test, doctest)), no_std)]

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2021_incompatible_closure_captures)]
#![warn(rust_2021_prelude_collisions)]
#![warn(rust_2021_incompatible_or_patterns)]
#![warn(rust_2021_prefixes_incompatible_syntax)]
#![warn(missing_debug_implementations)]

extern crate alloc;

mod iter;
mod view;
mod ops;
mod toodee;
mod flattenexact;

#[cfg(feature = "sort")] mod sort;
#[cfg(feature = "sort")] mod tests_sort;
#[cfg(feature = "sort")] pub use crate::sort::*;

#[cfg(feature = "translate")] mod translate;
#[cfg(feature = "translate")] mod tests_translate;
#[cfg(feature = "translate")] pub use crate::translate::*;

#[cfg(feature = "transpose")] mod transpose;
#[cfg(feature = "transpose")] mod tests_transpose;
#[cfg(feature = "transpose")] pub use crate::transpose::*;

#[cfg(feature = "copy")] mod copy;
#[cfg(feature = "copy")] mod tests_copy;
#[cfg(feature = "copy")] pub use crate::copy::*;

#[cfg(feature = "serde")] mod serde;
#[cfg(feature = "serde")] mod tests_serde;

mod tests;
mod tests_iter;

pub use crate::iter::*;
pub use crate::view::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::flattenexact::*;

