#[macro_use]
mod debug;

#[macro_use]
mod back;

#[macro_use]
mod front;

#[macro_use]
mod len;

#[macro_use]
mod from;

#[macro_use]
mod into;

use super::{cons, Cons};
use std::{fmt::Debug};

#[derive(Debug)]
pub struct HNil;

#[macro_export]
macro_rules! HList {
    ($a:ty, $($bs:ty),*) => {
        Cons<$a, HList!($($bs),*)>
    };
    ($a:ty) => {
        Cons<$a, HNil>
    };
}

#[macro_export]
macro_rules! hlist {
    ($a:expr, $($bs:expr),*) => {
        cons($a, hlist!($($bs),*))
    };
    ($a:expr) => {
        cons($a, HNil)
    };
}

macro_rules! impl_hlist {
    ($a:ident, $($bs:ident),*) => {
        impl_hlist_debug!($a, $($bs),*);
        impl_hlist_front!($a, $($bs),*);
        impl_hlist_back!($a, $($bs),*);
        impl_hlist_len!($a, $($bs),*);
        impl_hlist_from!($a, $($bs),*);
        impl_hlist_into!($a, $($bs),*);
        impl_hlist!($($bs),*);
    };
    ($a:ident) => {
        impl_hlist_debug!($a);
        impl_hlist_front!($a);
        impl_hlist_back!($a);
        impl_hlist_len!($a);
        impl_hlist_from!($a);
        impl_hlist_into!($a);
    };
}

impl_hlist!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
