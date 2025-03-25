//! Upgrading to static lifetimes.
//!
//! This crate provides the [`IntoStatic`] trait, which allows upgrading all lifetimes to `'static`.
//!
//! [`IntoStatic`] is implemented for [`Option<T>`] and [`Result<T, E>`], provided
//! `T` and `E` are also [`IntoStatic`].
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
