#![allow(non_upper_case_globals)]

use crate::{
    error::{OsuError, ParsingError},
    model::GameMode,
};

use bitflags::bitflags;
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    convert::{Into, TryFrom},
    fmt,
    str::FromStr,
};

bitflags! {
    /// Enum for all game modifications.
    /// Implemented as [bitflags](https://crates.io/crates/bitflags).
    ///
    /// # Example
    /// ```
    /// use rosu_v2::model::GameMods;
    /// use std::str::FromStr;
    ///
    /// let nomod = GameMods::default();
    /// assert_eq!(nomod, GameMods::NoMod);
    ///
    /// // Bitwise creating, or from u32
    /// let hdhr_1 = GameMods::HardRock | GameMods::Hidden;
    /// let hdhr_2 = GameMods::from_bits(8 + 16).unwrap();
    /// assert_eq!(hdhr_1, hdhr_2);
    ///
    /// // contains, intersects, and a few more methods from bitflags
    /// let ezhdpf = GameMods::Easy | GameMods::Hidden | GameMods::Perfect;
    /// assert!(!ezhdpf.contains(GameMods::HardRock));
    /// let hdpf = GameMods::Hidden | GameMods::Perfect;
    /// assert!(ezhdpf.intersects(hdpf));
    ///
    /// // Try converting from &str
    /// let hdhrdt = GameMods::from_str("dthdhr").unwrap();
    /// assert_eq!(hdhrdt.bits(), 8 + 16 + 64);
    /// // Implements fmt::Display
    /// assert_eq!(hdhrdt.to_string(), "HDHRDT".to_string());
    ///
    /// // Iterator
    /// let mut mod_iter = GameMods::from_bits(536871512).unwrap().iter();
    /// assert_eq!(mod_iter.next(), Some(GameMods::Hidden));
    /// assert_eq!(mod_iter.next(), Some(GameMods::HardRock));
    /// assert_eq!(mod_iter.next(), Some(GameMods::NightCore));
    /// assert_eq!(mod_iter.next(), Some(GameMods::ScoreV2));
    /// assert_eq!(mod_iter.next(), None);
    /// ```
    #[derive(Default)]
    pub struct GameMods: u32 {
        const NoMod = 0;
        const NoFail = 1;
        const Easy = 2;
        const TouchDevice = 4;
        const Hidden = 8;
        const HardRock = 16;
        const SuddenDeath = 32;
        const DoubleTime = 64;
        const Relax = 128;
        const HalfTime = 256;
        const NightCore = 512 | Self::DoubleTime.bits;
        const Flashlight = 1024;
        const SpunOut = 4096;
        const Perfect = 16_384 | Self::SuddenDeath.bits;
        const FadeIn = 1_048_576;
        const ScoreV2 = 536_870_912;
        const Mirror = 1_073_741_824;

        const Key1 = 67_108_864;
        const Key2 = 268_435_456;
        const Key3 = 134_217_728;
        const Key4 = 32_768;
        const Key5 = 65_536;
        const Key6 = 131_072;
        const Key7 = 262_144;
        const Key8 = 524_288;
        const Key9 = 16_777_216;
        const KeyCoop = 33_554_432;

        const Autoplay = 2048;
        const Autopilot = 8192;
        const Cinema = 4_194_304;
        const Random = 2_097_152;
        const Target = 8_388_608;
    }
}

#[allow(clippy::len_without_is_empty)]
impl GameMods {
    /// Method that checks whether the [`GameMods`](crate::model::GameMods) contain one of osu!mania's key mods.
    ///
    /// # Examples
    /// ```
    /// use rosu_v2::model::GameMods;
    ///
    /// let mods = GameMods::Hidden | GameMods::Key4;
    /// assert_eq!(mods.has_key_mod(), Some(GameMods::Key4));
    /// assert_eq!(GameMods::Hidden.has_key_mod(), None);
    /// ```
    pub fn has_key_mod(self) -> Option<GameMods> {
        if self.contains(GameMods::Key1) {
            Some(GameMods::Key1)
        } else if self.contains(GameMods::Key2) {
            Some(GameMods::Key2)
        } else if self.contains(GameMods::Key3) {
            Some(GameMods::Key3)
        } else if self.contains(GameMods::Key4) {
            Some(GameMods::Key4)
        } else if self.contains(GameMods::Key5) {
            Some(GameMods::Key5)
        } else if self.contains(GameMods::Key6) {
            Some(GameMods::Key6)
        } else if self.contains(GameMods::Key7) {
            Some(GameMods::Key7)
        } else if self.contains(GameMods::Key8) {
            Some(GameMods::Key8)
        } else if self.contains(GameMods::Key9) {
            Some(GameMods::Key9)
        } else {
            None
        }
    }

