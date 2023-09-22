use std::{
    borrow::Cow,
    fmt::{Debug, Formatter, Result as FmtResult},
};

use itoa::Buffer;
use serde::{
    de::{Error as DeError, Visitor},
    Deserialize, Deserializer,
};

use crate::{error::GenResult, writer::Writer};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ruleset {
    Osu,
    Taiko,
    #[serde(rename = "fruits")]
    Catch,
    Mania,
}

impl Ruleset {
    pub fn as_capitalized_str(self) -> &'static str {
        match self {
            Ruleset::Osu => "Osu",
            Ruleset::Taiko => "Taiko",
            Ruleset::Catch => "Catch",
            Ruleset::Mania => "Mania",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RulesetMods {
    pub name: Ruleset,
    pub mods: Box<[GameMod]>,
}

impl RulesetMods {
    pub fn process(rulesets: &mut [Self]) {
        for ruleset in rulesets.iter_mut() {
            ruleset.process_mod_names();

            // make sure no gamemod excludes itself explicitly
            for gamemod in ruleset.mods.iter_mut() {
                gamemod
                    .incompatible_mods
                    .retain(|incompatible| incompatible != &gamemod.acronym);
            }
        }
    }

    /// Removes whitespace and adds the mode as suffix
    fn process_mod_names(&mut self) {
        for gamemod in self.mods.iter_mut() {
            let match_indices = gamemod.name.match_indices(' ');

            let mut res = Cow::default();
            let mut last_start = 0;

            unsafe {
                for (index, matched) in match_indices {
                    res += gamemod.name.get_unchecked(last_start..index);

                    last_start = index + matched.len();
                }

                res += gamemod.name.get_unchecked(last_start..);
            }

            let mut name = res.into_owned();
            name.push_str(self.name.as_capitalized_str());
            gamemod.name = name.into_boxed_str();
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Acronym([u8; 3]);

impl Acronym {
    fn from_str(s: &str) -> Option<Self> {
        match <[u8; 2]>::try_from(s.as_bytes()) {
            Ok([a, b]) => Some(Self([0, a, b])),
            Err(_) => s.as_bytes().try_into().map(Self).ok(),
        }
    }

    pub fn as_str(&self) -> &str {
        let start_idx = (self.0[0] == 0) as usize;

        unsafe { std::str::from_utf8_unchecked(&self.0[start_idx..]) }
    }

    pub fn write(self, writer: &mut Writer) -> GenResult {
        writer.write("Acronym::from_str_unchecked(\"")?;
        writer.write(self.as_str())?;

        writer.write("\")")
    }
}

impl Debug for Acronym {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Acronym {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct AcronymVisitor;

        impl<'de> Visitor<'de> for AcronymVisitor {
            type Value = Acronym;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("an Acronym")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                Acronym::from_str(v)
                    .ok_or_else(|| DeError::custom(format!("invalid length for acronym `{v}`")))
            }
        }

        d.deserialize_str(AcronymVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameMod {
    pub acronym: Acronym,
    pub name: Box<str>,
    pub description: Box<str>,
    #[serde(rename = "Type")]
    pub kind: Box<str>,
    pub settings: Box<[Setting]>,
    pub incompatible_mods: Vec<Acronym>,
}

impl GameMod {
    pub fn bits(&self) -> Option<u32> {
        match self.acronym.as_str() {
            "NM" => Some(0),
            "NF" => Some(1 << 0),
            "EZ" => Some(1 << 1),
            "TD" => Some(1 << 2),
            "HD" => Some(1 << 3),
            "HR" => Some(1 << 4),
            "SD" => Some(1 << 5),
            "DT" => Some(1 << 6),
            "RX" => Some(1 << 7),
            "HT" => Some(1 << 8),
            "NC" => Some((1 << 6) + (1 << 9)),
            "FL" => Some(1 << 10),
            "AT" => Some(1 << 11),
            "SO" => Some(1 << 12),
            "AP" => Some(1 << 13),
            "PF" => Some((1 << 5) + (1 << 14)),
            "4K" => Some(1 << 15),
            "5K" => Some(1 << 16),
            "6K" => Some(1 << 17),
            "7K" => Some(1 << 18),
            "8K" => Some(1 << 19),
            "FI" => Some(1 << 20),
            "RD" => Some(1 << 21),
            "CN" => Some(1 << 22),
            "TP" => Some(1 << 23),
            "9K" => Some(1 << 24),
            "DS" => Some(1 << 25),
            "1K" => Some(1 << 26),
            "3K" => Some(1 << 27),
            "2K" => Some(1 << 28),
            "SV2" => Some(1 << 29),
            "MR" => Some(1 << 30),
            _ => None,
        }
    }

    pub fn discriminant(&self) -> Option<u8> {
        self.bits().map(|n| (n.ilog2() as u8) + 1)
    }

    pub fn write(&self, writer: &mut Writer, itoa_buf: &mut Buffer) -> GenResult {
        self.define_struct(writer)?;
        self.define_fns(writer, itoa_buf)?;
        self.impl_serde(writer)?;

        Ok(())
    }

    fn define_struct(&self, writer: &mut Writer) -> GenResult {
        writer.write("/// ")?;
        writer.write(&self.description)?;
        writer.write(
            "\n\
            #[derive(",
        )?;

        if self.settings.is_empty() {
            writer.write("Copy, Eq, ")?;
        }

        writer.write(
            "Clone, Debug, Default, PartialEq)]\
            #[cfg_attr(feature = \"rkyv\", derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)")?;

        if self.settings.is_empty() {
            writer.write(", archive(as = \"Self\")")?;
        }

        writer.write(
            "\
            )]\
            pub struct ",
        )?;
        writer.write(&self.name)?;
        writer.write(b'{')?;

        for setting in self.settings.iter() {
            setting.write(writer)?;
        }

        writer.write(b'}')
    }

    fn define_fns(&self, writer: &mut Writer, itoa_buf: &mut Buffer) -> GenResult {
        writer.write("impl ")?;
        writer.write(&self.name)?;
        writer.write(b'{')?;

        self.define_fn_acronym(writer)?;
        self.define_fn_incompatible_mods(writer)?;
        self.define_fn_description(writer)?;
        self.define_fn_kind(writer)?;
        self.define_fn_bits(writer, itoa_buf)?;

        writer.write(b'}')
    }

    fn define_fn_acronym(&self, writer: &mut Writer) -> GenResult {
        writer.write("/// The acronym of [`")?;
        writer.write(&self.name)?;
        writer.write(
            "`]\n\
            pub const fn acronym() -> Acronym {\
                unsafe {",
        )?;

        self.acronym.write(writer)?;

        writer.write(
            "\
                }\
            }",
        )
    }

    fn define_fn_incompatible_mods(&self, writer: &mut Writer) -> GenResult {
        writer.write("/// Iterator of [`Acronym`] for mods that are incompatible with [`")?;
        writer.write(&self.name)?;
        writer.write(
            "`]\n\
            pub fn incompatible_mods() -> impl Iterator<Item = Acronym> {",
        )?;

        if self.incompatible_mods.is_empty() {
            writer.write("[]")?;
        } else {
            writer.write("unsafe { [")?;

            for incompatible in self.incompatible_mods.iter() {
                incompatible.write(writer)?;
                writer.write(b',')?;
            }

            writer.write("] }")?;
        }

        writer.write(
            "\
                .into_iter()\
            }",
        )
    }

    fn define_fn_description(&self, writer: &mut Writer) -> GenResult {
        writer.write("/// The description of [`")?;
        writer.write(&self.name)?;
        writer.write(
            "`]\n\
            pub const fn description() -> &'static str {\
                \"",
        )?;
        writer.write(&self.description)?;

        writer.write("\"}")
    }

    fn define_fn_kind(&self, writer: &mut Writer) -> GenResult {
        writer.write("/// The [`GameModKind`] of [`")?;
        writer.write(&self.name)?;
        writer.write(
            "`]\n\
            pub const fn kind() -> GameModKind {\
                GameModKind::",
        )?;
        writer.write(&self.kind)?;

        writer.write(b'}')
    }

    fn define_fn_bits(&self, writer: &mut Writer, itoa_buf: &mut Buffer) -> GenResult {
        let Some(bits) = self.bits() else {
            return Ok(());
        };
        writer.write("/// Bit value of [`")?;
        writer.write(&self.name)?;

        writer.write(
            "`]\n\
            ///\n\
            /// See <https://github.com/ppy/osu-api/wiki#mods>\n\
            pub const fn bits() -> u32 {",
        )?;

        writer.write(itoa_buf.format(bits))?;

        writer.write(b'}')
    }

    fn impl_serde(&self, writer: &mut Writer) -> GenResult {
        writer.write("impl<'de> Deserialize<'de> for ")?;
        writer.write(&self.name)?;
        writer.write(
            " {\
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\
                struct ",
        )?;
        writer.write(&self.name)?;
        writer.write(
            "Visitor;\
                impl<'de> Visitor<'de> for ",
        )?;
        writer.write(&self.name)?;
        writer.write(
            "Visitor {\
                    type Value = ",
        )?;
        writer.write(&self.name)?;
        writer.write(
            ";\
                    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {\
                        f.write_str(\"",
        )?;
        writer.write(&self.name)?;
        writer.write(
            "\")\
                    }\
                    fn visit_map<A: MapAccess<'de>>(self, ",
        )?;

        if self.settings.is_empty() {
            writer.write(b'_')?;
        } else {
            writer.write("mut map")?;
        }

        writer.write(": A) -> Result<Self::Value, A::Error> {")?;

        if !self.settings.is_empty() {
            for setting in self.settings.iter() {
                writer.write("let mut ")?;
                writer.write(&setting.name)?;
                writer.write(" = None;")?;
            }

            writer.write(
                "\
                        while let Some(key) = map.next_key()? {\
                            match key {",
            )?;

            for setting in self.settings.iter() {
                writer.write(b'"')?;
                writer.write(&setting.name)?;
                writer.write("\" => ")?;
                writer.write(&setting.name)?;
                writer.write(" = Some(map.next_value()?),")?;
            }

            writer.write(
                "\
                                _ => {\
                                    let _: IgnoredAny = map.next_value()?;\
                                }\
                            }\
                        }\
                        Ok(Self::Value {",
            )?;

            for setting in self.settings.iter() {
                writer.write(&setting.name)?;
                writer.write(b':')?;
                writer.write(&setting.name)?;
                writer.write(".unwrap_or_default(),")?;
            }
        } else {
            writer.write("Ok(Self::Value {")?;
        }

        writer.write(
            "\
                        })\
                    }\
                }\
                d.deserialize_map(",
        )?;
        writer.write(&self.name)?;
        writer.write(
            "\
                Visitor)\
            }\
        }",
        )?;

        writer.write(
            "#[cfg(feature = \"serialize\")]\
            impl serde::Serialize for ",
        )?;
        writer.write(&self.name)?;
        writer.write(
            " {\
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                use serde::ser::SerializeMap;\
                let field_count = ",
        )?;

        let mut settings = self.settings.iter();

        if let Some(setting) = settings.next() {
            writer.write("self.")?;
            writer.write(&setting.name)?;
            writer.write(".is_some() as usize")?;

            for setting in settings {
                writer.write(" + self.")?;
                writer.write(&setting.name)?;
                writer.write(".is_some() as usize")?;
            }
        } else {
            writer.write(b'0')?;
        }

        writer.write(
            ";\
                let ",
        )?;

        if !self.settings.is_empty() {
            writer.write("mut ")?;
        }

        writer.write("map = s.serialize_map(Some(field_count))?;")?;

        for setting in self.settings.iter() {
            writer.write(
                "\
                if let Some(ref x) = self.",
            )?;
            writer.write(&setting.name)?;
            writer.write(
                " {\
                    map.serialize_entry(\"",
            )?;
            writer.write(&setting.name)?;
            writer.write(
                "\
                    \", x)?;\
                }",
            )?;
        }

        writer.write(
            "\
                map.end()\
            }\
        }",
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    #[serde(rename = "Name")]
    pub name: Box<str>,
    #[serde(rename = "Type")]
    kind: SettingType,
}

impl Setting {
    pub fn write(&self, writer: &mut Writer) -> GenResult {
        writer.write("pub ")?;
        writer.write(&self.name)?;
        writer.write(": Option<")?;
        self.kind.write(writer)?;

        writer.write(">,")
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
enum SettingType {
    #[serde(rename = "boolean")]
    Bool,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
}

impl SettingType {
    pub fn write(self, writer: &mut Writer) -> GenResult {
        match self {
            SettingType::Bool => writer.write("bool"),
            SettingType::Number => writer.write("f32"),
            SettingType::String => writer.write("String"),
        }
    }
}
