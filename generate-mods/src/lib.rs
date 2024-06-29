use std::collections::HashMap;

use itoa::Buffer;

pub use self::{error::GenResult, model::RulesetMods, writer::Writer};

mod error;
mod model;
mod writer;

pub fn specify_preamble(writer: &mut Writer, url: &str, disclaimer: &str) -> GenResult {
    writer.write(disclaimer)?;
    writer.write(
        "\n\
        //!\n\
        //! See <",
    )?;
    writer.write(url)?;

    writer.write(
        ">\n\n\
        use std::{\
            borrow::Borrow,\
            cmp::Ordering,\
            num::NonZeroU8,\
            fmt::{Display, Formatter, Result as FmtResult},\
        };\n\n",
    )?;
    writer.write(
        "use serde::{\
            Deserialize,\
            de::{value::MapAccessDeserializer, Deserializer, DeserializeSeed, Error as DeError, IgnoredAny, MapAccess, Visitor}\
        };",
    )?;
    writer.write("use serde_json::value::RawValue;")?;
    writer.write(
        "\nuse crate::model::{\
            mods::{Acronym, ModeAsSeed},\
            GameMode,\
        };",
    )?;

    Ok(())
}

pub fn define_gamemod_structs(
    rulesets: &[RulesetMods],
    writer: &mut Writer,
    itoa_buf: &mut Buffer,
) -> GenResult {
    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            gamemod.write(writer, itoa_buf)?;
        }
    }

    write_unknown_mod(writer)?;

    Ok(())
}

pub fn define_gamemod_kind(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    // hard-coded to simplify Ord implementation
    let expected = [
        "DifficultyReduction",
        "DifficultyIncrease",
        "Conversion",
        "Automation",
        "Fun",
        "System",
    ];

    for ruleset in rulesets.iter() {
        for gamemod in ruleset.mods.iter() {
            if !expected.contains(&&*gamemod.kind) {
                panic!("unexpected GameModKind `{}`", gamemod.kind);
            }
        }
    }

    writer.write(
        "/// The different types of a [`GameMod`]\n\
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]\
        #[cfg_attr(feature = \"serialize\", derive(serde::Serialize))]\
        pub enum GameModKind {\
            DifficultyReduction,\
            DifficultyIncrease,\
            Conversion,\
            Automation,\
            Fun,\
            System,\
        }",
    )
}