    /// Calculate the multiplier of the mods which will
    /// influence a [`Score`](crate::model::score::Score)'s playscore
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::model::{GameMods, GameMode};
    ///
    /// let ezhd = GameMods::from_bits(2 + 8).unwrap();
    /// assert_eq!(ezhd.score_multiplier(GameMode::STD), 0.53);
    /// assert_eq!(ezhd.score_multiplier(GameMode::MNA), 0.5);
    /// ```
    pub fn score_multiplier(self, mode: GameMode) -> f32 {
        self.into_iter()
            .map(|m| match mode {
                GameMode::STD => match m {
                    GameMods::HalfTime => 0.3,
                    GameMods::Easy | GameMods::NoFail => 0.5,
                    GameMods::SpunOut => 0.9,
                    GameMods::HardRock | GameMods::Hidden => 1.06,
                    GameMods::DoubleTime | GameMods::NightCore | GameMods::Flashlight => 1.12,
                    _ => 1.0,
                },
                GameMode::TKO => match m {
                    GameMods::HalfTime => 0.3,
                    GameMods::Easy | GameMods::NoFail => 0.5,
                    GameMods::HardRock | GameMods::Hidden => 1.06,
                    GameMods::DoubleTime | GameMods::NightCore | GameMods::Flashlight => 1.12,
                    _ => 1.0,
                },
                GameMode::CTB => match m {
                    GameMods::HalfTime => 0.3,
                    GameMods::Easy | GameMods::NoFail => 0.5,
                    GameMods::DoubleTime | GameMods::NightCore | GameMods::Hidden => 1.06,
                    GameMods::HardRock | GameMods::Flashlight => 1.12,
                    _ => 1.0,
                },
                GameMode::MNA => match m {
                    GameMods::Easy | GameMods::NoFail | GameMods::HalfTime => 0.5,
                    _ => 1.0,
                },
            })
            .product()
    }

    /// Check if a [`Score`](crate::model::score::Score)'s playscore will be increased
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::model::{GameMods, GameMode};
    ///
    /// let hrso = GameMods::HardRock | GameMods::SpunOut;
    /// assert!(!hrso.increases_score(GameMode::STD));
    /// assert!(GameMods::DoubleTime.increases_score(GameMode::TKO));
    /// ```
    #[inline]
    pub fn increases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) > 1.0
    }

    /// Check if a [`Score`](crate::model::score::Score)'s playscore will be decreased
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::model::{GameMods, GameMode};
    ///
    /// let hrso = GameMods::HardRock | GameMods::SpunOut;
    /// assert!(hrso.decreases_score(GameMode::STD));
    /// assert!(!GameMods::DoubleTime.decreases_score(GameMode::TKO));
    /// ```
    #[inline]
    pub fn decreases_score(self, mode: GameMode) -> bool {
        self.score_multiplier(mode) < 1.0
    }

    /// Check if a [`Beatmap`](crate::model::beatmap::Beatmap)'s star rating for the given [`GameMode`](crate::model::GameMode) will be influenced.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::model::{GameMode, GameMods};
    ///
    /// let hdhr = GameMods::Hidden | GameMods::HardRock;
    /// assert!(hdhr.changes_stars(GameMode::STD));
    /// assert!(!hdhr.changes_stars(GameMode::MNA));
    /// let nc = GameMods::NightCore;
    /// assert!(nc.changes_stars(GameMode::MNA));
    /// ```
    #[inline]
    pub fn changes_stars(self, mode: GameMode) -> bool {
        if self.intersects(GameMods::DoubleTime | GameMods::HalfTime) {
            true
        } else if self.intersects(GameMods::HardRock | GameMods::Easy) {
            matches!(mode, GameMode::STD | GameMode::CTB)
        } else {
            false
        }
    }

    /// Returns an iterator. Alias of `into_iter`.
    ///
    /// # Example
    /// ```
    /// use rosu_v2::model::GameMods;
    ///
    /// let mods = GameMods::from_bits(8 + 16 + 64 + 128).unwrap();
    /// let mut mod_iter = mods.iter();
    /// assert_eq!(mod_iter.next(), Some(GameMods::Hidden));
    /// assert_eq!(mod_iter.next(), Some(GameMods::HardRock));
    /// assert_eq!(mod_iter.next(), Some(GameMods::DoubleTime));
    /// assert_eq!(mod_iter.next(), Some(GameMods::Relax));
    /// assert_eq!(mod_iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(self) -> GameModsIter {
        self.into_iter()
    }

    /// Returns the amount of contained mods.
    ///
    /// # Example
    /// ```
    /// use rosu_v2::model::GameMods;
    ///
    /// assert_eq!(GameMods::NoMod.len(), 0);
    /// let mods = GameMods::from_bits(8 + 16 + 64 + 128).unwrap();
    /// assert_eq!(mods.len(), 4);
    /// ```
    #[inline]
    pub fn len(self) -> usize {
        self.bits().count_ones() as usize
            - self.contains(GameMods::NightCore) as usize
            - self.contains(GameMods::Perfect) as usize
    }
}

