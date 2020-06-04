/*!

A lightweight two-dimensional wrapper around a `Vec`.

*/

#![cfg_attr(not(any(test, doctest)), no_std)]

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

mod iter;
mod view;
mod ops;
mod toodee;
mod flattenexact;
mod copy;

#[cfg(feature = "sort")] mod sort;
#[cfg(feature = "sort")] mod tests_sort;
#[cfg(feature = "sort")] pub use crate::sort::*;

#[cfg(feature = "translate")] mod translate;
#[cfg(feature = "translate")] mod tests_translate;
#[cfg(feature = "translate")] pub use crate::translate::*;

mod tests;
mod tests_iter;
mod tests_copy;

pub use crate::iter::*;
pub use crate::view::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::flattenexact::*;
pub use crate::copy::*;

