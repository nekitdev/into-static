# Changelog

<!-- changelogging: start -->

## [0.2.1](https://github.com/nekitdev/into-static/tree/v0.2.1) (2025-04-20)

### Features

- Implemented `IntoStatic` for `Vec<T>`, provided `T` is `IntoStatic`.

## [0.2.0](https://github.com/nekitdev/into-static/tree/v0.2.0) (2025-03-31)

### Features

- The `IntoStatic` trait is now implemented for `[T; N]` where `T: IntoStatic`.

- The `IntoStatic` trait is now implemented for non-empty tuples containing up to 12 items,
  provided each type in the tuple implements `IntoStatic`.

## [0.1.0](https://github.com/nekitdev/into-static/tree/v0.1.0) (2025-03-25)

No significant changes.