pub fn define_gamemod_intermode(
    rulesets: &[RulesetMods],
    writer: &mut Writer,
    itoa_buf: &mut Buffer,
) -> GenResult {
    let mut mods = rulesets
        .iter()
        .flat_map(|ruleset| {
            let suffix_len = ruleset.name.as_capitalized_str().len();

            ruleset.mods.iter().map(move |gamemod| {
                let name = &gamemod.name[..gamemod.name.len() - suffix_len];
                let bits = gamemod.bits();
                let kind = gamemod.kind.as_ref();

                (name, (bits, gamemod.acronym, kind))
            })
        })
        .collect::<HashMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();

    mods.sort_unstable_by(|(a, ..), (b, ..)| a.cmp(b));

    writer.write(
        "/// The kind of a [`GameMod`] when the mode is ignored\n\
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]\
        #[non_exhaustive]\
        pub enum GameModIntermode {",
    )?;

    for (name, _) in mods.iter() {
        writer.write(*name)?;
        writer.write(b',')?;
    }

    writer.write("Unknown(UnknownMod),")?;
    writer.write(b'}')?;

    writer.write(
        "impl GameModIntermode {\
            /// The [`Acronym`] of this [`GameModIntermode`]\n\
            pub const fn acronym(&self) -> Acronym {\
                unsafe { match self {",
    )?;

    for (name, (_, acronym, _)) in mods.iter() {
        writer.write("Self::")?;
        writer.write(*name)?;
        writer.write(" => ")?;
        acronym.write(writer)?;
        writer.write(b',')?;
    }

    writer.write(
        "\
                        Self::Unknown(m) => m.acronym(),\
                    }\
                }\
            }\
            /// Bit value of the [`GameModIntermode`]\n\
            ///\n\
            /// See <https://github.com/ppy/osu-api/wiki#mods>\n\
            pub const fn bits(self) -> Option<u32> {\
                match self {",
    )?;

    for (name, (bits, ..)) in mods.iter() {
        writer.write("Self::")?;
        writer.write(*name)?;
        writer.write(" => ")?;

        match bits {
            Some(bits) => {
                writer.write("Some(")?;
                writer.write(itoa_buf.format(*bits))?;
                writer.write(b')')?;
            }
            None => writer.write("None")?,
        }

        writer.write(b',')?;
    }

    writer.write(
        "\
                    Self::Unknown(_) => None,\
                }\
            }\
            /// The [`GameModKind`] of this [`GameModIntermode`]\n\
            pub const fn kind(&self) -> GameModKind {\
                match self {",
    )?;

    for (name, (.., kind)) in mods.iter() {
        writer.write("Self::")?;
        writer.write(*name)?;
        writer.write(" => GameModKind::")?;
        writer.write(*kind)?;
        writer.write(b',')?;
    }

    writer.write(
        "\
                    Self::Unknown(_) => GameModKind::System,\
                }\
            }\
            /// Parse an [`Acronym`] into a [`GameModIntermode`]\n\
            pub fn from_acronym(acronym: Acronym) -> Self {\
                match acronym.as_str() {",
    )?;

    for (name, (_, acronym, _)) in mods.iter() {
        writer.write(b'"')?;
        writer.write(acronym.as_str())?;
        writer.write("\" => Self::")?;
        writer.write(*name)?;
        writer.write(b',')?;
    }

    writer.write(
        "\
                    _ => Self::Unknown(UnknownMod { acronym }),\
                }\
            }\
        }",
    )?;

    writer.write(
        "impl PartialOrd for GameModIntermode {\
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\
                Some(self.cmp(other))\
            }\
        }\
        impl Ord for GameModIntermode {\
            fn cmp(&self, other: &Self) -> Ordering {\
                match (self.bits(), other.bits()) {\
                    (Some(self_bits), Some(other_bits)) => self_bits.cmp(&other_bits),\
                    (Some(_), None) => Ordering::Less,\
                    (None, Some(_)) => Ordering::Greater,\
                    (None, None) => self.acronym().as_str().cmp(other.acronym().as_str()),\
                }\
            }\
        }\
        impl Display for GameModIntermode {\
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {\
                f.write_str(self.acronym().as_str())\
            }\
        }\
        impl From<&GameModIntermode> for GameModIntermode {\
            fn from(gamemod: &GameModIntermode) -> Self {\
                *gamemod\
            }\
        }\
        impl From<GameMod> for GameModIntermode {\
            fn from(gamemod: GameMod) -> Self {\
                gamemod.intermode()\
            }\
        }\
        #[cfg(feature = \"serialize\")]\
        impl serde::Serialize for GameModIntermode {\
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                s.serialize_str(self.acronym().as_str())\
            }\
        }",
    )
}

