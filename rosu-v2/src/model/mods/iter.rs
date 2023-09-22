use std::{
    collections::{
        btree_map::{IntoValues, Values, ValuesMut},
        btree_set::{IntoIter, Iter},
    },
    fmt::{Debug, Formatter, Result as FmtResult},
    iter::{Copied, FusedIterator},
};

use super::{GameMod, GameModIntermode, GameModOrder};

macro_rules! mods_iter {
    (
        $( #[$meta:meta] )*
        $name:ident $( < $outer_lifetime:lifetime > )? :
        $inner:ident < $( $inner_generic:tt ),+ > =>
        $item:ty
    ) => {
        $( #[ $meta ] )*
        pub struct $name $( < $outer_lifetime > )? {
            inner: $inner < $( $inner_generic ),* >,
        }

        impl $( < $outer_lifetime > )? $name $( < $outer_lifetime > )? {
            pub(super) fn new(inner: $inner < $( $inner_generic ),* >) -> Self {
                Self { inner }
            }
        }

        impl $( < $outer_lifetime > )? Debug for $name $( < $outer_lifetime > )? {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                Debug::fmt(&self.inner, f)
            }
        }

        impl $( < $outer_lifetime > )? Iterator for $name $( < $outer_lifetime > )? {
            type Item = $item;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.inner.size_hint()
            }

            #[inline]
            fn last(mut self) -> Option<Self::Item> {
                self.inner.next_back()
            }
        }

        impl $( < $outer_lifetime > )? DoubleEndedIterator for $name $( < $outer_lifetime > )? {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.inner.next_back()
            }
        }

        impl $( < $outer_lifetime > )? ExactSizeIterator for $name $( < $outer_lifetime > )? {
            #[inline]
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl $( < $outer_lifetime > )? FusedIterator for $name $( < $outer_lifetime > )? {}
    };
}

mods_iter! {
    #[derive(Clone)]
    #[doc = "Iterates over [`GameMod`] references"]
    GameModsIter<'m>: Values<'m, GameModOrder, GameMod> => &'m GameMod
}
mods_iter! {
    #[doc = "Iterates over mutable [`GameMod`] references"]
    GameModsIterMut<'m>: ValuesMut<'m, GameModOrder, GameMod> => &'m mut GameMod
}
mods_iter! {
    #[doc = "Iterates over [`GameMod`]"]
    IntoGameModsIter: IntoValues<GameModOrder, GameMod> => GameMod
}

type GameModsIntermodeIterInner<'m> = Copied<Iter<'m, GameModIntermode>>;

mods_iter! {
    #[derive(Clone)]
    #[doc = "Iterates over [`GameModIntermode`]"]
    GameModsIntermodeIter<'m>: GameModsIntermodeIterInner<'m> => GameModIntermode
}
mods_iter! {
    #[doc = "Iterates over [`GameModIntermode`]"]
    IntoGameModsIntermodeIter: IntoIter<GameModIntermode> => GameModIntermode
}
