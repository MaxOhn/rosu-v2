use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::BTreeSet,
    convert::Infallible,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{BitOr, BitOrAssign, Sub, SubAssign},
    str::FromStr,
};

use serde::{
    de::{Error as DeError, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::prelude::{DoubleTimeOsu, GameMode, NightcoreOsu, PerfectOsu, SuddenDeathOsu};

use super::{
    intersection::{GameModsIntermodeIntersection, IntersectionInner},
    iter::{GameModsIntermodeIter, IntoGameModsIntermodeIter},
    Acronym, GameMod, GameModIntermode, GameMods,
};

/// Combination of [`GameModIntermode`]s.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(transparent))]
pub struct GameModsIntermode {
    inner: BTreeSet<GameModIntermode>,
}

impl GameModsIntermode {
    /// Returns empty mods i.e. "NoMod"
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the accumulated bit values of all contained mods.
    ///
    /// Mods that don't have bit values will be ignored.
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::mods;
    ///
    /// let hdhrdtwu = mods!(HD HR DT WU);
    /// assert_eq!(hdhrdtwu.bits(), 8 + 16 + 64);
    /// ```
    #[inline]
    pub fn bits(&self) -> u32 {
        self.inner
            .iter()
            .copied()
            .flat_map(GameModIntermode::bits)
            .fold(0, u32::bitor)
    }

    /// Return the accumulated bit values of all contained mods.
    ///
    /// If any contained mod has no bit value `None` is returned.
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::mods;
    ///
    /// let hdhrdt = mods!(HD HR DT);
    /// assert_eq!(hdhrdt.checked_bits(), Some(8 + 16 + 64));
    ///
    /// let hdhrdtwu = mods!(HD HR DT WU);
    /// assert_eq!(hdhrdtwu.checked_bits(), None);
    /// ```
    #[inline]
    pub fn checked_bits(&self) -> Option<u32> {
        self.inner
            .iter()
            .copied()
            .map(GameModIntermode::bits)
            .try_fold(0, |bits, next| Some(next? | bits))
    }

    /// Returns `true` if no mods are contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods = GameModsIntermode::new();
    /// assert!(mods.is_empty());
    ///
    /// mods.insert(GameModIntermode::Hidden);
    /// assert!(!mods.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the amount of contained mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameModIntermode, GameModsIntermode};
    ///
    /// let hdhrdt = mods!(HD HR DT);
    /// assert_eq!(hdhrdt.len(), 3);
    ///
    /// let mut nm = GameModsIntermode::new();
    /// assert_eq!(nm.len(), 0);
    /// assert_eq!(nm.to_string(), "NM");
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Add a [`GameModIntermode`]
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods = GameModsIntermode::new();
    /// assert_eq!(mods.to_string(), "NM");
    ///
    /// mods.insert(GameModIntermode::Traceable);
    /// assert_eq!(mods.to_string(), "TC");
    ///
    /// mods.insert(GameModIntermode::HardRock);
    /// assert_eq!(mods.to_string(), "HRTC");
    /// ```
    #[inline]
    pub fn insert(&mut self, gamemod: GameModIntermode) {
        self.inner.insert(gamemod);
    }

    /// Check whether a given mod is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameModIntermode};
    ///
    /// let hd = mods!(HD);
    /// assert!(hd.contains(GameModIntermode::Hidden));
    /// assert!(!hd.contains(GameModIntermode::HardRock));
    /// ```
    #[inline]
    pub fn contains<M>(&self, gamemod: M) -> bool
    where
        GameModIntermode: From<M>,
    {
        self.inner.contains(&GameModIntermode::from(gamemod))
    }

    /// Check whether a given [`Acronym`] is contained.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, Acronym};
    ///
    /// let nc = mods!(NC);
    /// assert!(nc.contains_acronym("NC".parse::<Acronym>().unwrap()));
    /// assert!(!nc.contains_acronym("DT".parse::<Acronym>().unwrap()));
    /// ```
    #[inline]
    pub fn contains_acronym(&self, acronym: Acronym) -> bool {
        self.inner
            .iter()
            .any(|gamemod| gamemod.acronym() == acronym)
    }

    /// Remove a gamemod and return whether it was contained.
    ///
    /// # Example
    /// ```
    /// use rosu_v2::prelude::{mods, GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods: GameModsIntermode = mods!(HD HR);
    ///
    /// assert!(mods.remove(GameModIntermode::Hidden));
    /// assert_eq!(mods.to_string(), "HR");
    /// assert!(!mods.remove(GameModIntermode::DoubleTime));
    /// ```
    #[inline]
    pub fn remove<M>(&mut self, gamemod: M) -> bool
    where
        GameModIntermode: From<M>,
    {
        self.inner.remove(&GameModIntermode::from(gamemod))
    }

    /// Remove all mods contained in the iterator.
    ///
    /// # Example
    /// ```
    /// use rosu_v2::prelude::{mods, GameModIntermode, GameModsIntermode};
    ///
    /// let mut mods: GameModsIntermode = mods!(HD HR WG DT BR);
    ///
    /// mods.remove_all([GameModIntermode::Hidden, GameModIntermode::Easy]);
    /// assert_eq!(mods.to_string(), "HRDTBRWG");
    ///
    /// mods.remove_all(mods!(NF WG));
    /// assert_eq!(mods.to_string(), "HRDTBR")
    /// ```
    #[inline]
    pub fn remove_all<I, M>(&mut self, mods: I)
    where
        I: IntoIterator<Item = M>,
        GameModIntermode: From<M>,
    {
        for gamemod in mods {
            self.remove(gamemod);
        }
    }

    /// Parse bitflags into [`GameModsIntermode`]
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameModsIntermode};
    ///
    /// let bits = 8 + 64 + 512 + 1024;
    /// assert_eq!(GameModsIntermode::from_bits(bits), mods!(FL HD NC))
    /// ```
    pub fn from_bits(mut bits: u32) -> Self {
        struct BitIterator(u32);

        impl Iterator for BitIterator {
            type Item = bool;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 {
                    None
                } else {
                    let bit = self.0 & 0b1;
                    self.0 >>= 1;

                    Some(bit == 1)
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                ((self.0 > 0) as usize, None)
            }
        }

        // Special handling for NC and PF since they require two bits
        bits &= if (bits & NightcoreOsu::bits()) == NightcoreOsu::bits() {
            !DoubleTimeOsu::bits()
        } else {
            !(1 << 9)
        };

        bits &= if (bits & PerfectOsu::bits()) == PerfectOsu::bits() {
            !SuddenDeathOsu::bits()
        } else {
            !(1 << 14)
        };

        const BITFLAG_MODS: [GameModIntermode; 31] = [
            GameModIntermode::NoFail,
            GameModIntermode::Easy,
            GameModIntermode::TouchDevice,
            GameModIntermode::Hidden,
            GameModIntermode::HardRock,
            GameModIntermode::SuddenDeath,
            GameModIntermode::DoubleTime,
            GameModIntermode::Relax,
            GameModIntermode::HalfTime,
            GameModIntermode::Nightcore,
            GameModIntermode::Flashlight,
            GameModIntermode::Autoplay,
            GameModIntermode::SpunOut,
            GameModIntermode::Autopilot,
            GameModIntermode::Perfect,
            GameModIntermode::FourKeys,
            GameModIntermode::FiveKeys,
            GameModIntermode::SixKeys,
            GameModIntermode::SevenKeys,
            GameModIntermode::EightKeys,
            GameModIntermode::FadeIn,
            GameModIntermode::Random,
            GameModIntermode::Cinema,
            GameModIntermode::TargetPractice,
            GameModIntermode::NineKeys,
            GameModIntermode::DualStages,
            GameModIntermode::OneKey,
            GameModIntermode::ThreeKeys,
            GameModIntermode::TwoKeys,
            GameModIntermode::ScoreV2,
            GameModIntermode::Mirror,
        ];

        let inner = BitIterator(bits)
            .zip(BITFLAG_MODS)
            .filter_map(|(is_set, gamemod)| is_set.then_some(gamemod))
            .collect();

        Self { inner }
    }

    /// Try to parse a combination of mod acronyms into [`GameModsIntermode`].
    ///
    /// Returns `None` if an unknown acronym was encountered.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::GameModsIntermode;
    ///
    /// let hdhrwu = GameModsIntermode::try_from_acronyms("HRWUHD").unwrap();
    /// assert_eq!(hdhrwu.to_string(), "HDHRWU");
    ///
    /// assert!(GameModsIntermode::try_from_acronyms("QQQ").is_none());
    /// ```
    pub fn try_from_acronyms(s: &str) -> Option<Self> {
        let uppercased = to_uppercase(s);

        if uppercased == "NM" {
            return Some(Self::new());
        }

        // We currently don't allow a gamemod to have an acronym of length 1
        if s.len() == 1 {
            return None;
        }

        let mut remaining = uppercased.as_ref();
        let mut mods = BTreeSet::new();

        while !remaining.is_empty() {
            // Split off the first two characters and check if it's an acronym
            let (candidate, rest) = split_prefix::<2>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 2 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let gamemod = GameModIntermode::from_acronym(acronym);

            if !matches!(gamemod, GameModIntermode::Unknown(_)) && rest.len() != 1 {
                mods.insert(gamemod);
                remaining = rest;

                continue;
            }

            // Repeat for the first three characters
            let (candidate, rest) = split_prefix::<3>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 3 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let gamemod = GameModIntermode::from_acronym(acronym);

            if matches!(gamemod, GameModIntermode::Unknown(_)) {
                return None;
            }

            mods.insert(gamemod);
            remaining = rest;
        }

        Some(Self { inner: mods })
    }

    /// Parse a combination of mod acronyms into [`GameModsIntermode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::GameModsIntermode;
    ///
    /// let hdhrwu = GameModsIntermode::from_acronyms("HRWUHD");
    /// assert_eq!(hdhrwu.len(), 3);
    /// assert_eq!(hdhrwu.to_string(), "HDHRWU");
    ///
    /// let mut iter = GameModsIntermode::from_acronyms("QQhdQ").into_iter();
    /// assert_eq!(iter.next().unwrap().to_string(), "HDQ"); // unknown mod
    /// assert_eq!(iter.next().unwrap().to_string(), "QQ");  // unknown mod
    /// assert!(iter.next().is_none());
    /// ```
    pub fn from_acronyms(s: &str) -> Self {
        let uppercased = to_uppercase(s);

        if uppercased == "NM" {
            return Self::new();
        }

        let mut mods = BTreeSet::new();

        // We currently don't allow a gamemod to have an acronym of length 1
        let mut remaining = if s.len() == 1 {
            mods.insert(GameModIntermode::Unknown(Default::default()));

            ""
        } else {
            uppercased.as_ref()
        };

        while !remaining.is_empty() {
            // Split off the first two characters and check if it's an acronym
            let (candidate, rest) = split_prefix::<2>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 2 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let gamemod = GameModIntermode::from_acronym(acronym);

            if !matches!(gamemod, GameModIntermode::Unknown(_)) && rest.len() != 1 {
                mods.insert(gamemod);
                remaining = rest;

                continue;
            }

            // Repeat for the first three characters
            let (candidate, three_letter_rest) = split_prefix::<3>(remaining);

            // SAFETY: `candidate` is guaranteed to be of length 3 and has been capitalized
            let acronym = unsafe { Acronym::from_str_unchecked(candidate) };
            let three_letter_gamemod = GameModIntermode::from_acronym(acronym);

            if !matches!(three_letter_gamemod, GameModIntermode::Unknown(_))
                || three_letter_rest.is_empty()
            {
                mods.insert(three_letter_gamemod);
                remaining = three_letter_rest;
            } else {
                mods.insert(gamemod);
                remaining = rest;
            }
        }

        Self { inner: mods }
    }

    /// Returns an iterator over all mods that appear in both [`GameModsIntermode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameModIntermode};
    ///
    /// let hd = mods!(HD);
    /// let hdhr = mods!(HD HR);
    /// let mut intersection = hd.intersection(&hdhr);
    /// assert_eq!(intersection.next(), Some(GameModIntermode::Hidden));
    /// assert_eq!(intersection.next(), None);
    /// ```
    // https://github.com/rust-lang/rust/blob/c1d3610ac1ddd1cd605479274047fd0a3f37d220/library/alloc/src/collections/btree/set.rs#L517
    pub fn intersection<'m>(
        &'m self,
        other: &'m GameModsIntermode,
    ) -> GameModsIntermodeIntersection<'m> {
        let (self_min, self_max) =
            if let (Some(self_min), Some(self_max)) = (self.inner.first(), self.inner.last()) {
                (*self_min, *self_max)
            } else {
                return GameModsIntermodeIntersection {
                    inner: IntersectionInner::Answer(None),
                };
            };

        let (other_min, other_max) =
            if let (Some(other_min), Some(other_max)) = (other.inner.first(), other.inner.last()) {
                (*other_min, *other_max)
            } else {
                return GameModsIntermodeIntersection {
                    inner: IntersectionInner::Answer(None),
                };
            };

        GameModsIntermodeIntersection {
            inner: match (self_min.cmp(&other_max), self_max.cmp(&other_min)) {
                (Ordering::Greater, _) | (_, Ordering::Less) => IntersectionInner::Answer(None),
                (Ordering::Equal, _) => IntersectionInner::Answer(Some(self_min)),
                (_, Ordering::Equal) => IntersectionInner::Answer(Some(self_max)),
                _ => IntersectionInner::new_stitch(self.inner.iter(), other.inner.iter()),
            },
        }
    }

    /// Check whether the two [`GameMods`] have any common mods.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::mods;
    ///
    /// let hd = mods!(HD);
    /// assert!(!hd.intersects(&mods!(HR)));
    /// assert!(hd.intersects(&mods!(HD HR)));
    /// ```
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).next().is_some()
    }

    /// The legacy clock rate of the [`GameModsIntermode`].
    ///
    /// Looks for the first occurrence of DT, NC, HT, or DC
    /// and returns `1.5`, `0.75`, or `1.0` accordingly.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameModIntermode};
    ///
    /// let hd = mods!(HD);
    /// assert_eq!(hd.legacy_clock_rate(), 1.0);
    ///
    /// let mut hddt = hd;
    /// hddt.insert(GameModIntermode::DoubleTime);
    /// assert_eq!(hddt.legacy_clock_rate(), 1.5);
    /// ```
    #[inline]
    pub fn legacy_clock_rate(&self) -> f32 {
        if self.inner.contains(&GameModIntermode::DoubleTime)
            || self.inner.contains(&GameModIntermode::Nightcore)
        {
            1.5
        } else if self.inner.contains(&GameModIntermode::HalfTime)
            || self.inner.contains(&GameModIntermode::Daycore)
        {
            0.75
        } else {
            1.0
        }
    }

    /// Returns an iterator over all contained mods.
    ///
    /// Note that the iterator will immediately yield `None` in case of "NoMod".
    pub fn iter(&self) -> GameModsIntermodeIter<'_> {
        GameModsIntermodeIter::new(self.inner.iter().copied())
    }

    /// Tries to turn a [`GameModsIntermode`] into a [`GameMods`].
    ///
    /// Returns `None` if any contained [`GameModIntermode`] is unknown for the
    /// given [`GameMode`].
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameMods, GameMode};
    ///
    /// let dtfi: GameMods = mods!(DT FI).try_with_mode(GameMode::Mania).unwrap();
    ///
    /// // The FadeIn mod doesn't exist in Taiko
    /// assert!(mods!(DT FI).try_with_mode(GameMode::Taiko).is_none());
    /// ```
    pub fn try_with_mode(self, mode: GameMode) -> Option<GameMods> {
        self.inner
            .into_iter()
            .map(|gamemod| GameMod::new(gamemod.acronym().as_str(), mode))
            .try_fold(GameMods::default(), |mut mods, next| {
                if matches!(
                    next,
                    GameMod::UnknownOsu(_)
                        | GameMod::UnknownTaiko(_)
                        | GameMod::UnknownCatch(_)
                        | GameMod::UnknownMania(_)
                ) {
                    None
                } else {
                    mods.insert(next);

                    Some(mods)
                }
            })
    }

    /// Turn a [`GameModsIntermode`] into a [`GameMods`].
    ///
    /// Any contained [`GameModIntermode`] that's unknown for the given
    /// [`GameMode`] will be replaced with `GameModIntermode::Unknown`.
    ///
    /// # Example
    /// ```rust
    /// use rosu_v2::prelude::{mods, GameMods, GameMode};
    ///
    /// let dtfi: GameMods = mods!(DT FI).with_mode(GameMode::Mania);
    ///
    /// // The FadeIn mod doesn't exist in Taiko
    /// let dt_unknown: GameMods = mods!(DT FI).with_mode(GameMode::Taiko);
    /// assert_eq!(dt_unknown.to_string(), "DTFI");
    /// ```
    pub fn with_mode(self, mode: GameMode) -> GameMods {
        self.inner
            .into_iter()
            .map(|gamemod| GameMod::new(gamemod.acronym().as_str(), mode))
            .collect()
    }
}