pub fn define_gamemod_order(
    rulesets: &[RulesetMods],
    writer: &mut Writer,
    itoa_buf: &mut Buffer,
) -> GenResult {
    writer.write(
        "#[derive(Copy, Clone, PartialEq, Eq)]\
        pub(crate) struct GameModOrder {\
            mode: GameMode,\
            index: Option<NonZeroU8>,\
            intermode: GameModIntermode,\
        }\
        impl From<&GameMod> for GameModOrder {\
            fn from(gamemod: &GameMod) -> Self {\
                const fn inner(gamemod: &GameMod) -> GameModOrder {\
                    macro_rules! arm {\
                        ($mode:ident, $gamemod:ident, Some($discriminant:literal), $intermode:ident) => {\
                            arm!(\
                                $mode,\
                                $gamemod,\
                                Some(unsafe { NonZeroU8::new_unchecked($discriminant) }),\
                                $intermode,\
                            )\
                        };\
                        ($mode:ident, $gamemod:ident, $index:expr, $intermode:ident $(,)?) => {\
                            GameModOrder {\
                                mode: GameMode::$mode,\
                                index: $index,\
                                intermode: GameModIntermode::$intermode,\
                            }\
                        };\
                    }\
                    match gamemod {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            writer.write("GameMod::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => arm!(")?;
            writer.write(ruleset_str)?;
            writer.write(b',')?;
            writer.write(&gamemod.name)?;
            writer.write(b',')?;

            match gamemod.discriminant() {
                Some(discriminant) => {
                    writer.write("Some(")?;
                    writer.write(itoa_buf.format(discriminant))?;
                    writer.write(b')')?;
                }
                None => {
                    writer.write("None")?;
                }
            }

            let intermode = &gamemod.name[..gamemod.name.len() - ruleset_str.len()];
            writer.write(b',')?;
            writer.write(intermode)?;
            writer.write("),")?;
        }

        writer.write("GameMod::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write(
            "(m) => GameModOrder {\
                mode: GameMode::",
        )?;
        writer.write(ruleset_str)?;
        writer.write(
            ",\
                index: None,\
                intermode: GameModIntermode::Unknown(*m),\
            },",
        )?;
    }

    writer.write(
        "\
                    }\
                }\
                inner(gamemod)\
            }\
        }\
        impl PartialOrd for GameModOrder {\
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\
                Some(self.cmp(other))\
            }\
        }\
        impl Ord for GameModOrder {\
            fn cmp(&self, other: &Self) -> Ordering {\
                self.mode.cmp(&other.mode)\
                    .then_with(|| match (self.index, other.index) {\
                        (Some(self_idx), Some(other_idx)) => self_idx.cmp(&other_idx),\
                        (Some(_), None) => Ordering::Less,\
                        (None, Some(_)) => Ordering::Greater,\
                        (None, None) => self\
                            .intermode\
                            .acronym()\
                            .as_str()\
                            .cmp(other.intermode.acronym().as_str()),\
                    })\
            }\
        }\
        impl PartialEq<GameModIntermode> for GameModOrder {\
            fn eq(&self, other: &GameModIntermode) -> bool {\
                self.intermode.eq(other)\
            }\
        }\
        impl Borrow<GameModIntermode> for GameModOrder {\
            fn borrow(&self) -> &GameModIntermode {\
                &self.intermode\
            }\
        }",
    )
}

pub fn define_gamemod_enum(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// A single game mod\n\
        #[derive(Clone, Debug, PartialEq)]\
        #[non_exhaustive]\
        pub enum GameMod {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write(&gamemod.name)?;
            writer.write(b'(')?;
            writer.write(&gamemod.name)?;
            writer.write("),")?;
        }

        writer.write("Unknown")?;
        writer.write(ruleset.name.as_capitalized_str())?;
        writer.write("(UnknownMod),")?;
    }

    writer.write(b'}')
}

pub fn define_gamemod_fns(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write("impl GameMod {")?;

    define_gamemod_fn_new(rulesets, writer)?;
    define_gamemod_fn_acronym(rulesets, writer)?;
    define_gamemod_fn_incompatible_mods(rulesets, writer)?;
    define_gamemod_fn_description(rulesets, writer)?;
    define_gamemod_fn_kind(rulesets, writer)?;
    define_gamemod_fn_bits(rulesets, writer)?;
    define_gamemod_fn_mode(rulesets, writer)?;
    define_gamemod_fn_intermode(rulesets, writer)?;

    writer.write(b'}')
}

