/*!

A lightweight two-dimensional wrapper around a slice.

*/

#![cfg_attr(not(any(test, doctest)), no_std)]

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod iter;
mod view;
mod ops;
mod toodee;
mod flattenexact;

#[cfg(feature = "slide")]
mod slide;

mod tests;
mod tests_iter;
#[cfg(feature = "slide")]
mod tests_slide;

pub use crate::iter::*;
pub use crate::view::*;
pub use crate::ops::*;
pub use crate::toodee::*;
pub use crate::flattenexact::*;

#[cfg(feature = "slide")]
pub use crate::slide::*;
