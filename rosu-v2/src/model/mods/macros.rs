/// Short-hand macro to easily create [`GameMods`](crate::model::mods::GameMods)
/// or [`GameModsIntermode`](crate::model::mods::GameModsIntermode).
///
/// To create [`GameModsIntermode`](crate::model::mods::GameModsIntermode),
/// specify a space-separated list of acronyms.
///
/// To create [`GameMods`](crate::model::mods::GameMods),
/// specify `Osu`, `Taiko`, `Catch`, or `Mania`, followed
/// by a colon (`:`), followed by a space-separated list of acronyms.
/// Note that creating [`GameMods`](crate::model::mods::GameMods) requires the `macros` feature flag.
///
/// # Example
///
/// ```rust
/// # use rosu_v2::{mods, model::mods::GameModsIntermode};
#[cfg_attr(feature = "macros", doc = "# use rosu_v2::model::mods::GameMods;")]
#[cfg_attr(feature = "macros", doc = "let mods: GameMods = mods!(Taiko: NC HR);")]
#[cfg_attr(feature = "macros", doc = r#"assert_eq!(mods.to_string(), "HRNC");"#)] // FIXME: seems to not be rendered properly by rustdoc
#[cfg_attr(feature = "macros", doc = "")]
/// let mods: GameModsIntermode = mods!(DT HR TC);
/// assert_eq!(mods.to_string(), "HRDTTC");
/// ```
#[macro_export(local_inner_macros)]
macro_rules! mods {
    ( $mode:ident: $first:ident $( $rest:ident )* ) => {{
        #[cfg(not(feature = "macros"))]
        {
            std::compile_error!("must enable `macros` feature to use `mods!` macro for a GameMode");

            Default::default()
        }

        #[cfg(feature = "macros")]
        mods_inner!([$mode] $first $( $rest )* )
    }};

    ( Osu ) => { mods_inner!( [ Osu ] ) };
    ( Taiko ) => { mods_inner!( [ Taiko ] ) };
    ( Catch ) => { mods_inner!( [ Catch ] ) };
    ( Mania ) => { mods_inner!( [ Mania ] ) };
    ( ) => { mods_inner!([]) };

    ( $first:ident $( $rest:ident )* ) => {
        mods_inner!([] $first $( $rest )* )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_intermode() {
        let mods = mods!();
        assert!(mods.is_empty())
    }

    #[test]
    fn single_intermode() {
        let mods = mods!(WG);
        assert_eq!(mods.len(), 1);
    }

    #[test]
    fn full_intermode() {
        let mods = mods!(HD DT DT HR TC);
        assert_eq!(mods.to_string(), "HDHRDTTC");
    }

    #[test]
    fn empty_catch() {
        let mods = mods!(Catch);
        assert!(mods.is_empty());
    }

    #[cfg(feature = "macros")]
    #[test]
    fn full_taiko() {
        let mods = mods!(Taiko: HR PF);
        assert_eq!(mods.len(), 2);
    }
}