fn write_unknown_mod(writer: &mut Writer) -> GenResult {
    writer.write(
        "/// Any mod unknown to `rosu-v2`\n\
        #[derive(Copy, Eq, Clone, Debug, PartialEq, PartialOrd, Ord, Hash)]\
        pub struct UnknownMod {\
            pub acronym: Acronym,\
        }\
        impl UnknownMod {\
            /// The default [`Acronym`] for an unknown mod without specific\n\
            /// acronym.\n\
            pub const UNKNOWN_ACRONYM: Acronym = unsafe { Acronym::from_str_unchecked(\"??\") };\n\
            /// A custom [`Acronym`] for any unknown mod\n\
            pub const fn acronym(self) -> Acronym {\
                self.acronym\
            }\
            /// Returns an empty iterator\n\
            pub const fn incompatible_mods() -> std::iter::Empty<Acronym> {\
                std::iter::empty()\
            }\
            /// A custom description for any unknown mod\n\
            pub const fn description() -> &'static str {\
                \"Any mod unknown to the rosu-v2 crate\"\
            }\
            /// A manually assigned [`GameModKind`] for any unknown mod\n\
            pub const fn kind() -> GameModKind {\
                GameModKind::System\
            }\
        }\
        impl Default for UnknownMod {\
            fn default() -> Self {\
                Self {\
                    acronym: Self::UNKNOWN_ACRONYM,\
                }\
            }\
        }\
        impl<'de> Deserialize<'de> for UnknownMod {\
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\
                struct UnknownModVisitor;\
                impl<'de> Visitor<'de> for UnknownModVisitor {\
                    type Value = UnknownMod;\
                    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {\
                        f.write_str(\"any unknown mod\")\
                    }\
                    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {\
                        while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}\
                        Ok(UnknownMod { acronym: UnknownMod::UNKNOWN_ACRONYM })\
                    }\
                }\
                d.deserialize_map(UnknownModVisitor)\
            }\
        }\
        #[cfg(feature = \"serialize\")]\
        impl serde::Serialize for UnknownMod {\
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                s.serialize_map(Some(0)).and_then(serde::ser::SerializeMap::end)\
            }\
        }"
    )
}

fn define_gamemod_fn_new(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// Create a new [`GameMod`]\n\
        pub fn new(acronym: &str, mode: GameMode) -> Self {\
            match (acronym, mode) {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            writer.write("(\"")?;
            writer.write(gamemod.acronym.as_str())?;
            writer.write("\", GameMode::")?;
            writer.write(ruleset_str)?;
            writer.write(") => Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(Default::default()),")?;
        }
    }

    writer.write(
        "_ => {\
            let acronym = <Acronym as std::str::FromStr>::from_str(acronym)\
                .unwrap_or(UnknownMod::UNKNOWN_ACRONYM);\
            let unknown = UnknownMod { acronym };\
            match mode {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();
        writer.write("GameMode::")?;
        writer.write(ruleset_str)?;
        writer.write(" => GameMod::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write("(unknown),")?;
    }

    writer.write(
        "\
                    }\
                }\
            }\
        }",
    )
}

fn define_gamemod_fn_acronym(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The acronym of this [`GameMod`]\n\
        pub const fn acronym(&self) -> Acronym {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::acronym(),")?;
        }
    }

    let mut rulesets_iter = rulesets.iter();

    if let Some(ruleset) = rulesets_iter.next() {
        writer.write("Self::Unknown")?;
        writer.write(ruleset.name.as_capitalized_str())?;
        writer.write("(m)")?;

        for ruleset in rulesets_iter {
            writer.write(" | Self::Unknown")?;
            writer.write(ruleset.name.as_capitalized_str())?;
            writer.write("(m)")?;
        }

        writer.write(" => m.acronym(),")?;
    }

    writer.write(
        "\
            }\
        }",
    )
}

fn define_gamemod_fn_incompatible_mods(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// List of [`Acronym`] for mods that are incompatible with this [`GameMod`]\n\
        pub fn incompatible_mods(&self) -> Box<[Acronym]> {\
        match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::incompatible_mods().collect(),")?;
        }
    }

    writer.write("_ => UnknownMod::incompatible_mods().collect(),")?;

    writer.write("}}")
}

fn define_gamemod_fn_description(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The description of this [`GameMod`]\n\
        pub const fn description(&self) -> &'static str {\
        match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::description(),")?;
        }
    }

    writer.write("_ => UnknownMod::description(),")?;

    writer.write("}}")
}