impl Debug for GameModsIntermode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for GameModsIntermode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_empty() {
            f.write_str("NM")
        } else {
            for gamemod in self.iter() {
                f.write_str(gamemod.acronym().as_str())?;
            }

            Ok(())
        }
    }
}

impl IntoIterator for GameModsIntermode {
    type Item = GameModIntermode;
    type IntoIter = IntoGameModsIntermodeIter;

    /// Turns [`GameModsIntermode`] into an iterator over all contained mods.
    ///
    /// Note that the iterator will immediately yield `None` in case of "NoMod".
    fn into_iter(self) -> Self::IntoIter {
        IntoGameModsIntermodeIter::new(self.inner.into_iter())
    }
}

impl<M> FromIterator<M> for GameModsIntermode
where
    GameModIntermode: From<M>,
{
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self {
        Self {
            inner: iter.into_iter().map(GameModIntermode::from).collect(),
        }
    }
}

impl<M> Extend<M> for GameModsIntermode
where
    GameModIntermode: From<M>,
{
    fn extend<T: IntoIterator<Item = M>>(&mut self, iter: T) {
        self.inner
            .extend(iter.into_iter().map(GameModIntermode::from))
    }
}

impl BitOr<GameModIntermode> for GameModsIntermode {
    type Output = Self;

