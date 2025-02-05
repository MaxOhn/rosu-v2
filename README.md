[![crates.io](https://img.shields.io/crates/v/rosu-v2.svg)](https://crates.io/crates/rosu-v2) [![docs](https://docs.rs/rosu-v2/badge.svg)](https://docs.rs/rosu-v2)

# Temporary note

The osu!api is currently in an exceptionally unstable state regarding data structure due to the shift towards lazer.
Instead of regularly re-publishing a new version, the `rosu-v2/lazer` branch will be kept updated.

To use `rosu-v2`, it is recommended to put the following in your `Cargo.toml`:
```toml
rosu-v2 = { git = "https://github.com/MaxOhn/rosu-v2", branch = "lazer" }
```
Be warned though, breaking changes are likely to occur regularly for the time being.

# rosu-v2

<!-- cargo-rdme start -->

rosu-v2 is a wrapper for the [osu!api v2].
As such, it provides a bunch of additional endpoints and data over [`rosu`] which wraps the [osu!api v1].

However, osu!'s api v2 is still very much a WIP and also weakly documented. Hence, there is a chance that some things might break either because of changes in the api or because the response is not being parsed properly.

Feel free to open an issue when things don't work as expected.

### Authentication

Unlike api v1, api v2 does not require an api key by users. Instead, it requires a client id and a client secret.

To get those, you must register an application [here](https://osu.ppy.sh/home/account/edit#new-oauth-application).
Unless you're interested in logging into the API through an osu! account, the callback URL here does not matter and can be left blank.

If you went through the OAuth process for a user, you can provide the callback URL and received code
when creating the client in order to make requests on behalf of the authenticated user.

### Endpoints

The following endpoints are currently supported:

- `beatmaps/lookup`: A specific beatmap including its beatmapset
- `beatmaps`: Up to 50 beatmaps at once including their beatmapsets
- `beatmaps/{map_id}/attributes`: The difficulty attributes of a beatmap
- `beatmaps/{map_id}/scores`: The global score leaderboard for a beatmap
- `beatmaps/{map_id}/scores/users/{user_id}[/all]`: Get (all) top score(s) of a user on a beatmap. Defaults to the play with the __max score__, not pp
- `beatmapsets/{mapset_id}`: The beatmapset including all of its difficulty beatmaps
- `beatmapsets/events`: Various events around a beatmapset such as status, genre, or language updates, kudosu transfers, or new issues
- `beatmapsets/search`: Search for beatmapsets; the same search as on the osu! website
- `comments`: Most recent comments and their replies up to two levels deep
- `events`: Collection of events in order of creation time
- `forums/topics/{topic_id}`: A forum topic and its posts
- `friends`: List of authenticated user's friends
- `matches`: List of currently open multiplayer lobbies
- `matches/{match_id}`: More specific data about a specific multiplayer lobby including participating players and occured events
- `me[/{mode}]`: Detailed info about the authenticated user [in the specified mode] (requires OAuth)
- `news`: Recent news
- `rankings/{mode}/{ranking_type}`: The global leaderboard of either performance points, ranked score, countries, or a spotlight
- `users/{user_id}/{recent_activity}`: List of a user's recent events like achieved medals, ranks on a beatmaps, username changes, supporter status updates, beatmapset status updates, ...
- `scores/{mode}/{score_id}`: A specific score including its beatmap, beatmapset, and user
- `scores`: Up to 1000 most recently processed scores (passes)
- `seasonal-backgrounds`: List of seasonal backgrounds i.e. their URL and artists
- `spotlights`: List of overviews of all spotlights
- `users/{user_id}[/{mode}]`: Detailed info about a user [in the specified mode]
- `users/{user_id}/{beatmapsets/{map_type}`: List of beatmapsets either created, favourited, or most played by the user
- `users/{user_id}/kudosu`: A user's recent kudosu transfers
- `users/{user_id}/scores/{score_type}`: Either top, recent, pinned, or global #1 scores of a user
- `users`: Up to 50 users at once including statistics for all modes
- `wiki/{locale}[/{path}]`: The general wiki page or a specific topic if the path is specified

The api itself provides a bunch more endpoints which are not yet implemented because they're either niche and/or missing any documentation.

If you find an endpoint on the [api page](https://osu.ppy.sh/docs/index.html) that you want to use but is missing in rosu-v2, feel free to open an issue.

### Usage

```rust
// For convenience sake, all types can be found in the prelude module
use rosu_v2::prelude::*;

#[tokio::main]
async fn main() {
    // Initialize the client
    let client_id: u64 = 123;
    let client_secret = String::from("my_secret");
    let osu = Osu::new(client_id, client_secret).await.unwrap();

    // Get peppy's top 10-15 scores in osu!standard.
    // Note that the username here can only be used because of the `cache` feature.
    // If you are fine with just providing user ids, consider disabling this feature.
    let scores: Vec<Score> = osu.user_scores("peppy")
        .mode(GameMode::Osu)
        .best() // top scores; alternatively .recent(), .pinned(), or .firsts()
        .offset(10)
        .limit(5)
        .await
        .unwrap();

    // Search non-nsfw loved mania maps matching the given query.
    // Note that the order of called methods doesn't matter for any endpoint.
    let search_result: BeatmapsetSearchResult = osu.beatmapset_search()
        .nsfw(false)
        .status(Some(RankStatus::Loved))
        .mode(GameMode::Mania)
        .query("blue army stars>3")
        .await
        .unwrap();

    // Get the french wiki page on the osu file format
    let wiki_page: WikiPage = osu.wiki("fr")
        .page("Client/File_formats/osu_%28file_format%29")
        .await
        .unwrap();
}
```

### Features

| Flag          | Description                              | Dependencies
| ------------- | ---------------------------------------- | ------------
| `default`     | Enable the `cache` and `macros` features |
| `cache`       | Cache username-user_id pairs so that usernames can be used on all user endpoints instead of only user ids | [`dashmap`]
| `macros`      | Re-exports `rosu-mods`'s `mods!` macro to easily create mods for a given mode | [`paste`]
| `serialize`   | Implement `serde::Serialize` for most types, allowing for manual serialization |
| `metrics`     | Uses the global metrics registry to store response time for each endpoint | [`metrics`]
| `replay`      | Enables the method `Osu::replay` to parse a replay. Note that `Osu::replay_raw` is available without this feature but provides raw bytes instead of a parsed replay | [`osu-db`]
| `local_oauth` | Enables the method `OsuBuilder::with_local_authorization` to perform the full OAuth procedure | `tokio/net` feature

[osu!api v2]: https://osu.ppy.sh/docs/index.html
[`rosu`]: https://github.com/MaxOhn/rosu
[osu!api v1]: https://github.com/ppy/osu-api/wiki
[`dashmap`]: https://docs.rs/dashmap
[`paste`]: https://docs.rs/paste
[`metrics`]: https://docs.rs/metrics
[`osu-db`]: https://docs.rs/osu-db

<!-- cargo-rdme end -->