fn define_gamemod_fn_kind(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The [`GameModKind`] of this [`GameMod`]\n\
        pub const fn kind(&self) -> GameModKind {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => ")?;
            writer.write(&gamemod.name)?;
            writer.write("::kind(),")?;
        }
    }

    writer.write("_ => UnknownMod::kind(),")?;

    writer.write(
        "\
            }\
        }",
    )
}

fn define_gamemod_fn_bits(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// Optional bit value of this [`GameMod`]\n\
        ///\n\
        /// See <https://github.com/ppy/osu-api/wiki#mods>\n\
        pub const fn bits(&self) -> Option<u32> {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            if gamemod.bits().is_some() {
                writer.write("Self::")?;
                writer.write(&gamemod.name)?;
                writer.write("(_) => Some(")?;
                writer.write(&gamemod.name)?;
                writer.write("::bits()),")?;
            }
        }
    }

    writer.write(
        "\
                _ => None,\
            }\
        }",
    )
}

fn define_gamemod_fn_intermode(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The kind of a [`GameMod`] when ignoring the mode\n\
        pub const fn intermode(&self) -> GameModIntermode {\
            match self {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            let intermode = &gamemod.name[..gamemod.name.len() - ruleset_str.len()];

            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) => GameModIntermode::")?;
            writer.write(intermode)?;
            writer.write(b',')?;
        }
    }

    let mut ruleset_iter = rulesets.iter();

    if let Some(ruleset) = ruleset_iter.next() {
        writer.write("Self::Unknown")?;
        writer.write(ruleset.name.as_capitalized_str())?;
        writer.write("(m)")?;

        for ruleset in ruleset_iter {
            writer.write(" | Self::Unknown")?;
            writer.write(ruleset.name.as_capitalized_str())?;
            writer.write("(m)")?;
        }

        writer.write(" => GameModIntermode::Unknown(*m),")?;
    }

    writer.write(
        "\
            }\
        }",
    )
}

fn define_gamemod_fn_mode(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "/// The [`GameMode`] of a [`GameMod`]\n\
        pub const fn mode(&self) -> GameMode {\
            match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write("(_) | ")?;
        }

        let ruleset_str = ruleset.name.as_capitalized_str();
        writer.write("Self::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write("(_) => GameMode::")?;
        writer.write(ruleset_str)?;
        writer.write(b',')?;
    }

    writer.write("}}")
}

pub fn impl_gamemod_traits(writer: &mut Writer) -> GenResult {
    writer.write(
        "impl PartialOrd for GameMod {\
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {\
                self\
                    .bits()\
                    .zip(other.bits())\
                    .map(|(self_bits, other_bits)| self_bits.cmp(&other_bits))\
            }\
        }",
    )
}

