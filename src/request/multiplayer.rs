use serde::Serialize;

use crate::{
    model::multiplayer::{Room, RoomCategory},
    prelude::{PlaylistScores, PlaylistScoresSort, RoomEvents, RoomLeaderboard},
    request::{Query, Request},
    routing::Route,
    Osu,
};

/// Get [`PlaylistScores`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetPlaylistScores<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    room_id: u64,
    #[serde(skip)]
    playlist_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort: Option<PlaylistScoresSort>,
    #[serde(
        rename = "cursor_string",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    cursor: Option<&'a str>,
}

impl<'a> GetPlaylistScores<'a> {
    pub(crate) const fn new(osu: &'a Osu, room_id: u64, playlist_id: u32) -> Self {
        Self {
            osu,
            room_id,
            playlist_id,
            limit: None,
            sort: None,
            cursor: None,
        }
    }

    /// Maximum number of results.
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Sort order.
    #[inline]
    pub const fn sort(mut self, sort: PlaylistScoresSort) -> Self {
        self.sort = Some(sort);

        self
    }

    /// Cursor for pagination.
    #[inline]
    pub const fn cursor(mut self, cursor: &'a str) -> Self {
        self.cursor = Some(cursor);

        self
    }
}

into_future! {
    |self: GetPlaylistScores<'_>| -> PlaylistScores {
        let route = Route::GetPlaylistScores {
            room_id: self.room_id,
            playlist_id: self.playlist_id,
        };

        let req = Request::with_query(route, Query::encode(&self));

        (req, (self.room_id, self.playlist_id))
    } => |scores, data: (u64, u32)| -> PlaylistScores {
        let (room_id, playlist_id) = data;
        scores.room_id = room_id;
        scores.playlist_id = playlist_id;

        Ok(scores)
    }
}

/// Get a [`Room`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetRoom<'a> {
    osu: &'a Osu,
    room_id: u64,
}

impl<'a> GetRoom<'a> {
    pub(crate) const fn new(osu: &'a Osu, room_id: u64) -> Self {
        Self { osu, room_id }
    }
}

into_future! {
    |self: GetRoom<'_>| -> Room {
        Request::new(Route::GetRoom { room_id: self.room_id })
    }
}

/// Get [`RoomEvents`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetRoomEvents<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(skip)]
    room_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    after: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    before: Option<u64>,
}

impl<'a> GetRoomEvents<'a> {
    pub(crate) const fn new(osu: &'a Osu, room_id: u64) -> Self {
        Self {
            osu,
            room_id,
            limit: None,
            after: None,
            before: None,
        }
    }

    /// Maximum number of results.
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        // FIXME: awaiting usize::clamp to be const
        const fn clamp(value: usize, min: usize, max: usize) -> usize {
            if value < min {
                min
            } else if value > max {
                max
            } else {
                value
            }
        }

        self.limit = Some(clamp(limit, 1, 100));

        self
    }

    /// Only include events after the given event id.
    #[inline]
    pub const fn after(mut self, event_id: u64) -> Self {
        self.after = Some(event_id);

        self
    }

    /// Only include events before the given event id.
    #[inline]
    pub const fn before(mut self, event_id: u64) -> Self {
        self.before = Some(event_id);

        self
    }
}

into_future! {
    |self: GetRoomEvents<'_>| -> RoomEvents {
        Request::with_query(
            Route::GetRoomEvents { room_id: self.room_id },
            Query::encode(&self),
        )
    }
}

/// Get a [`RoomLeaderboard`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone)]
pub struct GetRoomLeaderboard<'a> {
    osu: &'a Osu,
    room_id: u64,
}

impl<'a> GetRoomLeaderboard<'a> {
    pub(crate) const fn new(osu: &'a Osu, room_id: u64) -> Self {
        Self { osu, room_id }
    }
}

into_future! {
    |self: GetRoomLeaderboard<'_>| -> RoomLeaderboard {
        Request::new(Route::GetRoomLeaderboard { room_id: self.room_id })
    }
}

/// The "sort" for [`GetRooms`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RoomsSort {
    Ended,
    Created,
}

/// The "type group" for [`GetRooms`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RoomsTypeGroup {
    #[default]
    Playlists,
    Realtime,
}

/// The "filter" for [`GetRooms`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RoomsFilter {
    #[default]
    Active,
    All,
    Ended,
    Participated,
    Owned,
}

/// Get a vec of [`Room`].
#[must_use = "requests must be configured and executed"]
#[derive(Clone, Serialize)]
pub struct GetRooms<'a> {
    #[serde(skip)]
    osu: &'a Osu,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
    filter: Option<RoomsFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    season_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort: Option<RoomsSort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    type_group: Option<RoomsTypeGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    category: Option<RoomCategory>,
}

impl<'a> GetRooms<'a> {
    pub(crate) const fn new(osu: &'a Osu) -> Self {
        Self {
            osu,
            limit: None,
            filter: None,
            season_id: None,
            sort: None,
            type_group: None,
            category: None,
        }
    }

    /// Maximum number of results.
    #[inline]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);

        self
    }

    /// Filter mode.
    #[inline]
    pub const fn filter(mut self, filter: RoomsFilter) -> Self {
        self.filter = Some(filter);

        self
    }

    /// Season ID to return rooms from.
    #[inline]
    pub const fn season_id(mut self, season_id: u64) -> Self {
        self.season_id = Some(season_id);

        self
    }

    /// Sort order.
    #[inline]
    pub const fn sort(mut self, sort: RoomsSort) -> Self {
        self.sort = Some(sort);

        self
    }

    #[inline]
    pub const fn type_group(mut self, type_group: RoomsTypeGroup) -> Self {
        self.type_group = Some(type_group);

        self
    }

    #[inline]
    pub const fn category(mut self, category: RoomCategory) -> Self {
        self.category = Some(category);

        self
    }
}

into_future! {
    |self: GetRooms<'_>| -> Vec<Room> {
        Request::with_query(Route::GetRooms, Query::encode(&self))
    }
}
