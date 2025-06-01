# Changelog

<!-- changelogging: start -->

## [0.5.0](https://github.com/nekitdev/into-static/tree/v0.5.0) (2025-06-01)

### Features

- Implemented `IntoStatic` for `Cow<'_, [u8]>`.

### Internal

- `cfg-if` is now used in order to reduce boilerplate.

## [0.4.0](https://github.com/nekitdev/into-static/tree/v0.4.0) (2025-04-30)

### Changes

- The general implementation of `IntoStatic` for `Cow<'_, T>` was removed in favor of
  more sensible implementations:

  - `Cow<'_, str>` returning `Cow<'static, str>`;
  - `Cow<'_, [T]>` returning `Cow<'static, [T::Static]>` provided `T: Clone + IntoStatic<Static:
  Clone>`.

## [0.3.0](https://github.com/nekitdev/into-static/tree/v0.3.0) (2025-04-24)

### Changes

- Restricted `IntoStatic` to be idempotent.
  The additional `IntoStatic<Static = Self::Static>` bound to `Static` of `IntoStatic` was added.
  ([#2](https://github.com/nekitdev/into-static/pull/2))

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