    /// Adds a [`GameModIntermode`] to the [`GameModsIntermode`].
    fn bitor(mut self, rhs: GameModIntermode) -> Self::Output {
        self |= rhs;

        self
    }
}

impl BitOrAssign<GameModIntermode> for GameModsIntermode {
    /// Adds a [`GameModIntermode`] to the [`GameModsIntermode`].
    fn bitor_assign(&mut self, rhs: GameModIntermode) {
        self.insert(rhs);
    }
}

impl Sub<GameModIntermode> for GameModsIntermode {
    type Output = Self;

    /// Removes a [`GameModIntermode`] from the [`GameModsIntermode`]
    fn sub(mut self, rhs: GameModIntermode) -> Self::Output {
        self -= rhs;

        self
    }
}

impl SubAssign<GameModIntermode> for GameModsIntermode {
    /// Removes a [`GameModIntermode`] from the [`GameModsIntermode`]
    fn sub_assign(&mut self, rhs: GameModIntermode) {
        self.remove(rhs);
    }
}

impl<'de> Deserialize<'de> for GameModsIntermode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct GameModsIntermodeVisitor;

        impl<'de> Visitor<'de> for GameModsIntermodeVisitor {
            type Value = GameModsIntermode;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("integer bitflags, mod acronyms, or a sequence of mod acronyms")
            }

            fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
                u32::try_from(v)
                    .map_err(|_| DeError::custom("GameModsIntermode bitflags must fit in a u32"))
                    .map(GameModsIntermode::from_bits)
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                Ok(GameModsIntermode::from_acronyms(v))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut inner = BTreeSet::new();

                while let Some(elem) = seq.next_element()? {
                    let acronym = Acronym::from_str(elem).map_err(DeError::custom)?;
                    inner.insert(GameModIntermode::from_acronym(acronym));
                }

                Ok(GameModsIntermode { inner })
            }
        }

        d.deserialize_any(GameModsIntermodeVisitor)
    }
}

