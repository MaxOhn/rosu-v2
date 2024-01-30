use std::{
    collections::BTreeMap,
    fmt::{Formatter, Result as FmtResult},
    marker::PhantomData,
};

use serde::{
    de::{DeserializeSeed, Error as DeError, SeqAccess, Visitor},
    Deserializer,
};

use crate::prelude::GameMode;

use super::{GameMod, GameModOrder, GameMods, GameModsIntermode};

/// Struct to pass a [`GameMode`] into some deserialization via [`DeserializeSeed`].
pub struct ModeAsSeed<T> {
    pub(crate) mode: GameMode,
    pub(crate) phantom: PhantomData<T>,
}

impl<T> Clone for ModeAsSeed<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ModeAsSeed<T> {}

impl<T> ModeAsSeed<T> {
    pub fn new(mode: GameMode) -> Self {
        Self {
            mode,
            phantom: PhantomData,
        }
    }

    pub fn cast<U>(self) -> ModeAsSeed<U> {
        ModeAsSeed::new(self.mode)
    }
}

impl<'de> Visitor<'de> for ModeAsSeed<GameMods> {
    type Value = GameMods;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of GameMod")
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        v.parse::<GameModsIntermode>()
            .map_err(DeError::custom)?
            .with_mode(self.mode)
            .ok_or_else(|| DeError::custom(format!("invalid mods for mode {:?}", self.mode)))
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        let bits = u32::try_from(v).map_err(|_| DeError::custom("bitflags must fit in a u32"))?;

        GameModsIntermode::from_bits(bits)
            .with_mode(self.mode)
            .ok_or_else(|| DeError::custom(format!("invalid mods for mode {:?}", self.mode)))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut inner = BTreeMap::new();

        while let Some(res) = seq.next_element_seed(self.cast::<GameMod>())? {
            inner.insert(GameModOrder::from(&res), res);
        }

        Ok(GameMods { inner })
    }
}

impl<'de> DeserializeSeed<'de> for ModeAsSeed<GameMods> {
    type Value = GameMods;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

impl<'de> DeserializeSeed<'de> for ModeAsSeed<GameMod> {
    type Value = GameMod;

    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}