impl fmt::Display for GameMods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in self.into_iter() {
            let abbrev = match m {
                GameMods::NoMod => "NM",
                GameMods::NoFail => "NF",
                GameMods::Easy => "EZ",
                GameMods::TouchDevice => "TD",
                GameMods::Hidden => "HD",
                GameMods::HardRock => "HR",
                GameMods::SuddenDeath => "SD",
                GameMods::DoubleTime => "DT",
                GameMods::Relax => "RX",
                GameMods::HalfTime => "HT",
                GameMods::NightCore => "NC",
                GameMods::Flashlight => "FL",
                GameMods::SpunOut => "SO",
                GameMods::Autopilot => "AP",
                GameMods::Perfect => "PF",
                GameMods::FadeIn => "FI",
                GameMods::Random => "RD",
                GameMods::Target => "TP",
                GameMods::ScoreV2 => "V2",
                GameMods::Mirror => "MR",
                GameMods::Key1 => "1K",
                GameMods::Key2 => "2K",
                GameMods::Key3 => "3K",
                GameMods::Key4 => "4K",
                GameMods::Key5 => "5K",
                GameMods::Key6 => "6K",
                GameMods::Key7 => "7K",
                GameMods::Key8 => "8K",
                GameMods::Key9 => "9K",
                GameMods::Autoplay => "",
                GameMods::Cinema => "",
                GameMods::KeyCoop => "",
                _ => unreachable!(),
            };

            f.write_str(abbrev)?;
        }

        Ok(())
    }
}

impl From<GameMods> for u32 {
    #[inline]
    fn from(mods: GameMods) -> Self {
        mods.bits
    }
}

impl TryFrom<u32> for GameMods {
    type Error = OsuError;

    #[inline]
    fn try_from(m: u32) -> Result<Self, Self::Error> {
        GameMods::from_bits(m).ok_or_else(|| ParsingError::ModsU32(m).into())
    }
}

impl FromStr for GameMods {
    type Err = OsuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = GameMods::default();
        let upper = util::to_uppercase(s);