impl From<GameMods> for GameModsIntermode {
    fn from(mods: GameMods) -> Self {
        Self {
            inner: mods.inner.values().map(GameMod::intermode).collect(),
        }
    }
}

impl From<GameModIntermode> for GameModsIntermode {
    fn from(gamemod: GameModIntermode) -> Self {
        let mut mods = Self::new();
        mods.insert(gamemod);

        mods
    }
}

impl FromStr for GameModsIntermode {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_acronyms(s))
    }
}

/// Splits the first `N` characters off
fn split_prefix<const N: usize>(s: &str) -> (&str, &str) {
    let end_idx = s
        .char_indices()
        .nth(N - 1)
        .map_or_else(|| s.len(), |(idx, c)| idx + c.len_utf8());

    s.split_at(end_idx)
}

/// Put a `&str` into ASCII uppercase. Doesn't allocate if it already is uppercase.
fn to_uppercase(s: &str) -> Cow<'_, str> {
    match s.char_indices().find(|(_, c)| c.is_ascii_lowercase()) {
        Some((pos, _)) => {
            let mut output = s.to_owned();

            // SAFETY: `char_indices` is guaranteed to provide a valid index
            unsafe { output.get_unchecked_mut(pos..) }.make_ascii_uppercase();

            Cow::Owned(output)
        }
        None => Cow::Borrowed(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut mods = mods!();
        mods.insert(GameModIntermode::HardRock);
        mods.insert(GameModIntermode::Wiggle);

        assert_eq!(mods.len(), 2);
        assert_eq!(mods.to_string(), "HRWG");
    }

    #[test]
    fn from_bits_nomod() {
        assert!(GameModsIntermode::from_bits(0).is_empty());
    }

    #[test]
    fn from_bits_valid() {
        assert_eq!(GameModsIntermode::from_bits(8 + 64 + 512), mods!(NC HD));
    }

    #[test]
    fn from_bits_invalid_nightcore() {
        assert_eq!(GameModsIntermode::from_bits(512), GameModsIntermode::new());
    }

    #[test]
    fn from_str_nonempty() {
        let mods: GameModsIntermode = "TCWGFLWU".parse().unwrap();
        assert_eq!(mods, mods!(FL TC WG WU));
    }

    #[test]
    fn from_str_unknown() {
        let mut iter = "YYQQQ".parse::<GameModsIntermode>().unwrap().into_iter();

        // Since acronyms of length 1 are not valid, it picks the last three
        // characters.
        // Also, "QQQ" comes before "YY" to it'll be the first mod.
        assert_eq!(iter.next().unwrap().to_string(), "QQQ");
        assert_eq!(iter.next().unwrap().to_string(), "YY");
        assert!(iter.next().is_none());
    }

    #[test]
    fn contains() {
        let mods = mods!(HD HR NC);
        assert!(mods.contains(GameModIntermode::Nightcore));
        assert!(mods.contains(GameModIntermode::Hidden));
        assert!(!mods.contains(GameModIntermode::DoubleTime));
    }

    #[test]
    fn checked_bits() {
        let mods = mods!(HD TC DT);
        assert_eq!(mods.checked_bits(), None);
    }

    #[test]
    fn unchecked_bits() {
        let mods = mods!(TC DT HD);
        assert_eq!(mods.bits(), 72);
    }

    #[test]
    fn intersection() {
        let a = mods!(HD WU HR);
        let b = mods!(WU CL HR);
        let mut intersection = a.intersection(&b);
        assert_eq!(intersection.next(), Some(GameModIntermode::HardRock));
        assert_eq!(intersection.next(), Some(GameModIntermode::WindUp));
        assert_eq!(intersection.next(), None);
    }

    #[test]
    fn deser_str() {
        let json = r#""HDHRWG""#;
        let mods = serde_json::from_str::<GameModsIntermode>(json).unwrap();
        assert_eq!(mods, mods!(HD HR WG));
    }

    #[test]
    fn deser_bits() {
        let json = r#"1096"#;
        let mods = serde_json::from_str::<GameModsIntermode>(json).unwrap();
        assert_eq!(mods, mods!(HD DT FL));
    }

    #[test]
    fn deser_seq() {
        let json = r#"["WU", "BL", "EZ"]"#;
        let mods = serde_json::from_str::<GameModsIntermode>(json).unwrap();
        assert_eq!(mods, mods!(BL EZ WU));
    }
}
