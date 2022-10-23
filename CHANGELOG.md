## Upcoming

- __Fixes:__
  - Fixed deserialization when requesting mapset id 3
  - Fixed deserialization of datetimes in comments

# v0.5.0 (2022-10-08)

- __Adjustments:__
  - If the `cache` feature is enabled, the cache now fills proactively and also updates with respect to username changes
- __Additions:__
  - Added a metric for the amount of Username-UserId pairs that are currently cached
  - Added method `Osu::beatmap_difficulty_attributes` to retrieve the `BeatmapDifficultyAttributes` of a beatmap.
  - Added method `OsuBuilder::retries` to specify how often requests should be retried in case they timeout. Defaults to 2 i.e. 3 attempts in total.
  - Added method `OsuBuilder::ratelimit` to specify how many requests per seconds can be made. Value will be clamped between 1 and 20 and defaults to 15.
  - Added method `Osu::beatmapset_from_map_id` to retrieve a mapset using a map ID.
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
