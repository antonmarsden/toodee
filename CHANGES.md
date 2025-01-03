# TooDee Release Notes

## Changes

### Version 0.6.0
- Fixed heap buffer overflow in DrainCol destructor - thanks George Androutsopoulos.

### Version 0.5.0

- Comparison (vs) benchmarks added
- _Breaking change_: Removed the `bounds()` function due to additional complexity and limited value.

### Version 0.4.2

- `serde` integration

### Version 0.4.1

- Check for multiply overflow when creating `TooDee` or `TooDeeView` objects.
- Clippy fixes

### Version 0.4.0

- Now using the 2021 Rust edition
- Consistency improvement: only increment rows (for `insert_row`) and cols (for `insert_col`) if data was actually inserted.

### Version 0.3.0

- Now requires **rustc >= 1.50** to build
- Fix for issue #13 (Panic Safety and soundness issue in `insert_row`)
- Fixed panic safety and soundness issues in `insert_col`, and reduced the number of times it calls `ptr::copy`
- Uses the stabilised `slice_fill` feature
- Uses the stabilised `bool::then` function in some scenarios
- Replaced some `Into` implementations with `From` implementations
- Minor test case tidy up

### Version 0.2.4

- The `Col` and `ColMut` iterators are now indexable.

### Version 0.2.3

- Introduced `get_unchecked()` and `get_unchecked_mut()` functions for faster (unsafe) access

### Version 0.2.2

- Use of slice.get_unchecked() and slice.get_unchecked_mut() where possible for overall performance improvement
- Performance improvements to `TooDeeOpsMut.swap_rows()` and the `SortOps`
- Minor code style improvements
- Small documentation updates

### Version 0.2.1

- Re-organised fields within structs (data/vec fields specified first)
- `TooDee` now has derived `Hash`, `Eq`, and `PartialEq` implementations
- Added `Into<Box<[T]>>` and `AsRef<Vec<T>>` implementations

### Version 0.2.0

- Lots more documentation, with examples.
- Various performance improvements by leveraging unsafe (lookup cell by `Coordinate`, `insert_row()`, `insert_col()`)
- Performance enhancements and potentially breaking changes to `remove_col()` and `remove_row()` (minor alterations to method signatures)

### Version 0.1.4

- Improved performance of `SortOps` functions, particularly `sort_by_row` and variants.
- More documentation improvements
- Added `Index<Coordinate>` and `IndexMut<Coordinate>` implementations

### Version 0.1.3

- Fixed divide by zero bug in size_hint() when there were zero columns
- Improved documentation of `TooDee<T>`, and added some examples

### Version 0.1.2

- Added custom `Default` implementation for `TooDee<T>` to allow construction when `T`
  does not implement `Default`.
- Added `IntoIterator` implementation for `TooDee<T>`
