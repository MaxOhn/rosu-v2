## Upcoming

- Fixed `Grade` calculation for taiko `Score`s
- Added feature `rkyv` to provide `Archive`, `Deserialize`, and `Serialize` impls of rkyv for insanely fast (de)serialization

# v0.3.1

- Added method `Osu::beatmaps` to retrieve multiple maps at once (up to 50).
- Added method `Osu::score` to retrieve a specific score.
- Removed metrics for multiplayer endpoints.
- Added `UserId` to the prelude module
- Added `Clone`, `Eq`, `PartialEq`, and `Hash` impls for `UserId`
- Improved compile time by removing `build.rs` file
- Added method `GetUserScores::pinned` to retrieve the pinned scores of a user
- Bumped dashmap to v5.1.0

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
