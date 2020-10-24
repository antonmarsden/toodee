# TooDee Release Notes

## Changes

### Unreleased

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
