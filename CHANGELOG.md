## Upcoming

- Nothing as of now

# v0.2.0

- Dont only consider HD when calculating grade but also Flashlight and FadeIn
- Implemented `Default` for `Language`, `Genre`, `ScoringType`, `TeamType`, and `Team` enums
- Made checking for `Score` equivalency more lenient w.r.t. their timestamps
- [Breaking] Removed deprecated `cover_url` field from `User`; use `cover.url` instead
- [Breaking] `description` field of `Group` is now `Option<String>` instead of `String`
- [Breaking] Added new `BeatmapsetEvent` variant `OwnerChange` and declared `BeatmapsetEvent` as non-exhaustive
- [Breaking] `OsuBuilder` no longer accepts a reqwest client since its now using a hyper client
- [Breaking] Removed all endpoint-specific cursor structs and replaced them by a single struct `Cursor`
- [Breaking] Adjusted / Renamed / Added some `OsuError` variants
- [Breaking] `User` and `UserCompact` fields `ranked_and_approved_beatmapset_count`, `unranked_beatmapset_count`, `favourite_beatmapset_count`, `graveyard_beatmapset_count`, and `loved_beatmapset_count` where replaced with `ranked_mapset_count`, `pending_mapset_count`, `favourite_mapset_count`, `graveyard_mapset_count`, and `loved_mapset_count`, respectively
- [Breaking] `GetUserBeatmapsets` methods `ranked_and_approved` and `unranked` were replaced with `ranked` and `pending`, respectively
- [Breaking] Removed `GetUserBeatmapset::favourite` method

## v0.1.0

- Initial release