pub fn impl_serde(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    writer.write(
        "struct GameModSettings<'a> {\
            acronym: &'a str,\
            mode: GameMode,\
        }\
        impl<'de> DeserializeSeed<'de> for GameModSettings<'de> {\
            type Value = <Self as Visitor<'de>>::Value;\
            fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {\
                d.deserialize_any(self)\
            }\
        }",
    )?;

    writer.write(
        "impl<'de> Visitor<'de> for GameModSettings<'de> {\
            type Value = GameMod;\
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {\
                f.write_str(\"GameMod settings\")\
            }\
            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {\
                let d = MapAccessDeserializer::new(map);\
                let res = match (self.acronym, self.mode) {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();

        for gamemod in ruleset.mods.iter() {
            writer.write("(\"")?;
            writer.write(gamemod.acronym.as_str())?;
            writer.write("\", GameMode::")?;
            writer.write(ruleset_str)?;
            writer.write(") => GameMod::")?;
            writer.write(&gamemod.name)?;
            writer.write("(Deserialize::deserialize(d)?),")?;
        }
    }

    writer.write(
        "_ => {\
            let acronym = <Acronym as std::str::FromStr>::from_str(self.acronym).map_err(DeError::custom)?;\n\
            // All fields are specified already but we still want to clear\n\
            // out content from the deserializer.\n\
            #[allow(clippy::needless_update)]\
            let unknown = UnknownMod {\
                acronym,\
                ..Deserialize::deserialize(d)?\
            };\
            match self.mode {",
    )?;

    for ruleset in rulesets {
        let ruleset_str = ruleset.name.as_capitalized_str();
        writer.write("GameMode::")?;
        writer.write(ruleset_str)?;
        writer.write(" => GameMod::Unknown")?;
        writer.write(ruleset_str)?;
        writer.write("(unknown),")?;
    }

    writer.write(
        "\
                        }\
                    }\
                };\
                Ok(res)\
            }\
        }",
    )?;

    writer.write(
        "impl<'de> Visitor<'de> for ModeAsSeed<GameMod> {\
            type Value = GameMod;\
            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {\
                f.write_str(\"a GameMod\")\
            }\
            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {\
                Ok(GameMod::new(v, self.mode))\
            }\
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {\
                // Using RawValue avoids an allocation since serde_json generally\n\
                // deserializes into String to handle escaped characters.\n\
                let key = map.next_key::<&RawValue>()?.map(RawValue::get);\
                let Some(r#\"\"acronym\"\"#) = key else {\
                    return Err(DeError::custom(\"expected `acronym` as first field\"));\
                };\
                let acronym: &'de str = map.next_value()?;\
                let mut gamemod = None;\
                while let Some(key) = map.next_key::<&str>()? {\
                    if key == \"settings\" {\
                        gamemod = Some(map.next_value_seed(GameModSettings { acronym, mode: self.mode })?);\
                    } else {\
                        let _: IgnoredAny = map.next_value()?;\
                    }\
                }\
                Ok(gamemod.unwrap_or_else(|| GameMod::new(acronym, self.mode)))\
            }\
        }",
    )?;

    writer.write(
        "#[cfg(feature = \"serialize\")]\
        impl serde::Serialize for GameMod {\
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {\
                use serde::ser::SerializeMap;\
                let mut s = s.serialize_map(None)?;\
                s.serialize_entry(\"acronym\", self.acronym().as_str())?;\
                match self {",
    )?;

    for ruleset in rulesets {
        for gamemod in ruleset.mods.iter() {
            if gamemod.settings.is_empty() {
                continue;
            }

            writer.write("Self::")?;
            writer.write(&gamemod.name)?;
            writer.write(
                "\
                    (m) => {\
                        let has_some = ",
            )?;

            let mut settings = gamemod.settings.iter();

            if let Some(setting) = settings.next() {
                writer.write("m.")?;
                writer.write(&setting.name)?;
                writer.write(".is_some()")?;

                for setting in settings {
                    writer.write(" || m.")?;
                    writer.write(&setting.name)?;
                    writer.write(".is_some()")?;
                }
            }

            writer.write(
                "\
                        ;\
                        if has_some {\
                            s.serialize_entry(\"settings\", m)?;\
                        }\
                    },",
            )?;
        }
    }

    writer.write(
        "\
                    _ => {},\
                }\
                s.end()\
            }\
        }",
    )
}

pub fn impl_macro(rulesets: &[RulesetMods], writer: &mut Writer) -> GenResult {
    let mut intermodes = rulesets
        .iter()
        .flat_map(|ruleset| {
            let ruleset_str = ruleset.name.as_capitalized_str();

            ruleset.mods.iter().map(|gamemod| {
                let intermode = &gamemod.name[..gamemod.name.len() - ruleset_str.len()];

                (gamemod.acronym.as_str(), intermode)
            })
        })
        .collect::<HashMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();

    intermodes.sort_unstable();

    writer.write(
        "#[macro_export(local_inner_macros)]\
        #[doc(hidden)]\
        macro_rules! mods_inner {",
    )?;

    for (acronym, gamemod) in intermodes {
        writer.write_raw(
            b"
    ( [ $( $mode:ident )? ] ",
        )?;
        writer.write(acronym)?;

        writer.write_raw(
            b" \
$( $rest:tt )* ) => {
        mods_inner!( [ $( $mode )? ] $( $rest )* ",
        )?;

        writer.write(gamemod)?;

        writer.write_raw(
            b" )
    };",
        )?;
    }

    writer.write_raw(
        b"
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
}",
    )
}
