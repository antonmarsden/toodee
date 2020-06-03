# TooDee Release Notes

## Changes

### Version 0.1.3

- Fixed divide by zero bug in size_hint() when there were zero columns
- Improved documentation of `TooDee<T>`, and added some examples

### Version 0.1.2

- Added custom `Default` implementation for `TooDee<T>` to allow construction when `T`
  does not implement `Default`.
- Added `IntoIterator` implementation for `TooDee<T>`
