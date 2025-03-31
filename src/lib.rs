//! Upgrading to static lifetimes.
//!
//! This crate provides the [`IntoStatic`] trait, which allows upgrading all lifetimes to `'static`.
//!
//! [`IntoStatic`] is implemented for [`Option<T>`] and [`Result<T, E>`], provided
//! `T` and `E` are also [`IntoStatic`].
//!
//! The trait is also implemented for arrays [`[T; N]`](array) and tuples containing to 12 items,
//! provided each type is [`IntoStatic`].
//!
//! When the `std` or `alloc` feature is enabled, [`IntoStatic`] is also
//! implemented for `Cow<'_, T>`, yielding `Cow<'static, T>` (provided `T: 'static` is satisfied).

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "std")]
use std::borrow::{Cow, ToOwned};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::borrow::{Cow, ToOwned};

/// Upgrading to `'static` lifetimes.
pub trait IntoStatic {
    /// The type with `'static` lifetimes.
    type Static: 'static;

    /// Upgrades [`Self`] to [`Self::Static`].
    fn into_static(self) -> Self::Static;
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: ToOwned + ?Sized + 'static> IntoStatic for Cow<'_, T> {
    type Static = Cow<'static, T>;

    fn into_static(self) -> Self::Static {
        Self::Static::Owned(self.into_owned())
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
