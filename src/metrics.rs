#![cfg(feature = "metrics")]

use prometheus::{IntCounter, IntCounterVec, Opts};

pub(crate) struct Metrics {
    pub(crate) counters: IntCounterVec,

    pub(crate) beatmap: IntCounter,
    pub(crate) beatmap_scores: IntCounter,
    pub(crate) beatmap_user_score: IntCounter,
    pub(crate) beatmapset: IntCounter,
    pub(crate) beatmapset_events: IntCounter,

    pub(crate) comments: IntCounter,

    pub(crate) forum_posts: IntCounter,

    pub(crate) osu_match: IntCounter,
    pub(crate) match_list: IntCounter,

    pub(crate) multiplayer_score: IntCounter,
    pub(crate) multiplayer_scores: IntCounter,
    pub(crate) multiplayer_user_highscore: IntCounter,

    pub(crate) news: IntCounter,

    pub(crate) rankings: IntCounter,
    pub(crate) spotlights: IntCounter,

    pub(crate) user: IntCounter,
    pub(crate) user_beatmapsets: IntCounter,
    pub(crate) recent_events: IntCounter,
    pub(crate) user_kudosu: IntCounter,
    pub(crate) most_played: IntCounter,
    pub(crate) user_top_scores: IntCounter,
    pub(crate) user_recent_scores: IntCounter,
    pub(crate) user_first_scores: IntCounter,
    pub(crate) users: IntCounter,

    pub(crate) wiki: IntCounter,
}

impl Metrics {
    #[cold]
    pub(crate) fn new() -> Self {
        let opts = Opts::new("osu_requests", "osu!api request count");
        let counters = IntCounterVec::new(opts, &["type"]).unwrap();

        Self {
            beatmap: counters.with_label_values(&["Beatmap"]),
            beatmap_scores: counters.with_label_values(&["Beatmap scores"]),
            beatmap_user_score: counters.with_label_values(&["Beatmap user scores"]),
            beatmapset: counters.with_label_values(&["Beatmapset"]),
            beatmapset_events: counters.with_label_values(&["Beatmapset events"]),

            comments: counters.with_label_values(&["Comments"]),

            forum_posts: counters.with_label_values(&["Forum posts"]),

            osu_match: counters.with_label_values(&["Matches"]),
            match_list: counters.with_label_values(&["Match list"]),

            multiplayer_score: counters.with_label_values(&["Multiplayer score"]),
            multiplayer_scores: counters.with_label_values(&["Multiplayer scores"]),
            multiplayer_user_highscore: counters.with_label_values(&["Multiplayer user highscore"]),

            news: counters.with_label_values(&["News"]),

            rankings: counters.with_label_values(&["Rankings"]),
            spotlights: counters.with_label_values(&["Spotlights"]),

            user: counters.with_label_values(&["User"]),
            user_beatmapsets: counters.with_label_values(&["User mapsets"]),
            recent_events: counters.with_label_values(&["User events"]),
            user_kudosu: counters.with_label_values(&["User kudosu"]),
            most_played: counters.with_label_values(&["User most played"]),
            user_top_scores: counters.with_label_values(&["User top scores"]),
            user_recent_scores: counters.with_label_values(&["User recent scores"]),
            user_first_scores: counters.with_label_values(&["User first scores"]),
            users: counters.with_label_values(&["Users"]),

            wiki: counters.with_label_values(&["Wiki"]),

            counters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inc() {
        let metrics = Metrics::new();

        metrics.beatmap.inc();
        metrics.user.inc();
        metrics.wiki.inc();
        metrics.user.inc();

        let beatmap = metrics.counters.with_label_values(&["Beatmap"]).get();
        let user = metrics.counters.with_label_values(&["User"]).get();
        let wiki = metrics.counters.with_label_values(&["Wiki"]).get();

        assert_eq!(beatmap, 1);
        assert_eq!(user, 2);
        assert_eq!(wiki, 1);
    }
}
