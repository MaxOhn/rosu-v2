//! This file was generated automatically. Do not modify.
//!
//! See <https://raw.githubusercontent.com/ppy/osu-web/master/database/mods.json>

use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU8,
};

use crate::model::{
    mods::{Acronym, ModeAsSeed},
    GameMode,
};
use serde::{
    de::{
        value::MapAccessDeserializer, DeserializeSeed, Deserializer, Error as DeError, IgnoredAny,
        MapAccess, Visitor,
    },
    Deserialize,
};
use serde_json::value::RawValue;
/// Larger circles, more forgiving HP drain, less accuracy required, and three lives!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct EasyOsu {
    pub retries: Option<f32>,
}
impl EasyOsu {
    /// The acronym of [`EasyOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyOsu`]
    pub const fn description() -> &'static str {
        "Larger circles, more forgiving HP drain, less accuracy required, and three lives!"
    }
    /// The [`GameModKind`] of [`EasyOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl<'de> Deserialize<'de> for EasyOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct EasyOsuVisitor;
        impl<'de> Visitor<'de> for EasyOsuVisitor {
            type Value = EasyOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("EasyOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut retries = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "retries" => retries = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    retries: retries.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(EasyOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EasyOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.retries.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.retries {
            map.serialize_entry("retries", x)?;
        }
        map.end()
    }
}
/// You can't fail, no matter what.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct NoFailOsu {}
impl NoFailOsu {
    /// The acronym of [`NoFailOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailOsu`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl<'de> Deserialize<'de> for NoFailOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NoFailOsuVisitor;
        impl<'de> Visitor<'de> for NoFailOsuVisitor {
            type Value = NoFailOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NoFailOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(NoFailOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NoFailOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Less zoom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HalfTimeOsu {
    pub speed_change: Option<f32>,
}
impl HalfTimeOsu {
    /// The acronym of [`HalfTimeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeOsu`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl<'de> Deserialize<'de> for HalfTimeOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HalfTimeOsuVisitor;
        impl<'de> Visitor<'de> for HalfTimeOsuVisitor {
            type Value = HalfTimeOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HalfTimeOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(HalfTimeOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HalfTimeOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Whoaaaaa...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DaycoreOsu {
    pub speed_change: Option<f32>,
}
impl DaycoreOsu {
    /// The acronym of [`DaycoreOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreOsu`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl<'de> Deserialize<'de> for DaycoreOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DaycoreOsuVisitor;
        impl<'de> Visitor<'de> for DaycoreOsuVisitor {
            type Value = DaycoreOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DaycoreOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DaycoreOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DaycoreOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Everything just got a bit harder...
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HardRockOsu {}
impl HardRockOsu {
    /// The acronym of [`HardRockOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
                Acronym::from_str_unchecked("MR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockOsu`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl<'de> Deserialize<'de> for HardRockOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HardRockOsuVisitor;
        impl<'de> Visitor<'de> for HardRockOsuVisitor {
            type Value = HardRockOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HardRockOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HardRockOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HardRockOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Miss and fail.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct SuddenDeathOsu {
    pub restart: Option<bool>,
}
impl SuddenDeathOsu {
    /// The acronym of [`SuddenDeathOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathOsu`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl<'de> Deserialize<'de> for SuddenDeathOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SuddenDeathOsuVisitor;
        impl<'de> Visitor<'de> for SuddenDeathOsuVisitor {
            type Value = SuddenDeathOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SuddenDeathOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(SuddenDeathOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SuddenDeathOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// SS or quit.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct PerfectOsu {
    pub restart: Option<bool>,
}
impl PerfectOsu {
    /// The acronym of [`PerfectOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectOsu`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl<'de> Deserialize<'de> for PerfectOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct PerfectOsuVisitor;
        impl<'de> Visitor<'de> for PerfectOsuVisitor {
            type Value = PerfectOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("PerfectOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(PerfectOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PerfectOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Zoooooooooom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DoubleTimeOsu {
    pub speed_change: Option<f32>,
}
impl DoubleTimeOsu {
    /// The acronym of [`DoubleTimeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeOsu`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl<'de> Deserialize<'de> for DoubleTimeOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DoubleTimeOsuVisitor;
        impl<'de> Visitor<'de> for DoubleTimeOsuVisitor {
            type Value = DoubleTimeOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DoubleTimeOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DoubleTimeOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DoubleTimeOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Uguuuuuuuu...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct NightcoreOsu {
    pub speed_change: Option<f32>,
}
impl NightcoreOsu {
    /// The acronym of [`NightcoreOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreOsu`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl<'de> Deserialize<'de> for NightcoreOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NightcoreOsuVisitor;
        impl<'de> Visitor<'de> for NightcoreOsuVisitor {
            type Value = NightcoreOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NightcoreOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(NightcoreOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NightcoreOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Play with no approach circles and fading circles/sliders.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HiddenOsu {
    pub only_fade_approach_circles: Option<bool>,
}
impl HiddenOsu {
    /// The acronym of [`HiddenOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HiddenOsu`]
    pub const fn description() -> &'static str {
        "Play with no approach circles and fading circles/sliders."
    }
    /// The [`GameModKind`] of [`HiddenOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl<'de> Deserialize<'de> for HiddenOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HiddenOsuVisitor;
        impl<'de> Visitor<'de> for HiddenOsuVisitor {
            type Value = HiddenOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HiddenOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut only_fade_approach_circles = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "only_fade_approach_circles" => {
                            only_fade_approach_circles = Some(map.next_value()?)
                        }
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    only_fade_approach_circles: only_fade_approach_circles.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(HiddenOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HiddenOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.only_fade_approach_circles.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.only_fade_approach_circles {
            map.serialize_entry("only_fade_approach_circles", x)?;
        }
        map.end()
    }
}
/// Restricted view area.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct FlashlightOsu {
    pub follow_delay: Option<f32>,
    pub size_multiplier: Option<f32>,
    pub combo_based_size: Option<bool>,
}
impl FlashlightOsu {
    /// The acronym of [`FlashlightOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("BL")] }.into_iter()
    }
    /// The description of [`FlashlightOsu`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl<'de> Deserialize<'de> for FlashlightOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FlashlightOsuVisitor;
        impl<'de> Visitor<'de> for FlashlightOsuVisitor {
            type Value = FlashlightOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FlashlightOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut follow_delay = None;
                let mut size_multiplier = None;
                let mut combo_based_size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "follow_delay" => follow_delay = Some(map.next_value()?),
                        "size_multiplier" => size_multiplier = Some(map.next_value()?),
                        "combo_based_size" => combo_based_size = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    follow_delay: follow_delay.unwrap_or_default(),
                    size_multiplier: size_multiplier.unwrap_or_default(),
                    combo_based_size: combo_based_size.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(FlashlightOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FlashlightOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.follow_delay.is_some() as usize
            + self.size_multiplier.is_some() as usize
            + self.combo_based_size.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.follow_delay {
            map.serialize_entry("follow_delay", x)?;
        }
        if let Some(ref x) = self.size_multiplier {
            map.serialize_entry("size_multiplier", x)?;
        }
        if let Some(ref x) = self.combo_based_size {
            map.serialize_entry("combo_based_size", x)?;
        }
        map.end()
    }
}
/// Play with blinds on your screen.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct BlindsOsu {}
impl BlindsOsu {
    /// The acronym of [`BlindsOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("BL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`BlindsOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("FL")] }.into_iter()
    }
    /// The description of [`BlindsOsu`]
    pub const fn description() -> &'static str {
        "Play with blinds on your screen."
    }
    /// The [`GameModKind`] of [`BlindsOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl<'de> Deserialize<'de> for BlindsOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct BlindsOsuVisitor;
        impl<'de> Visitor<'de> for BlindsOsuVisitor {
            type Value = BlindsOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("BlindsOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(BlindsOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BlindsOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Once you start a slider, follow precisely or get a miss.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct StrictTrackingOsu {}
impl StrictTrackingOsu {
    /// The acronym of [`StrictTrackingOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("ST") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`StrictTrackingOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("CL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`StrictTrackingOsu`]
    pub const fn description() -> &'static str {
        "Once you start a slider, follow precisely or get a miss."
    }
    /// The [`GameModKind`] of [`StrictTrackingOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl<'de> Deserialize<'de> for StrictTrackingOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct StrictTrackingOsuVisitor;
        impl<'de> Visitor<'de> for StrictTrackingOsuVisitor {
            type Value = StrictTrackingOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("StrictTrackingOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(StrictTrackingOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for StrictTrackingOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Fail if your accuracy drops too low!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AccuracyChallengeOsu {
    pub minimum_accuracy: Option<f32>,
    pub restart: Option<bool>,
}
impl AccuracyChallengeOsu {
    /// The acronym of [`AccuracyChallengeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeOsu`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl<'de> Deserialize<'de> for AccuracyChallengeOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AccuracyChallengeOsuVisitor;
        impl<'de> Visitor<'de> for AccuracyChallengeOsuVisitor {
            type Value = AccuracyChallengeOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AccuracyChallengeOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut minimum_accuracy = None;
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AccuracyChallengeOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccuracyChallengeOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.minimum_accuracy.is_some() as usize + self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.minimum_accuracy {
            map.serialize_entry("minimum_accuracy", x)?;
        }
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Practice keeping up with the beat of the song.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct TargetPracticeOsu {
    pub seed: Option<f32>,
    pub metronome: Option<bool>,
}
impl TargetPracticeOsu {
    /// The acronym of [`TargetPracticeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TargetPracticeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("ST"),
                Acronym::from_str_unchecked("RD"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TargetPracticeOsu`]
    pub const fn description() -> &'static str {
        "Practice keeping up with the beat of the song."
    }
    /// The [`GameModKind`] of [`TargetPracticeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`TargetPracticeOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8388608
    }
}
impl<'de> Deserialize<'de> for TargetPracticeOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct TargetPracticeOsuVisitor;
        impl<'de> Visitor<'de> for TargetPracticeOsuVisitor {
            type Value = TargetPracticeOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("TargetPracticeOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut seed = None;
                let mut metronome = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "seed" => seed = Some(map.next_value()?),
                        "metronome" => metronome = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    seed: seed.unwrap_or_default(),
                    metronome: metronome.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(TargetPracticeOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TargetPracticeOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.seed.is_some() as usize + self.metronome.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.seed {
            map.serialize_entry("seed", x)?;
        }
        if let Some(ref x) = self.metronome {
            map.serialize_entry("metronome", x)?;
        }
        map.end()
    }
}
/// Override a beatmap's difficulty settings.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DifficultyAdjustOsu {
    pub circle_size: Option<f32>,
    pub approach_rate: Option<f32>,
    pub drain_rate: Option<f32>,
    pub overall_difficulty: Option<f32>,
    pub extended_limits: Option<bool>,
}
impl DifficultyAdjustOsu {
    /// The acronym of [`DifficultyAdjustOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustOsu`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for DifficultyAdjustOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DifficultyAdjustOsuVisitor;
        impl<'de> Visitor<'de> for DifficultyAdjustOsuVisitor {
            type Value = DifficultyAdjustOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DifficultyAdjustOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut circle_size = None;
                let mut approach_rate = None;
                let mut drain_rate = None;
                let mut overall_difficulty = None;
                let mut extended_limits = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "circle_size" => circle_size = Some(map.next_value()?),
                        "approach_rate" => approach_rate = Some(map.next_value()?),
                        "drain_rate" => drain_rate = Some(map.next_value()?),
                        "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                        "extended_limits" => extended_limits = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    circle_size: circle_size.unwrap_or_default(),
                    approach_rate: approach_rate.unwrap_or_default(),
                    drain_rate: drain_rate.unwrap_or_default(),
                    overall_difficulty: overall_difficulty.unwrap_or_default(),
                    extended_limits: extended_limits.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DifficultyAdjustOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DifficultyAdjustOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.circle_size.is_some() as usize
            + self.approach_rate.is_some() as usize
            + self.drain_rate.is_some() as usize
            + self.overall_difficulty.is_some() as usize
            + self.extended_limits.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.circle_size {
            map.serialize_entry("circle_size", x)?;
        }
        if let Some(ref x) = self.approach_rate {
            map.serialize_entry("approach_rate", x)?;
        }
        if let Some(ref x) = self.drain_rate {
            map.serialize_entry("drain_rate", x)?;
        }
        if let Some(ref x) = self.overall_difficulty {
            map.serialize_entry("overall_difficulty", x)?;
        }
        if let Some(ref x) = self.extended_limits {
            map.serialize_entry("extended_limits", x)?;
        }
        map.end()
    }
}
/// Feeling nostalgic?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ClassicOsu {
    pub no_slider_head_accuracy: Option<bool>,
    pub no_slider_head_movement: Option<bool>,
    pub classic_note_lock: Option<bool>,
    pub always_play_tail_sample: Option<bool>,
    pub fade_hit_circle_early: Option<bool>,
}
impl ClassicOsu {
    /// The acronym of [`ClassicOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("ST")] }.into_iter()
    }
    /// The description of [`ClassicOsu`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for ClassicOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ClassicOsuVisitor;
        impl<'de> Visitor<'de> for ClassicOsuVisitor {
            type Value = ClassicOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ClassicOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut no_slider_head_accuracy = None;
                let mut no_slider_head_movement = None;
                let mut classic_note_lock = None;
                let mut always_play_tail_sample = None;
                let mut fade_hit_circle_early = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "no_slider_head_accuracy" => {
                            no_slider_head_accuracy = Some(map.next_value()?)
                        }
                        "no_slider_head_movement" => {
                            no_slider_head_movement = Some(map.next_value()?)
                        }
                        "classic_note_lock" => classic_note_lock = Some(map.next_value()?),
                        "always_play_tail_sample" => {
                            always_play_tail_sample = Some(map.next_value()?)
                        }
                        "fade_hit_circle_early" => fade_hit_circle_early = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    no_slider_head_accuracy: no_slider_head_accuracy.unwrap_or_default(),
                    no_slider_head_movement: no_slider_head_movement.unwrap_or_default(),
                    classic_note_lock: classic_note_lock.unwrap_or_default(),
                    always_play_tail_sample: always_play_tail_sample.unwrap_or_default(),
                    fade_hit_circle_early: fade_hit_circle_early.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(ClassicOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClassicOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.no_slider_head_accuracy.is_some() as usize
            + self.no_slider_head_movement.is_some() as usize
            + self.classic_note_lock.is_some() as usize
            + self.always_play_tail_sample.is_some() as usize
            + self.fade_hit_circle_early.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.no_slider_head_accuracy {
            map.serialize_entry("no_slider_head_accuracy", x)?;
        }
        if let Some(ref x) = self.no_slider_head_movement {
            map.serialize_entry("no_slider_head_movement", x)?;
        }
        if let Some(ref x) = self.classic_note_lock {
            map.serialize_entry("classic_note_lock", x)?;
        }
        if let Some(ref x) = self.always_play_tail_sample {
            map.serialize_entry("always_play_tail_sample", x)?;
        }
        if let Some(ref x) = self.fade_hit_circle_early {
            map.serialize_entry("fade_hit_circle_early", x)?;
        }
        map.end()
    }
}
/// It never gets boring!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct RandomOsu {
    pub angle_sharpness: Option<f32>,
    pub seed: Option<f32>,
}
impl RandomOsu {
    /// The acronym of [`RandomOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RandomOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("TP")] }.into_iter()
    }
    /// The description of [`RandomOsu`]
    pub const fn description() -> &'static str {
        "It never gets boring!"
    }
    /// The [`GameModKind`] of [`RandomOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`RandomOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2097152
    }
}
impl<'de> Deserialize<'de> for RandomOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RandomOsuVisitor;
        impl<'de> Visitor<'de> for RandomOsuVisitor {
            type Value = RandomOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RandomOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut angle_sharpness = None;
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "angle_sharpness" => angle_sharpness = Some(map.next_value()?),
                        "seed" => seed = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    angle_sharpness: angle_sharpness.unwrap_or_default(),
                    seed: seed.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(RandomOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RandomOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.angle_sharpness.is_some() as usize + self.seed.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.angle_sharpness {
            map.serialize_entry("angle_sharpness", x)?;
        }
        if let Some(ref x) = self.seed {
            map.serialize_entry("seed", x)?;
        }
        map.end()
    }
}
/// Flip objects on the chosen axes.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MirrorOsu {
    pub reflection: Option<String>,
}
impl MirrorOsu {
    /// The acronym of [`MirrorOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MirrorOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("HR")] }.into_iter()
    }
    /// The description of [`MirrorOsu`]
    pub const fn description() -> &'static str {
        "Flip objects on the chosen axes."
    }
    /// The [`GameModKind`] of [`MirrorOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`MirrorOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1073741824
    }
}
impl<'de> Deserialize<'de> for MirrorOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MirrorOsuVisitor;
        impl<'de> Visitor<'de> for MirrorOsuVisitor {
            type Value = MirrorOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MirrorOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut reflection = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "reflection" => reflection = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    reflection: reflection.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(MirrorOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MirrorOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.reflection.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.reflection {
            map.serialize_entry("reflection", x)?;
        }
        map.end()
    }
}
/// Don't use the same key twice in a row!
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct AlternateOsu {}
impl AlternateOsu {
    /// The acronym of [`AlternateOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AlternateOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AlternateOsu`]
    pub const fn description() -> &'static str {
        "Don't use the same key twice in a row!"
    }
    /// The [`GameModKind`] of [`AlternateOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for AlternateOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AlternateOsuVisitor;
        impl<'de> Visitor<'de> for AlternateOsuVisitor {
            type Value = AlternateOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AlternateOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(AlternateOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AlternateOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// You must only use one key!
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SingleTapOsu {}
impl SingleTapOsu {
    /// The acronym of [`SingleTapOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SingleTapOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SingleTapOsu`]
    pub const fn description() -> &'static str {
        "You must only use one key!"
    }
    /// The [`GameModKind`] of [`SingleTapOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for SingleTapOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SingleTapOsuVisitor;
        impl<'de> Visitor<'de> for SingleTapOsuVisitor {
            type Value = SingleTapOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SingleTapOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SingleTapOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SingleTapOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch a perfect automated play through the song.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct AutoplayOsu {}
impl AutoplayOsu {
    /// The acronym of [`AutoplayOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayOsu`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl<'de> Deserialize<'de> for AutoplayOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AutoplayOsuVisitor;
        impl<'de> Visitor<'de> for AutoplayOsuVisitor {
            type Value = AutoplayOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AutoplayOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(AutoplayOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutoplayOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch the video without visual distractions.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct CinemaOsu {}
impl CinemaOsu {
    /// The acronym of [`CinemaOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaOsu`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl<'de> Deserialize<'de> for CinemaOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct CinemaOsuVisitor;
        impl<'de> Visitor<'de> for CinemaOsuVisitor {
            type Value = CinemaOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("CinemaOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(CinemaOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CinemaOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// You don't need to click. Give your clicking/tapping fingers a break from the heat of things.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct RelaxOsu {}
impl RelaxOsu {
    /// The acronym of [`RelaxOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RX") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RelaxOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AL"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("MG"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RelaxOsu`]
    pub const fn description() -> &'static str {
        "You don't need to click. Give your clicking/tapping fingers a break from the heat of things."
    }
    /// The [`GameModKind`] of [`RelaxOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`RelaxOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        128
    }
}
impl<'de> Deserialize<'de> for RelaxOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RelaxOsuVisitor;
        impl<'de> Visitor<'de> for RelaxOsuVisitor {
            type Value = RelaxOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RelaxOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(RelaxOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RelaxOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Automatic cursor movement - just follow the rhythm.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct AutopilotOsu {}
impl AutopilotOsu {
    /// The acronym of [`AutopilotOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutopilotOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("SO"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutopilotOsu`]
    pub const fn description() -> &'static str {
        "Automatic cursor movement - just follow the rhythm."
    }
    /// The [`GameModKind`] of [`AutopilotOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutopilotOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8192
    }
}
impl<'de> Deserialize<'de> for AutopilotOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AutopilotOsuVisitor;
        impl<'de> Visitor<'de> for AutopilotOsuVisitor {
            type Value = AutopilotOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AutopilotOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(AutopilotOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutopilotOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Spinners will be automatically completed.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SpunOutOsu {}
impl SpunOutOsu {
    /// The acronym of [`SpunOutOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SO") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SpunOutOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SpunOutOsu`]
    pub const fn description() -> &'static str {
        "Spinners will be automatically completed."
    }
    /// The [`GameModKind`] of [`SpunOutOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`SpunOutOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4096
    }
}
impl<'de> Deserialize<'de> for SpunOutOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SpunOutOsuVisitor;
        impl<'de> Visitor<'de> for SpunOutOsuVisitor {
            type Value = SpunOutOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SpunOutOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SpunOutOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SpunOutOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Everything rotates. EVERYTHING.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct TransformOsu {}
impl TransformOsu {
    /// The acronym of [`TransformOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TransformOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TransformOsu`]
    pub const fn description() -> &'static str {
        "Everything rotates. EVERYTHING."
    }
    /// The [`GameModKind`] of [`TransformOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for TransformOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct TransformOsuVisitor;
        impl<'de> Visitor<'de> for TransformOsuVisitor {
            type Value = TransformOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("TransformOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(TransformOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TransformOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// They just won't stay still...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WiggleOsu {
    pub strength: Option<f32>,
}
impl WiggleOsu {
    /// The acronym of [`WiggleOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WiggleOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("MG"),
                Acronym::from_str_unchecked("RP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WiggleOsu`]
    pub const fn description() -> &'static str {
        "They just won't stay still..."
    }
    /// The [`GameModKind`] of [`WiggleOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WiggleOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WiggleOsuVisitor;
        impl<'de> Visitor<'de> for WiggleOsuVisitor {
            type Value = WiggleOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WiggleOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut strength = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "strength" => strength = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    strength: strength.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WiggleOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WiggleOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.strength.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.strength {
            map.serialize_entry("strength", x)?;
        }
        map.end()
    }
}
/// Circles spin in. No approach circles.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SpinInOsu {}
impl SpinInOsu {
    /// The acronym of [`SpinInOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SI") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SpinInOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SpinInOsu`]
    pub const fn description() -> &'static str {
        "Circles spin in. No approach circles."
    }
    /// The [`GameModKind`] of [`SpinInOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for SpinInOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SpinInOsuVisitor;
        impl<'de> Visitor<'de> for SpinInOsuVisitor {
            type Value = SpinInOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SpinInOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SpinInOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SpinInOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Hit them at the right size!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct GrowOsu {
    pub start_scale: Option<f32>,
}
impl GrowOsu {
    /// The acronym of [`GrowOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("GR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`GrowOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`GrowOsu`]
    pub const fn description() -> &'static str {
        "Hit them at the right size!"
    }
    /// The [`GameModKind`] of [`GrowOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for GrowOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct GrowOsuVisitor;
        impl<'de> Visitor<'de> for GrowOsuVisitor {
            type Value = GrowOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("GrowOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut start_scale = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "start_scale" => start_scale = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    start_scale: start_scale.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(GrowOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GrowOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.start_scale.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.start_scale {
            map.serialize_entry("start_scale", x)?;
        }
        map.end()
    }
}
/// Hit them at the right size!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DeflateOsu {
    pub start_scale: Option<f32>,
}
impl DeflateOsu {
    /// The acronym of [`DeflateOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DeflateOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("TC"),
                Acronym::from_str_unchecked("AD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DeflateOsu`]
    pub const fn description() -> &'static str {
        "Hit them at the right size!"
    }
    /// The [`GameModKind`] of [`DeflateOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for DeflateOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DeflateOsuVisitor;
        impl<'de> Visitor<'de> for DeflateOsuVisitor {
            type Value = DeflateOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DeflateOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut start_scale = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "start_scale" => start_scale = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    start_scale: start_scale.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DeflateOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DeflateOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.start_scale.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.start_scale {
            map.serialize_entry("start_scale", x)?;
        }
        map.end()
    }
}
/// Can you keep up?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindUpOsu {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindUpOsu {
    /// The acronym of [`WindUpOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpOsu`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindUpOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindUpOsuVisitor;
        impl<'de> Visitor<'de> for WindUpOsuVisitor {
            type Value = WindUpOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindUpOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindUpOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindUpOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Sloooow doooown...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindDownOsu {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindDownOsu {
    /// The acronym of [`WindDownOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownOsu`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindDownOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindDownOsuVisitor;
        impl<'de> Visitor<'de> for WindDownOsuVisitor {
            type Value = WindDownOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindDownOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindDownOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindDownOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Put your faith in the approach circles...
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct TraceableOsu {}
impl TraceableOsu {
    /// The acronym of [`TraceableOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TraceableOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TraceableOsu`]
    pub const fn description() -> &'static str {
        "Put your faith in the approach circles..."
    }
    /// The [`GameModKind`] of [`TraceableOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for TraceableOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct TraceableOsuVisitor;
        impl<'de> Visitor<'de> for TraceableOsuVisitor {
            type Value = TraceableOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("TraceableOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(TraceableOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TraceableOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// The whole playfield is on a wheel!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct BarrelRollOsu {
    pub spin_speed: Option<f32>,
    pub direction: Option<String>,
}
impl BarrelRollOsu {
    /// The acronym of [`BarrelRollOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("BR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`BarrelRollOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`BarrelRollOsu`]
    pub const fn description() -> &'static str {
        "The whole playfield is on a wheel!"
    }
    /// The [`GameModKind`] of [`BarrelRollOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for BarrelRollOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct BarrelRollOsuVisitor;
        impl<'de> Visitor<'de> for BarrelRollOsuVisitor {
            type Value = BarrelRollOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("BarrelRollOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut spin_speed = None;
                let mut direction = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "spin_speed" => spin_speed = Some(map.next_value()?),
                        "direction" => direction = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    spin_speed: spin_speed.unwrap_or_default(),
                    direction: direction.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(BarrelRollOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for BarrelRollOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.spin_speed.is_some() as usize + self.direction.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.spin_speed {
            map.serialize_entry("spin_speed", x)?;
        }
        if let Some(ref x) = self.direction {
            map.serialize_entry("direction", x)?;
        }
        map.end()
    }
}
/// Never trust the approach circles...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ApproachDifferentOsu {
    pub scale: Option<f32>,
    pub style: Option<String>,
}
impl ApproachDifferentOsu {
    /// The acronym of [`ApproachDifferentOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ApproachDifferentOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("TP"),
                Acronym::from_str_unchecked("SI"),
                Acronym::from_str_unchecked("GR"),
                Acronym::from_str_unchecked("DF"),
                Acronym::from_str_unchecked("FR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`ApproachDifferentOsu`]
    pub const fn description() -> &'static str {
        "Never trust the approach circles..."
    }
    /// The [`GameModKind`] of [`ApproachDifferentOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for ApproachDifferentOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ApproachDifferentOsuVisitor;
        impl<'de> Visitor<'de> for ApproachDifferentOsuVisitor {
            type Value = ApproachDifferentOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ApproachDifferentOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut scale = None;
                let mut style = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "scale" => scale = Some(map.next_value()?),
                        "style" => style = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    scale: scale.unwrap_or_default(),
                    style: style.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(ApproachDifferentOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ApproachDifferentOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.scale.is_some() as usize + self.style.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.scale {
            map.serialize_entry("scale", x)?;
        }
        if let Some(ref x) = self.style {
            map.serialize_entry("style", x)?;
        }
        map.end()
    }
}
/// Can you still feel the rhythm without music?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MutedOsu {
    pub inverse_muting: Option<bool>,
    pub enable_metronome: Option<bool>,
    pub mute_combo_count: Option<f32>,
    pub affects_hit_sounds: Option<bool>,
}
impl MutedOsu {
    /// The acronym of [`MutedOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedOsu`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for MutedOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MutedOsuVisitor;
        impl<'de> Visitor<'de> for MutedOsuVisitor {
            type Value = MutedOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MutedOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut inverse_muting = None;
                let mut enable_metronome = None;
                let mut mute_combo_count = None;
                let mut affects_hit_sounds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "inverse_muting" => inverse_muting = Some(map.next_value()?),
                        "enable_metronome" => enable_metronome = Some(map.next_value()?),
                        "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                        "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    inverse_muting: inverse_muting.unwrap_or_default(),
                    enable_metronome: enable_metronome.unwrap_or_default(),
                    mute_combo_count: mute_combo_count.unwrap_or_default(),
                    affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(MutedOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MutedOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.inverse_muting.is_some() as usize
            + self.enable_metronome.is_some() as usize
            + self.mute_combo_count.is_some() as usize
            + self.affects_hit_sounds.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.inverse_muting {
            map.serialize_entry("inverse_muting", x)?;
        }
        if let Some(ref x) = self.enable_metronome {
            map.serialize_entry("enable_metronome", x)?;
        }
        if let Some(ref x) = self.mute_combo_count {
            map.serialize_entry("mute_combo_count", x)?;
        }
        if let Some(ref x) = self.affects_hit_sounds {
            map.serialize_entry("affects_hit_sounds", x)?;
        }
        map.end()
    }
}
/// Where's the cursor?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct NoScopeOsu {
    pub hidden_combo_count: Option<f32>,
}
impl NoScopeOsu {
    /// The acronym of [`NoScopeOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoScopeOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`NoScopeOsu`]
    pub const fn description() -> &'static str {
        "Where's the cursor?"
    }
    /// The [`GameModKind`] of [`NoScopeOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for NoScopeOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NoScopeOsuVisitor;
        impl<'de> Visitor<'de> for NoScopeOsuVisitor {
            type Value = NoScopeOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NoScopeOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hidden_combo_count = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "hidden_combo_count" => hidden_combo_count = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    hidden_combo_count: hidden_combo_count.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(NoScopeOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NoScopeOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.hidden_combo_count.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.hidden_combo_count {
            map.serialize_entry("hidden_combo_count", x)?;
        }
        map.end()
    }
}
/// No need to chase the circles – your cursor is a magnet!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MagnetisedOsu {
    pub attraction_strength: Option<f32>,
}
impl MagnetisedOsu {
    /// The acronym of [`MagnetisedOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MagnetisedOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("RP"),
            ]
        }
        .into_iter()
    }
    /// The description of [`MagnetisedOsu`]
    pub const fn description() -> &'static str {
        "No need to chase the circles – your cursor is a magnet!"
    }
    /// The [`GameModKind`] of [`MagnetisedOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for MagnetisedOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MagnetisedOsuVisitor;
        impl<'de> Visitor<'de> for MagnetisedOsuVisitor {
            type Value = MagnetisedOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MagnetisedOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attraction_strength = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "attraction_strength" => attraction_strength = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    attraction_strength: attraction_strength.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(MagnetisedOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MagnetisedOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.attraction_strength.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.attraction_strength {
            map.serialize_entry("attraction_strength", x)?;
        }
        map.end()
    }
}
/// Hit objects run away!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct RepelOsu {
    pub repulsion_strength: Option<f32>,
}
impl RepelOsu {
    /// The acronym of [`RepelOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RP") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RepelOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AP"),
                Acronym::from_str_unchecked("TR"),
                Acronym::from_str_unchecked("WG"),
                Acronym::from_str_unchecked("MG"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RepelOsu`]
    pub const fn description() -> &'static str {
        "Hit objects run away!"
    }
    /// The [`GameModKind`] of [`RepelOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for RepelOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RepelOsuVisitor;
        impl<'de> Visitor<'de> for RepelOsuVisitor {
            type Value = RepelOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RepelOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut repulsion_strength = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "repulsion_strength" => repulsion_strength = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    repulsion_strength: repulsion_strength.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(RepelOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RepelOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.repulsion_strength.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.repulsion_strength {
            map.serialize_entry("repulsion_strength", x)?;
        }
        map.end()
    }
}
/// Let track speed adapt to you.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AdaptiveSpeedOsu {
    pub initial_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl AdaptiveSpeedOsu {
    /// The acronym of [`AdaptiveSpeedOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AdaptiveSpeedOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AdaptiveSpeedOsu`]
    pub const fn description() -> &'static str {
        "Let track speed adapt to you."
    }
    /// The [`GameModKind`] of [`AdaptiveSpeedOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for AdaptiveSpeedOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AdaptiveSpeedOsuVisitor;
        impl<'de> Visitor<'de> for AdaptiveSpeedOsuVisitor {
            type Value = AdaptiveSpeedOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AdaptiveSpeedOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AdaptiveSpeedOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AdaptiveSpeedOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.initial_rate.is_some() as usize + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Burn the notes into your memory.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct FreezeFrameOsu {}
impl FreezeFrameOsu {
    /// The acronym of [`FreezeFrameOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FreezeFrameOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("AD")] }.into_iter()
    }
    /// The description of [`FreezeFrameOsu`]
    pub const fn description() -> &'static str {
        "Burn the notes into your memory."
    }
    /// The [`GameModKind`] of [`FreezeFrameOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for FreezeFrameOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FreezeFrameOsuVisitor;
        impl<'de> Visitor<'de> for FreezeFrameOsuVisitor {
            type Value = FreezeFrameOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FreezeFrameOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(FreezeFrameOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FreezeFrameOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Automatically applied to plays on devices with a touchscreen.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct TouchDeviceOsu {}
impl TouchDeviceOsu {
    /// The acronym of [`TouchDeviceOsu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("TD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TouchDeviceOsu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`TouchDeviceOsu`]
    pub const fn description() -> &'static str {
        "Automatically applied to plays on devices with a touchscreen."
    }
    /// The [`GameModKind`] of [`TouchDeviceOsu`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`TouchDeviceOsu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4
    }
}
impl<'de> Deserialize<'de> for TouchDeviceOsu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct TouchDeviceOsuVisitor;
        impl<'de> Visitor<'de> for TouchDeviceOsuVisitor {
            type Value = TouchDeviceOsu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("TouchDeviceOsu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(TouchDeviceOsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TouchDeviceOsu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Uses the V2 scoring system
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ScoreV2Osu {}
impl ScoreV2Osu {
    /// The acronym of [`ScoreV2Osu`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("V2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Osu`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Osu`]
    pub const fn description() -> &'static str {
        "Uses the V2 scoring system"
    }
    /// The [`GameModKind`] of [`ScoreV2Osu`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Osu`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl<'de> Deserialize<'de> for ScoreV2Osu {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ScoreV2OsuVisitor;
        impl<'de> Visitor<'de> for ScoreV2OsuVisitor {
            type Value = ScoreV2Osu;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ScoreV2Osu")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ScoreV2OsuVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ScoreV2Osu {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Beats move slower, and less accuracy required!
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct EasyTaiko {}
impl EasyTaiko {
    /// The acronym of [`EasyTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyTaiko`]
    pub const fn description() -> &'static str {
        "Beats move slower, and less accuracy required!"
    }
    /// The [`GameModKind`] of [`EasyTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl<'de> Deserialize<'de> for EasyTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct EasyTaikoVisitor;
        impl<'de> Visitor<'de> for EasyTaikoVisitor {
            type Value = EasyTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("EasyTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(EasyTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EasyTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// You can't fail, no matter what.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct NoFailTaiko {}
impl NoFailTaiko {
    /// The acronym of [`NoFailTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailTaiko`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl<'de> Deserialize<'de> for NoFailTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NoFailTaikoVisitor;
        impl<'de> Visitor<'de> for NoFailTaikoVisitor {
            type Value = NoFailTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NoFailTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(NoFailTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NoFailTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Less zoom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HalfTimeTaiko {
    pub speed_change: Option<f32>,
}
impl HalfTimeTaiko {
    /// The acronym of [`HalfTimeTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeTaiko`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl<'de> Deserialize<'de> for HalfTimeTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HalfTimeTaikoVisitor;
        impl<'de> Visitor<'de> for HalfTimeTaikoVisitor {
            type Value = HalfTimeTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HalfTimeTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(HalfTimeTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HalfTimeTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Whoaaaaa...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DaycoreTaiko {
    pub speed_change: Option<f32>,
}
impl DaycoreTaiko {
    /// The acronym of [`DaycoreTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreTaiko`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl<'de> Deserialize<'de> for DaycoreTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DaycoreTaikoVisitor;
        impl<'de> Visitor<'de> for DaycoreTaikoVisitor {
            type Value = DaycoreTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DaycoreTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DaycoreTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DaycoreTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Everything just got a bit harder...
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HardRockTaiko {}
impl HardRockTaiko {
    /// The acronym of [`HardRockTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockTaiko`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl<'de> Deserialize<'de> for HardRockTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HardRockTaikoVisitor;
        impl<'de> Visitor<'de> for HardRockTaikoVisitor {
            type Value = HardRockTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HardRockTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HardRockTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HardRockTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Miss and fail.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct SuddenDeathTaiko {
    pub restart: Option<bool>,
}
impl SuddenDeathTaiko {
    /// The acronym of [`SuddenDeathTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathTaiko`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl<'de> Deserialize<'de> for SuddenDeathTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SuddenDeathTaikoVisitor;
        impl<'de> Visitor<'de> for SuddenDeathTaikoVisitor {
            type Value = SuddenDeathTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SuddenDeathTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(SuddenDeathTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SuddenDeathTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// SS or quit.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct PerfectTaiko {
    pub restart: Option<bool>,
}
impl PerfectTaiko {
    /// The acronym of [`PerfectTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectTaiko`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl<'de> Deserialize<'de> for PerfectTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct PerfectTaikoVisitor;
        impl<'de> Visitor<'de> for PerfectTaikoVisitor {
            type Value = PerfectTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("PerfectTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(PerfectTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PerfectTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Zoooooooooom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DoubleTimeTaiko {
    pub speed_change: Option<f32>,
}
impl DoubleTimeTaiko {
    /// The acronym of [`DoubleTimeTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeTaiko`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl<'de> Deserialize<'de> for DoubleTimeTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DoubleTimeTaikoVisitor;
        impl<'de> Visitor<'de> for DoubleTimeTaikoVisitor {
            type Value = DoubleTimeTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DoubleTimeTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DoubleTimeTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DoubleTimeTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Uguuuuuuuu...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct NightcoreTaiko {
    pub speed_change: Option<f32>,
}
impl NightcoreTaiko {
    /// The acronym of [`NightcoreTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreTaiko`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl<'de> Deserialize<'de> for NightcoreTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NightcoreTaikoVisitor;
        impl<'de> Visitor<'de> for NightcoreTaikoVisitor {
            type Value = NightcoreTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NightcoreTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(NightcoreTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NightcoreTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Beats fade out before you hit them!
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HiddenTaiko {}
impl HiddenTaiko {
    /// The acronym of [`HiddenTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`HiddenTaiko`]
    pub const fn description() -> &'static str {
        "Beats fade out before you hit them!"
    }
    /// The [`GameModKind`] of [`HiddenTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl<'de> Deserialize<'de> for HiddenTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HiddenTaikoVisitor;
        impl<'de> Visitor<'de> for HiddenTaikoVisitor {
            type Value = HiddenTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HiddenTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HiddenTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HiddenTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Restricted view area.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct FlashlightTaiko {
    pub size_multiplier: Option<f32>,
    pub combo_based_size: Option<bool>,
}
impl FlashlightTaiko {
    /// The acronym of [`FlashlightTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`FlashlightTaiko`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl<'de> Deserialize<'de> for FlashlightTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FlashlightTaikoVisitor;
        impl<'de> Visitor<'de> for FlashlightTaikoVisitor {
            type Value = FlashlightTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FlashlightTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut size_multiplier = None;
                let mut combo_based_size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "size_multiplier" => size_multiplier = Some(map.next_value()?),
                        "combo_based_size" => combo_based_size = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    size_multiplier: size_multiplier.unwrap_or_default(),
                    combo_based_size: combo_based_size.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(FlashlightTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FlashlightTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.size_multiplier.is_some() as usize + self.combo_based_size.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.size_multiplier {
            map.serialize_entry("size_multiplier", x)?;
        }
        if let Some(ref x) = self.combo_based_size {
            map.serialize_entry("combo_based_size", x)?;
        }
        map.end()
    }
}
/// Fail if your accuracy drops too low!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AccuracyChallengeTaiko {
    pub minimum_accuracy: Option<f32>,
    pub restart: Option<bool>,
}
impl AccuracyChallengeTaiko {
    /// The acronym of [`AccuracyChallengeTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeTaiko`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl<'de> Deserialize<'de> for AccuracyChallengeTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AccuracyChallengeTaikoVisitor;
        impl<'de> Visitor<'de> for AccuracyChallengeTaikoVisitor {
            type Value = AccuracyChallengeTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AccuracyChallengeTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut minimum_accuracy = None;
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AccuracyChallengeTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccuracyChallengeTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.minimum_accuracy.is_some() as usize + self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.minimum_accuracy {
            map.serialize_entry("minimum_accuracy", x)?;
        }
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Shuffle around the colours!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct RandomTaiko {
    pub seed: Option<f32>,
}
impl RandomTaiko {
    /// The acronym of [`RandomTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RandomTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("SW")] }.into_iter()
    }
    /// The description of [`RandomTaiko`]
    pub const fn description() -> &'static str {
        "Shuffle around the colours!"
    }
    /// The [`GameModKind`] of [`RandomTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`RandomTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2097152
    }
}
impl<'de> Deserialize<'de> for RandomTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RandomTaikoVisitor;
        impl<'de> Visitor<'de> for RandomTaikoVisitor {
            type Value = RandomTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RandomTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "seed" => seed = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    seed: seed.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(RandomTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RandomTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.seed.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.seed {
            map.serialize_entry("seed", x)?;
        }
        map.end()
    }
}
/// Override a beatmap's difficulty settings.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DifficultyAdjustTaiko {
    pub scroll_speed: Option<f32>,
    pub drain_rate: Option<f32>,
    pub overall_difficulty: Option<f32>,
    pub extended_limits: Option<bool>,
}
impl DifficultyAdjustTaiko {
    /// The acronym of [`DifficultyAdjustTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustTaiko`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for DifficultyAdjustTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DifficultyAdjustTaikoVisitor;
        impl<'de> Visitor<'de> for DifficultyAdjustTaikoVisitor {
            type Value = DifficultyAdjustTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DifficultyAdjustTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut scroll_speed = None;
                let mut drain_rate = None;
                let mut overall_difficulty = None;
                let mut extended_limits = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "scroll_speed" => scroll_speed = Some(map.next_value()?),
                        "drain_rate" => drain_rate = Some(map.next_value()?),
                        "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                        "extended_limits" => extended_limits = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    scroll_speed: scroll_speed.unwrap_or_default(),
                    drain_rate: drain_rate.unwrap_or_default(),
                    overall_difficulty: overall_difficulty.unwrap_or_default(),
                    extended_limits: extended_limits.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DifficultyAdjustTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DifficultyAdjustTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.scroll_speed.is_some() as usize
            + self.drain_rate.is_some() as usize
            + self.overall_difficulty.is_some() as usize
            + self.extended_limits.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.scroll_speed {
            map.serialize_entry("scroll_speed", x)?;
        }
        if let Some(ref x) = self.drain_rate {
            map.serialize_entry("drain_rate", x)?;
        }
        if let Some(ref x) = self.overall_difficulty {
            map.serialize_entry("overall_difficulty", x)?;
        }
        if let Some(ref x) = self.extended_limits {
            map.serialize_entry("extended_limits", x)?;
        }
        map.end()
    }
}
/// Feeling nostalgic?
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ClassicTaiko {}
impl ClassicTaiko {
    /// The acronym of [`ClassicTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ClassicTaiko`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for ClassicTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ClassicTaikoVisitor;
        impl<'de> Visitor<'de> for ClassicTaikoVisitor {
            type Value = ClassicTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ClassicTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ClassicTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClassicTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Dons become kats, kats become dons
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SwapTaiko {}
impl SwapTaiko {
    /// The acronym of [`SwapTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SW") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SwapTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("RD")] }.into_iter()
    }
    /// The description of [`SwapTaiko`]
    pub const fn description() -> &'static str {
        "Dons become kats, kats become dons"
    }
    /// The [`GameModKind`] of [`SwapTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for SwapTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SwapTaikoVisitor;
        impl<'de> Visitor<'de> for SwapTaikoVisitor {
            type Value = SwapTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SwapTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SwapTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SwapTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// One key for dons, one key for kats.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SingleTapTaiko {}
impl SingleTapTaiko {
    /// The acronym of [`SingleTapTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SG") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SingleTapTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SingleTapTaiko`]
    pub const fn description() -> &'static str {
        "One key for dons, one key for kats."
    }
    /// The [`GameModKind`] of [`SingleTapTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for SingleTapTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SingleTapTaikoVisitor;
        impl<'de> Visitor<'de> for SingleTapTaikoVisitor {
            type Value = SingleTapTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SingleTapTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SingleTapTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SingleTapTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch a perfect automated play through the song.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct AutoplayTaiko {}
impl AutoplayTaiko {
    /// The acronym of [`AutoplayTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayTaiko`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl<'de> Deserialize<'de> for AutoplayTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AutoplayTaikoVisitor;
        impl<'de> Visitor<'de> for AutoplayTaikoVisitor {
            type Value = AutoplayTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AutoplayTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(AutoplayTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutoplayTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch the video without visual distractions.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct CinemaTaiko {}
impl CinemaTaiko {
    /// The acronym of [`CinemaTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("RX"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaTaiko`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl<'de> Deserialize<'de> for CinemaTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct CinemaTaikoVisitor;
        impl<'de> Visitor<'de> for CinemaTaikoVisitor {
            type Value = CinemaTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("CinemaTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(CinemaTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CinemaTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// No ninja-like spinners, demanding drumrolls or unexpected katus.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct RelaxTaiko {}
impl RelaxTaiko {
    /// The acronym of [`RelaxTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RX") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RelaxTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("SG"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RelaxTaiko`]
    pub const fn description() -> &'static str {
        "No ninja-like spinners, demanding drumrolls or unexpected katus."
    }
    /// The [`GameModKind`] of [`RelaxTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`RelaxTaiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        128
    }
}
impl<'de> Deserialize<'de> for RelaxTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RelaxTaikoVisitor;
        impl<'de> Visitor<'de> for RelaxTaikoVisitor {
            type Value = RelaxTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RelaxTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(RelaxTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RelaxTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Can you keep up?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindUpTaiko {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindUpTaiko {
    /// The acronym of [`WindUpTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpTaiko`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindUpTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindUpTaikoVisitor;
        impl<'de> Visitor<'de> for WindUpTaikoVisitor {
            type Value = WindUpTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindUpTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindUpTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindUpTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Sloooow doooown...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindDownTaiko {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindDownTaiko {
    /// The acronym of [`WindDownTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownTaiko`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindDownTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindDownTaikoVisitor;
        impl<'de> Visitor<'de> for WindDownTaikoVisitor {
            type Value = WindDownTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindDownTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindDownTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindDownTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Can you still feel the rhythm without music?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MutedTaiko {
    pub inverse_muting: Option<bool>,
    pub enable_metronome: Option<bool>,
    pub mute_combo_count: Option<f32>,
    pub affects_hit_sounds: Option<bool>,
}
impl MutedTaiko {
    /// The acronym of [`MutedTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedTaiko`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for MutedTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MutedTaikoVisitor;
        impl<'de> Visitor<'de> for MutedTaikoVisitor {
            type Value = MutedTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MutedTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut inverse_muting = None;
                let mut enable_metronome = None;
                let mut mute_combo_count = None;
                let mut affects_hit_sounds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "inverse_muting" => inverse_muting = Some(map.next_value()?),
                        "enable_metronome" => enable_metronome = Some(map.next_value()?),
                        "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                        "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    inverse_muting: inverse_muting.unwrap_or_default(),
                    enable_metronome: enable_metronome.unwrap_or_default(),
                    mute_combo_count: mute_combo_count.unwrap_or_default(),
                    affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(MutedTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MutedTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.inverse_muting.is_some() as usize
            + self.enable_metronome.is_some() as usize
            + self.mute_combo_count.is_some() as usize
            + self.affects_hit_sounds.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.inverse_muting {
            map.serialize_entry("inverse_muting", x)?;
        }
        if let Some(ref x) = self.enable_metronome {
            map.serialize_entry("enable_metronome", x)?;
        }
        if let Some(ref x) = self.mute_combo_count {
            map.serialize_entry("mute_combo_count", x)?;
        }
        if let Some(ref x) = self.affects_hit_sounds {
            map.serialize_entry("affects_hit_sounds", x)?;
        }
        map.end()
    }
}
/// Let track speed adapt to you.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AdaptiveSpeedTaiko {
    pub initial_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl AdaptiveSpeedTaiko {
    /// The acronym of [`AdaptiveSpeedTaiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AdaptiveSpeedTaiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AdaptiveSpeedTaiko`]
    pub const fn description() -> &'static str {
        "Let track speed adapt to you."
    }
    /// The [`GameModKind`] of [`AdaptiveSpeedTaiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for AdaptiveSpeedTaiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AdaptiveSpeedTaikoVisitor;
        impl<'de> Visitor<'de> for AdaptiveSpeedTaikoVisitor {
            type Value = AdaptiveSpeedTaiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AdaptiveSpeedTaiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AdaptiveSpeedTaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AdaptiveSpeedTaiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.initial_rate.is_some() as usize + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Uses the V2 scoring system
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ScoreV2Taiko {}
impl ScoreV2Taiko {
    /// The acronym of [`ScoreV2Taiko`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("V2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Taiko`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Taiko`]
    pub const fn description() -> &'static str {
        "Uses the V2 scoring system"
    }
    /// The [`GameModKind`] of [`ScoreV2Taiko`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Taiko`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl<'de> Deserialize<'de> for ScoreV2Taiko {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ScoreV2TaikoVisitor;
        impl<'de> Visitor<'de> for ScoreV2TaikoVisitor {
            type Value = ScoreV2Taiko;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ScoreV2Taiko")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ScoreV2TaikoVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ScoreV2Taiko {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Larger fruits, more forgiving HP drain, less accuracy required, and three lives!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct EasyCatch {
    pub retries: Option<f32>,
}
impl EasyCatch {
    /// The acronym of [`EasyCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyCatch`]
    pub const fn description() -> &'static str {
        "Larger fruits, more forgiving HP drain, less accuracy required, and three lives!"
    }
    /// The [`GameModKind`] of [`EasyCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl<'de> Deserialize<'de> for EasyCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct EasyCatchVisitor;
        impl<'de> Visitor<'de> for EasyCatchVisitor {
            type Value = EasyCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("EasyCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut retries = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "retries" => retries = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    retries: retries.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(EasyCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EasyCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.retries.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.retries {
            map.serialize_entry("retries", x)?;
        }
        map.end()
    }
}
/// You can't fail, no matter what.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct NoFailCatch {}
impl NoFailCatch {
    /// The acronym of [`NoFailCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailCatch`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl<'de> Deserialize<'de> for NoFailCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NoFailCatchVisitor;
        impl<'de> Visitor<'de> for NoFailCatchVisitor {
            type Value = NoFailCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NoFailCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(NoFailCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NoFailCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Less zoom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HalfTimeCatch {
    pub speed_change: Option<f32>,
}
impl HalfTimeCatch {
    /// The acronym of [`HalfTimeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeCatch`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl<'de> Deserialize<'de> for HalfTimeCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HalfTimeCatchVisitor;
        impl<'de> Visitor<'de> for HalfTimeCatchVisitor {
            type Value = HalfTimeCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HalfTimeCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(HalfTimeCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HalfTimeCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Whoaaaaa...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DaycoreCatch {
    pub speed_change: Option<f32>,
}
impl DaycoreCatch {
    /// The acronym of [`DaycoreCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreCatch`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl<'de> Deserialize<'de> for DaycoreCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DaycoreCatchVisitor;
        impl<'de> Visitor<'de> for DaycoreCatchVisitor {
            type Value = DaycoreCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DaycoreCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DaycoreCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DaycoreCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Everything just got a bit harder...
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HardRockCatch {}
impl HardRockCatch {
    /// The acronym of [`HardRockCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockCatch`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl<'de> Deserialize<'de> for HardRockCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HardRockCatchVisitor;
        impl<'de> Visitor<'de> for HardRockCatchVisitor {
            type Value = HardRockCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HardRockCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HardRockCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HardRockCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Miss and fail.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct SuddenDeathCatch {
    pub restart: Option<bool>,
}
impl SuddenDeathCatch {
    /// The acronym of [`SuddenDeathCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathCatch`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl<'de> Deserialize<'de> for SuddenDeathCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SuddenDeathCatchVisitor;
        impl<'de> Visitor<'de> for SuddenDeathCatchVisitor {
            type Value = SuddenDeathCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SuddenDeathCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(SuddenDeathCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SuddenDeathCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// SS or quit.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct PerfectCatch {
    pub restart: Option<bool>,
}
impl PerfectCatch {
    /// The acronym of [`PerfectCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectCatch`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl<'de> Deserialize<'de> for PerfectCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct PerfectCatchVisitor;
        impl<'de> Visitor<'de> for PerfectCatchVisitor {
            type Value = PerfectCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("PerfectCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(PerfectCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PerfectCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Zoooooooooom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DoubleTimeCatch {
    pub speed_change: Option<f32>,
}
impl DoubleTimeCatch {
    /// The acronym of [`DoubleTimeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeCatch`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl<'de> Deserialize<'de> for DoubleTimeCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DoubleTimeCatchVisitor;
        impl<'de> Visitor<'de> for DoubleTimeCatchVisitor {
            type Value = DoubleTimeCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DoubleTimeCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DoubleTimeCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DoubleTimeCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Uguuuuuuuu...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct NightcoreCatch {
    pub speed_change: Option<f32>,
}
impl NightcoreCatch {
    /// The acronym of [`NightcoreCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreCatch`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl<'de> Deserialize<'de> for NightcoreCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NightcoreCatchVisitor;
        impl<'de> Visitor<'de> for NightcoreCatchVisitor {
            type Value = NightcoreCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NightcoreCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(NightcoreCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NightcoreCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Play with fading fruits.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HiddenCatch {}
impl HiddenCatch {
    /// The acronym of [`HiddenCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`HiddenCatch`]
    pub const fn description() -> &'static str {
        "Play with fading fruits."
    }
    /// The [`GameModKind`] of [`HiddenCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl<'de> Deserialize<'de> for HiddenCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HiddenCatchVisitor;
        impl<'de> Visitor<'de> for HiddenCatchVisitor {
            type Value = HiddenCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HiddenCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HiddenCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HiddenCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Restricted view area.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct FlashlightCatch {
    pub size_multiplier: Option<f32>,
    pub combo_based_size: Option<bool>,
}
impl FlashlightCatch {
    /// The acronym of [`FlashlightCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`FlashlightCatch`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl<'de> Deserialize<'de> for FlashlightCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FlashlightCatchVisitor;
        impl<'de> Visitor<'de> for FlashlightCatchVisitor {
            type Value = FlashlightCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FlashlightCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut size_multiplier = None;
                let mut combo_based_size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "size_multiplier" => size_multiplier = Some(map.next_value()?),
                        "combo_based_size" => combo_based_size = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    size_multiplier: size_multiplier.unwrap_or_default(),
                    combo_based_size: combo_based_size.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(FlashlightCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FlashlightCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.size_multiplier.is_some() as usize + self.combo_based_size.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.size_multiplier {
            map.serialize_entry("size_multiplier", x)?;
        }
        if let Some(ref x) = self.combo_based_size {
            map.serialize_entry("combo_based_size", x)?;
        }
        map.end()
    }
}
/// Fail if your accuracy drops too low!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AccuracyChallengeCatch {
    pub minimum_accuracy: Option<f32>,
    pub restart: Option<bool>,
}
impl AccuracyChallengeCatch {
    /// The acronym of [`AccuracyChallengeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeCatch`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl<'de> Deserialize<'de> for AccuracyChallengeCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AccuracyChallengeCatchVisitor;
        impl<'de> Visitor<'de> for AccuracyChallengeCatchVisitor {
            type Value = AccuracyChallengeCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AccuracyChallengeCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut minimum_accuracy = None;
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AccuracyChallengeCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccuracyChallengeCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.minimum_accuracy.is_some() as usize + self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.minimum_accuracy {
            map.serialize_entry("minimum_accuracy", x)?;
        }
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Override a beatmap's difficulty settings.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DifficultyAdjustCatch {
    pub circle_size: Option<f32>,
    pub approach_rate: Option<f32>,
    pub hard_rock_offsets: Option<bool>,
    pub drain_rate: Option<f32>,
    pub overall_difficulty: Option<f32>,
    pub extended_limits: Option<bool>,
}
impl DifficultyAdjustCatch {
    /// The acronym of [`DifficultyAdjustCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustCatch`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for DifficultyAdjustCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DifficultyAdjustCatchVisitor;
        impl<'de> Visitor<'de> for DifficultyAdjustCatchVisitor {
            type Value = DifficultyAdjustCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DifficultyAdjustCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut circle_size = None;
                let mut approach_rate = None;
                let mut hard_rock_offsets = None;
                let mut drain_rate = None;
                let mut overall_difficulty = None;
                let mut extended_limits = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "circle_size" => circle_size = Some(map.next_value()?),
                        "approach_rate" => approach_rate = Some(map.next_value()?),
                        "hard_rock_offsets" => hard_rock_offsets = Some(map.next_value()?),
                        "drain_rate" => drain_rate = Some(map.next_value()?),
                        "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                        "extended_limits" => extended_limits = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    circle_size: circle_size.unwrap_or_default(),
                    approach_rate: approach_rate.unwrap_or_default(),
                    hard_rock_offsets: hard_rock_offsets.unwrap_or_default(),
                    drain_rate: drain_rate.unwrap_or_default(),
                    overall_difficulty: overall_difficulty.unwrap_or_default(),
                    extended_limits: extended_limits.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DifficultyAdjustCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DifficultyAdjustCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.circle_size.is_some() as usize
            + self.approach_rate.is_some() as usize
            + self.hard_rock_offsets.is_some() as usize
            + self.drain_rate.is_some() as usize
            + self.overall_difficulty.is_some() as usize
            + self.extended_limits.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.circle_size {
            map.serialize_entry("circle_size", x)?;
        }
        if let Some(ref x) = self.approach_rate {
            map.serialize_entry("approach_rate", x)?;
        }
        if let Some(ref x) = self.hard_rock_offsets {
            map.serialize_entry("hard_rock_offsets", x)?;
        }
        if let Some(ref x) = self.drain_rate {
            map.serialize_entry("drain_rate", x)?;
        }
        if let Some(ref x) = self.overall_difficulty {
            map.serialize_entry("overall_difficulty", x)?;
        }
        if let Some(ref x) = self.extended_limits {
            map.serialize_entry("extended_limits", x)?;
        }
        map.end()
    }
}
/// Feeling nostalgic?
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ClassicCatch {}
impl ClassicCatch {
    /// The acronym of [`ClassicCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ClassicCatch`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for ClassicCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ClassicCatchVisitor;
        impl<'de> Visitor<'de> for ClassicCatchVisitor {
            type Value = ClassicCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ClassicCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ClassicCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClassicCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Fruits are flipped horizontally.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct MirrorCatch {}
impl MirrorCatch {
    /// The acronym of [`MirrorCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MirrorCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MirrorCatch`]
    pub const fn description() -> &'static str {
        "Fruits are flipped horizontally."
    }
    /// The [`GameModKind`] of [`MirrorCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`MirrorCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1073741824
    }
}
impl<'de> Deserialize<'de> for MirrorCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MirrorCatchVisitor;
        impl<'de> Visitor<'de> for MirrorCatchVisitor {
            type Value = MirrorCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MirrorCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(MirrorCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MirrorCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch a perfect automated play through the song.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct AutoplayCatch {}
impl AutoplayCatch {
    /// The acronym of [`AutoplayCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayCatch`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl<'de> Deserialize<'de> for AutoplayCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AutoplayCatchVisitor;
        impl<'de> Visitor<'de> for AutoplayCatchVisitor {
            type Value = AutoplayCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AutoplayCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(AutoplayCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutoplayCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch the video without visual distractions.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct CinemaCatch {}
impl CinemaCatch {
    /// The acronym of [`CinemaCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("RX"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaCatch`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl<'de> Deserialize<'de> for CinemaCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct CinemaCatchVisitor;
        impl<'de> Visitor<'de> for CinemaCatchVisitor {
            type Value = CinemaCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("CinemaCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(CinemaCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CinemaCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Use the mouse to control the catcher.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct RelaxCatch {}
impl RelaxCatch {
    /// The acronym of [`RelaxCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RX") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RelaxCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`RelaxCatch`]
    pub const fn description() -> &'static str {
        "Use the mouse to control the catcher."
    }
    /// The [`GameModKind`] of [`RelaxCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`RelaxCatch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        128
    }
}
impl<'de> Deserialize<'de> for RelaxCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RelaxCatchVisitor;
        impl<'de> Visitor<'de> for RelaxCatchVisitor {
            type Value = RelaxCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RelaxCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(RelaxCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RelaxCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Can you keep up?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindUpCatch {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindUpCatch {
    /// The acronym of [`WindUpCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpCatch`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindUpCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindUpCatchVisitor;
        impl<'de> Visitor<'de> for WindUpCatchVisitor {
            type Value = WindUpCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindUpCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindUpCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindUpCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Sloooow doooown...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindDownCatch {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindDownCatch {
    /// The acronym of [`WindDownCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownCatch`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindDownCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindDownCatchVisitor;
        impl<'de> Visitor<'de> for WindDownCatchVisitor {
            type Value = WindDownCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindDownCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindDownCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindDownCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// The fruits are... floating?
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct FloatingFruitsCatch {}
impl FloatingFruitsCatch {
    /// The acronym of [`FloatingFruitsCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FloatingFruitsCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`FloatingFruitsCatch`]
    pub const fn description() -> &'static str {
        "The fruits are... floating?"
    }
    /// The [`GameModKind`] of [`FloatingFruitsCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for FloatingFruitsCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FloatingFruitsCatchVisitor;
        impl<'de> Visitor<'de> for FloatingFruitsCatchVisitor {
            type Value = FloatingFruitsCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FloatingFruitsCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(FloatingFruitsCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FloatingFruitsCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Can you still feel the rhythm without music?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MutedCatch {
    pub inverse_muting: Option<bool>,
    pub enable_metronome: Option<bool>,
    pub mute_combo_count: Option<f32>,
    pub affects_hit_sounds: Option<bool>,
}
impl MutedCatch {
    /// The acronym of [`MutedCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedCatch`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for MutedCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MutedCatchVisitor;
        impl<'de> Visitor<'de> for MutedCatchVisitor {
            type Value = MutedCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MutedCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut inverse_muting = None;
                let mut enable_metronome = None;
                let mut mute_combo_count = None;
                let mut affects_hit_sounds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "inverse_muting" => inverse_muting = Some(map.next_value()?),
                        "enable_metronome" => enable_metronome = Some(map.next_value()?),
                        "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                        "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    inverse_muting: inverse_muting.unwrap_or_default(),
                    enable_metronome: enable_metronome.unwrap_or_default(),
                    mute_combo_count: mute_combo_count.unwrap_or_default(),
                    affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(MutedCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MutedCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.inverse_muting.is_some() as usize
            + self.enable_metronome.is_some() as usize
            + self.mute_combo_count.is_some() as usize
            + self.affects_hit_sounds.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.inverse_muting {
            map.serialize_entry("inverse_muting", x)?;
        }
        if let Some(ref x) = self.enable_metronome {
            map.serialize_entry("enable_metronome", x)?;
        }
        if let Some(ref x) = self.mute_combo_count {
            map.serialize_entry("mute_combo_count", x)?;
        }
        if let Some(ref x) = self.affects_hit_sounds {
            map.serialize_entry("affects_hit_sounds", x)?;
        }
        map.end()
    }
}
/// Where's the catcher?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct NoScopeCatch {
    pub hidden_combo_count: Option<f32>,
}
impl NoScopeCatch {
    /// The acronym of [`NoScopeCatch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoScopeCatch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`NoScopeCatch`]
    pub const fn description() -> &'static str {
        "Where's the catcher?"
    }
    /// The [`GameModKind`] of [`NoScopeCatch`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for NoScopeCatch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NoScopeCatchVisitor;
        impl<'de> Visitor<'de> for NoScopeCatchVisitor {
            type Value = NoScopeCatch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NoScopeCatch")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hidden_combo_count = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "hidden_combo_count" => hidden_combo_count = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    hidden_combo_count: hidden_combo_count.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(NoScopeCatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NoScopeCatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.hidden_combo_count.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.hidden_combo_count {
            map.serialize_entry("hidden_combo_count", x)?;
        }
        map.end()
    }
}
/// Uses the V2 scoring system
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ScoreV2Catch {}
impl ScoreV2Catch {
    /// The acronym of [`ScoreV2Catch`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("V2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Catch`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Catch`]
    pub const fn description() -> &'static str {
        "Uses the V2 scoring system"
    }
    /// The [`GameModKind`] of [`ScoreV2Catch`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Catch`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl<'de> Deserialize<'de> for ScoreV2Catch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ScoreV2CatchVisitor;
        impl<'de> Visitor<'de> for ScoreV2CatchVisitor {
            type Value = ScoreV2Catch;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ScoreV2Catch")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ScoreV2CatchVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ScoreV2Catch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// More forgiving HP drain, less accuracy required, and three lives!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct EasyMania {
    pub retries: Option<f32>,
}
impl EasyMania {
    /// The acronym of [`EasyMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("EZ") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EasyMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HR"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EasyMania`]
    pub const fn description() -> &'static str {
        "More forgiving HP drain, less accuracy required, and three lives!"
    }
    /// The [`GameModKind`] of [`EasyMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`EasyMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2
    }
}
impl<'de> Deserialize<'de> for EasyMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct EasyManiaVisitor;
        impl<'de> Visitor<'de> for EasyManiaVisitor {
            type Value = EasyMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("EasyMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut retries = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "retries" => retries = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    retries: retries.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(EasyManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EasyMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.retries.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.retries {
            map.serialize_entry("retries", x)?;
        }
        map.end()
    }
}
/// You can't fail, no matter what.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct NoFailMania {}
impl NoFailMania {
    /// The acronym of [`NoFailMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NoFailMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NoFailMania`]
    pub const fn description() -> &'static str {
        "You can't fail, no matter what."
    }
    /// The [`GameModKind`] of [`NoFailMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`NoFailMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1
    }
}
impl<'de> Deserialize<'de> for NoFailMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NoFailManiaVisitor;
        impl<'de> Visitor<'de> for NoFailManiaVisitor {
            type Value = NoFailMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NoFailMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(NoFailManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NoFailMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Less zoom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HalfTimeMania {
    pub speed_change: Option<f32>,
}
impl HalfTimeMania {
    /// The acronym of [`HalfTimeMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HalfTimeMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HalfTimeMania`]
    pub const fn description() -> &'static str {
        "Less zoom..."
    }
    /// The [`GameModKind`] of [`HalfTimeMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
    /// Bit value of [`HalfTimeMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        256
    }
}
impl<'de> Deserialize<'de> for HalfTimeMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HalfTimeManiaVisitor;
        impl<'de> Visitor<'de> for HalfTimeManiaVisitor {
            type Value = HalfTimeMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HalfTimeMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(HalfTimeManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HalfTimeMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Whoaaaaa...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DaycoreMania {
    pub speed_change: Option<f32>,
}
impl DaycoreMania {
    /// The acronym of [`DaycoreMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DaycoreMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DaycoreMania`]
    pub const fn description() -> &'static str {
        "Whoaaaaa..."
    }
    /// The [`GameModKind`] of [`DaycoreMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyReduction
    }
}
impl<'de> Deserialize<'de> for DaycoreMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DaycoreManiaVisitor;
        impl<'de> Visitor<'de> for DaycoreManiaVisitor {
            type Value = DaycoreMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DaycoreMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DaycoreManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DaycoreMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Everything just got a bit harder...
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HardRockMania {}
impl HardRockMania {
    /// The acronym of [`HardRockMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HardRockMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("DA"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HardRockMania`]
    pub const fn description() -> &'static str {
        "Everything just got a bit harder..."
    }
    /// The [`GameModKind`] of [`HardRockMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HardRockMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16
    }
}
impl<'de> Deserialize<'de> for HardRockMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HardRockManiaVisitor;
        impl<'de> Visitor<'de> for HardRockManiaVisitor {
            type Value = HardRockMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HardRockMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HardRockManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HardRockMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Miss and fail.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct SuddenDeathMania {
    pub restart: Option<bool>,
}
impl SuddenDeathMania {
    /// The acronym of [`SuddenDeathMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("SD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SuddenDeathMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SuddenDeathMania`]
    pub const fn description() -> &'static str {
        "Miss and fail."
    }
    /// The [`GameModKind`] of [`SuddenDeathMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`SuddenDeathMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32
    }
}
impl<'de> Deserialize<'de> for SuddenDeathMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SuddenDeathManiaVisitor;
        impl<'de> Visitor<'de> for SuddenDeathManiaVisitor {
            type Value = SuddenDeathMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SuddenDeathMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(SuddenDeathManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SuddenDeathMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// SS or quit.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct PerfectMania {
    pub restart: Option<bool>,
}
impl PerfectMania {
    /// The acronym of [`PerfectMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("PF") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`PerfectMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`PerfectMania`]
    pub const fn description() -> &'static str {
        "SS or quit."
    }
    /// The [`GameModKind`] of [`PerfectMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`PerfectMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16416
    }
}
impl<'de> Deserialize<'de> for PerfectMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct PerfectManiaVisitor;
        impl<'de> Visitor<'de> for PerfectManiaVisitor {
            type Value = PerfectMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("PerfectMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(PerfectManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PerfectMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Zoooooooooom...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DoubleTimeMania {
    pub speed_change: Option<f32>,
}
impl DoubleTimeMania {
    /// The acronym of [`DoubleTimeMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DoubleTimeMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DoubleTimeMania`]
    pub const fn description() -> &'static str {
        "Zoooooooooom..."
    }
    /// The [`GameModKind`] of [`DoubleTimeMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`DoubleTimeMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        64
    }
}
impl<'de> Deserialize<'de> for DoubleTimeMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DoubleTimeManiaVisitor;
        impl<'de> Visitor<'de> for DoubleTimeManiaVisitor {
            type Value = DoubleTimeMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DoubleTimeMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DoubleTimeManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DoubleTimeMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Uguuuuuuuu...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct NightcoreMania {
    pub speed_change: Option<f32>,
}
impl NightcoreMania {
    /// The acronym of [`NightcoreMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("NC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NightcoreMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NightcoreMania`]
    pub const fn description() -> &'static str {
        "Uguuuuuuuu..."
    }
    /// The [`GameModKind`] of [`NightcoreMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`NightcoreMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        576
    }
}
impl<'de> Deserialize<'de> for NightcoreMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NightcoreManiaVisitor;
        impl<'de> Visitor<'de> for NightcoreManiaVisitor {
            type Value = NightcoreMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NightcoreMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut speed_change = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "speed_change" => speed_change = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    speed_change: speed_change.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(NightcoreManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NightcoreMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.speed_change.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.speed_change {
            map.serialize_entry("speed_change", x)?;
        }
        map.end()
    }
}
/// Keys appear out of nowhere!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct FadeInMania {
    pub coverage: Option<f32>,
}
impl FadeInMania {
    /// The acronym of [`FadeInMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FI") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FadeInMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HD"),
                Acronym::from_str_unchecked("FL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FadeInMania`]
    pub const fn description() -> &'static str {
        "Keys appear out of nowhere!"
    }
    /// The [`GameModKind`] of [`FadeInMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FadeInMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1048576
    }
}
impl<'de> Deserialize<'de> for FadeInMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FadeInManiaVisitor;
        impl<'de> Visitor<'de> for FadeInManiaVisitor {
            type Value = FadeInMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FadeInMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut coverage = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "coverage" => coverage = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    coverage: coverage.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(FadeInManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FadeInMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.coverage.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.coverage {
            map.serialize_entry("coverage", x)?;
        }
        map.end()
    }
}
/// Keys fade out before you hit them!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct HiddenMania {
    pub coverage: Option<f32>,
}
impl HiddenMania {
    /// The acronym of [`HiddenMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HiddenMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("FI"),
                Acronym::from_str_unchecked("FL"),
            ]
        }
        .into_iter()
    }
    /// The description of [`HiddenMania`]
    pub const fn description() -> &'static str {
        "Keys fade out before you hit them!"
    }
    /// The [`GameModKind`] of [`HiddenMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`HiddenMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        8
    }
}
impl<'de> Deserialize<'de> for HiddenMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HiddenManiaVisitor;
        impl<'de> Visitor<'de> for HiddenManiaVisitor {
            type Value = HiddenMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HiddenMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut coverage = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "coverage" => coverage = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    coverage: coverage.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(HiddenManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HiddenMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.coverage.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.coverage {
            map.serialize_entry("coverage", x)?;
        }
        map.end()
    }
}
/// Restricted view area.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct FlashlightMania {
    pub size_multiplier: Option<f32>,
    pub combo_based_size: Option<bool>,
}
impl FlashlightMania {
    /// The acronym of [`FlashlightMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("FL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FlashlightMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("FI"),
                Acronym::from_str_unchecked("HD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FlashlightMania`]
    pub const fn description() -> &'static str {
        "Restricted view area."
    }
    /// The [`GameModKind`] of [`FlashlightMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
    /// Bit value of [`FlashlightMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1024
    }
}
impl<'de> Deserialize<'de> for FlashlightMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FlashlightManiaVisitor;
        impl<'de> Visitor<'de> for FlashlightManiaVisitor {
            type Value = FlashlightMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FlashlightMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut size_multiplier = None;
                let mut combo_based_size = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "size_multiplier" => size_multiplier = Some(map.next_value()?),
                        "combo_based_size" => combo_based_size = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    size_multiplier: size_multiplier.unwrap_or_default(),
                    combo_based_size: combo_based_size.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(FlashlightManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FlashlightMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.size_multiplier.is_some() as usize + self.combo_based_size.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.size_multiplier {
            map.serialize_entry("size_multiplier", x)?;
        }
        if let Some(ref x) = self.combo_based_size {
            map.serialize_entry("combo_based_size", x)?;
        }
        map.end()
    }
}
/// Fail if your accuracy drops too low!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AccuracyChallengeMania {
    pub minimum_accuracy: Option<f32>,
    pub restart: Option<bool>,
}
impl AccuracyChallengeMania {
    /// The acronym of [`AccuracyChallengeMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AC") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AccuracyChallengeMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AccuracyChallengeMania`]
    pub const fn description() -> &'static str {
        "Fail if your accuracy drops too low!"
    }
    /// The [`GameModKind`] of [`AccuracyChallengeMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::DifficultyIncrease
    }
}
impl<'de> Deserialize<'de> for AccuracyChallengeMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AccuracyChallengeManiaVisitor;
        impl<'de> Visitor<'de> for AccuracyChallengeManiaVisitor {
            type Value = AccuracyChallengeMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AccuracyChallengeMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut minimum_accuracy = None;
                let mut restart = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "minimum_accuracy" => minimum_accuracy = Some(map.next_value()?),
                        "restart" => restart = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    minimum_accuracy: minimum_accuracy.unwrap_or_default(),
                    restart: restart.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AccuracyChallengeManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AccuracyChallengeMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.minimum_accuracy.is_some() as usize + self.restart.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.minimum_accuracy {
            map.serialize_entry("minimum_accuracy", x)?;
        }
        if let Some(ref x) = self.restart {
            map.serialize_entry("restart", x)?;
        }
        map.end()
    }
}
/// Play with four keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct FourKeysMania {}
impl FourKeysMania {
    /// The acronym of [`FourKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("4K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FourKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FourKeysMania`]
    pub const fn description() -> &'static str {
        "Play with four keys."
    }
    /// The [`GameModKind`] of [`FourKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`FourKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        32768
    }
}
impl<'de> Deserialize<'de> for FourKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FourKeysManiaVisitor;
        impl<'de> Visitor<'de> for FourKeysManiaVisitor {
            type Value = FourKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FourKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(FourKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FourKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with five keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct FiveKeysMania {}
impl FiveKeysMania {
    /// The acronym of [`FiveKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("5K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`FiveKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`FiveKeysMania`]
    pub const fn description() -> &'static str {
        "Play with five keys."
    }
    /// The [`GameModKind`] of [`FiveKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`FiveKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        65536
    }
}
impl<'de> Deserialize<'de> for FiveKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct FiveKeysManiaVisitor;
        impl<'de> Visitor<'de> for FiveKeysManiaVisitor {
            type Value = FiveKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("FiveKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(FiveKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for FiveKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with six keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SixKeysMania {}
impl SixKeysMania {
    /// The acronym of [`SixKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("6K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SixKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SixKeysMania`]
    pub const fn description() -> &'static str {
        "Play with six keys."
    }
    /// The [`GameModKind`] of [`SixKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`SixKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        131072
    }
}
impl<'de> Deserialize<'de> for SixKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SixKeysManiaVisitor;
        impl<'de> Visitor<'de> for SixKeysManiaVisitor {
            type Value = SixKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SixKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SixKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SixKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with seven keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct SevenKeysMania {}
impl SevenKeysMania {
    /// The acronym of [`SevenKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("7K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`SevenKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`SevenKeysMania`]
    pub const fn description() -> &'static str {
        "Play with seven keys."
    }
    /// The [`GameModKind`] of [`SevenKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`SevenKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        262144
    }
}
impl<'de> Deserialize<'de> for SevenKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct SevenKeysManiaVisitor;
        impl<'de> Visitor<'de> for SevenKeysManiaVisitor {
            type Value = SevenKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("SevenKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(SevenKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SevenKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with eight keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct EightKeysMania {}
impl EightKeysMania {
    /// The acronym of [`EightKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("8K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`EightKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`EightKeysMania`]
    pub const fn description() -> &'static str {
        "Play with eight keys."
    }
    /// The [`GameModKind`] of [`EightKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`EightKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        524288
    }
}
impl<'de> Deserialize<'de> for EightKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct EightKeysManiaVisitor;
        impl<'de> Visitor<'de> for EightKeysManiaVisitor {
            type Value = EightKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("EightKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(EightKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for EightKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with nine keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct NineKeysMania {}
impl NineKeysMania {
    /// The acronym of [`NineKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("9K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`NineKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`NineKeysMania`]
    pub const fn description() -> &'static str {
        "Play with nine keys."
    }
    /// The [`GameModKind`] of [`NineKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`NineKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        16777216
    }
}
impl<'de> Deserialize<'de> for NineKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct NineKeysManiaVisitor;
        impl<'de> Visitor<'de> for NineKeysManiaVisitor {
            type Value = NineKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("NineKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(NineKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for NineKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with ten keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct TenKeysMania {}
impl TenKeysMania {
    /// The acronym of [`TenKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("10K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TenKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TenKeysMania`]
    pub const fn description() -> &'static str {
        "Play with ten keys."
    }
    /// The [`GameModKind`] of [`TenKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for TenKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct TenKeysManiaVisitor;
        impl<'de> Visitor<'de> for TenKeysManiaVisitor {
            type Value = TenKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("TenKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(TenKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TenKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with one key.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct OneKeyMania {}
impl OneKeyMania {
    /// The acronym of [`OneKeyMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("1K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`OneKeyMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("2K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`OneKeyMania`]
    pub const fn description() -> &'static str {
        "Play with one key."
    }
    /// The [`GameModKind`] of [`OneKeyMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`OneKeyMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        67108864
    }
}
impl<'de> Deserialize<'de> for OneKeyMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct OneKeyManiaVisitor;
        impl<'de> Visitor<'de> for OneKeyManiaVisitor {
            type Value = OneKeyMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("OneKeyMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(OneKeyManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for OneKeyMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with two keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct TwoKeysMania {}
impl TwoKeysMania {
    /// The acronym of [`TwoKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("2K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`TwoKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("3K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`TwoKeysMania`]
    pub const fn description() -> &'static str {
        "Play with two keys."
    }
    /// The [`GameModKind`] of [`TwoKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`TwoKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        268435456
    }
}
impl<'de> Deserialize<'de> for TwoKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct TwoKeysManiaVisitor;
        impl<'de> Visitor<'de> for TwoKeysManiaVisitor {
            type Value = TwoKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("TwoKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(TwoKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TwoKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Play with three keys.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ThreeKeysMania {}
impl ThreeKeysMania {
    /// The acronym of [`ThreeKeysMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("3K") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ThreeKeysMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("4K"),
                Acronym::from_str_unchecked("5K"),
                Acronym::from_str_unchecked("6K"),
                Acronym::from_str_unchecked("7K"),
                Acronym::from_str_unchecked("8K"),
                Acronym::from_str_unchecked("9K"),
                Acronym::from_str_unchecked("10K"),
                Acronym::from_str_unchecked("1K"),
                Acronym::from_str_unchecked("2K"),
            ]
        }
        .into_iter()
    }
    /// The description of [`ThreeKeysMania`]
    pub const fn description() -> &'static str {
        "Play with three keys."
    }
    /// The [`GameModKind`] of [`ThreeKeysMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`ThreeKeysMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        134217728
    }
}
impl<'de> Deserialize<'de> for ThreeKeysMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ThreeKeysManiaVisitor;
        impl<'de> Visitor<'de> for ThreeKeysManiaVisitor {
            type Value = ThreeKeysMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ThreeKeysMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ThreeKeysManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ThreeKeysMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Shuffle around the keys!
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct RandomMania {
    pub seed: Option<f32>,
}
impl RandomMania {
    /// The acronym of [`RandomMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("RD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`RandomMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`RandomMania`]
    pub const fn description() -> &'static str {
        "Shuffle around the keys!"
    }
    /// The [`GameModKind`] of [`RandomMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`RandomMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2097152
    }
}
impl<'de> Deserialize<'de> for RandomMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct RandomManiaVisitor;
        impl<'de> Visitor<'de> for RandomManiaVisitor {
            type Value = RandomMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("RandomMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "seed" => seed = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    seed: seed.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(RandomManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RandomMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.seed.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.seed {
            map.serialize_entry("seed", x)?;
        }
        map.end()
    }
}
/// Double the stages, double the fun!
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct DualStagesMania {}
impl DualStagesMania {
    /// The acronym of [`DualStagesMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DualStagesMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`DualStagesMania`]
    pub const fn description() -> &'static str {
        "Double the stages, double the fun!"
    }
    /// The [`GameModKind`] of [`DualStagesMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`DualStagesMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        33554432
    }
}
impl<'de> Deserialize<'de> for DualStagesMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DualStagesManiaVisitor;
        impl<'de> Visitor<'de> for DualStagesManiaVisitor {
            type Value = DualStagesMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DualStagesMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(DualStagesManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DualStagesMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Notes are flipped horizontally.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct MirrorMania {}
impl MirrorMania {
    /// The acronym of [`MirrorMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MR") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MirrorMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MirrorMania`]
    pub const fn description() -> &'static str {
        "Notes are flipped horizontally."
    }
    /// The [`GameModKind`] of [`MirrorMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
    /// Bit value of [`MirrorMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        1073741824
    }
}
impl<'de> Deserialize<'de> for MirrorMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MirrorManiaVisitor;
        impl<'de> Visitor<'de> for MirrorManiaVisitor {
            type Value = MirrorMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MirrorMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(MirrorManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MirrorMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Override a beatmap's difficulty settings.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct DifficultyAdjustMania {
    pub drain_rate: Option<f32>,
    pub overall_difficulty: Option<f32>,
    pub extended_limits: Option<bool>,
}
impl DifficultyAdjustMania {
    /// The acronym of [`DifficultyAdjustMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("DA") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`DifficultyAdjustMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("EZ"),
                Acronym::from_str_unchecked("HR"),
            ]
        }
        .into_iter()
    }
    /// The description of [`DifficultyAdjustMania`]
    pub const fn description() -> &'static str {
        "Override a beatmap's difficulty settings."
    }
    /// The [`GameModKind`] of [`DifficultyAdjustMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for DifficultyAdjustMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct DifficultyAdjustManiaVisitor;
        impl<'de> Visitor<'de> for DifficultyAdjustManiaVisitor {
            type Value = DifficultyAdjustMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("DifficultyAdjustMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut drain_rate = None;
                let mut overall_difficulty = None;
                let mut extended_limits = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "drain_rate" => drain_rate = Some(map.next_value()?),
                        "overall_difficulty" => overall_difficulty = Some(map.next_value()?),
                        "extended_limits" => extended_limits = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    drain_rate: drain_rate.unwrap_or_default(),
                    overall_difficulty: overall_difficulty.unwrap_or_default(),
                    extended_limits: extended_limits.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(DifficultyAdjustManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DifficultyAdjustMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.drain_rate.is_some() as usize
            + self.overall_difficulty.is_some() as usize
            + self.extended_limits.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.drain_rate {
            map.serialize_entry("drain_rate", x)?;
        }
        if let Some(ref x) = self.overall_difficulty {
            map.serialize_entry("overall_difficulty", x)?;
        }
        if let Some(ref x) = self.extended_limits {
            map.serialize_entry("extended_limits", x)?;
        }
        map.end()
    }
}
/// Feeling nostalgic?
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ClassicMania {}
impl ClassicMania {
    /// The acronym of [`ClassicMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CL") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ClassicMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ClassicMania`]
    pub const fn description() -> &'static str {
        "Feeling nostalgic?"
    }
    /// The [`GameModKind`] of [`ClassicMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for ClassicMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ClassicManiaVisitor;
        impl<'de> Visitor<'de> for ClassicManiaVisitor {
            type Value = ClassicMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ClassicMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ClassicManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClassicMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Hold the keys. To the beat.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct InvertMania {}
impl InvertMania {
    /// The acronym of [`InvertMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("IN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`InvertMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("HO")] }.into_iter()
    }
    /// The description of [`InvertMania`]
    pub const fn description() -> &'static str {
        "Hold the keys. To the beat."
    }
    /// The [`GameModKind`] of [`InvertMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for InvertMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct InvertManiaVisitor;
        impl<'de> Visitor<'de> for InvertManiaVisitor {
            type Value = InvertMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("InvertMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(InvertManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvertMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// No more tricky speed changes!
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ConstantSpeedMania {}
impl ConstantSpeedMania {
    /// The acronym of [`ConstantSpeedMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ConstantSpeedMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ConstantSpeedMania`]
    pub const fn description() -> &'static str {
        "No more tricky speed changes!"
    }
    /// The [`GameModKind`] of [`ConstantSpeedMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for ConstantSpeedMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ConstantSpeedManiaVisitor;
        impl<'de> Visitor<'de> for ConstantSpeedManiaVisitor {
            type Value = ConstantSpeedMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ConstantSpeedMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ConstantSpeedManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConstantSpeedMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Replaces all hold notes with normal notes.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct HoldOffMania {}
impl HoldOffMania {
    /// The acronym of [`HoldOffMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("HO") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`HoldOffMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe { [Acronym::from_str_unchecked("IN")] }.into_iter()
    }
    /// The description of [`HoldOffMania`]
    pub const fn description() -> &'static str {
        "Replaces all hold notes with normal notes."
    }
    /// The [`GameModKind`] of [`HoldOffMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Conversion
    }
}
impl<'de> Deserialize<'de> for HoldOffMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct HoldOffManiaVisitor;
        impl<'de> Visitor<'de> for HoldOffManiaVisitor {
            type Value = HoldOffMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("HoldOffMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(HoldOffManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for HoldOffMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch a perfect automated play through the song.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct AutoplayMania {}
impl AutoplayMania {
    /// The acronym of [`AutoplayMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AT") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AutoplayMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AutoplayMania`]
    pub const fn description() -> &'static str {
        "Watch a perfect automated play through the song."
    }
    /// The [`GameModKind`] of [`AutoplayMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`AutoplayMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        2048
    }
}
impl<'de> Deserialize<'de> for AutoplayMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AutoplayManiaVisitor;
        impl<'de> Visitor<'de> for AutoplayManiaVisitor {
            type Value = AutoplayMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AutoplayMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(AutoplayManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AutoplayMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Watch the video without visual distractions.
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct CinemaMania {}
impl CinemaMania {
    /// The acronym of [`CinemaMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("CN") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`CinemaMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("NF"),
                Acronym::from_str_unchecked("SD"),
                Acronym::from_str_unchecked("PF"),
                Acronym::from_str_unchecked("AC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`CinemaMania`]
    pub const fn description() -> &'static str {
        "Watch the video without visual distractions."
    }
    /// The [`GameModKind`] of [`CinemaMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Automation
    }
    /// Bit value of [`CinemaMania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        4194304
    }
}
impl<'de> Deserialize<'de> for CinemaMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct CinemaManiaVisitor;
        impl<'de> Visitor<'de> for CinemaManiaVisitor {
            type Value = CinemaMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("CinemaMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(CinemaManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CinemaMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// Can you keep up?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindUpMania {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindUpMania {
    /// The acronym of [`WindUpMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindUpMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WD"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindUpMania`]
    pub const fn description() -> &'static str {
        "Can you keep up?"
    }
    /// The [`GameModKind`] of [`WindUpMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindUpMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindUpManiaVisitor;
        impl<'de> Visitor<'de> for WindUpManiaVisitor {
            type Value = WindUpMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindUpMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindUpManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindUpMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Sloooow doooown...
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct WindDownMania {
    pub initial_rate: Option<f32>,
    pub final_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl WindDownMania {
    /// The acronym of [`WindDownMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("WD") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`WindDownMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("AS"),
            ]
        }
        .into_iter()
    }
    /// The description of [`WindDownMania`]
    pub const fn description() -> &'static str {
        "Sloooow doooown..."
    }
    /// The [`GameModKind`] of [`WindDownMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for WindDownMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct WindDownManiaVisitor;
        impl<'de> Visitor<'de> for WindDownManiaVisitor {
            type Value = WindDownMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("WindDownMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut final_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "final_rate" => final_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    final_rate: final_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(WindDownManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for WindDownMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.initial_rate.is_some() as usize
            + self.final_rate.is_some() as usize
            + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.final_rate {
            map.serialize_entry("final_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Can you still feel the rhythm without music?
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct MutedMania {
    pub inverse_muting: Option<bool>,
    pub enable_metronome: Option<bool>,
    pub mute_combo_count: Option<f32>,
    pub affects_hit_sounds: Option<bool>,
}
impl MutedMania {
    /// The acronym of [`MutedMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("MU") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`MutedMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`MutedMania`]
    pub const fn description() -> &'static str {
        "Can you still feel the rhythm without music?"
    }
    /// The [`GameModKind`] of [`MutedMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for MutedMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct MutedManiaVisitor;
        impl<'de> Visitor<'de> for MutedManiaVisitor {
            type Value = MutedMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("MutedMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut inverse_muting = None;
                let mut enable_metronome = None;
                let mut mute_combo_count = None;
                let mut affects_hit_sounds = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "inverse_muting" => inverse_muting = Some(map.next_value()?),
                        "enable_metronome" => enable_metronome = Some(map.next_value()?),
                        "mute_combo_count" => mute_combo_count = Some(map.next_value()?),
                        "affects_hit_sounds" => affects_hit_sounds = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    inverse_muting: inverse_muting.unwrap_or_default(),
                    enable_metronome: enable_metronome.unwrap_or_default(),
                    mute_combo_count: mute_combo_count.unwrap_or_default(),
                    affects_hit_sounds: affects_hit_sounds.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(MutedManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for MutedMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = self.inverse_muting.is_some() as usize
            + self.enable_metronome.is_some() as usize
            + self.mute_combo_count.is_some() as usize
            + self.affects_hit_sounds.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.inverse_muting {
            map.serialize_entry("inverse_muting", x)?;
        }
        if let Some(ref x) = self.enable_metronome {
            map.serialize_entry("enable_metronome", x)?;
        }
        if let Some(ref x) = self.mute_combo_count {
            map.serialize_entry("mute_combo_count", x)?;
        }
        if let Some(ref x) = self.affects_hit_sounds {
            map.serialize_entry("affects_hit_sounds", x)?;
        }
        map.end()
    }
}
/// Let track speed adapt to you.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AdaptiveSpeedMania {
    pub initial_rate: Option<f32>,
    pub adjust_pitch: Option<bool>,
}
impl AdaptiveSpeedMania {
    /// The acronym of [`AdaptiveSpeedMania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("AS") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`AdaptiveSpeedMania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        unsafe {
            [
                Acronym::from_str_unchecked("HT"),
                Acronym::from_str_unchecked("DC"),
                Acronym::from_str_unchecked("DT"),
                Acronym::from_str_unchecked("NC"),
                Acronym::from_str_unchecked("AT"),
                Acronym::from_str_unchecked("CN"),
                Acronym::from_str_unchecked("WU"),
                Acronym::from_str_unchecked("WD"),
            ]
        }
        .into_iter()
    }
    /// The description of [`AdaptiveSpeedMania`]
    pub const fn description() -> &'static str {
        "Let track speed adapt to you."
    }
    /// The [`GameModKind`] of [`AdaptiveSpeedMania`]
    pub const fn kind() -> GameModKind {
        GameModKind::Fun
    }
}
impl<'de> Deserialize<'de> for AdaptiveSpeedMania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AdaptiveSpeedManiaVisitor;
        impl<'de> Visitor<'de> for AdaptiveSpeedManiaVisitor {
            type Value = AdaptiveSpeedMania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("AdaptiveSpeedMania")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut initial_rate = None;
                let mut adjust_pitch = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        "initial_rate" => initial_rate = Some(map.next_value()?),
                        "adjust_pitch" => adjust_pitch = Some(map.next_value()?),
                        _ => {
                            let _: IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(Self::Value {
                    initial_rate: initial_rate.unwrap_or_default(),
                    adjust_pitch: adjust_pitch.unwrap_or_default(),
                })
            }
        }
        d.deserialize_map(AdaptiveSpeedManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for AdaptiveSpeedMania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count =
            self.initial_rate.is_some() as usize + self.adjust_pitch.is_some() as usize;
        let mut map = s.serialize_map(Some(field_count))?;
        if let Some(ref x) = self.initial_rate {
            map.serialize_entry("initial_rate", x)?;
        }
        if let Some(ref x) = self.adjust_pitch {
            map.serialize_entry("adjust_pitch", x)?;
        }
        map.end()
    }
}
/// Uses the V2 scoring system
#[derive(Copy, Eq, Clone, Debug, Default, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct ScoreV2Mania {}
impl ScoreV2Mania {
    /// The acronym of [`ScoreV2Mania`]
    pub const fn acronym() -> Acronym {
        unsafe { Acronym::from_str_unchecked("V2") }
    }
    /// Iterator of [`Acronym`] for mods that are incompatible with [`ScoreV2Mania`]
    pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {
        [].into_iter()
    }
    /// The description of [`ScoreV2Mania`]
    pub const fn description() -> &'static str {
        "Uses the V2 scoring system"
    }
    /// The [`GameModKind`] of [`ScoreV2Mania`]
    pub const fn kind() -> GameModKind {
        GameModKind::System
    }
    /// Bit value of [`ScoreV2Mania`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits() -> u32 {
        536870912
    }
}
impl<'de> Deserialize<'de> for ScoreV2Mania {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct ScoreV2ManiaVisitor;
        impl<'de> Visitor<'de> for ScoreV2ManiaVisitor {
            type Value = ScoreV2Mania;
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("ScoreV2Mania")
            }
            fn visit_map<A: MapAccess<'de>>(self, _: A) -> Result<Self::Value, A::Error> {
                Ok(Self::Value {})
            }
        }
        d.deserialize_map(ScoreV2ManiaVisitor)
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ScoreV2Mania {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let field_count = 0;
        let map = s.serialize_map(Some(field_count))?;
        map.end()
    }
}
/// The different types of a [`GameMod`]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum GameModKind {
    DifficultyReduction,
    DifficultyIncrease,
    Conversion,
    Automation,
    Fun,
    System,
}
/// The kind of a [`GameMod`] when the mode is ignored
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
#[non_exhaustive]
pub enum GameModIntermode {
    AccuracyChallenge,
    AdaptiveSpeed,
    Alternate,
    ApproachDifferent,
    Autopilot,
    Autoplay,
    BarrelRoll,
    Blinds,
    Cinema,
    Classic,
    ConstantSpeed,
    Daycore,
    Deflate,
    DifficultyAdjust,
    DoubleTime,
    DualStages,
    Easy,
    EightKeys,
    FadeIn,
    FiveKeys,
    Flashlight,
    FloatingFruits,
    FourKeys,
    FreezeFrame,
    Grow,
    HalfTime,
    HardRock,
    Hidden,
    HoldOff,
    Invert,
    Magnetised,
    Mirror,
    Muted,
    Nightcore,
    NineKeys,
    NoFail,
    NoScope,
    OneKey,
    Perfect,
    Random,
    Relax,
    Repel,
    ScoreV2,
    SevenKeys,
    SingleTap,
    SixKeys,
    SpinIn,
    SpunOut,
    StrictTracking,
    SuddenDeath,
    Swap,
    TargetPractice,
    TenKeys,
    ThreeKeys,
    TouchDevice,
    Traceable,
    Transform,
    TwoKeys,
    Wiggle,
    WindDown,
    WindUp,
}
impl GameModIntermode {
    /// The [`Acronym`] of this [`GameModIntermode`]
    pub const fn acronym(&self) -> Acronym {
        unsafe {
            match self {
                Self::AccuracyChallenge => Acronym::from_str_unchecked("AC"),
                Self::AdaptiveSpeed => Acronym::from_str_unchecked("AS"),
                Self::Alternate => Acronym::from_str_unchecked("AL"),
                Self::ApproachDifferent => Acronym::from_str_unchecked("AD"),
                Self::Autopilot => Acronym::from_str_unchecked("AP"),
                Self::Autoplay => Acronym::from_str_unchecked("AT"),
                Self::BarrelRoll => Acronym::from_str_unchecked("BR"),
                Self::Blinds => Acronym::from_str_unchecked("BL"),
                Self::Cinema => Acronym::from_str_unchecked("CN"),
                Self::Classic => Acronym::from_str_unchecked("CL"),
                Self::ConstantSpeed => Acronym::from_str_unchecked("CS"),
                Self::Daycore => Acronym::from_str_unchecked("DC"),
                Self::Deflate => Acronym::from_str_unchecked("DF"),
                Self::DifficultyAdjust => Acronym::from_str_unchecked("DA"),
                Self::DoubleTime => Acronym::from_str_unchecked("DT"),
                Self::DualStages => Acronym::from_str_unchecked("DS"),
                Self::Easy => Acronym::from_str_unchecked("EZ"),
                Self::EightKeys => Acronym::from_str_unchecked("8K"),
                Self::FadeIn => Acronym::from_str_unchecked("FI"),
                Self::FiveKeys => Acronym::from_str_unchecked("5K"),
                Self::Flashlight => Acronym::from_str_unchecked("FL"),
                Self::FloatingFruits => Acronym::from_str_unchecked("FF"),
                Self::FourKeys => Acronym::from_str_unchecked("4K"),
                Self::FreezeFrame => Acronym::from_str_unchecked("FR"),
                Self::Grow => Acronym::from_str_unchecked("GR"),
                Self::HalfTime => Acronym::from_str_unchecked("HT"),
                Self::HardRock => Acronym::from_str_unchecked("HR"),
                Self::Hidden => Acronym::from_str_unchecked("HD"),
                Self::HoldOff => Acronym::from_str_unchecked("HO"),
                Self::Invert => Acronym::from_str_unchecked("IN"),
                Self::Magnetised => Acronym::from_str_unchecked("MG"),
                Self::Mirror => Acronym::from_str_unchecked("MR"),
                Self::Muted => Acronym::from_str_unchecked("MU"),
                Self::Nightcore => Acronym::from_str_unchecked("NC"),
                Self::NineKeys => Acronym::from_str_unchecked("9K"),
                Self::NoFail => Acronym::from_str_unchecked("NF"),
                Self::NoScope => Acronym::from_str_unchecked("NS"),
                Self::OneKey => Acronym::from_str_unchecked("1K"),
                Self::Perfect => Acronym::from_str_unchecked("PF"),
                Self::Random => Acronym::from_str_unchecked("RD"),
                Self::Relax => Acronym::from_str_unchecked("RX"),
                Self::Repel => Acronym::from_str_unchecked("RP"),
                Self::ScoreV2 => Acronym::from_str_unchecked("V2"),
                Self::SevenKeys => Acronym::from_str_unchecked("7K"),
                Self::SingleTap => Acronym::from_str_unchecked("SG"),
                Self::SixKeys => Acronym::from_str_unchecked("6K"),
                Self::SpinIn => Acronym::from_str_unchecked("SI"),
                Self::SpunOut => Acronym::from_str_unchecked("SO"),
                Self::StrictTracking => Acronym::from_str_unchecked("ST"),
                Self::SuddenDeath => Acronym::from_str_unchecked("SD"),
                Self::Swap => Acronym::from_str_unchecked("SW"),
                Self::TargetPractice => Acronym::from_str_unchecked("TP"),
                Self::TenKeys => Acronym::from_str_unchecked("10K"),
                Self::ThreeKeys => Acronym::from_str_unchecked("3K"),
                Self::TouchDevice => Acronym::from_str_unchecked("TD"),
                Self::Traceable => Acronym::from_str_unchecked("TC"),
                Self::Transform => Acronym::from_str_unchecked("TR"),
                Self::TwoKeys => Acronym::from_str_unchecked("2K"),
                Self::Wiggle => Acronym::from_str_unchecked("WG"),
                Self::WindDown => Acronym::from_str_unchecked("WD"),
                Self::WindUp => Acronym::from_str_unchecked("WU"),
            }
        }
    }
    /// Bit value of the [`GameModIntermode`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits(self) -> Option<u32> {
        match self {
            Self::AccuracyChallenge => None,
            Self::AdaptiveSpeed => None,
            Self::Alternate => None,
            Self::ApproachDifferent => None,
            Self::Autopilot => Some(8192),
            Self::Autoplay => Some(2048),
            Self::BarrelRoll => None,
            Self::Blinds => None,
            Self::Cinema => Some(4194304),
            Self::Classic => None,
            Self::ConstantSpeed => None,
            Self::Daycore => None,
            Self::Deflate => None,
            Self::DifficultyAdjust => None,
            Self::DoubleTime => Some(64),
            Self::DualStages => Some(33554432),
            Self::Easy => Some(2),
            Self::EightKeys => Some(524288),
            Self::FadeIn => Some(1048576),
            Self::FiveKeys => Some(65536),
            Self::Flashlight => Some(1024),
            Self::FloatingFruits => None,
            Self::FourKeys => Some(32768),
            Self::FreezeFrame => None,
            Self::Grow => None,
            Self::HalfTime => Some(256),
            Self::HardRock => Some(16),
            Self::Hidden => Some(8),
            Self::HoldOff => None,
            Self::Invert => None,
            Self::Magnetised => None,
            Self::Mirror => Some(1073741824),
            Self::Muted => None,
            Self::Nightcore => Some(576),
            Self::NineKeys => Some(16777216),
            Self::NoFail => Some(1),
            Self::NoScope => None,
            Self::OneKey => Some(67108864),
            Self::Perfect => Some(16416),
            Self::Random => Some(2097152),
            Self::Relax => Some(128),
            Self::Repel => None,
            Self::ScoreV2 => Some(536870912),
            Self::SevenKeys => Some(262144),
            Self::SingleTap => None,
            Self::SixKeys => Some(131072),
            Self::SpinIn => None,
            Self::SpunOut => Some(4096),
            Self::StrictTracking => None,
            Self::SuddenDeath => Some(32),
            Self::Swap => None,
            Self::TargetPractice => Some(8388608),
            Self::TenKeys => None,
            Self::ThreeKeys => Some(134217728),
            Self::TouchDevice => Some(4),
            Self::Traceable => None,
            Self::Transform => None,
            Self::TwoKeys => Some(268435456),
            Self::Wiggle => None,
            Self::WindDown => None,
            Self::WindUp => None,
        }
    }
    /// The [`GameModKind`] of this [`GameModIntermode`]
    pub const fn kind(&self) -> GameModKind {
        match self {
            Self::AccuracyChallenge => GameModKind::DifficultyIncrease,
            Self::AdaptiveSpeed => GameModKind::Fun,
            Self::Alternate => GameModKind::Conversion,
            Self::ApproachDifferent => GameModKind::Fun,
            Self::Autopilot => GameModKind::Automation,
            Self::Autoplay => GameModKind::Automation,
            Self::BarrelRoll => GameModKind::Fun,
            Self::Blinds => GameModKind::DifficultyIncrease,
            Self::Cinema => GameModKind::Automation,
            Self::Classic => GameModKind::Conversion,
            Self::ConstantSpeed => GameModKind::Conversion,
            Self::Daycore => GameModKind::DifficultyReduction,
            Self::Deflate => GameModKind::Fun,
            Self::DifficultyAdjust => GameModKind::Conversion,
            Self::DoubleTime => GameModKind::DifficultyIncrease,
            Self::DualStages => GameModKind::Conversion,
            Self::Easy => GameModKind::DifficultyReduction,
            Self::EightKeys => GameModKind::Conversion,
            Self::FadeIn => GameModKind::DifficultyIncrease,
            Self::FiveKeys => GameModKind::Conversion,
            Self::Flashlight => GameModKind::DifficultyIncrease,
            Self::FloatingFruits => GameModKind::Fun,
            Self::FourKeys => GameModKind::Conversion,
            Self::FreezeFrame => GameModKind::Fun,
            Self::Grow => GameModKind::Fun,
            Self::HalfTime => GameModKind::DifficultyReduction,
            Self::HardRock => GameModKind::DifficultyIncrease,
            Self::Hidden => GameModKind::DifficultyIncrease,
            Self::HoldOff => GameModKind::Conversion,
            Self::Invert => GameModKind::Conversion,
            Self::Magnetised => GameModKind::Fun,
            Self::Mirror => GameModKind::Conversion,
            Self::Muted => GameModKind::Fun,
            Self::Nightcore => GameModKind::DifficultyIncrease,
            Self::NineKeys => GameModKind::Conversion,
            Self::NoFail => GameModKind::DifficultyReduction,
            Self::NoScope => GameModKind::Fun,
            Self::OneKey => GameModKind::Conversion,
            Self::Perfect => GameModKind::DifficultyIncrease,
            Self::Random => GameModKind::Conversion,
            Self::Relax => GameModKind::Automation,
            Self::Repel => GameModKind::Fun,
            Self::ScoreV2 => GameModKind::System,
            Self::SevenKeys => GameModKind::Conversion,
            Self::SingleTap => GameModKind::Conversion,
            Self::SixKeys => GameModKind::Conversion,
            Self::SpinIn => GameModKind::Fun,
            Self::SpunOut => GameModKind::Automation,
            Self::StrictTracking => GameModKind::DifficultyIncrease,
            Self::SuddenDeath => GameModKind::DifficultyIncrease,
            Self::Swap => GameModKind::Conversion,
            Self::TargetPractice => GameModKind::Conversion,
            Self::TenKeys => GameModKind::Conversion,
            Self::ThreeKeys => GameModKind::Conversion,
            Self::TouchDevice => GameModKind::System,
            Self::Traceable => GameModKind::Fun,
            Self::Transform => GameModKind::Fun,
            Self::TwoKeys => GameModKind::Conversion,
            Self::Wiggle => GameModKind::Fun,
            Self::WindDown => GameModKind::Fun,
            Self::WindUp => GameModKind::Fun,
        }
    }
    /// Try to parse an [`Acronym`] into a [`GameModIntermode`]
    pub fn from_acronym(acronym: Acronym) -> Option<Self> {
        match acronym.as_str() {
            "AC" => Some(Self::AccuracyChallenge),
            "AS" => Some(Self::AdaptiveSpeed),
            "AL" => Some(Self::Alternate),
            "AD" => Some(Self::ApproachDifferent),
            "AP" => Some(Self::Autopilot),
            "AT" => Some(Self::Autoplay),
            "BR" => Some(Self::BarrelRoll),
            "BL" => Some(Self::Blinds),
            "CN" => Some(Self::Cinema),
            "CL" => Some(Self::Classic),
            "CS" => Some(Self::ConstantSpeed),
            "DC" => Some(Self::Daycore),
            "DF" => Some(Self::Deflate),
            "DA" => Some(Self::DifficultyAdjust),
            "DT" => Some(Self::DoubleTime),
            "DS" => Some(Self::DualStages),
            "EZ" => Some(Self::Easy),
            "8K" => Some(Self::EightKeys),
            "FI" => Some(Self::FadeIn),
            "5K" => Some(Self::FiveKeys),
            "FL" => Some(Self::Flashlight),
            "FF" => Some(Self::FloatingFruits),
            "4K" => Some(Self::FourKeys),
            "FR" => Some(Self::FreezeFrame),
            "GR" => Some(Self::Grow),
            "HT" => Some(Self::HalfTime),
            "HR" => Some(Self::HardRock),
            "HD" => Some(Self::Hidden),
            "HO" => Some(Self::HoldOff),
            "IN" => Some(Self::Invert),
            "MG" => Some(Self::Magnetised),
            "MR" => Some(Self::Mirror),
            "MU" => Some(Self::Muted),
            "NC" => Some(Self::Nightcore),
            "9K" => Some(Self::NineKeys),
            "NF" => Some(Self::NoFail),
            "NS" => Some(Self::NoScope),
            "1K" => Some(Self::OneKey),
            "PF" => Some(Self::Perfect),
            "RD" => Some(Self::Random),
            "RX" => Some(Self::Relax),
            "RP" => Some(Self::Repel),
            "V2" => Some(Self::ScoreV2),
            "7K" => Some(Self::SevenKeys),
            "SG" => Some(Self::SingleTap),
            "6K" => Some(Self::SixKeys),
            "SI" => Some(Self::SpinIn),
            "SO" => Some(Self::SpunOut),
            "ST" => Some(Self::StrictTracking),
            "SD" => Some(Self::SuddenDeath),
            "SW" => Some(Self::Swap),
            "TP" => Some(Self::TargetPractice),
            "10K" => Some(Self::TenKeys),
            "3K" => Some(Self::ThreeKeys),
            "TD" => Some(Self::TouchDevice),
            "TC" => Some(Self::Traceable),
            "TR" => Some(Self::Transform),
            "2K" => Some(Self::TwoKeys),
            "WG" => Some(Self::Wiggle),
            "WD" => Some(Self::WindDown),
            "WU" => Some(Self::WindUp),
            _ => None,
        }
    }
}
impl PartialOrd for GameModIntermode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits()
            .zip(other.bits())
            .map(|(self_bits, other_bits)| self_bits.cmp(&other_bits))
    }
}
impl Ord for GameModIntermode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.bits(), other.bits()) {
            (Some(self_bits), Some(other_bits)) => self_bits.cmp(&other_bits),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => self.acronym().as_str().cmp(other.acronym().as_str()),
        }
    }
}
impl Display for GameModIntermode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.acronym().as_str())
    }
}
impl From<&GameModIntermode> for GameModIntermode {
    fn from(gamemod: &GameModIntermode) -> Self {
        *gamemod
    }
}
impl From<GameMod> for GameModIntermode {
    fn from(gamemod: GameMod) -> Self {
        gamemod.intermode()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GameModIntermode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.acronym().as_str())
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct GameModOrder {
    mode: GameMode,
    index: Option<NonZeroU8>,
    intermode: GameModIntermode,
}
impl From<&GameMod> for GameModOrder {
    fn from(gamemod: &GameMod) -> Self {
        const fn inner(gamemod: &GameMod) -> GameModOrder {
            macro_rules! arm {
                ($mode:ident, $gamemod:ident, Some($discriminant:literal), $intermode:ident) => {
                    arm!(
                        $mode,
                        $gamemod,
                        Some(unsafe { NonZeroU8::new_unchecked($discriminant) }),
                        $intermode,
                    )
                };
                ($mode:ident, $gamemod:ident, $index:expr, $intermode:ident $(,)?) => {
                    GameModOrder {
                        mode: GameMode::$mode,
                        index: $index,
                        intermode: GameModIntermode::$intermode,
                    }
                };
            }
            match gamemod {
                GameMod::EasyOsu(_) => arm!(Osu, EasyOsu, Some(2), Easy),
                GameMod::NoFailOsu(_) => arm!(Osu, NoFailOsu, Some(1), NoFail),
                GameMod::HalfTimeOsu(_) => arm!(Osu, HalfTimeOsu, Some(9), HalfTime),
                GameMod::DaycoreOsu(_) => arm!(Osu, DaycoreOsu, None, Daycore),
                GameMod::HardRockOsu(_) => arm!(Osu, HardRockOsu, Some(5), HardRock),
                GameMod::SuddenDeathOsu(_) => arm!(Osu, SuddenDeathOsu, Some(6), SuddenDeath),
                GameMod::PerfectOsu(_) => arm!(Osu, PerfectOsu, Some(15), Perfect),
                GameMod::DoubleTimeOsu(_) => arm!(Osu, DoubleTimeOsu, Some(7), DoubleTime),
                GameMod::NightcoreOsu(_) => arm!(Osu, NightcoreOsu, Some(10), Nightcore),
                GameMod::HiddenOsu(_) => arm!(Osu, HiddenOsu, Some(4), Hidden),
                GameMod::FlashlightOsu(_) => arm!(Osu, FlashlightOsu, Some(11), Flashlight),
                GameMod::BlindsOsu(_) => arm!(Osu, BlindsOsu, None, Blinds),
                GameMod::StrictTrackingOsu(_) => arm!(Osu, StrictTrackingOsu, None, StrictTracking),
                GameMod::AccuracyChallengeOsu(_) => {
                    arm!(Osu, AccuracyChallengeOsu, None, AccuracyChallenge)
                }
                GameMod::TargetPracticeOsu(_) => {
                    arm!(Osu, TargetPracticeOsu, Some(24), TargetPractice)
                }
                GameMod::DifficultyAdjustOsu(_) => {
                    arm!(Osu, DifficultyAdjustOsu, None, DifficultyAdjust)
                }
                GameMod::ClassicOsu(_) => arm!(Osu, ClassicOsu, None, Classic),
                GameMod::RandomOsu(_) => arm!(Osu, RandomOsu, Some(22), Random),
                GameMod::MirrorOsu(_) => arm!(Osu, MirrorOsu, Some(31), Mirror),
                GameMod::AlternateOsu(_) => arm!(Osu, AlternateOsu, None, Alternate),
                GameMod::SingleTapOsu(_) => arm!(Osu, SingleTapOsu, None, SingleTap),
                GameMod::AutoplayOsu(_) => arm!(Osu, AutoplayOsu, Some(12), Autoplay),
                GameMod::CinemaOsu(_) => arm!(Osu, CinemaOsu, Some(23), Cinema),
                GameMod::RelaxOsu(_) => arm!(Osu, RelaxOsu, Some(8), Relax),
                GameMod::AutopilotOsu(_) => arm!(Osu, AutopilotOsu, Some(14), Autopilot),
                GameMod::SpunOutOsu(_) => arm!(Osu, SpunOutOsu, Some(13), SpunOut),
                GameMod::TransformOsu(_) => arm!(Osu, TransformOsu, None, Transform),
                GameMod::WiggleOsu(_) => arm!(Osu, WiggleOsu, None, Wiggle),
                GameMod::SpinInOsu(_) => arm!(Osu, SpinInOsu, None, SpinIn),
                GameMod::GrowOsu(_) => arm!(Osu, GrowOsu, None, Grow),
                GameMod::DeflateOsu(_) => arm!(Osu, DeflateOsu, None, Deflate),
                GameMod::WindUpOsu(_) => arm!(Osu, WindUpOsu, None, WindUp),
                GameMod::WindDownOsu(_) => arm!(Osu, WindDownOsu, None, WindDown),
                GameMod::TraceableOsu(_) => arm!(Osu, TraceableOsu, None, Traceable),
                GameMod::BarrelRollOsu(_) => arm!(Osu, BarrelRollOsu, None, BarrelRoll),
                GameMod::ApproachDifferentOsu(_) => {
                    arm!(Osu, ApproachDifferentOsu, None, ApproachDifferent)
                }
                GameMod::MutedOsu(_) => arm!(Osu, MutedOsu, None, Muted),
                GameMod::NoScopeOsu(_) => arm!(Osu, NoScopeOsu, None, NoScope),
                GameMod::MagnetisedOsu(_) => arm!(Osu, MagnetisedOsu, None, Magnetised),
                GameMod::RepelOsu(_) => arm!(Osu, RepelOsu, None, Repel),
                GameMod::AdaptiveSpeedOsu(_) => arm!(Osu, AdaptiveSpeedOsu, None, AdaptiveSpeed),
                GameMod::FreezeFrameOsu(_) => arm!(Osu, FreezeFrameOsu, None, FreezeFrame),
                GameMod::TouchDeviceOsu(_) => arm!(Osu, TouchDeviceOsu, Some(3), TouchDevice),
                GameMod::ScoreV2Osu(_) => arm!(Osu, ScoreV2Osu, Some(30), ScoreV2),
                GameMod::EasyTaiko(_) => arm!(Taiko, EasyTaiko, Some(2), Easy),
                GameMod::NoFailTaiko(_) => arm!(Taiko, NoFailTaiko, Some(1), NoFail),
                GameMod::HalfTimeTaiko(_) => arm!(Taiko, HalfTimeTaiko, Some(9), HalfTime),
                GameMod::DaycoreTaiko(_) => arm!(Taiko, DaycoreTaiko, None, Daycore),
                GameMod::HardRockTaiko(_) => arm!(Taiko, HardRockTaiko, Some(5), HardRock),
                GameMod::SuddenDeathTaiko(_) => arm!(Taiko, SuddenDeathTaiko, Some(6), SuddenDeath),
                GameMod::PerfectTaiko(_) => arm!(Taiko, PerfectTaiko, Some(15), Perfect),
                GameMod::DoubleTimeTaiko(_) => arm!(Taiko, DoubleTimeTaiko, Some(7), DoubleTime),
                GameMod::NightcoreTaiko(_) => arm!(Taiko, NightcoreTaiko, Some(10), Nightcore),
                GameMod::HiddenTaiko(_) => arm!(Taiko, HiddenTaiko, Some(4), Hidden),
                GameMod::FlashlightTaiko(_) => arm!(Taiko, FlashlightTaiko, Some(11), Flashlight),
                GameMod::AccuracyChallengeTaiko(_) => {
                    arm!(Taiko, AccuracyChallengeTaiko, None, AccuracyChallenge)
                }
                GameMod::RandomTaiko(_) => arm!(Taiko, RandomTaiko, Some(22), Random),
                GameMod::DifficultyAdjustTaiko(_) => {
                    arm!(Taiko, DifficultyAdjustTaiko, None, DifficultyAdjust)
                }
                GameMod::ClassicTaiko(_) => arm!(Taiko, ClassicTaiko, None, Classic),
                GameMod::SwapTaiko(_) => arm!(Taiko, SwapTaiko, None, Swap),
                GameMod::SingleTapTaiko(_) => arm!(Taiko, SingleTapTaiko, None, SingleTap),
                GameMod::AutoplayTaiko(_) => arm!(Taiko, AutoplayTaiko, Some(12), Autoplay),
                GameMod::CinemaTaiko(_) => arm!(Taiko, CinemaTaiko, Some(23), Cinema),
                GameMod::RelaxTaiko(_) => arm!(Taiko, RelaxTaiko, Some(8), Relax),
                GameMod::WindUpTaiko(_) => arm!(Taiko, WindUpTaiko, None, WindUp),
                GameMod::WindDownTaiko(_) => arm!(Taiko, WindDownTaiko, None, WindDown),
                GameMod::MutedTaiko(_) => arm!(Taiko, MutedTaiko, None, Muted),
                GameMod::AdaptiveSpeedTaiko(_) => {
                    arm!(Taiko, AdaptiveSpeedTaiko, None, AdaptiveSpeed)
                }
                GameMod::ScoreV2Taiko(_) => arm!(Taiko, ScoreV2Taiko, Some(30), ScoreV2),
                GameMod::EasyCatch(_) => arm!(Catch, EasyCatch, Some(2), Easy),
                GameMod::NoFailCatch(_) => arm!(Catch, NoFailCatch, Some(1), NoFail),
                GameMod::HalfTimeCatch(_) => arm!(Catch, HalfTimeCatch, Some(9), HalfTime),
                GameMod::DaycoreCatch(_) => arm!(Catch, DaycoreCatch, None, Daycore),
                GameMod::HardRockCatch(_) => arm!(Catch, HardRockCatch, Some(5), HardRock),
                GameMod::SuddenDeathCatch(_) => arm!(Catch, SuddenDeathCatch, Some(6), SuddenDeath),
                GameMod::PerfectCatch(_) => arm!(Catch, PerfectCatch, Some(15), Perfect),
                GameMod::DoubleTimeCatch(_) => arm!(Catch, DoubleTimeCatch, Some(7), DoubleTime),
                GameMod::NightcoreCatch(_) => arm!(Catch, NightcoreCatch, Some(10), Nightcore),
                GameMod::HiddenCatch(_) => arm!(Catch, HiddenCatch, Some(4), Hidden),
                GameMod::FlashlightCatch(_) => arm!(Catch, FlashlightCatch, Some(11), Flashlight),
                GameMod::AccuracyChallengeCatch(_) => {
                    arm!(Catch, AccuracyChallengeCatch, None, AccuracyChallenge)
                }
                GameMod::DifficultyAdjustCatch(_) => {
                    arm!(Catch, DifficultyAdjustCatch, None, DifficultyAdjust)
                }
                GameMod::ClassicCatch(_) => arm!(Catch, ClassicCatch, None, Classic),
                GameMod::MirrorCatch(_) => arm!(Catch, MirrorCatch, Some(31), Mirror),
                GameMod::AutoplayCatch(_) => arm!(Catch, AutoplayCatch, Some(12), Autoplay),
                GameMod::CinemaCatch(_) => arm!(Catch, CinemaCatch, Some(23), Cinema),
                GameMod::RelaxCatch(_) => arm!(Catch, RelaxCatch, Some(8), Relax),
                GameMod::WindUpCatch(_) => arm!(Catch, WindUpCatch, None, WindUp),
                GameMod::WindDownCatch(_) => arm!(Catch, WindDownCatch, None, WindDown),
                GameMod::FloatingFruitsCatch(_) => {
                    arm!(Catch, FloatingFruitsCatch, None, FloatingFruits)
                }
                GameMod::MutedCatch(_) => arm!(Catch, MutedCatch, None, Muted),
                GameMod::NoScopeCatch(_) => arm!(Catch, NoScopeCatch, None, NoScope),
                GameMod::ScoreV2Catch(_) => arm!(Catch, ScoreV2Catch, Some(30), ScoreV2),
                GameMod::EasyMania(_) => arm!(Mania, EasyMania, Some(2), Easy),
                GameMod::NoFailMania(_) => arm!(Mania, NoFailMania, Some(1), NoFail),
                GameMod::HalfTimeMania(_) => arm!(Mania, HalfTimeMania, Some(9), HalfTime),
                GameMod::DaycoreMania(_) => arm!(Mania, DaycoreMania, None, Daycore),
                GameMod::HardRockMania(_) => arm!(Mania, HardRockMania, Some(5), HardRock),
                GameMod::SuddenDeathMania(_) => arm!(Mania, SuddenDeathMania, Some(6), SuddenDeath),
                GameMod::PerfectMania(_) => arm!(Mania, PerfectMania, Some(15), Perfect),
                GameMod::DoubleTimeMania(_) => arm!(Mania, DoubleTimeMania, Some(7), DoubleTime),
                GameMod::NightcoreMania(_) => arm!(Mania, NightcoreMania, Some(10), Nightcore),
                GameMod::FadeInMania(_) => arm!(Mania, FadeInMania, Some(21), FadeIn),
                GameMod::HiddenMania(_) => arm!(Mania, HiddenMania, Some(4), Hidden),
                GameMod::FlashlightMania(_) => arm!(Mania, FlashlightMania, Some(11), Flashlight),
                GameMod::AccuracyChallengeMania(_) => {
                    arm!(Mania, AccuracyChallengeMania, None, AccuracyChallenge)
                }
                GameMod::FourKeysMania(_) => arm!(Mania, FourKeysMania, Some(16), FourKeys),
                GameMod::FiveKeysMania(_) => arm!(Mania, FiveKeysMania, Some(17), FiveKeys),
                GameMod::SixKeysMania(_) => arm!(Mania, SixKeysMania, Some(18), SixKeys),
                GameMod::SevenKeysMania(_) => arm!(Mania, SevenKeysMania, Some(19), SevenKeys),
                GameMod::EightKeysMania(_) => arm!(Mania, EightKeysMania, Some(20), EightKeys),
                GameMod::NineKeysMania(_) => arm!(Mania, NineKeysMania, Some(25), NineKeys),
                GameMod::TenKeysMania(_) => arm!(Mania, TenKeysMania, None, TenKeys),
                GameMod::OneKeyMania(_) => arm!(Mania, OneKeyMania, Some(27), OneKey),
                GameMod::TwoKeysMania(_) => arm!(Mania, TwoKeysMania, Some(29), TwoKeys),
                GameMod::ThreeKeysMania(_) => arm!(Mania, ThreeKeysMania, Some(28), ThreeKeys),
                GameMod::RandomMania(_) => arm!(Mania, RandomMania, Some(22), Random),
                GameMod::DualStagesMania(_) => arm!(Mania, DualStagesMania, Some(26), DualStages),
                GameMod::MirrorMania(_) => arm!(Mania, MirrorMania, Some(31), Mirror),
                GameMod::DifficultyAdjustMania(_) => {
                    arm!(Mania, DifficultyAdjustMania, None, DifficultyAdjust)
                }
                GameMod::ClassicMania(_) => arm!(Mania, ClassicMania, None, Classic),
                GameMod::InvertMania(_) => arm!(Mania, InvertMania, None, Invert),
                GameMod::ConstantSpeedMania(_) => {
                    arm!(Mania, ConstantSpeedMania, None, ConstantSpeed)
                }
                GameMod::HoldOffMania(_) => arm!(Mania, HoldOffMania, None, HoldOff),
                GameMod::AutoplayMania(_) => arm!(Mania, AutoplayMania, Some(12), Autoplay),
                GameMod::CinemaMania(_) => arm!(Mania, CinemaMania, Some(23), Cinema),
                GameMod::WindUpMania(_) => arm!(Mania, WindUpMania, None, WindUp),
                GameMod::WindDownMania(_) => arm!(Mania, WindDownMania, None, WindDown),
                GameMod::MutedMania(_) => arm!(Mania, MutedMania, None, Muted),
                GameMod::AdaptiveSpeedMania(_) => {
                    arm!(Mania, AdaptiveSpeedMania, None, AdaptiveSpeed)
                }
                GameMod::ScoreV2Mania(_) => arm!(Mania, ScoreV2Mania, Some(30), ScoreV2),
            }
        }
        inner(gamemod)
    }
}
impl PartialOrd for GameModOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.mode.cmp(&other.mode) {
            Ordering::Equal => match (self.index, other.index) {
                (Some(self_idx), Some(other_idx)) => Some(self_idx.cmp(&other_idx)),
                _ => None,
            },
            cmp => Some(cmp),
        }
    }
}
impl Ord for GameModOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mode
            .cmp(&other.mode)
            .then_with(|| match (self.index, other.index) {
                (Some(self_idx), Some(other_idx)) => self_idx.cmp(&other_idx),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => self
                    .intermode
                    .acronym()
                    .as_str()
                    .cmp(other.intermode.acronym().as_str()),
            })
    }
}
impl PartialEq<GameModIntermode> for GameModOrder {
    fn eq(&self, other: &GameModIntermode) -> bool {
        self.intermode.eq(other)
    }
}
impl Borrow<GameModIntermode> for GameModOrder {
    fn borrow(&self) -> &GameModIntermode {
        &self.intermode
    }
}
/// A single game mod
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
#[non_exhaustive]
pub enum GameMod {
    EasyOsu(EasyOsu),
    NoFailOsu(NoFailOsu),
    HalfTimeOsu(HalfTimeOsu),
    DaycoreOsu(DaycoreOsu),
    HardRockOsu(HardRockOsu),
    SuddenDeathOsu(SuddenDeathOsu),
    PerfectOsu(PerfectOsu),
    DoubleTimeOsu(DoubleTimeOsu),
    NightcoreOsu(NightcoreOsu),
    HiddenOsu(HiddenOsu),
    FlashlightOsu(FlashlightOsu),
    BlindsOsu(BlindsOsu),
    StrictTrackingOsu(StrictTrackingOsu),
    AccuracyChallengeOsu(AccuracyChallengeOsu),
    TargetPracticeOsu(TargetPracticeOsu),
    DifficultyAdjustOsu(DifficultyAdjustOsu),
    ClassicOsu(ClassicOsu),
    RandomOsu(RandomOsu),
    MirrorOsu(MirrorOsu),
    AlternateOsu(AlternateOsu),
    SingleTapOsu(SingleTapOsu),
    AutoplayOsu(AutoplayOsu),
    CinemaOsu(CinemaOsu),
    RelaxOsu(RelaxOsu),
    AutopilotOsu(AutopilotOsu),
    SpunOutOsu(SpunOutOsu),
    TransformOsu(TransformOsu),
    WiggleOsu(WiggleOsu),
    SpinInOsu(SpinInOsu),
    GrowOsu(GrowOsu),
    DeflateOsu(DeflateOsu),
    WindUpOsu(WindUpOsu),
    WindDownOsu(WindDownOsu),
    TraceableOsu(TraceableOsu),
    BarrelRollOsu(BarrelRollOsu),
    ApproachDifferentOsu(ApproachDifferentOsu),
    MutedOsu(MutedOsu),
    NoScopeOsu(NoScopeOsu),
    MagnetisedOsu(MagnetisedOsu),
    RepelOsu(RepelOsu),
    AdaptiveSpeedOsu(AdaptiveSpeedOsu),
    FreezeFrameOsu(FreezeFrameOsu),
    TouchDeviceOsu(TouchDeviceOsu),
    ScoreV2Osu(ScoreV2Osu),
    EasyTaiko(EasyTaiko),
    NoFailTaiko(NoFailTaiko),
    HalfTimeTaiko(HalfTimeTaiko),
    DaycoreTaiko(DaycoreTaiko),
    HardRockTaiko(HardRockTaiko),
    SuddenDeathTaiko(SuddenDeathTaiko),
    PerfectTaiko(PerfectTaiko),
    DoubleTimeTaiko(DoubleTimeTaiko),
    NightcoreTaiko(NightcoreTaiko),
    HiddenTaiko(HiddenTaiko),
    FlashlightTaiko(FlashlightTaiko),
    AccuracyChallengeTaiko(AccuracyChallengeTaiko),
    RandomTaiko(RandomTaiko),
    DifficultyAdjustTaiko(DifficultyAdjustTaiko),
    ClassicTaiko(ClassicTaiko),
    SwapTaiko(SwapTaiko),
    SingleTapTaiko(SingleTapTaiko),
    AutoplayTaiko(AutoplayTaiko),
    CinemaTaiko(CinemaTaiko),
    RelaxTaiko(RelaxTaiko),
    WindUpTaiko(WindUpTaiko),
    WindDownTaiko(WindDownTaiko),
    MutedTaiko(MutedTaiko),
    AdaptiveSpeedTaiko(AdaptiveSpeedTaiko),
    ScoreV2Taiko(ScoreV2Taiko),
    EasyCatch(EasyCatch),
    NoFailCatch(NoFailCatch),
    HalfTimeCatch(HalfTimeCatch),
    DaycoreCatch(DaycoreCatch),
    HardRockCatch(HardRockCatch),
    SuddenDeathCatch(SuddenDeathCatch),
    PerfectCatch(PerfectCatch),
    DoubleTimeCatch(DoubleTimeCatch),
    NightcoreCatch(NightcoreCatch),
    HiddenCatch(HiddenCatch),
    FlashlightCatch(FlashlightCatch),
    AccuracyChallengeCatch(AccuracyChallengeCatch),
    DifficultyAdjustCatch(DifficultyAdjustCatch),
    ClassicCatch(ClassicCatch),
    MirrorCatch(MirrorCatch),
    AutoplayCatch(AutoplayCatch),
    CinemaCatch(CinemaCatch),
    RelaxCatch(RelaxCatch),
    WindUpCatch(WindUpCatch),
    WindDownCatch(WindDownCatch),
    FloatingFruitsCatch(FloatingFruitsCatch),
    MutedCatch(MutedCatch),
    NoScopeCatch(NoScopeCatch),
    ScoreV2Catch(ScoreV2Catch),
    EasyMania(EasyMania),
    NoFailMania(NoFailMania),
    HalfTimeMania(HalfTimeMania),
    DaycoreMania(DaycoreMania),
    HardRockMania(HardRockMania),
    SuddenDeathMania(SuddenDeathMania),
    PerfectMania(PerfectMania),
    DoubleTimeMania(DoubleTimeMania),
    NightcoreMania(NightcoreMania),
    FadeInMania(FadeInMania),
    HiddenMania(HiddenMania),
    FlashlightMania(FlashlightMania),
    AccuracyChallengeMania(AccuracyChallengeMania),
    FourKeysMania(FourKeysMania),
    FiveKeysMania(FiveKeysMania),
    SixKeysMania(SixKeysMania),
    SevenKeysMania(SevenKeysMania),
    EightKeysMania(EightKeysMania),
    NineKeysMania(NineKeysMania),
    TenKeysMania(TenKeysMania),
    OneKeyMania(OneKeyMania),
    TwoKeysMania(TwoKeysMania),
    ThreeKeysMania(ThreeKeysMania),
    RandomMania(RandomMania),
    DualStagesMania(DualStagesMania),
    MirrorMania(MirrorMania),
    DifficultyAdjustMania(DifficultyAdjustMania),
    ClassicMania(ClassicMania),
    InvertMania(InvertMania),
    ConstantSpeedMania(ConstantSpeedMania),
    HoldOffMania(HoldOffMania),
    AutoplayMania(AutoplayMania),
    CinemaMania(CinemaMania),
    WindUpMania(WindUpMania),
    WindDownMania(WindDownMania),
    MutedMania(MutedMania),
    AdaptiveSpeedMania(AdaptiveSpeedMania),
    ScoreV2Mania(ScoreV2Mania),
}
impl GameMod {
    /// Create a new [`GameMod`]
    ///
    /// Returns `None` if no [`GameMod`] matches the given acronym and mode.
    pub fn new(acronym: &str, mode: GameMode) -> Option<Self> {
        match (acronym, mode) {
            ("EZ", GameMode::Osu) => Some(Self::EasyOsu(Default::default())),
            ("NF", GameMode::Osu) => Some(Self::NoFailOsu(Default::default())),
            ("HT", GameMode::Osu) => Some(Self::HalfTimeOsu(Default::default())),
            ("DC", GameMode::Osu) => Some(Self::DaycoreOsu(Default::default())),
            ("HR", GameMode::Osu) => Some(Self::HardRockOsu(Default::default())),
            ("SD", GameMode::Osu) => Some(Self::SuddenDeathOsu(Default::default())),
            ("PF", GameMode::Osu) => Some(Self::PerfectOsu(Default::default())),
            ("DT", GameMode::Osu) => Some(Self::DoubleTimeOsu(Default::default())),
            ("NC", GameMode::Osu) => Some(Self::NightcoreOsu(Default::default())),
            ("HD", GameMode::Osu) => Some(Self::HiddenOsu(Default::default())),
            ("FL", GameMode::Osu) => Some(Self::FlashlightOsu(Default::default())),
            ("BL", GameMode::Osu) => Some(Self::BlindsOsu(Default::default())),
            ("ST", GameMode::Osu) => Some(Self::StrictTrackingOsu(Default::default())),
            ("AC", GameMode::Osu) => Some(Self::AccuracyChallengeOsu(Default::default())),
            ("TP", GameMode::Osu) => Some(Self::TargetPracticeOsu(Default::default())),
            ("DA", GameMode::Osu) => Some(Self::DifficultyAdjustOsu(Default::default())),
            ("CL", GameMode::Osu) => Some(Self::ClassicOsu(Default::default())),
            ("RD", GameMode::Osu) => Some(Self::RandomOsu(Default::default())),
            ("MR", GameMode::Osu) => Some(Self::MirrorOsu(Default::default())),
            ("AL", GameMode::Osu) => Some(Self::AlternateOsu(Default::default())),
            ("SG", GameMode::Osu) => Some(Self::SingleTapOsu(Default::default())),
            ("AT", GameMode::Osu) => Some(Self::AutoplayOsu(Default::default())),
            ("CN", GameMode::Osu) => Some(Self::CinemaOsu(Default::default())),
            ("RX", GameMode::Osu) => Some(Self::RelaxOsu(Default::default())),
            ("AP", GameMode::Osu) => Some(Self::AutopilotOsu(Default::default())),
            ("SO", GameMode::Osu) => Some(Self::SpunOutOsu(Default::default())),
            ("TR", GameMode::Osu) => Some(Self::TransformOsu(Default::default())),
            ("WG", GameMode::Osu) => Some(Self::WiggleOsu(Default::default())),
            ("SI", GameMode::Osu) => Some(Self::SpinInOsu(Default::default())),
            ("GR", GameMode::Osu) => Some(Self::GrowOsu(Default::default())),
            ("DF", GameMode::Osu) => Some(Self::DeflateOsu(Default::default())),
            ("WU", GameMode::Osu) => Some(Self::WindUpOsu(Default::default())),
            ("WD", GameMode::Osu) => Some(Self::WindDownOsu(Default::default())),
            ("TC", GameMode::Osu) => Some(Self::TraceableOsu(Default::default())),
            ("BR", GameMode::Osu) => Some(Self::BarrelRollOsu(Default::default())),
            ("AD", GameMode::Osu) => Some(Self::ApproachDifferentOsu(Default::default())),
            ("MU", GameMode::Osu) => Some(Self::MutedOsu(Default::default())),
            ("NS", GameMode::Osu) => Some(Self::NoScopeOsu(Default::default())),
            ("MG", GameMode::Osu) => Some(Self::MagnetisedOsu(Default::default())),
            ("RP", GameMode::Osu) => Some(Self::RepelOsu(Default::default())),
            ("AS", GameMode::Osu) => Some(Self::AdaptiveSpeedOsu(Default::default())),
            ("FR", GameMode::Osu) => Some(Self::FreezeFrameOsu(Default::default())),
            ("TD", GameMode::Osu) => Some(Self::TouchDeviceOsu(Default::default())),
            ("V2", GameMode::Osu) => Some(Self::ScoreV2Osu(Default::default())),
            ("EZ", GameMode::Taiko) => Some(Self::EasyTaiko(Default::default())),
            ("NF", GameMode::Taiko) => Some(Self::NoFailTaiko(Default::default())),
            ("HT", GameMode::Taiko) => Some(Self::HalfTimeTaiko(Default::default())),
            ("DC", GameMode::Taiko) => Some(Self::DaycoreTaiko(Default::default())),
            ("HR", GameMode::Taiko) => Some(Self::HardRockTaiko(Default::default())),
            ("SD", GameMode::Taiko) => Some(Self::SuddenDeathTaiko(Default::default())),
            ("PF", GameMode::Taiko) => Some(Self::PerfectTaiko(Default::default())),
            ("DT", GameMode::Taiko) => Some(Self::DoubleTimeTaiko(Default::default())),
            ("NC", GameMode::Taiko) => Some(Self::NightcoreTaiko(Default::default())),
            ("HD", GameMode::Taiko) => Some(Self::HiddenTaiko(Default::default())),
            ("FL", GameMode::Taiko) => Some(Self::FlashlightTaiko(Default::default())),
            ("AC", GameMode::Taiko) => Some(Self::AccuracyChallengeTaiko(Default::default())),
            ("RD", GameMode::Taiko) => Some(Self::RandomTaiko(Default::default())),
            ("DA", GameMode::Taiko) => Some(Self::DifficultyAdjustTaiko(Default::default())),
            ("CL", GameMode::Taiko) => Some(Self::ClassicTaiko(Default::default())),
            ("SW", GameMode::Taiko) => Some(Self::SwapTaiko(Default::default())),
            ("SG", GameMode::Taiko) => Some(Self::SingleTapTaiko(Default::default())),
            ("AT", GameMode::Taiko) => Some(Self::AutoplayTaiko(Default::default())),
            ("CN", GameMode::Taiko) => Some(Self::CinemaTaiko(Default::default())),
            ("RX", GameMode::Taiko) => Some(Self::RelaxTaiko(Default::default())),
            ("WU", GameMode::Taiko) => Some(Self::WindUpTaiko(Default::default())),
            ("WD", GameMode::Taiko) => Some(Self::WindDownTaiko(Default::default())),
            ("MU", GameMode::Taiko) => Some(Self::MutedTaiko(Default::default())),
            ("AS", GameMode::Taiko) => Some(Self::AdaptiveSpeedTaiko(Default::default())),
            ("V2", GameMode::Taiko) => Some(Self::ScoreV2Taiko(Default::default())),
            ("EZ", GameMode::Catch) => Some(Self::EasyCatch(Default::default())),
            ("NF", GameMode::Catch) => Some(Self::NoFailCatch(Default::default())),
            ("HT", GameMode::Catch) => Some(Self::HalfTimeCatch(Default::default())),
            ("DC", GameMode::Catch) => Some(Self::DaycoreCatch(Default::default())),
            ("HR", GameMode::Catch) => Some(Self::HardRockCatch(Default::default())),
            ("SD", GameMode::Catch) => Some(Self::SuddenDeathCatch(Default::default())),
            ("PF", GameMode::Catch) => Some(Self::PerfectCatch(Default::default())),
            ("DT", GameMode::Catch) => Some(Self::DoubleTimeCatch(Default::default())),
            ("NC", GameMode::Catch) => Some(Self::NightcoreCatch(Default::default())),
            ("HD", GameMode::Catch) => Some(Self::HiddenCatch(Default::default())),
            ("FL", GameMode::Catch) => Some(Self::FlashlightCatch(Default::default())),
            ("AC", GameMode::Catch) => Some(Self::AccuracyChallengeCatch(Default::default())),
            ("DA", GameMode::Catch) => Some(Self::DifficultyAdjustCatch(Default::default())),
            ("CL", GameMode::Catch) => Some(Self::ClassicCatch(Default::default())),
            ("MR", GameMode::Catch) => Some(Self::MirrorCatch(Default::default())),
            ("AT", GameMode::Catch) => Some(Self::AutoplayCatch(Default::default())),
            ("CN", GameMode::Catch) => Some(Self::CinemaCatch(Default::default())),
            ("RX", GameMode::Catch) => Some(Self::RelaxCatch(Default::default())),
            ("WU", GameMode::Catch) => Some(Self::WindUpCatch(Default::default())),
            ("WD", GameMode::Catch) => Some(Self::WindDownCatch(Default::default())),
            ("FF", GameMode::Catch) => Some(Self::FloatingFruitsCatch(Default::default())),
            ("MU", GameMode::Catch) => Some(Self::MutedCatch(Default::default())),
            ("NS", GameMode::Catch) => Some(Self::NoScopeCatch(Default::default())),
            ("V2", GameMode::Catch) => Some(Self::ScoreV2Catch(Default::default())),
            ("EZ", GameMode::Mania) => Some(Self::EasyMania(Default::default())),
            ("NF", GameMode::Mania) => Some(Self::NoFailMania(Default::default())),
            ("HT", GameMode::Mania) => Some(Self::HalfTimeMania(Default::default())),
            ("DC", GameMode::Mania) => Some(Self::DaycoreMania(Default::default())),
            ("HR", GameMode::Mania) => Some(Self::HardRockMania(Default::default())),
            ("SD", GameMode::Mania) => Some(Self::SuddenDeathMania(Default::default())),
            ("PF", GameMode::Mania) => Some(Self::PerfectMania(Default::default())),
            ("DT", GameMode::Mania) => Some(Self::DoubleTimeMania(Default::default())),
            ("NC", GameMode::Mania) => Some(Self::NightcoreMania(Default::default())),
            ("FI", GameMode::Mania) => Some(Self::FadeInMania(Default::default())),
            ("HD", GameMode::Mania) => Some(Self::HiddenMania(Default::default())),
            ("FL", GameMode::Mania) => Some(Self::FlashlightMania(Default::default())),
            ("AC", GameMode::Mania) => Some(Self::AccuracyChallengeMania(Default::default())),
            ("4K", GameMode::Mania) => Some(Self::FourKeysMania(Default::default())),
            ("5K", GameMode::Mania) => Some(Self::FiveKeysMania(Default::default())),
            ("6K", GameMode::Mania) => Some(Self::SixKeysMania(Default::default())),
            ("7K", GameMode::Mania) => Some(Self::SevenKeysMania(Default::default())),
            ("8K", GameMode::Mania) => Some(Self::EightKeysMania(Default::default())),
            ("9K", GameMode::Mania) => Some(Self::NineKeysMania(Default::default())),
            ("10K", GameMode::Mania) => Some(Self::TenKeysMania(Default::default())),
            ("1K", GameMode::Mania) => Some(Self::OneKeyMania(Default::default())),
            ("2K", GameMode::Mania) => Some(Self::TwoKeysMania(Default::default())),
            ("3K", GameMode::Mania) => Some(Self::ThreeKeysMania(Default::default())),
            ("RD", GameMode::Mania) => Some(Self::RandomMania(Default::default())),
            ("DS", GameMode::Mania) => Some(Self::DualStagesMania(Default::default())),
            ("MR", GameMode::Mania) => Some(Self::MirrorMania(Default::default())),
            ("DA", GameMode::Mania) => Some(Self::DifficultyAdjustMania(Default::default())),
            ("CL", GameMode::Mania) => Some(Self::ClassicMania(Default::default())),
            ("IN", GameMode::Mania) => Some(Self::InvertMania(Default::default())),
            ("CS", GameMode::Mania) => Some(Self::ConstantSpeedMania(Default::default())),
            ("HO", GameMode::Mania) => Some(Self::HoldOffMania(Default::default())),
            ("AT", GameMode::Mania) => Some(Self::AutoplayMania(Default::default())),
            ("CN", GameMode::Mania) => Some(Self::CinemaMania(Default::default())),
            ("WU", GameMode::Mania) => Some(Self::WindUpMania(Default::default())),
            ("WD", GameMode::Mania) => Some(Self::WindDownMania(Default::default())),
            ("MU", GameMode::Mania) => Some(Self::MutedMania(Default::default())),
            ("AS", GameMode::Mania) => Some(Self::AdaptiveSpeedMania(Default::default())),
            ("V2", GameMode::Mania) => Some(Self::ScoreV2Mania(Default::default())),
            _ => None,
        }
    }
    /// The acronym of this [`GameMod`]
    pub const fn acronym(&self) -> Acronym {
        match self {
            Self::EasyOsu(_) => EasyOsu::acronym(),
            Self::NoFailOsu(_) => NoFailOsu::acronym(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::acronym(),
            Self::DaycoreOsu(_) => DaycoreOsu::acronym(),
            Self::HardRockOsu(_) => HardRockOsu::acronym(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::acronym(),
            Self::PerfectOsu(_) => PerfectOsu::acronym(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::acronym(),
            Self::NightcoreOsu(_) => NightcoreOsu::acronym(),
            Self::HiddenOsu(_) => HiddenOsu::acronym(),
            Self::FlashlightOsu(_) => FlashlightOsu::acronym(),
            Self::BlindsOsu(_) => BlindsOsu::acronym(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::acronym(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::acronym(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::acronym(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::acronym(),
            Self::ClassicOsu(_) => ClassicOsu::acronym(),
            Self::RandomOsu(_) => RandomOsu::acronym(),
            Self::MirrorOsu(_) => MirrorOsu::acronym(),
            Self::AlternateOsu(_) => AlternateOsu::acronym(),
            Self::SingleTapOsu(_) => SingleTapOsu::acronym(),
            Self::AutoplayOsu(_) => AutoplayOsu::acronym(),
            Self::CinemaOsu(_) => CinemaOsu::acronym(),
            Self::RelaxOsu(_) => RelaxOsu::acronym(),
            Self::AutopilotOsu(_) => AutopilotOsu::acronym(),
            Self::SpunOutOsu(_) => SpunOutOsu::acronym(),
            Self::TransformOsu(_) => TransformOsu::acronym(),
            Self::WiggleOsu(_) => WiggleOsu::acronym(),
            Self::SpinInOsu(_) => SpinInOsu::acronym(),
            Self::GrowOsu(_) => GrowOsu::acronym(),
            Self::DeflateOsu(_) => DeflateOsu::acronym(),
            Self::WindUpOsu(_) => WindUpOsu::acronym(),
            Self::WindDownOsu(_) => WindDownOsu::acronym(),
            Self::TraceableOsu(_) => TraceableOsu::acronym(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::acronym(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::acronym(),
            Self::MutedOsu(_) => MutedOsu::acronym(),
            Self::NoScopeOsu(_) => NoScopeOsu::acronym(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::acronym(),
            Self::RepelOsu(_) => RepelOsu::acronym(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::acronym(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::acronym(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::acronym(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::acronym(),
            Self::EasyTaiko(_) => EasyTaiko::acronym(),
            Self::NoFailTaiko(_) => NoFailTaiko::acronym(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::acronym(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::acronym(),
            Self::HardRockTaiko(_) => HardRockTaiko::acronym(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::acronym(),
            Self::PerfectTaiko(_) => PerfectTaiko::acronym(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::acronym(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::acronym(),
            Self::HiddenTaiko(_) => HiddenTaiko::acronym(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::acronym(),
            Self::AccuracyChallengeTaiko(_) => AccuracyChallengeTaiko::acronym(),
            Self::RandomTaiko(_) => RandomTaiko::acronym(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::acronym(),
            Self::ClassicTaiko(_) => ClassicTaiko::acronym(),
            Self::SwapTaiko(_) => SwapTaiko::acronym(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::acronym(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::acronym(),
            Self::CinemaTaiko(_) => CinemaTaiko::acronym(),
            Self::RelaxTaiko(_) => RelaxTaiko::acronym(),
            Self::WindUpTaiko(_) => WindUpTaiko::acronym(),
            Self::WindDownTaiko(_) => WindDownTaiko::acronym(),
            Self::MutedTaiko(_) => MutedTaiko::acronym(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::acronym(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::acronym(),
            Self::EasyCatch(_) => EasyCatch::acronym(),
            Self::NoFailCatch(_) => NoFailCatch::acronym(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::acronym(),
            Self::DaycoreCatch(_) => DaycoreCatch::acronym(),
            Self::HardRockCatch(_) => HardRockCatch::acronym(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::acronym(),
            Self::PerfectCatch(_) => PerfectCatch::acronym(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::acronym(),
            Self::NightcoreCatch(_) => NightcoreCatch::acronym(),
            Self::HiddenCatch(_) => HiddenCatch::acronym(),
            Self::FlashlightCatch(_) => FlashlightCatch::acronym(),
            Self::AccuracyChallengeCatch(_) => AccuracyChallengeCatch::acronym(),
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::acronym(),
            Self::ClassicCatch(_) => ClassicCatch::acronym(),
            Self::MirrorCatch(_) => MirrorCatch::acronym(),
            Self::AutoplayCatch(_) => AutoplayCatch::acronym(),
            Self::CinemaCatch(_) => CinemaCatch::acronym(),
            Self::RelaxCatch(_) => RelaxCatch::acronym(),
            Self::WindUpCatch(_) => WindUpCatch::acronym(),
            Self::WindDownCatch(_) => WindDownCatch::acronym(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::acronym(),
            Self::MutedCatch(_) => MutedCatch::acronym(),
            Self::NoScopeCatch(_) => NoScopeCatch::acronym(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::acronym(),
            Self::EasyMania(_) => EasyMania::acronym(),
            Self::NoFailMania(_) => NoFailMania::acronym(),
            Self::HalfTimeMania(_) => HalfTimeMania::acronym(),
            Self::DaycoreMania(_) => DaycoreMania::acronym(),
            Self::HardRockMania(_) => HardRockMania::acronym(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::acronym(),
            Self::PerfectMania(_) => PerfectMania::acronym(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::acronym(),
            Self::NightcoreMania(_) => NightcoreMania::acronym(),
            Self::FadeInMania(_) => FadeInMania::acronym(),
            Self::HiddenMania(_) => HiddenMania::acronym(),
            Self::FlashlightMania(_) => FlashlightMania::acronym(),
            Self::AccuracyChallengeMania(_) => AccuracyChallengeMania::acronym(),
            Self::FourKeysMania(_) => FourKeysMania::acronym(),
            Self::FiveKeysMania(_) => FiveKeysMania::acronym(),
            Self::SixKeysMania(_) => SixKeysMania::acronym(),
            Self::SevenKeysMania(_) => SevenKeysMania::acronym(),
            Self::EightKeysMania(_) => EightKeysMania::acronym(),
            Self::NineKeysMania(_) => NineKeysMania::acronym(),
            Self::TenKeysMania(_) => TenKeysMania::acronym(),
            Self::OneKeyMania(_) => OneKeyMania::acronym(),
            Self::TwoKeysMania(_) => TwoKeysMania::acronym(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::acronym(),
            Self::RandomMania(_) => RandomMania::acronym(),
            Self::DualStagesMania(_) => DualStagesMania::acronym(),
            Self::MirrorMania(_) => MirrorMania::acronym(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::acronym(),
            Self::ClassicMania(_) => ClassicMania::acronym(),
            Self::InvertMania(_) => InvertMania::acronym(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::acronym(),
            Self::HoldOffMania(_) => HoldOffMania::acronym(),
            Self::AutoplayMania(_) => AutoplayMania::acronym(),
            Self::CinemaMania(_) => CinemaMania::acronym(),
            Self::WindUpMania(_) => WindUpMania::acronym(),
            Self::WindDownMania(_) => WindDownMania::acronym(),
            Self::MutedMania(_) => MutedMania::acronym(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::acronym(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::acronym(),
        }
    }
    /// List of [`Acronym`] for mods that are incompatible with this [`GameMod`]
    pub fn incompatible_mods(&self) -> Box<[Acronym]> {
        match self {
            Self::EasyOsu(_) => EasyOsu::incompatible_mods().collect(),
            Self::NoFailOsu(_) => NoFailOsu::incompatible_mods().collect(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::incompatible_mods().collect(),
            Self::DaycoreOsu(_) => DaycoreOsu::incompatible_mods().collect(),
            Self::HardRockOsu(_) => HardRockOsu::incompatible_mods().collect(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::incompatible_mods().collect(),
            Self::PerfectOsu(_) => PerfectOsu::incompatible_mods().collect(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::incompatible_mods().collect(),
            Self::NightcoreOsu(_) => NightcoreOsu::incompatible_mods().collect(),
            Self::HiddenOsu(_) => HiddenOsu::incompatible_mods().collect(),
            Self::FlashlightOsu(_) => FlashlightOsu::incompatible_mods().collect(),
            Self::BlindsOsu(_) => BlindsOsu::incompatible_mods().collect(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::incompatible_mods().collect(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::incompatible_mods().collect(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::incompatible_mods().collect(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::incompatible_mods().collect(),
            Self::ClassicOsu(_) => ClassicOsu::incompatible_mods().collect(),
            Self::RandomOsu(_) => RandomOsu::incompatible_mods().collect(),
            Self::MirrorOsu(_) => MirrorOsu::incompatible_mods().collect(),
            Self::AlternateOsu(_) => AlternateOsu::incompatible_mods().collect(),
            Self::SingleTapOsu(_) => SingleTapOsu::incompatible_mods().collect(),
            Self::AutoplayOsu(_) => AutoplayOsu::incompatible_mods().collect(),
            Self::CinemaOsu(_) => CinemaOsu::incompatible_mods().collect(),
            Self::RelaxOsu(_) => RelaxOsu::incompatible_mods().collect(),
            Self::AutopilotOsu(_) => AutopilotOsu::incompatible_mods().collect(),
            Self::SpunOutOsu(_) => SpunOutOsu::incompatible_mods().collect(),
            Self::TransformOsu(_) => TransformOsu::incompatible_mods().collect(),
            Self::WiggleOsu(_) => WiggleOsu::incompatible_mods().collect(),
            Self::SpinInOsu(_) => SpinInOsu::incompatible_mods().collect(),
            Self::GrowOsu(_) => GrowOsu::incompatible_mods().collect(),
            Self::DeflateOsu(_) => DeflateOsu::incompatible_mods().collect(),
            Self::WindUpOsu(_) => WindUpOsu::incompatible_mods().collect(),
            Self::WindDownOsu(_) => WindDownOsu::incompatible_mods().collect(),
            Self::TraceableOsu(_) => TraceableOsu::incompatible_mods().collect(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::incompatible_mods().collect(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::incompatible_mods().collect(),
            Self::MutedOsu(_) => MutedOsu::incompatible_mods().collect(),
            Self::NoScopeOsu(_) => NoScopeOsu::incompatible_mods().collect(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::incompatible_mods().collect(),
            Self::RepelOsu(_) => RepelOsu::incompatible_mods().collect(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::incompatible_mods().collect(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::incompatible_mods().collect(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::incompatible_mods().collect(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::incompatible_mods().collect(),
            Self::EasyTaiko(_) => EasyTaiko::incompatible_mods().collect(),
            Self::NoFailTaiko(_) => NoFailTaiko::incompatible_mods().collect(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::incompatible_mods().collect(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::incompatible_mods().collect(),
            Self::HardRockTaiko(_) => HardRockTaiko::incompatible_mods().collect(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::incompatible_mods().collect(),
            Self::PerfectTaiko(_) => PerfectTaiko::incompatible_mods().collect(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::incompatible_mods().collect(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::incompatible_mods().collect(),
            Self::HiddenTaiko(_) => HiddenTaiko::incompatible_mods().collect(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::incompatible_mods().collect(),
            Self::AccuracyChallengeTaiko(_) => {
                AccuracyChallengeTaiko::incompatible_mods().collect()
            }
            Self::RandomTaiko(_) => RandomTaiko::incompatible_mods().collect(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::incompatible_mods().collect(),
            Self::ClassicTaiko(_) => ClassicTaiko::incompatible_mods().collect(),
            Self::SwapTaiko(_) => SwapTaiko::incompatible_mods().collect(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::incompatible_mods().collect(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::incompatible_mods().collect(),
            Self::CinemaTaiko(_) => CinemaTaiko::incompatible_mods().collect(),
            Self::RelaxTaiko(_) => RelaxTaiko::incompatible_mods().collect(),
            Self::WindUpTaiko(_) => WindUpTaiko::incompatible_mods().collect(),
            Self::WindDownTaiko(_) => WindDownTaiko::incompatible_mods().collect(),
            Self::MutedTaiko(_) => MutedTaiko::incompatible_mods().collect(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::incompatible_mods().collect(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::incompatible_mods().collect(),
            Self::EasyCatch(_) => EasyCatch::incompatible_mods().collect(),
            Self::NoFailCatch(_) => NoFailCatch::incompatible_mods().collect(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::incompatible_mods().collect(),
            Self::DaycoreCatch(_) => DaycoreCatch::incompatible_mods().collect(),
            Self::HardRockCatch(_) => HardRockCatch::incompatible_mods().collect(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::incompatible_mods().collect(),
            Self::PerfectCatch(_) => PerfectCatch::incompatible_mods().collect(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::incompatible_mods().collect(),
            Self::NightcoreCatch(_) => NightcoreCatch::incompatible_mods().collect(),
            Self::HiddenCatch(_) => HiddenCatch::incompatible_mods().collect(),
            Self::FlashlightCatch(_) => FlashlightCatch::incompatible_mods().collect(),
            Self::AccuracyChallengeCatch(_) => {
                AccuracyChallengeCatch::incompatible_mods().collect()
            }
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::incompatible_mods().collect(),
            Self::ClassicCatch(_) => ClassicCatch::incompatible_mods().collect(),
            Self::MirrorCatch(_) => MirrorCatch::incompatible_mods().collect(),
            Self::AutoplayCatch(_) => AutoplayCatch::incompatible_mods().collect(),
            Self::CinemaCatch(_) => CinemaCatch::incompatible_mods().collect(),
            Self::RelaxCatch(_) => RelaxCatch::incompatible_mods().collect(),
            Self::WindUpCatch(_) => WindUpCatch::incompatible_mods().collect(),
            Self::WindDownCatch(_) => WindDownCatch::incompatible_mods().collect(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::incompatible_mods().collect(),
            Self::MutedCatch(_) => MutedCatch::incompatible_mods().collect(),
            Self::NoScopeCatch(_) => NoScopeCatch::incompatible_mods().collect(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::incompatible_mods().collect(),
            Self::EasyMania(_) => EasyMania::incompatible_mods().collect(),
            Self::NoFailMania(_) => NoFailMania::incompatible_mods().collect(),
            Self::HalfTimeMania(_) => HalfTimeMania::incompatible_mods().collect(),
            Self::DaycoreMania(_) => DaycoreMania::incompatible_mods().collect(),
            Self::HardRockMania(_) => HardRockMania::incompatible_mods().collect(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::incompatible_mods().collect(),
            Self::PerfectMania(_) => PerfectMania::incompatible_mods().collect(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::incompatible_mods().collect(),
            Self::NightcoreMania(_) => NightcoreMania::incompatible_mods().collect(),
            Self::FadeInMania(_) => FadeInMania::incompatible_mods().collect(),
            Self::HiddenMania(_) => HiddenMania::incompatible_mods().collect(),
            Self::FlashlightMania(_) => FlashlightMania::incompatible_mods().collect(),
            Self::AccuracyChallengeMania(_) => {
                AccuracyChallengeMania::incompatible_mods().collect()
            }
            Self::FourKeysMania(_) => FourKeysMania::incompatible_mods().collect(),
            Self::FiveKeysMania(_) => FiveKeysMania::incompatible_mods().collect(),
            Self::SixKeysMania(_) => SixKeysMania::incompatible_mods().collect(),
            Self::SevenKeysMania(_) => SevenKeysMania::incompatible_mods().collect(),
            Self::EightKeysMania(_) => EightKeysMania::incompatible_mods().collect(),
            Self::NineKeysMania(_) => NineKeysMania::incompatible_mods().collect(),
            Self::TenKeysMania(_) => TenKeysMania::incompatible_mods().collect(),
            Self::OneKeyMania(_) => OneKeyMania::incompatible_mods().collect(),
            Self::TwoKeysMania(_) => TwoKeysMania::incompatible_mods().collect(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::incompatible_mods().collect(),
            Self::RandomMania(_) => RandomMania::incompatible_mods().collect(),
            Self::DualStagesMania(_) => DualStagesMania::incompatible_mods().collect(),
            Self::MirrorMania(_) => MirrorMania::incompatible_mods().collect(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::incompatible_mods().collect(),
            Self::ClassicMania(_) => ClassicMania::incompatible_mods().collect(),
            Self::InvertMania(_) => InvertMania::incompatible_mods().collect(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::incompatible_mods().collect(),
            Self::HoldOffMania(_) => HoldOffMania::incompatible_mods().collect(),
            Self::AutoplayMania(_) => AutoplayMania::incompatible_mods().collect(),
            Self::CinemaMania(_) => CinemaMania::incompatible_mods().collect(),
            Self::WindUpMania(_) => WindUpMania::incompatible_mods().collect(),
            Self::WindDownMania(_) => WindDownMania::incompatible_mods().collect(),
            Self::MutedMania(_) => MutedMania::incompatible_mods().collect(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::incompatible_mods().collect(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::incompatible_mods().collect(),
        }
    }
    /// The description of this [`GameMod`]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::EasyOsu(_) => EasyOsu::description(),
            Self::NoFailOsu(_) => NoFailOsu::description(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::description(),
            Self::DaycoreOsu(_) => DaycoreOsu::description(),
            Self::HardRockOsu(_) => HardRockOsu::description(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::description(),
            Self::PerfectOsu(_) => PerfectOsu::description(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::description(),
            Self::NightcoreOsu(_) => NightcoreOsu::description(),
            Self::HiddenOsu(_) => HiddenOsu::description(),
            Self::FlashlightOsu(_) => FlashlightOsu::description(),
            Self::BlindsOsu(_) => BlindsOsu::description(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::description(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::description(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::description(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::description(),
            Self::ClassicOsu(_) => ClassicOsu::description(),
            Self::RandomOsu(_) => RandomOsu::description(),
            Self::MirrorOsu(_) => MirrorOsu::description(),
            Self::AlternateOsu(_) => AlternateOsu::description(),
            Self::SingleTapOsu(_) => SingleTapOsu::description(),
            Self::AutoplayOsu(_) => AutoplayOsu::description(),
            Self::CinemaOsu(_) => CinemaOsu::description(),
            Self::RelaxOsu(_) => RelaxOsu::description(),
            Self::AutopilotOsu(_) => AutopilotOsu::description(),
            Self::SpunOutOsu(_) => SpunOutOsu::description(),
            Self::TransformOsu(_) => TransformOsu::description(),
            Self::WiggleOsu(_) => WiggleOsu::description(),
            Self::SpinInOsu(_) => SpinInOsu::description(),
            Self::GrowOsu(_) => GrowOsu::description(),
            Self::DeflateOsu(_) => DeflateOsu::description(),
            Self::WindUpOsu(_) => WindUpOsu::description(),
            Self::WindDownOsu(_) => WindDownOsu::description(),
            Self::TraceableOsu(_) => TraceableOsu::description(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::description(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::description(),
            Self::MutedOsu(_) => MutedOsu::description(),
            Self::NoScopeOsu(_) => NoScopeOsu::description(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::description(),
            Self::RepelOsu(_) => RepelOsu::description(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::description(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::description(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::description(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::description(),
            Self::EasyTaiko(_) => EasyTaiko::description(),
            Self::NoFailTaiko(_) => NoFailTaiko::description(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::description(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::description(),
            Self::HardRockTaiko(_) => HardRockTaiko::description(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::description(),
            Self::PerfectTaiko(_) => PerfectTaiko::description(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::description(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::description(),
            Self::HiddenTaiko(_) => HiddenTaiko::description(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::description(),
            Self::AccuracyChallengeTaiko(_) => AccuracyChallengeTaiko::description(),
            Self::RandomTaiko(_) => RandomTaiko::description(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::description(),
            Self::ClassicTaiko(_) => ClassicTaiko::description(),
            Self::SwapTaiko(_) => SwapTaiko::description(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::description(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::description(),
            Self::CinemaTaiko(_) => CinemaTaiko::description(),
            Self::RelaxTaiko(_) => RelaxTaiko::description(),
            Self::WindUpTaiko(_) => WindUpTaiko::description(),
            Self::WindDownTaiko(_) => WindDownTaiko::description(),
            Self::MutedTaiko(_) => MutedTaiko::description(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::description(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::description(),
            Self::EasyCatch(_) => EasyCatch::description(),
            Self::NoFailCatch(_) => NoFailCatch::description(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::description(),
            Self::DaycoreCatch(_) => DaycoreCatch::description(),
            Self::HardRockCatch(_) => HardRockCatch::description(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::description(),
            Self::PerfectCatch(_) => PerfectCatch::description(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::description(),
            Self::NightcoreCatch(_) => NightcoreCatch::description(),
            Self::HiddenCatch(_) => HiddenCatch::description(),
            Self::FlashlightCatch(_) => FlashlightCatch::description(),
            Self::AccuracyChallengeCatch(_) => AccuracyChallengeCatch::description(),
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::description(),
            Self::ClassicCatch(_) => ClassicCatch::description(),
            Self::MirrorCatch(_) => MirrorCatch::description(),
            Self::AutoplayCatch(_) => AutoplayCatch::description(),
            Self::CinemaCatch(_) => CinemaCatch::description(),
            Self::RelaxCatch(_) => RelaxCatch::description(),
            Self::WindUpCatch(_) => WindUpCatch::description(),
            Self::WindDownCatch(_) => WindDownCatch::description(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::description(),
            Self::MutedCatch(_) => MutedCatch::description(),
            Self::NoScopeCatch(_) => NoScopeCatch::description(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::description(),
            Self::EasyMania(_) => EasyMania::description(),
            Self::NoFailMania(_) => NoFailMania::description(),
            Self::HalfTimeMania(_) => HalfTimeMania::description(),
            Self::DaycoreMania(_) => DaycoreMania::description(),
            Self::HardRockMania(_) => HardRockMania::description(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::description(),
            Self::PerfectMania(_) => PerfectMania::description(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::description(),
            Self::NightcoreMania(_) => NightcoreMania::description(),
            Self::FadeInMania(_) => FadeInMania::description(),
            Self::HiddenMania(_) => HiddenMania::description(),
            Self::FlashlightMania(_) => FlashlightMania::description(),
            Self::AccuracyChallengeMania(_) => AccuracyChallengeMania::description(),
            Self::FourKeysMania(_) => FourKeysMania::description(),
            Self::FiveKeysMania(_) => FiveKeysMania::description(),
            Self::SixKeysMania(_) => SixKeysMania::description(),
            Self::SevenKeysMania(_) => SevenKeysMania::description(),
            Self::EightKeysMania(_) => EightKeysMania::description(),
            Self::NineKeysMania(_) => NineKeysMania::description(),
            Self::TenKeysMania(_) => TenKeysMania::description(),
            Self::OneKeyMania(_) => OneKeyMania::description(),
            Self::TwoKeysMania(_) => TwoKeysMania::description(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::description(),
            Self::RandomMania(_) => RandomMania::description(),
            Self::DualStagesMania(_) => DualStagesMania::description(),
            Self::MirrorMania(_) => MirrorMania::description(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::description(),
            Self::ClassicMania(_) => ClassicMania::description(),
            Self::InvertMania(_) => InvertMania::description(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::description(),
            Self::HoldOffMania(_) => HoldOffMania::description(),
            Self::AutoplayMania(_) => AutoplayMania::description(),
            Self::CinemaMania(_) => CinemaMania::description(),
            Self::WindUpMania(_) => WindUpMania::description(),
            Self::WindDownMania(_) => WindDownMania::description(),
            Self::MutedMania(_) => MutedMania::description(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::description(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::description(),
        }
    }
    /// The [`GameModKind`] of this [`GameMod`]
    pub const fn kind(&self) -> GameModKind {
        match self {
            Self::EasyOsu(_) => EasyOsu::kind(),
            Self::NoFailOsu(_) => NoFailOsu::kind(),
            Self::HalfTimeOsu(_) => HalfTimeOsu::kind(),
            Self::DaycoreOsu(_) => DaycoreOsu::kind(),
            Self::HardRockOsu(_) => HardRockOsu::kind(),
            Self::SuddenDeathOsu(_) => SuddenDeathOsu::kind(),
            Self::PerfectOsu(_) => PerfectOsu::kind(),
            Self::DoubleTimeOsu(_) => DoubleTimeOsu::kind(),
            Self::NightcoreOsu(_) => NightcoreOsu::kind(),
            Self::HiddenOsu(_) => HiddenOsu::kind(),
            Self::FlashlightOsu(_) => FlashlightOsu::kind(),
            Self::BlindsOsu(_) => BlindsOsu::kind(),
            Self::StrictTrackingOsu(_) => StrictTrackingOsu::kind(),
            Self::AccuracyChallengeOsu(_) => AccuracyChallengeOsu::kind(),
            Self::TargetPracticeOsu(_) => TargetPracticeOsu::kind(),
            Self::DifficultyAdjustOsu(_) => DifficultyAdjustOsu::kind(),
            Self::ClassicOsu(_) => ClassicOsu::kind(),
            Self::RandomOsu(_) => RandomOsu::kind(),
            Self::MirrorOsu(_) => MirrorOsu::kind(),
            Self::AlternateOsu(_) => AlternateOsu::kind(),
            Self::SingleTapOsu(_) => SingleTapOsu::kind(),
            Self::AutoplayOsu(_) => AutoplayOsu::kind(),
            Self::CinemaOsu(_) => CinemaOsu::kind(),
            Self::RelaxOsu(_) => RelaxOsu::kind(),
            Self::AutopilotOsu(_) => AutopilotOsu::kind(),
            Self::SpunOutOsu(_) => SpunOutOsu::kind(),
            Self::TransformOsu(_) => TransformOsu::kind(),
            Self::WiggleOsu(_) => WiggleOsu::kind(),
            Self::SpinInOsu(_) => SpinInOsu::kind(),
            Self::GrowOsu(_) => GrowOsu::kind(),
            Self::DeflateOsu(_) => DeflateOsu::kind(),
            Self::WindUpOsu(_) => WindUpOsu::kind(),
            Self::WindDownOsu(_) => WindDownOsu::kind(),
            Self::TraceableOsu(_) => TraceableOsu::kind(),
            Self::BarrelRollOsu(_) => BarrelRollOsu::kind(),
            Self::ApproachDifferentOsu(_) => ApproachDifferentOsu::kind(),
            Self::MutedOsu(_) => MutedOsu::kind(),
            Self::NoScopeOsu(_) => NoScopeOsu::kind(),
            Self::MagnetisedOsu(_) => MagnetisedOsu::kind(),
            Self::RepelOsu(_) => RepelOsu::kind(),
            Self::AdaptiveSpeedOsu(_) => AdaptiveSpeedOsu::kind(),
            Self::FreezeFrameOsu(_) => FreezeFrameOsu::kind(),
            Self::TouchDeviceOsu(_) => TouchDeviceOsu::kind(),
            Self::ScoreV2Osu(_) => ScoreV2Osu::kind(),
            Self::EasyTaiko(_) => EasyTaiko::kind(),
            Self::NoFailTaiko(_) => NoFailTaiko::kind(),
            Self::HalfTimeTaiko(_) => HalfTimeTaiko::kind(),
            Self::DaycoreTaiko(_) => DaycoreTaiko::kind(),
            Self::HardRockTaiko(_) => HardRockTaiko::kind(),
            Self::SuddenDeathTaiko(_) => SuddenDeathTaiko::kind(),
            Self::PerfectTaiko(_) => PerfectTaiko::kind(),
            Self::DoubleTimeTaiko(_) => DoubleTimeTaiko::kind(),
            Self::NightcoreTaiko(_) => NightcoreTaiko::kind(),
            Self::HiddenTaiko(_) => HiddenTaiko::kind(),
            Self::FlashlightTaiko(_) => FlashlightTaiko::kind(),
            Self::AccuracyChallengeTaiko(_) => AccuracyChallengeTaiko::kind(),
            Self::RandomTaiko(_) => RandomTaiko::kind(),
            Self::DifficultyAdjustTaiko(_) => DifficultyAdjustTaiko::kind(),
            Self::ClassicTaiko(_) => ClassicTaiko::kind(),
            Self::SwapTaiko(_) => SwapTaiko::kind(),
            Self::SingleTapTaiko(_) => SingleTapTaiko::kind(),
            Self::AutoplayTaiko(_) => AutoplayTaiko::kind(),
            Self::CinemaTaiko(_) => CinemaTaiko::kind(),
            Self::RelaxTaiko(_) => RelaxTaiko::kind(),
            Self::WindUpTaiko(_) => WindUpTaiko::kind(),
            Self::WindDownTaiko(_) => WindDownTaiko::kind(),
            Self::MutedTaiko(_) => MutedTaiko::kind(),
            Self::AdaptiveSpeedTaiko(_) => AdaptiveSpeedTaiko::kind(),
            Self::ScoreV2Taiko(_) => ScoreV2Taiko::kind(),
            Self::EasyCatch(_) => EasyCatch::kind(),
            Self::NoFailCatch(_) => NoFailCatch::kind(),
            Self::HalfTimeCatch(_) => HalfTimeCatch::kind(),
            Self::DaycoreCatch(_) => DaycoreCatch::kind(),
            Self::HardRockCatch(_) => HardRockCatch::kind(),
            Self::SuddenDeathCatch(_) => SuddenDeathCatch::kind(),
            Self::PerfectCatch(_) => PerfectCatch::kind(),
            Self::DoubleTimeCatch(_) => DoubleTimeCatch::kind(),
            Self::NightcoreCatch(_) => NightcoreCatch::kind(),
            Self::HiddenCatch(_) => HiddenCatch::kind(),
            Self::FlashlightCatch(_) => FlashlightCatch::kind(),
            Self::AccuracyChallengeCatch(_) => AccuracyChallengeCatch::kind(),
            Self::DifficultyAdjustCatch(_) => DifficultyAdjustCatch::kind(),
            Self::ClassicCatch(_) => ClassicCatch::kind(),
            Self::MirrorCatch(_) => MirrorCatch::kind(),
            Self::AutoplayCatch(_) => AutoplayCatch::kind(),
            Self::CinemaCatch(_) => CinemaCatch::kind(),
            Self::RelaxCatch(_) => RelaxCatch::kind(),
            Self::WindUpCatch(_) => WindUpCatch::kind(),
            Self::WindDownCatch(_) => WindDownCatch::kind(),
            Self::FloatingFruitsCatch(_) => FloatingFruitsCatch::kind(),
            Self::MutedCatch(_) => MutedCatch::kind(),
            Self::NoScopeCatch(_) => NoScopeCatch::kind(),
            Self::ScoreV2Catch(_) => ScoreV2Catch::kind(),
            Self::EasyMania(_) => EasyMania::kind(),
            Self::NoFailMania(_) => NoFailMania::kind(),
            Self::HalfTimeMania(_) => HalfTimeMania::kind(),
            Self::DaycoreMania(_) => DaycoreMania::kind(),
            Self::HardRockMania(_) => HardRockMania::kind(),
            Self::SuddenDeathMania(_) => SuddenDeathMania::kind(),
            Self::PerfectMania(_) => PerfectMania::kind(),
            Self::DoubleTimeMania(_) => DoubleTimeMania::kind(),
            Self::NightcoreMania(_) => NightcoreMania::kind(),
            Self::FadeInMania(_) => FadeInMania::kind(),
            Self::HiddenMania(_) => HiddenMania::kind(),
            Self::FlashlightMania(_) => FlashlightMania::kind(),
            Self::AccuracyChallengeMania(_) => AccuracyChallengeMania::kind(),
            Self::FourKeysMania(_) => FourKeysMania::kind(),
            Self::FiveKeysMania(_) => FiveKeysMania::kind(),
            Self::SixKeysMania(_) => SixKeysMania::kind(),
            Self::SevenKeysMania(_) => SevenKeysMania::kind(),
            Self::EightKeysMania(_) => EightKeysMania::kind(),
            Self::NineKeysMania(_) => NineKeysMania::kind(),
            Self::TenKeysMania(_) => TenKeysMania::kind(),
            Self::OneKeyMania(_) => OneKeyMania::kind(),
            Self::TwoKeysMania(_) => TwoKeysMania::kind(),
            Self::ThreeKeysMania(_) => ThreeKeysMania::kind(),
            Self::RandomMania(_) => RandomMania::kind(),
            Self::DualStagesMania(_) => DualStagesMania::kind(),
            Self::MirrorMania(_) => MirrorMania::kind(),
            Self::DifficultyAdjustMania(_) => DifficultyAdjustMania::kind(),
            Self::ClassicMania(_) => ClassicMania::kind(),
            Self::InvertMania(_) => InvertMania::kind(),
            Self::ConstantSpeedMania(_) => ConstantSpeedMania::kind(),
            Self::HoldOffMania(_) => HoldOffMania::kind(),
            Self::AutoplayMania(_) => AutoplayMania::kind(),
            Self::CinemaMania(_) => CinemaMania::kind(),
            Self::WindUpMania(_) => WindUpMania::kind(),
            Self::WindDownMania(_) => WindDownMania::kind(),
            Self::MutedMania(_) => MutedMania::kind(),
            Self::AdaptiveSpeedMania(_) => AdaptiveSpeedMania::kind(),
            Self::ScoreV2Mania(_) => ScoreV2Mania::kind(),
        }
    }
    /// Optional bit value of this [`GameMod`]
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub const fn bits(&self) -> Option<u32> {
        match self {
            Self::EasyOsu(_) => Some(EasyOsu::bits()),
            Self::NoFailOsu(_) => Some(NoFailOsu::bits()),
            Self::HalfTimeOsu(_) => Some(HalfTimeOsu::bits()),
            Self::HardRockOsu(_) => Some(HardRockOsu::bits()),
            Self::SuddenDeathOsu(_) => Some(SuddenDeathOsu::bits()),
            Self::PerfectOsu(_) => Some(PerfectOsu::bits()),
            Self::DoubleTimeOsu(_) => Some(DoubleTimeOsu::bits()),
            Self::NightcoreOsu(_) => Some(NightcoreOsu::bits()),
            Self::HiddenOsu(_) => Some(HiddenOsu::bits()),
            Self::FlashlightOsu(_) => Some(FlashlightOsu::bits()),
            Self::TargetPracticeOsu(_) => Some(TargetPracticeOsu::bits()),
            Self::RandomOsu(_) => Some(RandomOsu::bits()),
            Self::MirrorOsu(_) => Some(MirrorOsu::bits()),
            Self::AutoplayOsu(_) => Some(AutoplayOsu::bits()),
            Self::CinemaOsu(_) => Some(CinemaOsu::bits()),
            Self::RelaxOsu(_) => Some(RelaxOsu::bits()),
            Self::AutopilotOsu(_) => Some(AutopilotOsu::bits()),
            Self::SpunOutOsu(_) => Some(SpunOutOsu::bits()),
            Self::TouchDeviceOsu(_) => Some(TouchDeviceOsu::bits()),
            Self::ScoreV2Osu(_) => Some(ScoreV2Osu::bits()),
            Self::EasyTaiko(_) => Some(EasyTaiko::bits()),
            Self::NoFailTaiko(_) => Some(NoFailTaiko::bits()),
            Self::HalfTimeTaiko(_) => Some(HalfTimeTaiko::bits()),
            Self::HardRockTaiko(_) => Some(HardRockTaiko::bits()),
            Self::SuddenDeathTaiko(_) => Some(SuddenDeathTaiko::bits()),
            Self::PerfectTaiko(_) => Some(PerfectTaiko::bits()),
            Self::DoubleTimeTaiko(_) => Some(DoubleTimeTaiko::bits()),
            Self::NightcoreTaiko(_) => Some(NightcoreTaiko::bits()),
            Self::HiddenTaiko(_) => Some(HiddenTaiko::bits()),
            Self::FlashlightTaiko(_) => Some(FlashlightTaiko::bits()),
            Self::RandomTaiko(_) => Some(RandomTaiko::bits()),
            Self::AutoplayTaiko(_) => Some(AutoplayTaiko::bits()),
            Self::CinemaTaiko(_) => Some(CinemaTaiko::bits()),
            Self::RelaxTaiko(_) => Some(RelaxTaiko::bits()),
            Self::ScoreV2Taiko(_) => Some(ScoreV2Taiko::bits()),
            Self::EasyCatch(_) => Some(EasyCatch::bits()),
            Self::NoFailCatch(_) => Some(NoFailCatch::bits()),
            Self::HalfTimeCatch(_) => Some(HalfTimeCatch::bits()),
            Self::HardRockCatch(_) => Some(HardRockCatch::bits()),
            Self::SuddenDeathCatch(_) => Some(SuddenDeathCatch::bits()),
            Self::PerfectCatch(_) => Some(PerfectCatch::bits()),
            Self::DoubleTimeCatch(_) => Some(DoubleTimeCatch::bits()),
            Self::NightcoreCatch(_) => Some(NightcoreCatch::bits()),
            Self::HiddenCatch(_) => Some(HiddenCatch::bits()),
            Self::FlashlightCatch(_) => Some(FlashlightCatch::bits()),
            Self::MirrorCatch(_) => Some(MirrorCatch::bits()),
            Self::AutoplayCatch(_) => Some(AutoplayCatch::bits()),
            Self::CinemaCatch(_) => Some(CinemaCatch::bits()),
            Self::RelaxCatch(_) => Some(RelaxCatch::bits()),
            Self::ScoreV2Catch(_) => Some(ScoreV2Catch::bits()),
            Self::EasyMania(_) => Some(EasyMania::bits()),
            Self::NoFailMania(_) => Some(NoFailMania::bits()),
            Self::HalfTimeMania(_) => Some(HalfTimeMania::bits()),
            Self::HardRockMania(_) => Some(HardRockMania::bits()),
            Self::SuddenDeathMania(_) => Some(SuddenDeathMania::bits()),
            Self::PerfectMania(_) => Some(PerfectMania::bits()),
            Self::DoubleTimeMania(_) => Some(DoubleTimeMania::bits()),
            Self::NightcoreMania(_) => Some(NightcoreMania::bits()),
            Self::FadeInMania(_) => Some(FadeInMania::bits()),
            Self::HiddenMania(_) => Some(HiddenMania::bits()),
            Self::FlashlightMania(_) => Some(FlashlightMania::bits()),
            Self::FourKeysMania(_) => Some(FourKeysMania::bits()),
            Self::FiveKeysMania(_) => Some(FiveKeysMania::bits()),
            Self::SixKeysMania(_) => Some(SixKeysMania::bits()),
            Self::SevenKeysMania(_) => Some(SevenKeysMania::bits()),
            Self::EightKeysMania(_) => Some(EightKeysMania::bits()),
            Self::NineKeysMania(_) => Some(NineKeysMania::bits()),
            Self::OneKeyMania(_) => Some(OneKeyMania::bits()),
            Self::TwoKeysMania(_) => Some(TwoKeysMania::bits()),
            Self::ThreeKeysMania(_) => Some(ThreeKeysMania::bits()),
            Self::RandomMania(_) => Some(RandomMania::bits()),
            Self::DualStagesMania(_) => Some(DualStagesMania::bits()),
            Self::MirrorMania(_) => Some(MirrorMania::bits()),
            Self::AutoplayMania(_) => Some(AutoplayMania::bits()),
            Self::CinemaMania(_) => Some(CinemaMania::bits()),
            Self::ScoreV2Mania(_) => Some(ScoreV2Mania::bits()),
            _ => None,
        }
    }
    /// The [`GameMode`] of a [`GameMod`]
    pub const fn mode(&self) -> GameMode {
        match self {
            Self::EasyOsu(_)
            | Self::NoFailOsu(_)
            | Self::HalfTimeOsu(_)
            | Self::DaycoreOsu(_)
            | Self::HardRockOsu(_)
            | Self::SuddenDeathOsu(_)
            | Self::PerfectOsu(_)
            | Self::DoubleTimeOsu(_)
            | Self::NightcoreOsu(_)
            | Self::HiddenOsu(_)
            | Self::FlashlightOsu(_)
            | Self::BlindsOsu(_)
            | Self::StrictTrackingOsu(_)
            | Self::AccuracyChallengeOsu(_)
            | Self::TargetPracticeOsu(_)
            | Self::DifficultyAdjustOsu(_)
            | Self::ClassicOsu(_)
            | Self::RandomOsu(_)
            | Self::MirrorOsu(_)
            | Self::AlternateOsu(_)
            | Self::SingleTapOsu(_)
            | Self::AutoplayOsu(_)
            | Self::CinemaOsu(_)
            | Self::RelaxOsu(_)
            | Self::AutopilotOsu(_)
            | Self::SpunOutOsu(_)
            | Self::TransformOsu(_)
            | Self::WiggleOsu(_)
            | Self::SpinInOsu(_)
            | Self::GrowOsu(_)
            | Self::DeflateOsu(_)
            | Self::WindUpOsu(_)
            | Self::WindDownOsu(_)
            | Self::TraceableOsu(_)
            | Self::BarrelRollOsu(_)
            | Self::ApproachDifferentOsu(_)
            | Self::MutedOsu(_)
            | Self::NoScopeOsu(_)
            | Self::MagnetisedOsu(_)
            | Self::RepelOsu(_)
            | Self::AdaptiveSpeedOsu(_)
            | Self::FreezeFrameOsu(_)
            | Self::TouchDeviceOsu(_)
            | Self::ScoreV2Osu(_) => GameMode::Osu,
            Self::EasyTaiko(_)
            | Self::NoFailTaiko(_)
            | Self::HalfTimeTaiko(_)
            | Self::DaycoreTaiko(_)
            | Self::HardRockTaiko(_)
            | Self::SuddenDeathTaiko(_)
            | Self::PerfectTaiko(_)
            | Self::DoubleTimeTaiko(_)
            | Self::NightcoreTaiko(_)
            | Self::HiddenTaiko(_)
            | Self::FlashlightTaiko(_)
            | Self::AccuracyChallengeTaiko(_)
            | Self::RandomTaiko(_)
            | Self::DifficultyAdjustTaiko(_)
            | Self::ClassicTaiko(_)
            | Self::SwapTaiko(_)
            | Self::SingleTapTaiko(_)
            | Self::AutoplayTaiko(_)
            | Self::CinemaTaiko(_)
            | Self::RelaxTaiko(_)
            | Self::WindUpTaiko(_)
            | Self::WindDownTaiko(_)
            | Self::MutedTaiko(_)
            | Self::AdaptiveSpeedTaiko(_)
            | Self::ScoreV2Taiko(_) => GameMode::Taiko,
            Self::EasyCatch(_)
            | Self::NoFailCatch(_)
            | Self::HalfTimeCatch(_)
            | Self::DaycoreCatch(_)
            | Self::HardRockCatch(_)
            | Self::SuddenDeathCatch(_)
            | Self::PerfectCatch(_)
            | Self::DoubleTimeCatch(_)
            | Self::NightcoreCatch(_)
            | Self::HiddenCatch(_)
            | Self::FlashlightCatch(_)
            | Self::AccuracyChallengeCatch(_)
            | Self::DifficultyAdjustCatch(_)
            | Self::ClassicCatch(_)
            | Self::MirrorCatch(_)
            | Self::AutoplayCatch(_)
            | Self::CinemaCatch(_)
            | Self::RelaxCatch(_)
            | Self::WindUpCatch(_)
            | Self::WindDownCatch(_)
            | Self::FloatingFruitsCatch(_)
            | Self::MutedCatch(_)
            | Self::NoScopeCatch(_)
            | Self::ScoreV2Catch(_) => GameMode::Catch,
            Self::EasyMania(_)
            | Self::NoFailMania(_)
            | Self::HalfTimeMania(_)
            | Self::DaycoreMania(_)
            | Self::HardRockMania(_)
            | Self::SuddenDeathMania(_)
            | Self::PerfectMania(_)
            | Self::DoubleTimeMania(_)
            | Self::NightcoreMania(_)
            | Self::FadeInMania(_)
            | Self::HiddenMania(_)
            | Self::FlashlightMania(_)
            | Self::AccuracyChallengeMania(_)
            | Self::FourKeysMania(_)
            | Self::FiveKeysMania(_)
            | Self::SixKeysMania(_)
            | Self::SevenKeysMania(_)
            | Self::EightKeysMania(_)
            | Self::NineKeysMania(_)
            | Self::TenKeysMania(_)
            | Self::OneKeyMania(_)
            | Self::TwoKeysMania(_)
            | Self::ThreeKeysMania(_)
            | Self::RandomMania(_)
            | Self::DualStagesMania(_)
            | Self::MirrorMania(_)
            | Self::DifficultyAdjustMania(_)
            | Self::ClassicMania(_)
            | Self::InvertMania(_)
            | Self::ConstantSpeedMania(_)
            | Self::HoldOffMania(_)
            | Self::AutoplayMania(_)
            | Self::CinemaMania(_)
            | Self::WindUpMania(_)
            | Self::WindDownMania(_)
            | Self::MutedMania(_)
            | Self::AdaptiveSpeedMania(_)
            | Self::ScoreV2Mania(_) => GameMode::Mania,
        }
    }
    /// The kind of a [`GameMod`] when ignoring the mode
    pub const fn intermode(&self) -> GameModIntermode {
        match self {
            Self::EasyOsu(_) => GameModIntermode::Easy,
            Self::NoFailOsu(_) => GameModIntermode::NoFail,
            Self::HalfTimeOsu(_) => GameModIntermode::HalfTime,
            Self::DaycoreOsu(_) => GameModIntermode::Daycore,
            Self::HardRockOsu(_) => GameModIntermode::HardRock,
            Self::SuddenDeathOsu(_) => GameModIntermode::SuddenDeath,
            Self::PerfectOsu(_) => GameModIntermode::Perfect,
            Self::DoubleTimeOsu(_) => GameModIntermode::DoubleTime,
            Self::NightcoreOsu(_) => GameModIntermode::Nightcore,
            Self::HiddenOsu(_) => GameModIntermode::Hidden,
            Self::FlashlightOsu(_) => GameModIntermode::Flashlight,
            Self::BlindsOsu(_) => GameModIntermode::Blinds,
            Self::StrictTrackingOsu(_) => GameModIntermode::StrictTracking,
            Self::AccuracyChallengeOsu(_) => GameModIntermode::AccuracyChallenge,
            Self::TargetPracticeOsu(_) => GameModIntermode::TargetPractice,
            Self::DifficultyAdjustOsu(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicOsu(_) => GameModIntermode::Classic,
            Self::RandomOsu(_) => GameModIntermode::Random,
            Self::MirrorOsu(_) => GameModIntermode::Mirror,
            Self::AlternateOsu(_) => GameModIntermode::Alternate,
            Self::SingleTapOsu(_) => GameModIntermode::SingleTap,
            Self::AutoplayOsu(_) => GameModIntermode::Autoplay,
            Self::CinemaOsu(_) => GameModIntermode::Cinema,
            Self::RelaxOsu(_) => GameModIntermode::Relax,
            Self::AutopilotOsu(_) => GameModIntermode::Autopilot,
            Self::SpunOutOsu(_) => GameModIntermode::SpunOut,
            Self::TransformOsu(_) => GameModIntermode::Transform,
            Self::WiggleOsu(_) => GameModIntermode::Wiggle,
            Self::SpinInOsu(_) => GameModIntermode::SpinIn,
            Self::GrowOsu(_) => GameModIntermode::Grow,
            Self::DeflateOsu(_) => GameModIntermode::Deflate,
            Self::WindUpOsu(_) => GameModIntermode::WindUp,
            Self::WindDownOsu(_) => GameModIntermode::WindDown,
            Self::TraceableOsu(_) => GameModIntermode::Traceable,
            Self::BarrelRollOsu(_) => GameModIntermode::BarrelRoll,
            Self::ApproachDifferentOsu(_) => GameModIntermode::ApproachDifferent,
            Self::MutedOsu(_) => GameModIntermode::Muted,
            Self::NoScopeOsu(_) => GameModIntermode::NoScope,
            Self::MagnetisedOsu(_) => GameModIntermode::Magnetised,
            Self::RepelOsu(_) => GameModIntermode::Repel,
            Self::AdaptiveSpeedOsu(_) => GameModIntermode::AdaptiveSpeed,
            Self::FreezeFrameOsu(_) => GameModIntermode::FreezeFrame,
            Self::TouchDeviceOsu(_) => GameModIntermode::TouchDevice,
            Self::ScoreV2Osu(_) => GameModIntermode::ScoreV2,
            Self::EasyTaiko(_) => GameModIntermode::Easy,
            Self::NoFailTaiko(_) => GameModIntermode::NoFail,
            Self::HalfTimeTaiko(_) => GameModIntermode::HalfTime,
            Self::DaycoreTaiko(_) => GameModIntermode::Daycore,
            Self::HardRockTaiko(_) => GameModIntermode::HardRock,
            Self::SuddenDeathTaiko(_) => GameModIntermode::SuddenDeath,
            Self::PerfectTaiko(_) => GameModIntermode::Perfect,
            Self::DoubleTimeTaiko(_) => GameModIntermode::DoubleTime,
            Self::NightcoreTaiko(_) => GameModIntermode::Nightcore,
            Self::HiddenTaiko(_) => GameModIntermode::Hidden,
            Self::FlashlightTaiko(_) => GameModIntermode::Flashlight,
            Self::AccuracyChallengeTaiko(_) => GameModIntermode::AccuracyChallenge,
            Self::RandomTaiko(_) => GameModIntermode::Random,
            Self::DifficultyAdjustTaiko(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicTaiko(_) => GameModIntermode::Classic,
            Self::SwapTaiko(_) => GameModIntermode::Swap,
            Self::SingleTapTaiko(_) => GameModIntermode::SingleTap,
            Self::AutoplayTaiko(_) => GameModIntermode::Autoplay,
            Self::CinemaTaiko(_) => GameModIntermode::Cinema,
            Self::RelaxTaiko(_) => GameModIntermode::Relax,
            Self::WindUpTaiko(_) => GameModIntermode::WindUp,
            Self::WindDownTaiko(_) => GameModIntermode::WindDown,
            Self::MutedTaiko(_) => GameModIntermode::Muted,
            Self::AdaptiveSpeedTaiko(_) => GameModIntermode::AdaptiveSpeed,
            Self::ScoreV2Taiko(_) => GameModIntermode::ScoreV2,
            Self::EasyCatch(_) => GameModIntermode::Easy,
            Self::NoFailCatch(_) => GameModIntermode::NoFail,
            Self::HalfTimeCatch(_) => GameModIntermode::HalfTime,
            Self::DaycoreCatch(_) => GameModIntermode::Daycore,
            Self::HardRockCatch(_) => GameModIntermode::HardRock,
            Self::SuddenDeathCatch(_) => GameModIntermode::SuddenDeath,
            Self::PerfectCatch(_) => GameModIntermode::Perfect,
            Self::DoubleTimeCatch(_) => GameModIntermode::DoubleTime,
            Self::NightcoreCatch(_) => GameModIntermode::Nightcore,
            Self::HiddenCatch(_) => GameModIntermode::Hidden,
            Self::FlashlightCatch(_) => GameModIntermode::Flashlight,
            Self::AccuracyChallengeCatch(_) => GameModIntermode::AccuracyChallenge,
            Self::DifficultyAdjustCatch(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicCatch(_) => GameModIntermode::Classic,
            Self::MirrorCatch(_) => GameModIntermode::Mirror,
            Self::AutoplayCatch(_) => GameModIntermode::Autoplay,
            Self::CinemaCatch(_) => GameModIntermode::Cinema,
            Self::RelaxCatch(_) => GameModIntermode::Relax,
            Self::WindUpCatch(_) => GameModIntermode::WindUp,
            Self::WindDownCatch(_) => GameModIntermode::WindDown,
            Self::FloatingFruitsCatch(_) => GameModIntermode::FloatingFruits,
            Self::MutedCatch(_) => GameModIntermode::Muted,
            Self::NoScopeCatch(_) => GameModIntermode::NoScope,
            Self::ScoreV2Catch(_) => GameModIntermode::ScoreV2,
            Self::EasyMania(_) => GameModIntermode::Easy,
            Self::NoFailMania(_) => GameModIntermode::NoFail,
            Self::HalfTimeMania(_) => GameModIntermode::HalfTime,
            Self::DaycoreMania(_) => GameModIntermode::Daycore,
            Self::HardRockMania(_) => GameModIntermode::HardRock,
            Self::SuddenDeathMania(_) => GameModIntermode::SuddenDeath,
            Self::PerfectMania(_) => GameModIntermode::Perfect,
            Self::DoubleTimeMania(_) => GameModIntermode::DoubleTime,
            Self::NightcoreMania(_) => GameModIntermode::Nightcore,
            Self::FadeInMania(_) => GameModIntermode::FadeIn,
            Self::HiddenMania(_) => GameModIntermode::Hidden,
            Self::FlashlightMania(_) => GameModIntermode::Flashlight,
            Self::AccuracyChallengeMania(_) => GameModIntermode::AccuracyChallenge,
            Self::FourKeysMania(_) => GameModIntermode::FourKeys,
            Self::FiveKeysMania(_) => GameModIntermode::FiveKeys,
            Self::SixKeysMania(_) => GameModIntermode::SixKeys,
            Self::SevenKeysMania(_) => GameModIntermode::SevenKeys,
            Self::EightKeysMania(_) => GameModIntermode::EightKeys,
            Self::NineKeysMania(_) => GameModIntermode::NineKeys,
            Self::TenKeysMania(_) => GameModIntermode::TenKeys,
            Self::OneKeyMania(_) => GameModIntermode::OneKey,
            Self::TwoKeysMania(_) => GameModIntermode::TwoKeys,
            Self::ThreeKeysMania(_) => GameModIntermode::ThreeKeys,
            Self::RandomMania(_) => GameModIntermode::Random,
            Self::DualStagesMania(_) => GameModIntermode::DualStages,
            Self::MirrorMania(_) => GameModIntermode::Mirror,
            Self::DifficultyAdjustMania(_) => GameModIntermode::DifficultyAdjust,
            Self::ClassicMania(_) => GameModIntermode::Classic,
            Self::InvertMania(_) => GameModIntermode::Invert,
            Self::ConstantSpeedMania(_) => GameModIntermode::ConstantSpeed,
            Self::HoldOffMania(_) => GameModIntermode::HoldOff,
            Self::AutoplayMania(_) => GameModIntermode::Autoplay,
            Self::CinemaMania(_) => GameModIntermode::Cinema,
            Self::WindUpMania(_) => GameModIntermode::WindUp,
            Self::WindDownMania(_) => GameModIntermode::WindDown,
            Self::MutedMania(_) => GameModIntermode::Muted,
            Self::AdaptiveSpeedMania(_) => GameModIntermode::AdaptiveSpeed,
            Self::ScoreV2Mania(_) => GameModIntermode::ScoreV2,
        }
    }
}
impl PartialOrd for GameMod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits()
            .zip(other.bits())
            .map(|(self_bits, other_bits)| self_bits.cmp(&other_bits))
    }
}
struct GameModSettings<'a> {
    acronym: &'a str,
    mode: GameMode,
}
impl<'de> DeserializeSeed<'de> for GameModSettings<'de> {
    type Value = <Self as Visitor<'de>>::Value;
    fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for GameModSettings<'de> {
    type Value = GameMod;
    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("GameMod settings")
    }
    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
        let d = MapAccessDeserializer::new(map);
        let res = match (self.acronym, self.mode) {
            ("EZ", GameMode::Osu) => GameMod::EasyOsu(Deserialize::deserialize(d)?),
            ("NF", GameMode::Osu) => GameMod::NoFailOsu(Deserialize::deserialize(d)?),
            ("HT", GameMode::Osu) => GameMod::HalfTimeOsu(Deserialize::deserialize(d)?),
            ("DC", GameMode::Osu) => GameMod::DaycoreOsu(Deserialize::deserialize(d)?),
            ("HR", GameMode::Osu) => GameMod::HardRockOsu(Deserialize::deserialize(d)?),
            ("SD", GameMode::Osu) => GameMod::SuddenDeathOsu(Deserialize::deserialize(d)?),
            ("PF", GameMode::Osu) => GameMod::PerfectOsu(Deserialize::deserialize(d)?),
            ("DT", GameMode::Osu) => GameMod::DoubleTimeOsu(Deserialize::deserialize(d)?),
            ("NC", GameMode::Osu) => GameMod::NightcoreOsu(Deserialize::deserialize(d)?),
            ("HD", GameMode::Osu) => GameMod::HiddenOsu(Deserialize::deserialize(d)?),
            ("FL", GameMode::Osu) => GameMod::FlashlightOsu(Deserialize::deserialize(d)?),
            ("BL", GameMode::Osu) => GameMod::BlindsOsu(Deserialize::deserialize(d)?),
            ("ST", GameMode::Osu) => GameMod::StrictTrackingOsu(Deserialize::deserialize(d)?),
            ("AC", GameMode::Osu) => GameMod::AccuracyChallengeOsu(Deserialize::deserialize(d)?),
            ("TP", GameMode::Osu) => GameMod::TargetPracticeOsu(Deserialize::deserialize(d)?),
            ("DA", GameMode::Osu) => GameMod::DifficultyAdjustOsu(Deserialize::deserialize(d)?),
            ("CL", GameMode::Osu) => GameMod::ClassicOsu(Deserialize::deserialize(d)?),
            ("RD", GameMode::Osu) => GameMod::RandomOsu(Deserialize::deserialize(d)?),
            ("MR", GameMode::Osu) => GameMod::MirrorOsu(Deserialize::deserialize(d)?),
            ("AL", GameMode::Osu) => GameMod::AlternateOsu(Deserialize::deserialize(d)?),
            ("SG", GameMode::Osu) => GameMod::SingleTapOsu(Deserialize::deserialize(d)?),
            ("AT", GameMode::Osu) => GameMod::AutoplayOsu(Deserialize::deserialize(d)?),
            ("CN", GameMode::Osu) => GameMod::CinemaOsu(Deserialize::deserialize(d)?),
            ("RX", GameMode::Osu) => GameMod::RelaxOsu(Deserialize::deserialize(d)?),
            ("AP", GameMode::Osu) => GameMod::AutopilotOsu(Deserialize::deserialize(d)?),
            ("SO", GameMode::Osu) => GameMod::SpunOutOsu(Deserialize::deserialize(d)?),
            ("TR", GameMode::Osu) => GameMod::TransformOsu(Deserialize::deserialize(d)?),
            ("WG", GameMode::Osu) => GameMod::WiggleOsu(Deserialize::deserialize(d)?),
            ("SI", GameMode::Osu) => GameMod::SpinInOsu(Deserialize::deserialize(d)?),
            ("GR", GameMode::Osu) => GameMod::GrowOsu(Deserialize::deserialize(d)?),
            ("DF", GameMode::Osu) => GameMod::DeflateOsu(Deserialize::deserialize(d)?),
            ("WU", GameMode::Osu) => GameMod::WindUpOsu(Deserialize::deserialize(d)?),
            ("WD", GameMode::Osu) => GameMod::WindDownOsu(Deserialize::deserialize(d)?),
            ("TC", GameMode::Osu) => GameMod::TraceableOsu(Deserialize::deserialize(d)?),
            ("BR", GameMode::Osu) => GameMod::BarrelRollOsu(Deserialize::deserialize(d)?),
            ("AD", GameMode::Osu) => GameMod::ApproachDifferentOsu(Deserialize::deserialize(d)?),
            ("MU", GameMode::Osu) => GameMod::MutedOsu(Deserialize::deserialize(d)?),
            ("NS", GameMode::Osu) => GameMod::NoScopeOsu(Deserialize::deserialize(d)?),
            ("MG", GameMode::Osu) => GameMod::MagnetisedOsu(Deserialize::deserialize(d)?),
            ("RP", GameMode::Osu) => GameMod::RepelOsu(Deserialize::deserialize(d)?),
            ("AS", GameMode::Osu) => GameMod::AdaptiveSpeedOsu(Deserialize::deserialize(d)?),
            ("FR", GameMode::Osu) => GameMod::FreezeFrameOsu(Deserialize::deserialize(d)?),
            ("TD", GameMode::Osu) => GameMod::TouchDeviceOsu(Deserialize::deserialize(d)?),
            ("V2", GameMode::Osu) => GameMod::ScoreV2Osu(Deserialize::deserialize(d)?),
            ("EZ", GameMode::Taiko) => GameMod::EasyTaiko(Deserialize::deserialize(d)?),
            ("NF", GameMode::Taiko) => GameMod::NoFailTaiko(Deserialize::deserialize(d)?),
            ("HT", GameMode::Taiko) => GameMod::HalfTimeTaiko(Deserialize::deserialize(d)?),
            ("DC", GameMode::Taiko) => GameMod::DaycoreTaiko(Deserialize::deserialize(d)?),
            ("HR", GameMode::Taiko) => GameMod::HardRockTaiko(Deserialize::deserialize(d)?),
            ("SD", GameMode::Taiko) => GameMod::SuddenDeathTaiko(Deserialize::deserialize(d)?),
            ("PF", GameMode::Taiko) => GameMod::PerfectTaiko(Deserialize::deserialize(d)?),
            ("DT", GameMode::Taiko) => GameMod::DoubleTimeTaiko(Deserialize::deserialize(d)?),
            ("NC", GameMode::Taiko) => GameMod::NightcoreTaiko(Deserialize::deserialize(d)?),
            ("HD", GameMode::Taiko) => GameMod::HiddenTaiko(Deserialize::deserialize(d)?),
            ("FL", GameMode::Taiko) => GameMod::FlashlightTaiko(Deserialize::deserialize(d)?),
            ("AC", GameMode::Taiko) => {
                GameMod::AccuracyChallengeTaiko(Deserialize::deserialize(d)?)
            }
            ("RD", GameMode::Taiko) => GameMod::RandomTaiko(Deserialize::deserialize(d)?),
            ("DA", GameMode::Taiko) => GameMod::DifficultyAdjustTaiko(Deserialize::deserialize(d)?),
            ("CL", GameMode::Taiko) => GameMod::ClassicTaiko(Deserialize::deserialize(d)?),
            ("SW", GameMode::Taiko) => GameMod::SwapTaiko(Deserialize::deserialize(d)?),
            ("SG", GameMode::Taiko) => GameMod::SingleTapTaiko(Deserialize::deserialize(d)?),
            ("AT", GameMode::Taiko) => GameMod::AutoplayTaiko(Deserialize::deserialize(d)?),
            ("CN", GameMode::Taiko) => GameMod::CinemaTaiko(Deserialize::deserialize(d)?),
            ("RX", GameMode::Taiko) => GameMod::RelaxTaiko(Deserialize::deserialize(d)?),
            ("WU", GameMode::Taiko) => GameMod::WindUpTaiko(Deserialize::deserialize(d)?),
            ("WD", GameMode::Taiko) => GameMod::WindDownTaiko(Deserialize::deserialize(d)?),
            ("MU", GameMode::Taiko) => GameMod::MutedTaiko(Deserialize::deserialize(d)?),
            ("AS", GameMode::Taiko) => GameMod::AdaptiveSpeedTaiko(Deserialize::deserialize(d)?),
            ("V2", GameMode::Taiko) => GameMod::ScoreV2Taiko(Deserialize::deserialize(d)?),
            ("EZ", GameMode::Catch) => GameMod::EasyCatch(Deserialize::deserialize(d)?),
            ("NF", GameMode::Catch) => GameMod::NoFailCatch(Deserialize::deserialize(d)?),
            ("HT", GameMode::Catch) => GameMod::HalfTimeCatch(Deserialize::deserialize(d)?),
            ("DC", GameMode::Catch) => GameMod::DaycoreCatch(Deserialize::deserialize(d)?),
            ("HR", GameMode::Catch) => GameMod::HardRockCatch(Deserialize::deserialize(d)?),
            ("SD", GameMode::Catch) => GameMod::SuddenDeathCatch(Deserialize::deserialize(d)?),
            ("PF", GameMode::Catch) => GameMod::PerfectCatch(Deserialize::deserialize(d)?),
            ("DT", GameMode::Catch) => GameMod::DoubleTimeCatch(Deserialize::deserialize(d)?),
            ("NC", GameMode::Catch) => GameMod::NightcoreCatch(Deserialize::deserialize(d)?),
            ("HD", GameMode::Catch) => GameMod::HiddenCatch(Deserialize::deserialize(d)?),
            ("FL", GameMode::Catch) => GameMod::FlashlightCatch(Deserialize::deserialize(d)?),
            ("AC", GameMode::Catch) => {
                GameMod::AccuracyChallengeCatch(Deserialize::deserialize(d)?)
            }
            ("DA", GameMode::Catch) => GameMod::DifficultyAdjustCatch(Deserialize::deserialize(d)?),
            ("CL", GameMode::Catch) => GameMod::ClassicCatch(Deserialize::deserialize(d)?),
            ("MR", GameMode::Catch) => GameMod::MirrorCatch(Deserialize::deserialize(d)?),
            ("AT", GameMode::Catch) => GameMod::AutoplayCatch(Deserialize::deserialize(d)?),
            ("CN", GameMode::Catch) => GameMod::CinemaCatch(Deserialize::deserialize(d)?),
            ("RX", GameMode::Catch) => GameMod::RelaxCatch(Deserialize::deserialize(d)?),
            ("WU", GameMode::Catch) => GameMod::WindUpCatch(Deserialize::deserialize(d)?),
            ("WD", GameMode::Catch) => GameMod::WindDownCatch(Deserialize::deserialize(d)?),
            ("FF", GameMode::Catch) => GameMod::FloatingFruitsCatch(Deserialize::deserialize(d)?),
            ("MU", GameMode::Catch) => GameMod::MutedCatch(Deserialize::deserialize(d)?),
            ("NS", GameMode::Catch) => GameMod::NoScopeCatch(Deserialize::deserialize(d)?),
            ("V2", GameMode::Catch) => GameMod::ScoreV2Catch(Deserialize::deserialize(d)?),
            ("EZ", GameMode::Mania) => GameMod::EasyMania(Deserialize::deserialize(d)?),
            ("NF", GameMode::Mania) => GameMod::NoFailMania(Deserialize::deserialize(d)?),
            ("HT", GameMode::Mania) => GameMod::HalfTimeMania(Deserialize::deserialize(d)?),
            ("DC", GameMode::Mania) => GameMod::DaycoreMania(Deserialize::deserialize(d)?),
            ("HR", GameMode::Mania) => GameMod::HardRockMania(Deserialize::deserialize(d)?),
            ("SD", GameMode::Mania) => GameMod::SuddenDeathMania(Deserialize::deserialize(d)?),
            ("PF", GameMode::Mania) => GameMod::PerfectMania(Deserialize::deserialize(d)?),
            ("DT", GameMode::Mania) => GameMod::DoubleTimeMania(Deserialize::deserialize(d)?),
            ("NC", GameMode::Mania) => GameMod::NightcoreMania(Deserialize::deserialize(d)?),
            ("FI", GameMode::Mania) => GameMod::FadeInMania(Deserialize::deserialize(d)?),
            ("HD", GameMode::Mania) => GameMod::HiddenMania(Deserialize::deserialize(d)?),
            ("FL", GameMode::Mania) => GameMod::FlashlightMania(Deserialize::deserialize(d)?),
            ("AC", GameMode::Mania) => {
                GameMod::AccuracyChallengeMania(Deserialize::deserialize(d)?)
            }
            ("4K", GameMode::Mania) => GameMod::FourKeysMania(Deserialize::deserialize(d)?),
            ("5K", GameMode::Mania) => GameMod::FiveKeysMania(Deserialize::deserialize(d)?),
            ("6K", GameMode::Mania) => GameMod::SixKeysMania(Deserialize::deserialize(d)?),
            ("7K", GameMode::Mania) => GameMod::SevenKeysMania(Deserialize::deserialize(d)?),
            ("8K", GameMode::Mania) => GameMod::EightKeysMania(Deserialize::deserialize(d)?),
            ("9K", GameMode::Mania) => GameMod::NineKeysMania(Deserialize::deserialize(d)?),
            ("10K", GameMode::Mania) => GameMod::TenKeysMania(Deserialize::deserialize(d)?),
            ("1K", GameMode::Mania) => GameMod::OneKeyMania(Deserialize::deserialize(d)?),
            ("2K", GameMode::Mania) => GameMod::TwoKeysMania(Deserialize::deserialize(d)?),
            ("3K", GameMode::Mania) => GameMod::ThreeKeysMania(Deserialize::deserialize(d)?),
            ("RD", GameMode::Mania) => GameMod::RandomMania(Deserialize::deserialize(d)?),
            ("DS", GameMode::Mania) => GameMod::DualStagesMania(Deserialize::deserialize(d)?),
            ("MR", GameMode::Mania) => GameMod::MirrorMania(Deserialize::deserialize(d)?),
            ("DA", GameMode::Mania) => GameMod::DifficultyAdjustMania(Deserialize::deserialize(d)?),
            ("CL", GameMode::Mania) => GameMod::ClassicMania(Deserialize::deserialize(d)?),
            ("IN", GameMode::Mania) => GameMod::InvertMania(Deserialize::deserialize(d)?),
            ("CS", GameMode::Mania) => GameMod::ConstantSpeedMania(Deserialize::deserialize(d)?),
            ("HO", GameMode::Mania) => GameMod::HoldOffMania(Deserialize::deserialize(d)?),
            ("AT", GameMode::Mania) => GameMod::AutoplayMania(Deserialize::deserialize(d)?),
            ("CN", GameMode::Mania) => GameMod::CinemaMania(Deserialize::deserialize(d)?),
            ("WU", GameMode::Mania) => GameMod::WindUpMania(Deserialize::deserialize(d)?),
            ("WD", GameMode::Mania) => GameMod::WindDownMania(Deserialize::deserialize(d)?),
            ("MU", GameMode::Mania) => GameMod::MutedMania(Deserialize::deserialize(d)?),
            ("AS", GameMode::Mania) => GameMod::AdaptiveSpeedMania(Deserialize::deserialize(d)?),
            ("V2", GameMode::Mania) => GameMod::ScoreV2Mania(Deserialize::deserialize(d)?),
            _ => {
                return Err(DeError::custom(format!(
                    "unknown acronym {} for mode {:?}",
                    self.acronym, self.mode
                )))
            }
        };
        Ok(res)
    }
}
impl<'de> Visitor<'de> for ModeAsSeed<GameMod> {
    type Value = GameMod;
    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a GameMod")
    }
    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        GameMod::new(v, self.mode).ok_or_else(|| {
            DeError::custom(format!("invalid acronym `{v}` for mode {:?}", self.mode))
        })
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        // Using RawValue avoids an allocation since serde_json generally
        // deserializes into String to handle escaped characters.
        let key = map.next_key::<&RawValue>()?.map(RawValue::get);
        let Some(r#""acronym""#) = key else {return Err(DeError::custom("expected `acronym` as first field"));};
        let acronym: &'de str = map.next_value()?;
        let mut gamemod = None;
        while let Some(key) = map.next_key::<&str>()? {
            if key == "settings" {
                gamemod = Some(map.next_value_seed(GameModSettings {
                    acronym,
                    mode: self.mode,
                })?);
            } else {
                let _: IgnoredAny = map.next_value()?;
            }
        }
        gamemod
            .or_else(|| GameMod::new(acronym, self.mode))
            .ok_or_else(|| DeError::missing_field("settings"))
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for GameMod {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut s = s.serialize_map(None)?;
        s.serialize_entry("acronym", self.acronym().as_str())?;
        match self {
            Self::EasyOsu(m) => {
                let has_some = m.retries.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::HalfTimeOsu(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DaycoreOsu(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::SuddenDeathOsu(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::PerfectOsu(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DoubleTimeOsu(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::NightcoreOsu(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::HiddenOsu(m) => {
                let has_some = m.only_fade_approach_circles.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::FlashlightOsu(m) => {
                let has_some = m.follow_delay.is_some()
                    || m.size_multiplier.is_some()
                    || m.combo_based_size.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AccuracyChallengeOsu(m) => {
                let has_some = m.minimum_accuracy.is_some() || m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::TargetPracticeOsu(m) => {
                let has_some = m.seed.is_some() || m.metronome.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DifficultyAdjustOsu(m) => {
                let has_some = m.circle_size.is_some()
                    || m.approach_rate.is_some()
                    || m.drain_rate.is_some()
                    || m.overall_difficulty.is_some()
                    || m.extended_limits.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::ClassicOsu(m) => {
                let has_some = m.no_slider_head_accuracy.is_some()
                    || m.no_slider_head_movement.is_some()
                    || m.classic_note_lock.is_some()
                    || m.always_play_tail_sample.is_some()
                    || m.fade_hit_circle_early.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::RandomOsu(m) => {
                let has_some = m.angle_sharpness.is_some() || m.seed.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::MirrorOsu(m) => {
                let has_some = m.reflection.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WiggleOsu(m) => {
                let has_some = m.strength.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::GrowOsu(m) => {
                let has_some = m.start_scale.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DeflateOsu(m) => {
                let has_some = m.start_scale.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindUpOsu(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindDownOsu(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::BarrelRollOsu(m) => {
                let has_some = m.spin_speed.is_some() || m.direction.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::ApproachDifferentOsu(m) => {
                let has_some = m.scale.is_some() || m.style.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::MutedOsu(m) => {
                let has_some = m.inverse_muting.is_some()
                    || m.enable_metronome.is_some()
                    || m.mute_combo_count.is_some()
                    || m.affects_hit_sounds.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::NoScopeOsu(m) => {
                let has_some = m.hidden_combo_count.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::MagnetisedOsu(m) => {
                let has_some = m.attraction_strength.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::RepelOsu(m) => {
                let has_some = m.repulsion_strength.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AdaptiveSpeedOsu(m) => {
                let has_some = m.initial_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::HalfTimeTaiko(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DaycoreTaiko(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::SuddenDeathTaiko(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::PerfectTaiko(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DoubleTimeTaiko(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::NightcoreTaiko(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::FlashlightTaiko(m) => {
                let has_some = m.size_multiplier.is_some() || m.combo_based_size.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AccuracyChallengeTaiko(m) => {
                let has_some = m.minimum_accuracy.is_some() || m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::RandomTaiko(m) => {
                let has_some = m.seed.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DifficultyAdjustTaiko(m) => {
                let has_some = m.scroll_speed.is_some()
                    || m.drain_rate.is_some()
                    || m.overall_difficulty.is_some()
                    || m.extended_limits.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindUpTaiko(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindDownTaiko(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::MutedTaiko(m) => {
                let has_some = m.inverse_muting.is_some()
                    || m.enable_metronome.is_some()
                    || m.mute_combo_count.is_some()
                    || m.affects_hit_sounds.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AdaptiveSpeedTaiko(m) => {
                let has_some = m.initial_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::EasyCatch(m) => {
                let has_some = m.retries.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::HalfTimeCatch(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DaycoreCatch(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::SuddenDeathCatch(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::PerfectCatch(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DoubleTimeCatch(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::NightcoreCatch(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::FlashlightCatch(m) => {
                let has_some = m.size_multiplier.is_some() || m.combo_based_size.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AccuracyChallengeCatch(m) => {
                let has_some = m.minimum_accuracy.is_some() || m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DifficultyAdjustCatch(m) => {
                let has_some = m.circle_size.is_some()
                    || m.approach_rate.is_some()
                    || m.hard_rock_offsets.is_some()
                    || m.drain_rate.is_some()
                    || m.overall_difficulty.is_some()
                    || m.extended_limits.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindUpCatch(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindDownCatch(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::MutedCatch(m) => {
                let has_some = m.inverse_muting.is_some()
                    || m.enable_metronome.is_some()
                    || m.mute_combo_count.is_some()
                    || m.affects_hit_sounds.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::NoScopeCatch(m) => {
                let has_some = m.hidden_combo_count.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::EasyMania(m) => {
                let has_some = m.retries.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::HalfTimeMania(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DaycoreMania(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::SuddenDeathMania(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::PerfectMania(m) => {
                let has_some = m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DoubleTimeMania(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::NightcoreMania(m) => {
                let has_some = m.speed_change.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::FadeInMania(m) => {
                let has_some = m.coverage.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::HiddenMania(m) => {
                let has_some = m.coverage.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::FlashlightMania(m) => {
                let has_some = m.size_multiplier.is_some() || m.combo_based_size.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AccuracyChallengeMania(m) => {
                let has_some = m.minimum_accuracy.is_some() || m.restart.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::RandomMania(m) => {
                let has_some = m.seed.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::DifficultyAdjustMania(m) => {
                let has_some = m.drain_rate.is_some()
                    || m.overall_difficulty.is_some()
                    || m.extended_limits.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindUpMania(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::WindDownMania(m) => {
                let has_some =
                    m.initial_rate.is_some() || m.final_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::MutedMania(m) => {
                let has_some = m.inverse_muting.is_some()
                    || m.enable_metronome.is_some()
                    || m.mute_combo_count.is_some()
                    || m.affects_hit_sounds.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            Self::AdaptiveSpeedMania(m) => {
                let has_some = m.initial_rate.is_some() || m.adjust_pitch.is_some();
                if has_some {
                    s.serialize_entry("settings", m)?;
                }
            }
            _ => {}
        }
        s.end()
    }
}
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! mods_inner {
    ( [ $( $mode:ident )? ] 10K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* TenKeys )
    };
    ( [ $( $mode:ident )? ] 1K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* OneKey )
    };
    ( [ $( $mode:ident )? ] 2K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* TwoKeys )
    };
    ( [ $( $mode:ident )? ] 3K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* ThreeKeys )
    };
    ( [ $( $mode:ident )? ] 4K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* FourKeys )
    };
    ( [ $( $mode:ident )? ] 5K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* FiveKeys )
    };
    ( [ $( $mode:ident )? ] 6K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* SixKeys )
    };
    ( [ $( $mode:ident )? ] 7K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* SevenKeys )
    };
    ( [ $( $mode:ident )? ] 8K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* EightKeys )
    };
    ( [ $( $mode:ident )? ] 9K $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* NineKeys )
    };
    ( [ $( $mode:ident )? ] AC $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* AccuracyChallenge )
    };
    ( [ $( $mode:ident )? ] AD $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* ApproachDifferent )
    };
    ( [ $( $mode:ident )? ] AL $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Alternate )
    };
    ( [ $( $mode:ident )? ] AP $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Autopilot )
    };
    ( [ $( $mode:ident )? ] AS $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* AdaptiveSpeed )
    };
    ( [ $( $mode:ident )? ] AT $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Autoplay )
    };
    ( [ $( $mode:ident )? ] BL $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Blinds )
    };
    ( [ $( $mode:ident )? ] BR $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* BarrelRoll )
    };
    ( [ $( $mode:ident )? ] CL $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Classic )
    };
    ( [ $( $mode:ident )? ] CN $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Cinema )
    };
    ( [ $( $mode:ident )? ] CS $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* ConstantSpeed )
    };
    ( [ $( $mode:ident )? ] DA $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* DifficultyAdjust )
    };
    ( [ $( $mode:ident )? ] DC $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Daycore )
    };
    ( [ $( $mode:ident )? ] DF $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Deflate )
    };
    ( [ $( $mode:ident )? ] DS $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* DualStages )
    };
    ( [ $( $mode:ident )? ] DT $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* DoubleTime )
    };
    ( [ $( $mode:ident )? ] EZ $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Easy )
    };
    ( [ $( $mode:ident )? ] FF $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* FloatingFruits )
    };
    ( [ $( $mode:ident )? ] FI $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* FadeIn )
    };
    ( [ $( $mode:ident )? ] FL $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Flashlight )
    };
    ( [ $( $mode:ident )? ] FR $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* FreezeFrame )
    };
    ( [ $( $mode:ident )? ] GR $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Grow )
    };
    ( [ $( $mode:ident )? ] HD $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Hidden )
    };
    ( [ $( $mode:ident )? ] HO $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* HoldOff )
    };
    ( [ $( $mode:ident )? ] HR $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* HardRock )
    };
    ( [ $( $mode:ident )? ] HT $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* HalfTime )
    };
    ( [ $( $mode:ident )? ] IN $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Invert )
    };
    ( [ $( $mode:ident )? ] MG $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Magnetised )
    };
    ( [ $( $mode:ident )? ] MR $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Mirror )
    };
    ( [ $( $mode:ident )? ] MU $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Muted )
    };
    ( [ $( $mode:ident )? ] NC $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Nightcore )
    };
    ( [ $( $mode:ident )? ] NF $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* NoFail )
    };
    ( [ $( $mode:ident )? ] NS $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* NoScope )
    };
    ( [ $( $mode:ident )? ] PF $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Perfect )
    };
    ( [ $( $mode:ident )? ] RD $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Random )
    };
    ( [ $( $mode:ident )? ] RP $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Repel )
    };
    ( [ $( $mode:ident )? ] RX $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Relax )
    };
    ( [ $( $mode:ident )? ] SD $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* SuddenDeath )
    };
    ( [ $( $mode:ident )? ] SG $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* SingleTap )
    };
    ( [ $( $mode:ident )? ] SI $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* SpinIn )
    };
    ( [ $( $mode:ident )? ] SO $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* SpunOut )
    };
    ( [ $( $mode:ident )? ] ST $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* StrictTracking )
    };
    ( [ $( $mode:ident )? ] SW $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Swap )
    };
    ( [ $( $mode:ident )? ] TC $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Traceable )
    };
    ( [ $( $mode:ident )? ] TD $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* TouchDevice )
    };
    ( [ $( $mode:ident )? ] TP $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* TargetPractice )
    };
    ( [ $( $mode:ident )? ] TR $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Transform )
    };
    ( [ $( $mode:ident )? ] V2 $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* ScoreV2 )
    };
    ( [ $( $mode:ident )? ] WD $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* WindDown )
    };
    ( [ $( $mode:ident )? ] WG $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* Wiggle )
    };
    ( [ $( $mode:ident )? ] WU $( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* WindUp )
    };
    ( [ $mode:ident ] ) => {{
        let _ = $crate::model::GameMode::$mode;
        $crate::model::mods::GameMods::new()
    }};
    ( [ $mode:ident ] $( $name:ident )* ) => {
        paste::paste! {{
            #[allow(unused_mut)]
            let mut mods = $crate::model::mods::GameMods::new();
            $( mods.insert($crate::model::mods::GameMod::[<$name $mode>](Default::default())); )*
            mods
        }}
    };
    ( [] $( $name:ident )* ) => {{
        #[allow(unused_mut)]
        let mut mods = $crate::model::mods::GameModsIntermode::new();
        $( mods.insert($crate::model::mods::GameModIntermode::$name); )*
        mods
    }};
}
