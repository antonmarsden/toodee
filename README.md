![Build Status](https://github.com/antonmarsden/toodee/workflows/Test/badge.svg)
[![Current Version](https://img.shields.io/crates/v/toodee.svg)](https://crates.io/crates/toodee)
[![Documentation](https://docs.rs/toodee/badge.svg)](https://docs.rs/toodee)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/toodee.svg)](#license)

# toodee

TooDee is a lightweight and high performance two-dimensional wrapper around a `Vec`.

## Core features

- Raw access to the underlying vector's slice via `data()` and `data_mut()`.
- Creation of performant two-dimensional subsets using `view()` and `view_mut()`.
- Get/set specific cells using indexing, e.g., `toodee[row][col] = val`.
- Index by row (i.e., row major) to get/set row slices, e.g., `toodee[row]`.
- Iteration, any which way - `rows()`, `rows_mut()`, `col()`, `col_mut()`, `cells()`, `cells_mut()`.
- `#[no_std]` compliant.
- Can create a new `TooDeeView`  from a `&[T]`, or a `TooDeeViewMut`  from  a `&mut [T]`.
- `insert_col()`, `remove_col()`, `insert_row()`, and `remove_row()` implementations with good performance.

## Extras

- `translate_with_wrap()` (scroll), `flip_rows()`, and `flip_cols()` operations.
- `sort_by_row()` and `sort_by_col()` operations.

## TODO

- More documentation, with examples.
- `tiles(..)` and `tiles_mut()`?
- Pathfinding algorithms?

## Motivation

Similar libraries do exist, but they lacked either performance, flexibility, or functionality. 

Here's a small feature comparison chart:

<table>
  <tr><th></th><th>Storage order</th><th>Structs supported</th><th>Growable?</th><th>Mutable views?</th><th>Raw data access?</th><th>Iterate over row slices?</th><th>Safe/checked access?</th><th>Notes</th></tr>
  <tr><td><code>toodee::TooDee</code></td><td>Row-major</td><td>Anything</td><td>Yes</td><td>Yes</td><td>Yes</td><td>Yes</td><td>No</td><td></td></tr>
  <tr><td><code>image::ImageBuffer</code></td><td>Row-major</td><td><code>image::Pixel</code></td><td>No</td><td>No</td><td>Yes</td><td>No</td><td>No</td><td>Good for image processing - see the <code>imageproc</code> crate.</tr>
  <tr><td><code>image::SubImage</code></td><td>Row-major</td><td><code>image::Pixel</code></td><td>No</td><td>Yes</td><td>No</td><td>No</td><td>No</td><td></td></tr>
  <tr><td><code>grid::Grid</code></td><td>Row-major</td><td><code>Clone</code></td><td>Yes</td><td>No</td><td>Yes</td><td>No</td><td>Yes</td><td>Similar to <code>TooDee</code>, but not as functionally rich.</td></tr>
  <tr><td><code>array2d::Array2D</code></td><td>Row-major</td><td><code>Clone</code></td><td>No</td><td>No</td><td>No</td><td>No</td><td>Yes</td><td></td></tr>
  <tr><td><code>imgref::Img</code></td><td>Row-major</td><td>Anything</td><td>No</td><td>Yes</td><td>Yes</td><td>No</td><td>No</td><td></td></tr>
  <tr><td><code>nalgebra::Matrix</code></td><td>Column-major</td><td><code>Scalar</code></td><td>Yes</td><td>Yes</td><td>Yes</td><td>No</td><td>No</td><td>Use this for vector/matrix math.</td></tr>
</table>

## Goals
 
- High performance and good flexibility, with the constraint of using a 1-D vector.
- Suitable for use in image processing, but not restricted to this problem domain.
- Provide solid implementations of non-trivial 2D operations.

## Non-goals
 
- GPU integration

## Limitations

- Views are not nested for the time being, The only impact is that the `bounds()` of a view
  are always relative to the underlying `TooDee` array.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
