## Upcoming

- __Breaking:__
  - `GameMods` have been reworked entirely to accommodate for osu!lazer's mods. They're no longer based on bitflags
     but instead on a `BTreeMap` collection of `GameMod`. `GameMod` variants account for a `GameMode`. The analogous
     mode-agnostic counterpart is `GameModsIntermode` and their `GameModIntermode` elements. E.g. there are the variants
     `GameMod::HiddenOsu` and `GameMod::HiddenTaiko`, as well as `GameModIntermode::Hidden`.
     The macro `mods!` can be used as shorthand for creating `GameMods` or `GameModsIntermode` based on acronyms
     e.g. `mods!(Catch: HD HR DT)` to create `GameMods` for `GameMode::Catch` or `mods!(HD HR DT)` to create `GameModsIntermode`.
  - `User`, `Beatmap`, and `Beatmapset` have been renamed to `UserExtended`, `BeatmapExtended`, and `BeatmapsetExtended`
     and `UserCompact`, `BeatmapCompact`, and `BeatmapsetCompact` have been renamed to `User`, `Beatmap`, and `Beatmapset`
  - Added the field `mapset_id` to `Beatmap`
  - The fields `FailTimes::fail` and `FailTimes::exit` are now of type `Option<Box<u32; 100>>` instead of `Option<Vec<u32>>`
  - Metrics are now recorded using the [`metrics`](https://github.com/metrics-rs/metrics/tree/main/metrics) crate instead of prometheus directly.
    That means they're no longer exposed through a method, but are recorded in the global metrics registry instead.
    You can install a global metrics registry e.g. through the [metrics-exporter-prometheus](https://github.com/metrics-rs/metrics/tree/main/metrics-exporter-prometheus) crate.

- __Fixes__:
  - Fixed deserializing `FailTimes` for `Beatmap` and `BeatmapExtended`

# v0.8.0 (2023-06-27)

- __Breaking:__
  - Added the field `map_id` to `Score`
  - Added the fields `description` and `permanent` to `AccountHistory`
  - Added the variant `TournamentBan` to `HistoryType`
  - Added the variant `TagsEdit` to `BeatmapsetEvent`
  - Types no longer implement `serde::Serialize` unless the `serialize` feature is specified ([#4])
  - Replaced the method `GetBeatmapScores::score_type` with `GetBeatmapScores::global` and `GetBeatmapScores::country`

- __Fixes:__
  - Anticipate `null` when deserializing user's `default_group`

- __Additions:__
  - Added the method `GetBeatmapScores::limit`
  - The method `GetBeatmapScores::mods` no longer shows the deprecation notice

## v0.7.0 (2022-12-25)

- __Adjustments:__
  - Implemented `rkyv::{Archive, Serialize, Deserialize}` for `BeatmapsetSearchSort`

- __Additions:__
  - Added the method `GameMods::clock_rate`
  - Added `Ord` and `PartialOrd` implementation for `GameMode`
  - Added the method `Osu::replay_raw` to request the bytes of a replay. If the `replay` feature is enabled, the new method `Osu::replay` requests the replay and parses it into a [`osu_db::Replay`](https://docs.rs/osu-db/latest/osu_db/replay/struct.Replay.html). Note that both of these methods **require OAuth** through `OsuBuilder::with_authorization`. ([#2] - [@mezo])

- __Breaking:__
  - Added the field `passed` to `Score` ([#3] - [@Jeglerjeg])
  - Instead of introducing custom archived types, some types now archive into themselves.
    Impacted types are: `Grade`, `KudosuAction`, `CommentSort`, `HistoryType`, `Playstyle`, `ProfilePage`, `BeatmapDifficultyAttributes`, and `GameModeAttributes`.

## v0.6.2 (2022-10-28)

- __Fixes:__
  - Fixed deserialization of datetimes and made them mode robust against future changes

- __Additions:__
  - Added the field `highest_rank` to `User` and `UserCompact`

## v0.6.1 (2022-10-24)

- __Fixes:__
  - Fixed deserialization when requesting mapset id 3
  - Fixed deserialization of datetimes in comments

- __Breaking changes:__
  - The serialization of all `OffsetDateTime` was changed. They used to be serialized into the amount of unix timestamp nanoseconds which was an i128. Since those could not be serialized into a `serde_json::Value` without significant performance loss, all datetimes are now serialized into a string of the same format given by the osu!api.

## v0.5.0 (2022-10-08)

- __Adjustments:__
  - If the `cache` feature is enabled, the cache now fills proactively and also updates with respect to username changes
- __Additions:__
  - Added a metric for the amount of Username-UserId pairs that are currently cached
  - Added method `Osu::beatmap_difficulty_attributes` to retrieve the `BeatmapDifficultyAttributes` of a beatmap.
  - Added method `OsuBuilder::retries` to specify how often requests should be retried in case they timeout. Defaults to 2 i.e. 3 attempts in total.
  - Added method `OsuBuilder::ratelimit` to specify how many requests per seconds can be made. Value will be clamped between 1 and 20 and defaults to 15.
  - Added method `Osu::beatmapset_from_map_id` to retrieve a mapset using a map ID. ([#1] - [@Jeglerjeg])
- __Breaking changes:__
  - Renamed the `GameMode` variants to make them more idiomatic
  - Replaced the `chrono` dependency with `time` so all datetime fields now come from the `time` crate. This includes fields being (de)serialized differently.
  - Now using the specific api version 20220705 which renamed a few fields but only one of those made it through to the interface: `Score::created_at` is now called `Score::ended_at`
  - The `Score::score_id` field is now of type `Option<u64>` instead of `u64`
  - `GameModeAttributes::Taiko` now has an additional field `peak_difficulty` and no longer has the field `ar`

## v0.4.0

- __Breaking:__
  - `MatchEvent::Create`'s `user_id` field is now of type `Option<u32>` (previously just `u32`)
  - `Score::replay` is now of type `Option<bool>` (previously just `bool`)
  - Added the field `guest_mapset_count` to `User` and `UserCompact`
  - Added the field `creator_id` to `Beatmap` and `BeatmapCompact`
  - The field `user_id` of `Comment` is now an `Option<u32>` instead of just `u32`.
  - The method `get_user` of `Comment` now returns `Option<GetUser<'_>>` instead of `GetUser<'_>`

- __Fixes:__
  - Now deserializing `medal` recent events properly
  - Added deserialization for mods in form of objects

## v0.3.2

- Fixed `Grade` calculation for taiko `Score`s
- Added feature `rkyv` to provide `Archive`, `Deserialize`, and `Serialize` impls of rkyv for insanely fast (de)serialization
- Bumped dashmap to v5.1.0
- Added `Osu::beatmap_user_scores` to get scores for all mod combinations of a user on a map

## v0.3.1

- Added method `Osu::beatmaps` to retrieve multiple maps at once (up to 50).
- Added method `Osu::score` to retrieve a specific score.
- Removed metrics for multiplayer endpoints.
- Added `UserId` to the prelude module
- Added `Clone`, `Eq`, `PartialEq`, and `Hash` impls for `UserId`
- Improved compile time by removing `build.rs` file
- Added method `GetUserScores::pinned` to retrieve the pinned scores of a user

## v0.3.0

- Added a bunch of documentation
- [Breaking] Adjusted some struct fields:
  - Added `Group::has_modes`
  - Added `WikiPage::available_locales`
  - Removed `User::skype`
  - Removed `User::is_restricted` and `UserCompact::is_restricted`
- [Breaking] Removed `Osu` methods `multiplayer_score`, `multiplayer_scores`, and `multiplayer_user_highscore`
- [Breaking] All fields representing a username are no longer `String` but `SmallString<[u8; 15]>` instead.
    Since usernames can't be longer than 15 characters, this type will prevent allocations. It's aliased as `Username`.
    Affected fields are:
  - `Beatmapset.creator_name`
  - `BeatmapsetCommentOwnerChange.new_username`
  - `BeatmapsetCompact.creator_name`
  - `Comment.legacy_name`
  - `KudosuAction::KudosuGiver.username`
  - `NewsPost.author`
  - `EventUser.username`
  - `EventUser.previous_username`
  - `User::username`
  - `User::previous_usernames`
  - `UserCompact.username`
  - `UserCompact.previous_usernames`
- Added `float` method to `UserLevel`
- [Breaking] All fields representing a country code are no longer `String` but `SmallString<[u8; 2]>` instead.
    Since country codes can't be longer than 2 characters, this type will prevent allocations. It's aliased as `CountryCode`.
    Affected fields are:
  - `CountryRanking.country_code`
  - `User.country_code`
  - `UserCompact.country_code`
  - `GetPerformanceRankings::country`

## v0.2.0

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

[@Jeglerjeg]: https://github.com/Jeglerjeg
[@mezo]: https://github.com/mezodev0

[#1]: https://github.com/MaxOhn/rosu-v2/pull/1
[#2]: https://github.com/MaxOhn/rosu-v2/pull/2
[#3]: https://github.com/MaxOhn/rosu-v2/pull/3
[#4]: https://github.com/MaxOhn/rosu-v2/pull/4