        for m in util::cut(&upper, 2) {
            let m = match m {
                "NM" => GameMods::NoMod,
                "NF" => GameMods::NoFail,
                "EZ" => GameMods::Easy,
                "TD" => GameMods::TouchDevice,
                "HD" => GameMods::Hidden,
                "HR" => GameMods::HardRock,
                "SD" => GameMods::SuddenDeath,
                "DT" => GameMods::DoubleTime,
                "RX" | "RL" => GameMods::Relax,
                "HT" => GameMods::HalfTime,
                "NC" => GameMods::NightCore,
                "FL" => GameMods::Flashlight,
                "SO" => GameMods::SpunOut,
                "AP" => GameMods::Autopilot,
                "PF" => GameMods::Perfect,
                "FI" => GameMods::FadeIn,
                "RD" => GameMods::Random,
                "TP" => GameMods::Target,
                "V2" => GameMods::ScoreV2,
                "MR" => GameMods::Mirror,
                "1K" | "K1" => GameMods::Key1,
                "2K" | "K2" => GameMods::Key2,
                "3K" | "K3" => GameMods::Key3,
                "4K" | "K4" => GameMods::Key4,
                "5K" | "K5" => GameMods::Key5,
                "6K" | "K6" => GameMods::Key6,
                "7K" | "K7" => GameMods::Key7,
                "8K" | "K8" => GameMods::Key8,
                "9K" | "K9" => GameMods::Key9,
                _ if upper == "NOMOD" => GameMods::NoMod,
                _ if upper == "RELAX" => GameMods::Relax,
                _ => return Err(ParsingError::ModsStr(s.to_owned()).into()),
            };

            res.insert(m);
        }

        Ok(res)
    }
}

pub struct GameModsIter {
    mods: GameMods,
    shift: usize,
}

impl Iterator for GameModsIter {
    type Item = GameMods;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.mods.is_empty() {
            loop {
                if self.shift == 32 {
                    return None;
                }

                let mut bit = 1 << self.shift;
                self.shift += 1;

                if (bit == 32 && self.mods.contains(GameMods::Perfect))
                    || (bit == 64 && self.mods.contains(GameMods::NightCore))
                {
                    continue;
                } else if bit == 512 {
                    bit += GameMods::DoubleTime.bits
                } else if bit == 16_384 {
                    bit += GameMods::SuddenDeath.bits
                }

                if self.mods.bits & bit == bit {
                    let mods = GameMods::from_bits(bit)?;
                    self.mods.remove(mods);

                    return Some(mods);
                }
            }
        } else if self.shift == 0 {
            self.shift = 32;

            Some(GameMods::NoMod)
        } else {
            None
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.mods.len() + (self.mods.is_empty() && self.shift == 0) as usize
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.mods.len() + (self.mods.is_empty() && self.shift == 0) as usize;

        (len, Some(len))
    }
}

impl IntoIterator for GameMods {
    type Item = GameMods;
    type IntoIter = GameModsIter;

    #[inline]
    fn into_iter(self) -> GameModsIter {
        GameModsIter {
            mods: self,
            shift: 0,
        }
    }
}

struct ModsVisitor;

impl<'de> Visitor<'de> for ModsVisitor {
    type Value = GameMods;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a u32, a stringified number, or a sequence")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        let mods = match util::parse_u32(v) {
            Some(n) => GameMods::from_bits(n),
            None => GameMods::from_str(v).ok(),
        };

        mods.ok_or_else(|| {
            Error::invalid_value(
                Unexpected::Str(v),
                &"a stringified u32 representing GameMods or a combination of mod abbriviations",
            )
        })
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        use std::convert::TryInto;

        v.try_into()
            .ok()
            .and_then(GameMods::from_bits)
            .ok_or_else(|| {
                Error::invalid_value(
                    Unexpected::Unsigned(v),
                    &"a valid u32 representing a mod combination",
                )
            })
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut mods = GameMods::default();

        while let Some(next) = seq.next_element()? {
            mods |= next;
        }

        Ok(mods)
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut mods = None;

        while let Some(key) = map.next_key()? {
            match key {
                "acronym" => mods = Some(map.next_value()?),
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        mods.ok_or_else(|| Error::missing_field("acronym"))
    }
}

impl<'de> Deserialize<'de> for GameMods {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ModsVisitor)
    }
}

impl Serialize for GameMods {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(self.bits)
    }
}

mod util {
    use std::borrow::Cow;

