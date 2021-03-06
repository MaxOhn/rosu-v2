mod beatmap;
mod comments;
mod event;
mod grade;
mod kudosu;
mod matches;
mod mode;
mod mods;
mod multiplayer;
mod news;
mod ranking;
mod score;
mod user;
mod wiki;

pub use beatmap::{
    Beatmap, BeatmapCompact, Beatmapset, BeatmapsetAvailability, BeatmapsetCompact,
    BeatmapsetCovers, BeatmapsetHype, BeatmapsetNominations, FailTimes, Mapset, MostPlayedMap,
    RankStatus,
};
pub use comments::{Comment, CommentBundle, CommentSort, CommentableMeta};
pub use event::{Event, EventBeatmap, EventBeatmapset, EventType, EventUser};
pub use grade::Grade;
pub use kudosu::{KudosuAction, KudosuGiver, KudosuHistory, KudosuPost};
pub use matches::{
    MatchEvent, MatchGame, MatchInfo, MatchList, MatchScore, OsuMatch, ScoringType, Team, TeamType,
};
pub use mode::GameMode;
pub use mods::GameMods;
pub use multiplayer::{MultiplayerScore, MultiplayerScores, ScoresAround};
pub use news::{News, NewsPost, NewsSearch, NewsSidebar};
pub use ranking::{Rankings, RankingsCursor, Spotlight};
pub use score::{BeatmapScores, BeatmapUserScore, Score, ScoreStatistics, ScoreWeight};
pub use user::{
    AccountHistory, Badge, Country, GradeCounts, Group, HistoryType, Medal, MedalCompact,
    MonthlyCount, Playstyle, ProfileBanner, ProfilePage, User, UserCompact, UserCover, UserKudosu,
    UserLevel, UserPage, UserStatistics,
};
pub use wiki::WikiPage;

pub(crate) use comments::CommentBundleCursor;
pub(crate) use matches::MatchListCursor;
pub(crate) use news::NewsCursor;

use serde::{Deserialize, Deserializer, Serializer};

pub fn inflate_acc<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
    let acc: f32 = Deserialize::deserialize(d)?;

    Ok(100.0 * acc)
}

pub fn deflate_acc<S: Serializer>(f: &f32, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_f32(*f / 100.0)
}
