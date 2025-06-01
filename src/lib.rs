//! Upgrading to static lifetimes.
//!
//! This crate provides the [`IntoStatic`] trait, which allows upgrading all lifetimes to `'static`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::Cow;
    } else if #[cfg(feature = "alloc")] {
        extern crate alloc;

        use alloc::{borrow::Cow, vec::Vec};
    }
}

/// Upgrading to `'static` lifetimes.
pub trait IntoStatic {
    /// The type with `'static` lifetimes.
    type Static: IntoStatic<Static = Self::Static> + 'static;

    /// Upgrades [`Self`] to [`Self::Static`].
    fn into_static(self) -> Self::Static;
}

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        impl IntoStatic for Cow<'_, str> {
            type Static = Cow<'static, str>;

            fn into_static(self) -> Self::Static {
                Self::Static::Owned(self.into_owned())
            }
        }

        impl IntoStatic for Cow<'_, [u8]> {
            type Static = Cow<'static, [u8]>;

            fn into_static(self) -> Self::Static {
                Self::Static::Owned(self.into_owned())
            }
        }

        impl<T: Clone + IntoStatic<Static: Clone>> IntoStatic for Cow<'_, [T]> {
            type Static = Cow<'static, [T::Static]>;

            fn into_static(self) -> Self::Static {
                Self::Static::Owned(self.into_owned().into_static())
            }
        }

        impl<T: IntoStatic> IntoStatic for Vec<T> {
            type Static = Vec<T::Static>;

            fn into_static(self) -> Self::Static {
                self.into_iter().map(IntoStatic::into_static).collect()
            }
        }
    }
}

impl<T: IntoStatic> IntoStatic for Option<T> {
    type Static = Option<T::Static>;

    fn into_static(self) -> Self::Static {
        self.map(IntoStatic::into_static)
    }
}

impl<T: IntoStatic, E: IntoStatic> IntoStatic for Result<T, E> {
    type Static = Result<T::Static, E::Static>;

    fn into_static(self) -> Self::Static {
        self.map(IntoStatic::into_static)
            .map_err(IntoStatic::into_static)
    }
}

impl<T: IntoStatic, const N: usize> IntoStatic for [T; N] {
    type Static = [T::Static; N];

    fn into_static(self) -> Self::Static {
        self.map(IntoStatic::into_static)
    }
}

macro_rules! impl_tuple {
    ($($name: ident: $type: ident),+) => {
        impl<$($type: $crate::IntoStatic),+> $crate::IntoStatic for ($($type,)+) {
            type Static = ($($type::Static,)+);

            fn into_static(self) -> Self::Static {
                let ($($name,)+) = self;

                ($($name.into_static(),)+)
            }
        }
    }
}

impl_tuple!(a: A);
impl_tuple!(a: A, b: B);
impl_tuple!(a: A, b: B, c: C);
impl_tuple!(a: A, b: B, c: C, d: D);
impl_tuple!(a: A, b: B, c: C, d: D, e: E);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);
