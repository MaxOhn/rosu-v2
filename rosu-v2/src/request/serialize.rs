use crate::model::GameMode;
use crate::prelude::{CommentSort, GameModsIntermode};

use crate::request::UserId;
use serde::ser::{SerializeMap, Serializer};
use std::cmp;

fn maybe<F, T, S>(option: &Option<T>, serializer: S, f: F) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    F: FnOnce(&T, S) -> Result<S::Ok, S::Error>,
{
    match option {
        Some(some) => f(some, serializer),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn maybe_mode_as_str<S: Serializer>(
    mode: &Option<GameMode>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    maybe(mode, serializer, mode_as_str)
}

pub(crate) fn mode_as_str<S: Serializer>(
    mode: &GameMode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(mode.as_str())
}

pub(crate) fn maybe_mods_as_list<S: Serializer>(
    mods: &Option<GameModsIntermode>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    maybe(mods, serializer, mods_as_list)
}

pub(crate) fn mods_as_list<S: Serializer>(
    mods: &GameModsIntermode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(cmp::max(mods.len(), 1)))?;

    if mods.is_empty() {
        map.serialize_entry("mods[]", "NM")?;
    } else {
        for m in mods.iter() {
            map.serialize_entry("mods[]", m.acronym().as_str())?;
        }
    }

    map.end()
}

pub(crate) fn maybe_comment_sort<S: Serializer>(
    sort: &Option<CommentSort>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    maybe(sort, serializer, CommentSort::serialize_as_query)
}

pub(crate) fn maybe_user_id_type<S: Serializer>(
    user_id: &Option<UserId>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    maybe(user_id, serializer, user_id_type)
}

pub(crate) fn user_id_type<S: Serializer>(
    user_id: &UserId,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match user_id {
        UserId::Id(_) => serializer.serialize_str("id"),
        UserId::Name(_) => serializer.serialize_str("username"),
    }
}

pub(crate) fn maybe_bool_as_u8<S: Serializer>(
    b: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    maybe(b, serializer, bool_as_u8)
}

pub(crate) fn bool_as_u8<S: Serializer>(b: &bool, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u8(if *b { 1 } else { 0 })
}
