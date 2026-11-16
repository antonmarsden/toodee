# TooDee Release Notes

## Changes

### Unreleased

- **Breaking:** `TooDeeViewMut` now implements `IntoIterator` by value, yielding `&mut T` and consuming the view, like `IntoIterator` for `&mut [T]`. This also makes `for c in view_mut` work. Previously, `view_mut.into_iter()` resolved to `(&view_mut).into_iter()`, yielding `&T` and leaving the view usable, so code that relied on that (e.g. `view_mut.into_iter().copied()`) no longer compiles. Use `view_mut.cells()` instead.
- Added `From<[[T; COLS]; ROWS]>` for `TooDee`, so a `TooDee` can be built directly from a 2D array ([PR #36](https://github.com/antonmarsden/toodee/pull/36)) - thanks Jeremy Gordon ([@migmoog](https://github.com/migmoog)).
- Added consuming iterator methods to views: `TooDeeView::into_rows()`, `into_col()` and `into_cells()`, and `TooDeeViewMut::into_rows()`, `into_col()` and `into_cells()`. The returned iterators borrow the underlying data rather than the view, so they can be returned from functions. `TooDeeView` also implements `IntoIterator` by value, yielding `&T`. Based on a proposal by [@cedtwo](https://github.com/cedtwo) in [PR #27](https://github.com/antonmarsden/toodee/pull/27).
- `TooDeeView` is now `Copy` and `Clone` for any `T`, not only when `T: Copy`.
- Added `TooDeeViewMut::reborrow()`, which returns a shorter-lived mutable view of the same area (like `&mut *slice`), and `TooDeeViewMut::as_view()`, which returns a read-only view of it. `TooDeeViewMut` can't be `Clone`, so use `reborrow()` to pass a mutable view by value, or to call a consuming method, and keep using it afterwards.
- Relaxed the lifetimes of `IntoIterator` for `&TooDeeView`, `&TooDeeViewMut` and `&mut TooDeeViewMut`, so the reference no longer needs to live as long as the borrowed data. `IntoIterator` for `&TooDeeView<'a, T>` now yields `&'a T`, so the items can outlive the borrow of the view.

### Version 0.6.3

- Fixed *_col_key sort, thanks Rohan-GRH.

### Version 0.6.2
- Fixed aliasing issues in unsafe swap and sorting operations, aliasing in `DrainCol`, and invalid backing-slice ranges for empty views ([#37](https://github.com/antonmarsden/toodee/issues/37), reported by [@zhouxt1](https://github.com/zhouxt1)).
- Made `insert_row` and `insert_col` panic-safe when input iterators panic ([#31](https://github.com/antonmarsden/toodee/issues/31)); the fix was informed by [@mvanhorn](https://github.com/mvanhorn)'s [PR #32](https://github.com/antonmarsden/toodee/pull/32).
- `TooDee::new()` now panics if one dimension is zero but the other is non-zero, as documented and as `init()` and `from_vec()` already did ([PR #33](https://github.com/antonmarsden/toodee/pull/33)) - thanks Sai Asish Y.
- `remove_col()` now removes the column immediately and returns its elements in a `DrainCol` backed by a new `Vec`, so forgetting the `DrainCol` leaves the `TooDee` valid ([PR #38](https://github.com/antonmarsden/toodee/pull/38)).
- `insert_col()` and `push_col()` no longer require the iterator to implement `DoubleEndedIterator`.
- Removed an unused generic parameter from `sort_row_ord()`, `sort_unstable_row_ord()` and `sort_col_ord()`, which previously could not be called without a turbofish.

### Version 0.6.1
- Fixed integer overflow in `nth_back()` - thanks Ruihan Gong.

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
