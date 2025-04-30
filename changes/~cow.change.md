The general implementation of `IntoStatic` for `Cow<'_, T>` was removed in favor of
more sensible implementations:

- `Cow<'_, str>` returning `Cow<'static, str>`;
- `Cow<'_, [T]>` returning `Cow<'static, [T::Static]>` provided `T: Clone + IntoStatic<Static: Clone>`.
