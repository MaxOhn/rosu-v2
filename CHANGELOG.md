# v0.11.0 (2025-05-21)

- __Breaking:__
  - `Score.classic_score` is now a `u64` instead of `u32`
  - The type `rosu_v2::model::matches::Team` was renamed to `MatchTeam`, the variant `ParsingError::Team` was renamed to `MatchTeam`, and added the field `team: Option<Team>` to `User` and `UserExtended` ([#44])
  - Adjusted fields in the `GameModeAttributes` enum ([#47])
  - The method `Osu::user_beatmapsets` now takes an additional `UserBeatmapsetsKind` argument, `GetUserBeatmapsets::status` was replaced by `kind` and `GetUserBeatmapsets::loved`, `ranked`, `pending`, and `graveyard` were removed ([#52])
  - The method `Osu::token` is no longer async ([#53])
  - The endpoint structs (e.g. `GetUser`, `GetBeatmapScores`, ...) no longer implement `Future` and instead they implement `IntoFuture` by transforming into the struct `OsuFuture`. Also, methods on `Osu` that took `u32` user ids now take `Into<UserId>`, the `body: String` fields of the `OsuError::Parsing` and `OsuError::Response` variants are replaced by `bytes: Bytes` and the variant `OsuError::ServiceUnavailable` now contains a `body: hyper::body::Incoming` field instead of an unnamed String ([#54])
  - `Osu::beatmap_scores` now returns `BeatmapScores` instead of `Vec<Score>` ([#55])

- __Additions:__
  - Added the method `Osu::team_rankings` ([#45])

- __Adjustments:__
  - Use proper rfc3339 to (de)serialize datetimes ([#49])

## v0.10.0 (2025-02-05)

- __Breaking:__
  - Added field `UserStatistics::rank_change_since_30_days` ([#30] - [@damaredayo])
  - Added field `Score::total_score_without_mods` ([#31])
  - Added field `Score::set_on_lazer`
  - Added field `UserExtended::daily_challenge_stats` ([#33])
  - Changed field types from `f32` to `f64` for `BeatmapDifficultyAttributes::stars`, `GameModeAttributes::Osu::{aim_difficulty, flashlight_difficulty, slider_factor, speed_difficulty}`, `GameModeAttributes::Taiko::{stamina_difficulty, rhythm_difficulty, colour_difficulty, peak_difficulty}` ([#35])
  - Added field `GameModeAttributes::Osu::speed_note_count` ([#35])
  - Removed field `GameModeAttributes::Mania::score_multiplier` ([#35])
  - `OsuBuilder::with{_local}_authorization` now takes an additional argument of type `Scopes` ([#37])
  - The method `ScoreStatistics::accuracy` now takes an additional argument `max_statistics: &ScoreStatistics`
  - Changed field type from `u32` to `i32` for `BeatmapsetVote::score`
  - Bumped `rosu-mods` to `0.2.0` ([#40] - [@natsukagami])
  - Bumped `hyper` to `1.0.0`, as well as a bump for `bytes`, `http-body-util`, `hyper-util`, and `hyper-rustls` ([#42])

- __Additions:__
  - Added method `Osu::friends` to fetch the authorized user's friends list ([#38])
  - Added methods `{Score, ScoreStatistics}::legacy_accuracy` as opposed to the types' `accuracy` methods
  - Added method `OsuBuilder::with_token` ([#41])
  - Added method `Osu::scores` to fetch recently processed scores (passes) ([#43])

## v0.9.0 (2024-07-10)

- __Breaking:__
  - All mods types are now re-exports from [`rosu-mods`](https://github.com/MaxOhn/rosu-mods) ([#28])
  - `User`, `Beatmap`, and `Beatmapset` have been renamed to `UserExtended`, `BeatmapExtended`, and `BeatmapsetExtended`;
     `UserCompact`, `BeatmapCompact`, and `BeatmapsetCompact` have been renamed to `User`, `Beatmap`, and `Beatmapset`
  - The fields `FailTimes::fail` and `FailTimes::exit` are now of type `Option<Box<[u32; 100]>>` instead of `Option<Vec<u32>>`
  - Metrics are now recorded using the [`metrics`](https://github.com/metrics-rs/metrics/tree/main/metrics) crate instead of prometheus directly.
    That means they're no longer exposed through a method, but are recorded in the global metrics registry instead.
    You can install a global metrics registry e.g. through the [metrics-exporter-prometheus](https://github.com/metrics-rs/metrics/tree/main/metrics-exporter-prometheus) crate.
  - Most fields of (optional) `User(Extended)`, `Beatmap(Extended)`, and `Beatmapset(Extended)` are now wrapped in a `Box`. ([#11])
  - The field `Medal::instructions` is now a `Option<String>` instead of `String` ([#12] - [@natsukagami])
  - Removed the method `GetBeatmapsetSearch::any_status` and instead the method `status` now takes an `Option<RankStatus>`; `None` means "any". Also renamed the variant `BeatmapsetSearchSort::RankedDate` to `ApprovedDate` and added the variants `Creator`, `Nominations`, and `LastUpdate`. ([#18])
  - Renamed the struct `RecentEvent` to `Event` and the method `Osu::recent_events` to `recent_activity`. Also added the method `Osu::events`. ([#19])
  - Removed the `Cursor` type. The osu!api now uses encoded strings as cursor value. ([#20])
  - The methods `Osu::{replay, replay_raw, score}` no longer take a `GameMode` as argument. Instead, their builders now have a `mode` method which allows setting a mode optionally. ([#24] - [@natsukagami])
  - Removed the `rkyv` feature ([#27])
  - Added fields:
    - `Beatmap::mapset_id`
    - `Score::classic_score`
    - `UserExtended::statistics_modes`
  - Removed the field `BeatmapsetNominations::required` and added `BeatmapsetNominations::{eligible_main_rulesets, required_meta}`.

- __Fixes:__
  - Fixed deserializing `FailTimes` for `Beatmap` and `BeatmapExtended`

- __Additions:__
  - The endpoint `Osu::users` is now usable without deprecation warning to retrieve up to 50 users at once. ([#16])
  - Endpoints to retrieve scores now provide a `legacy_scores` method to request score data in its legacy format. ([#14])
  - Endpoints to retrieve scores now provide a `legacy_only` method to only request non-lazer scores. ([#21])
  - Added the feature `local_oauth` to add the method `OsuBuilder::with_local_authorization` to perform the whole OAuth process locally. ([#29])

## v0.8.0 (2023-06-27)

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
[@natsukagami]: https://github.com/natsukagami
[@damaredayo]: https://github.com/damaredayo

[#1]: https://github.com/MaxOhn/rosu-v2/pull/1
[#2]: https://github.com/MaxOhn/rosu-v2/pull/2
[#3]: https://github.com/MaxOhn/rosu-v2/pull/3
[#4]: https://github.com/MaxOhn/rosu-v2/pull/4
[#11]: https://github.com/MaxOhn/rosu-v2/pull/11
[#12]: https://github.com/MaxOhn/rosu-v2/pull/12
[#14]: https://github.com/MaxOhn/rosu-v2/pull/14
[#16]: https://github.com/MaxOhn/rosu-v2/pull/16
[#18]: https://github.com/MaxOhn/rosu-v2/pull/18
[#19]: https://github.com/MaxOhn/rosu-v2/pull/19
[#20]: https://github.com/MaxOhn/rosu-v2/pull/20
[#21]: https://github.com/MaxOhn/rosu-v2/pull/21
[#24]: https://github.com/MaxOhn/rosu-v2/pull/24
[#27]: https://github.com/MaxOhn/rosu-v2/pull/27
[#28]: https://github.com/MaxOhn/rosu-v2/pull/28
[#29]: https://github.com/MaxOhn/rosu-v2/pull/29
[#30]: https://github.com/MaxOhn/rosu-v2/pull/30
[#31]: https://github.com/MaxOhn/rosu-v2/pull/31
[#33]: https://github.com/MaxOhn/rosu-v2/pull/33
[#35]: https://github.com/MaxOhn/rosu-v2/pull/35
[#37]: https://github.com/MaxOhn/rosu-v2/pull/37
[#38]: https://github.com/MaxOhn/rosu-v2/pull/38
[#40]: https://github.com/MaxOhn/rosu-v2/pull/40
[#41]: https://github.com/MaxOhn/rosu-v2/pull/41
[#42]: https://github.com/MaxOhn/rosu-v2/pull/42
[#43]: https://github.com/MaxOhn/rosu-v2/pull/43
[#44]: https://github.com/MaxOhn/rosu-v2/pull/44
[#45]: https://github.com/MaxOhn/rosu-v2/pull/45
[#47]: https://github.com/MaxOhn/rosu-v2/pull/47
[#49]: https://github.com/MaxOhn/rosu-v2/pull/49
[#52]: https://github.com/MaxOhn/rosu-v2/pull/52
[#53]: https://github.com/MaxOhn/rosu-v2/pull/53
[#54]: https://github.com/MaxOhn/rosu-v2/pull/54
[#55]: https://github.com/MaxOhn/rosu-v2/pull/55