    /// Provide an iterator over substrings of the given length on the given source string
    pub(crate) fn cut(mut source: &str, n: usize) -> impl Iterator<Item = &str> {
        std::iter::from_fn(move || {
            if source.is_empty() {
                None
            } else {
                let end_idx = source
                    .char_indices()
                    .nth(n - 1)
                    .map_or_else(|| source.len(), |(idx, c)| idx + c.len_utf8());

                let (sub_str, rest) = source.split_at(end_idx);
                source = rest;

                Some(sub_str)
            }
        })
    }

    /// Put a `&str` into ASCII uppercase. Doesn't allocate if it already is uppercase.
    pub(crate) fn to_uppercase(s: &str) -> Cow<'_, str> {
        match s.as_bytes().iter().position(u8::is_ascii_lowercase) {
            Some(pos) => {
                let mut output = s.to_owned();

                // SAFETY: Index is certain to be contained
                unsafe { output.get_unchecked_mut(pos..) }.make_ascii_uppercase();

                Cow::Owned(output)
            }
            None => Cow::Borrowed(s),
        }
    }

    /// Slight simplification of u32's FromStr implementation
    pub(crate) fn parse_u32(src: &str) -> Option<u32> {
        if src.is_empty() {
            return None;
        }

        let mut result: u32 = 0;

        for c in src.chars() {
            let digit = c.to_digit(10)?;
            result = result.checked_mul(10)?.checked_add(digit)?;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mods_try_from_str() {
        assert_eq!(GameMods::from_str("Nm").unwrap(), GameMods::NoMod);
        assert_eq!(GameMods::from_str("hD").unwrap(), GameMods::Hidden);

        let mods = GameMods::from_bits(24).unwrap();
        assert_eq!(GameMods::from_str("HRhD").unwrap(), mods);
        assert!(GameMods::from_str("HHDR").is_err());
    }

    #[test]
    fn mods_iter() {
        let mut iter = GameMods::default().iter();
        assert_eq!(iter.next(), Some(GameMods::NoMod));
        assert_eq!(iter.next(), None);

        let mut iter = GameMods::from_bits(24).unwrap().iter();
        assert_eq!(iter.next(), Some(GameMods::Hidden));
        assert_eq!(iter.next(), Some(GameMods::HardRock));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn cut() {
        let mut iter = util::cut("hDHrdTv2n", 2);

        assert_eq!(iter.next(), Some("hD"));
        assert_eq!(iter.next(), Some("Hr"));
        assert_eq!(iter.next(), Some("dT"));
        assert_eq!(iter.next(), Some("v2"));
        assert_eq!(iter.next(), Some("n"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn to_uppercase() {
        let upper = util::to_uppercase("MANAmE JeF");
        assert_eq!(upper.as_ref(), "MANAME JEF");

        let upper = util::to_uppercase("mAn4me jäf");
        assert_eq!(upper.as_ref(), "MAN4ME JäF");
    }

    #[test]
    fn parse_u32() {
        assert_eq!(util::parse_u32(""), None);
        assert_eq!(util::parse_u32("123"), Some(123));
        assert_eq!(util::parse_u32("+123"), None);
        assert_eq!(util::parse_u32("123a"), None);
        assert_eq!(util::parse_u32("5123456789"), None);
        assert_eq!(util::parse_u32("00123"), Some(123));
    }
}

#[cfg(feature = "rkyv")]
mod rkyv_impls {
    use rkyv::{Archive, Archived, Deserialize, Fallible, Resolver, Serialize};

    use super::GameMods;

    const _: () = {
        impl Archive for GameMods {
            type Archived = Archived<u32>;
            type Resolver = Resolver<u32>;

            #[inline]
            unsafe fn resolve(&self, pos: usize, _: Self::Resolver, out: *mut Self::Archived) {
                self.bits.resolve(pos, (), out);
            }
        }
    };

    const _: () = {
        impl<S: Fallible + ?Sized> Serialize<S> for GameMods {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }
    };

    const _: () = {
        impl<D: Fallible + ?Sized> Deserialize<GameMods, D> for Archived<GameMods> {
            #[inline]
            fn deserialize(&self, d: &mut D) -> Result<GameMods, D::Error> {
                let bits = Deserialize::<u32, D>::deserialize(self, d)?;

                Ok(GameMods::from_bits_truncate(bits))
            }
        }
    };
}
