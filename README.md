![Build Status](https://github.com/antonmarsden/toodee/workflows/Test/badge.svg)

# toodee

TooDee is a lightweight and high performance two-dimensional wrapper around a slice.

## Core features

- Raw access to the underlying slice via `data()` and `data_mut()`.
- Creation of performant two-dimensional subsets using `view()` and `view_mut()`.
- Get/set specific cells using indexing, e.g., `toodee[row][col] = val`.
- Index by row (i.e., row major) to get/set row slices, e.g., `toodee[row]`.
- Iteration, any which way - `rows()`, `rows_mut()`, `col()`, `col_mut()`, `cells()`, `cells_mut()`.
- `no_std` compliant.
- Can create a new TooDeeView or TooDeeViewMut directly from a &[T] or &mut [T] (respectively).

## Extras

- Translate (scroll/slide) operations with wrap and fill variants.

## TODO

- Pathfinding algorithms?
- Think about masks/clipping.
- Robustness - check for zero rows/cols (div/mod, etc.).
- Range operators that return a `TooDeeView[Mut]`?
- Implement `nth()` and `nth_back()` for FlattenExact.
- Implement `try_fold()` and `try_rfold()` for FlattenExact once the `Try` trait is stable.
- Mirror/reflection functionality.
- More documentation, with examples.
- `tiles(..)` and `tiles_mut()`?

## Motivation

Similar libraries do exist, but they lacked either performance, flexibility, or functionality. 

Here's a small feature comparison chart:

<table>
  <tr><th></th><th>Structs supported</th><th>Growable?</th><th>Mutable views?</th><th>Raw data access?</th><th>Iterate over row slices?</th><th>Safe/checked access?</th><th>Notes</th></tr>
  <tr><td>toodee::TooDee</td><td>Anything</td><td>No</td><td>Yes</td><td>Yes</td><td>Yes</td><td>No</td></tr>
  <tr><td>image::ImageBuffer</td><td><code>image::Pixel</code></td><td>No</td><td>No</td><td>Yes</td><td>No</td><td>No</td><td>Good for image processing - see the <code>imageproc</code> crate.</tr>
  <tr><td>image::SubImage</td><td><code>image::Pixel</code></td><td>No</td><td>Yes</td><td>No</td><td>No</td><td>No</td></tr>
  <tr><td>grid::Grid</td><td><code>Clone</code></td><td>Yes</td><td>No</td><td>Yes</td><td>Yes</td><td>Yes</td></tr>
  <tr><td>array2d::Array2D</td><td><code>Clone</code></td><td>No</td><td>No</td><td>No</td><td>No</td><td>Yes</td></tr>
  <tr><td>imgref::Img</td><td>Anything</td><td>No</td><td>Yes</td><td>Yes</td><td>No</td><td>No</td><td>The closest equivalent to <code>TooDee</code> that I could find.</td></tr>
</table>

## Goals
 
- High performance and good flexibility by providing a core data model that:
    - Uses a one-dimensional slice to store the two-dimensional array elements. Useful for frame buffers and
      other scenarios where you want raw access to the underlying data (use `data()` or `data_mut()`).
    - Provides views (subsets of the array) which have good performance.
    - Can access each row as a slice using either `self[row]` or a `rows()` iterator. Useful for bulk or vector-style manipulation.
    - Has useful iterators, e.g., `rows()`, `col()`, `cells()`
- Suitable for use in image processing, but not restricted to this problem domain.
- Provide solid implementations of non-trivial 2D operations.

## Non-goals
 
- GPU integration

## Limitations

- The underlying data is fixed in size.
- The array dimensions are immutable. This may change if `transpose()` is ever implemented
- Views are not nested for the time being, The only impact is that the `bounds()` of a view
  are not always relative to its parent.

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
