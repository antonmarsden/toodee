![Build Status](https://github.com/antonmarsden/toodee/workflows/Test/badge.svg)

# toodee

A lightweight two-dimensional wrapper around a slice.

TooDee provides a rich and high performance API at the expense of safety, i.e., most methods do not return an `Option`, and
will panic if you attempt to access data that is out of bounds. This approach is similar to that of `std::vec::Vec`,
e.g., `vec[bad_idx]`. Safer methods similar to `Vec::get()` could be added, but would degrade performance if used
excessively. I recommend doing your own bounds checks when writing algorithms.

## Features

- Raw access to the underlying slice via `data()` and `data_mut()`.
- Index by row (i.e., row major) to get/set row slices.
- Get/set specific cells using indexing, e.g., `toodee[row][col]` = val.
- Access to 2D subsets using `view()` and `view_mut()`.
- Slide (scroll/translate) operations with wrap and fill variants.

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
  <tr><th></th><th>Structs supported</th><th>Growable?</th><th>Mutable views?</th><th>Raw data access?</th><th>Iterate over row slices?</th><th>Safe/checked access?</tr>
  <tr><td>toodee::TooDee</td><td>Anything</td><td>No</td><td>Yes</td><td>Yes</td><td>Yes</td><td>No</td></tr>
  <tr><td>image::ImageBuffer</td><td><code>image::Pixel</code></td><td>No</td><td>No</td><td>Yes</td><td>No</td><td>No</td></tr>
  <tr><td>image::SubImage</td><td><code>image::Pixel</code></td><td>No</td><td>Yes</td><td>No</td><td>No</td><td>No</td></tr>
  <tr><td>grid::Grid</td><td><code>Clone</code></td><td>Yes</td><td>No</td><td>Yes</td><td>Yes</td><td>Yes</td></tr>
  <tr><td>array2d::Array2D</td><td><code>Clone</code></td><td>No</td><td>No</td><td>No</td><td>No</td><td>Yes</td></tr>
</table>

## Goals
 
- High performance and good flexibility by providing a core data model that:
    - Uses a one-dimensional array to store the two-dimensional array elements. Useful for frame buffers and
      other scenarios where you want raw access to the underlying data (use `data()` or `data_mut()`).
    - Provides views (subsets of the array) which have good performance.
    - Can access each row as a slice using either `self[row]` or a `rows()` iterator. Useful for bulk or vector-style manipulation.
    - Has useful iterators, e.g., `rows()`, `col()`, `cells()`
- Suitable for use in image processing, but not restricted to this problem domain.
- Provide solid implementations of non-trivial 2D operations.

## Non-goals
 
- GPU integration

## Limitations

- The underlying slice is fixed in size.
- The array dimensions are immutable. This may change if `transpose()` is ever implemented
- Views are not nested, i.e., a view created from a view points to the
   underlying TooDee object. This means that a view's `bounds()` are relative
   to the underlying TooDee array.

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
