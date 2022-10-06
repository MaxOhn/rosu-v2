extern crate rosu_v2;

use rosu_v2::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, fmt::Debug};
use time::{Date, OffsetDateTime};

fn ser_de<T>(val: &T)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    let serialized =
        serde_json::to_string(val).unwrap_or_else(|e| panic!("Failed to serialize: {}", e));

    let deserialized: T = serde_json::from_str(&serialized)
        .unwrap_or_else(|e| panic!("Failed to deserialize: {}\n{serialized}", e));

    assert_eq!(val, &deserialized);
}

fn get_chart_rankings() -> ChartRankings {
    ChartRankings {
        mapsets: vec![get_mapset()],
        ranking: vec![get_user_compact()],
        spotlight: get_spotlight(),
    }
}

fn get_country_ranking() -> CountryRanking {
    CountryRanking {
        active_users: 2,
        country: "belgiania".to_owned(),
        country_code: "be".into(),
        playcount: 420,
        pp: 123.45,
        ranked_score: 1_000_000_000_000_000,
    }
}

fn get_cursor() -> Cursor {
    let json = r#"{"cursor":{"a":123,"b":"henlo","c":true,"d":[1, 2, 3]}}"#;

    serde_json::from_str(json).unwrap()
}

fn get_date() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

fn get_forum_posts() -> ForumPosts {
    ForumPosts {
        cursor: Some(get_cursor()),
        posts: vec![ForumPost {
            created_at: get_date(),
            deleted_at: Some(get_date()),
            edited_at: Some(get_date()),
            edited_by_id: Some(123),
            forum_id: 1234,
            html: "big boi html".to_owned(),
            post_id: 12345,
            raw: "raaaaaaw html".to_owned(),
            topic_id: 1234567,
            user_id: 12345678,
        }],
        search: ForumPostsSearch {
            limit: 42,
            sort: "id_desc".to_owned(),
        },
        topic: ForumTopic {
            created_at: get_date(),
            deleted_at: Some(get_date()),
            first_post_id: 10,
            forum_id: 20,
            is_locked: false,
            kind: "some type".to_owned(),
            last_post_id: 30,
            post_count: 40,
            title: "epic topic".to_owned(),
            topic_id: 50,
            updated_at: Some(get_date()),
            user_id: 60,
        },
    }
}

fn get_mapset_covers() -> BeatmapsetCovers {
    BeatmapsetCovers {
        cover: String::new(),
        cover_2x: String::new(),
        card: String::new(),
        card_2x: String::new(),
        list: String::new(),
        list_2x: String::new(),
        slim_cover: String::new(),
        slim_cover_2x: String::new(),
    }
}

fn get_mapset() -> Beatmapset {
    Beatmapset {
        artist: "artist".to_owned(),
        artist_unicode: Some("äöü".to_owned()),
        availability: BeatmapsetAvailability {
            download_disabled: true,
            more_information: Some("hi".to_owned()),
        },
        bpm: 183.2,
        can_be_hyped: true,
        converts: Some(vec![]),
        covers: get_mapset_covers(),
        creator: Some(get_user_compact()),
        creator_name: "god".into(),
        creator_id: 2,
        description: Some("description".to_owned()),
        discussion_enabled: true,
        discussion_locked: false,
        favourite_count: 1_111_111,
        genre: Some(Genre::Electronic),
        hype: Some(BeatmapsetHype {
            current: 1,
            required: 2,
        }),
        is_scoreable: true,
        language: Some(Language::Spanish),
        last_updated: get_date(),
        legacy_thread_url: Some(String::new()),
        maps: Some(vec![]),
        mapset_id: 12345,
        nominations_summary: BeatmapsetNominations {
            current: 1,
            required: 2,
        },
        nsfw: true,
        playcount: 0,
        preview_url: "b.ppy.sh/preview/12345.mp3".to_owned(),
        ratings: Some(vec![1, 2, 3, 4, 5, 6]),
        ranked_date: Some(get_date()),
        recent_favourites: Some(vec![get_user_compact()]),
        source: String::new(),
        status: RankStatus::WIP,
        storyboard: true,
        submitted_date: Some(get_date()),
        tags: "tags".to_owned(),
        title: "title".to_owned(),
        title_unicode: Some(String::new()),
        video: false,
    }
}

fn get_map() -> Beatmap {
    Beatmap {
        ar: 9.3,
        bpm: 182.3,
        checksum: Some(String::new()),
        convert: false,
        count_circles: 1234,
        count_sliders: 123,
        count_spinners: 1,
        creator_id: 456,
        cs: 4.1,
        deleted_at: Some(get_date()),
        fail_times: Some(FailTimes {
            exit: Some(vec![1, 2, 3]),
            fail: Some(vec![4, 5, 6]),
        }),
        hp: 7.5,
        is_scoreable: true,
        last_updated: get_date(),
        map_id: 123456,
        mapset: Some(get_mapset()),
        mapset_id: 12345,
        max_combo: Some(1750),
        mode: GameMode::Osu,
        od: 7.5,
        passcount: 1_000,
        playcount: 10_000,
        seconds_drain: 234,
        seconds_total: 256,
        stars: 5.89,
        status: RankStatus::Approved,
        url: "https://osu.ppy.sh/beatmaps/123456".to_owned(),
        version: "Insane".to_owned(),
    }
}

fn get_map_compact() -> BeatmapCompact {
    BeatmapCompact {
        checksum: Some("ABC123".to_owned()),
        creator_id: 456,
        fail_times: None,
        map_id: 123456,
        mapset: Some(get_mapset_compact()),
        max_combo: Some(1000),
        mode: GameMode::Catch,
        seconds_total: 120,
        stars: 5.5,
        status: RankStatus::Loved,
        version: "HIAAAA".to_owned(),
    }
}

fn get_mapset_compact() -> BeatmapsetCompact {
    BeatmapsetCompact {
        artist: "artist".to_owned(),
        artist_unicode: Some("äöü".to_owned()),
        covers: get_mapset_covers(),
        creator_name: "god".into(),
        creator_id: 2,
        favourite_count: 1_234_567,
        genre: Some(Genre::Rock),
        hype: Some(BeatmapsetHype {
            current: 1,
            required: 2,
        }),
        language: Some(Language::German),
        mapset_id: 12345,
        nsfw: false,
        playcount: 56_789,
        preview_url: "b.ppy.sh/preview/12345.mp3".to_owned(),
        source: String::new(),
        status: RankStatus::Graveyard,
        title: "title".to_owned(),
        title_unicode: Some(String::new()),
        video: true,
    }
}

fn get_mapset_discussion() -> BeatmapsetDiscussion {
    BeatmapsetDiscussion {
        discussion_id: 0,
        mapset_id: 1,
        map_id: Some(2),
        user_id: 3,
        deleted_by_id: Some(4),
        message_type: "suggestion".to_owned(),
        parent_id: Some(5),
        timestamp: Some(6),
        resolved: false,
        can_be_resolved: true,
        can_grant_kudosu: false,
        created_at: get_date(),
        updated_at: Some(get_date()),
        deleted_at: Some(get_date()),
        last_post_at: get_date(),
        kudosu_denied: true,
        starting_post: BeatmapsetPost {
            post_id: 7,
            discussion_id: 0,
            user_id: 8,
            last_editor_id: Some(9),
            deleted_by_id: Some(10),
            system: false,
            message: "cool story bro".to_owned(),
            created_at: get_date(),
            updated_at: Some(get_date()),
            deleted_at: Some(get_date()),
        },
    }
}

fn get_mapset_events() -> BeatmapsetEvents {
    BeatmapsetEvents {
        events: vec![
            BeatmapsetEvent::Disqualify {
                event_id: 10,
                comment: BeatmapsetCommentId {
                    map_discussion_id: None,
                    map_discussion_post_id: None,
                    mapset_discussion_id: None,
                    mapset_discussion_post_id: None,
                },
                created_at: get_date(),
                mapset: get_mapset_compact(),
                user_id: 123456,
                discussion: get_mapset_discussion(),
            },
            BeatmapsetEvent::GenreEdit {
                event_id: 0,
                comment: BeatmapsetCommentEdit {
                    comment_id: BeatmapsetCommentId {
                        map_discussion_id: Some(0),
                        map_discussion_post_id: Some(1),
                        mapset_discussion_id: Some(0),
                        mapset_discussion_post_id: Some(1),
                    },
                    old: Genre::HipHop,
                    new: Genre::Unspecified,
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
            },
            BeatmapsetEvent::IssueReopen {
                event_id: 1,
                comment: BeatmapsetCommentId {
                    map_discussion_id: Some(2),
                    map_discussion_post_id: None,
                    mapset_discussion_id: Some(2),
                    mapset_discussion_post_id: None,
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
                discussion: get_mapset_discussion(),
            },
            BeatmapsetEvent::IssueResolve {
                event_id: 2,
                comment: BeatmapsetCommentId {
                    map_discussion_id: None,
                    map_discussion_post_id: Some(3),
                    mapset_discussion_id: None,
                    mapset_discussion_post_id: Some(3),
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
                discussion: get_mapset_discussion(),
            },
            BeatmapsetEvent::KudosuDeny {
                event_id: 8,
                comment: BeatmapsetCommentId {
                    map_discussion_id: None,
                    map_discussion_post_id: Some(3),
                    mapset_discussion_id: None,
                    mapset_discussion_post_id: Some(3),
                },
                created_at: get_date(),
                mapset: get_mapset_compact(),
                discussion: get_mapset_discussion(),
            },
            BeatmapsetEvent::KudosuGain {
                event_id: 3,
                comment: BeatmapsetCommentKudosuGain {
                    comment_id: BeatmapsetCommentId {
                        map_discussion_id: None,
                        map_discussion_post_id: None,
                        mapset_discussion_id: None,
                        mapset_discussion_post_id: None,
                    },
                    new_vote: BeatmapsetVote {
                        user_id: 111_111,
                        score: 42,
                    },
                    votes: vec![BeatmapsetVote {
                        user_id: 222_222,
                        score: 420,
                    }],
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
                discussion: get_mapset_discussion(),
            },
            BeatmapsetEvent::LanguageEdit {
                event_id: 4,
                comment: BeatmapsetCommentEdit {
                    comment_id: BeatmapsetCommentId {
                        map_discussion_id: None,
                        map_discussion_post_id: None,
                        mapset_discussion_id: None,
                        mapset_discussion_post_id: None,
                    },
                    old: Language::Any,
                    new: Language::Polish,
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
            },
            BeatmapsetEvent::Nominate {
                event_id: 5,
                comment: BeatmapsetCommentNominate {
                    modes: vec![
                        GameMode::Osu,
                        GameMode::Taiko,
                        GameMode::Catch,
                        GameMode::Mania,
                    ],
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
            },
            BeatmapsetEvent::NsfwToggle {
                event_id: 6,
                comment: BeatmapsetCommentEdit {
                    comment_id: BeatmapsetCommentId {
                        map_discussion_id: None,
                        map_discussion_post_id: None,
                        mapset_discussion_id: None,
                        mapset_discussion_post_id: None,
                    },
                    old: true,
                    new: false,
                },
                created_at: get_date(),
                user_id: 123456,
                mapset: get_mapset_compact(),
            },
            BeatmapsetEvent::OwnerChange {
                event_id: 9,
                comment: BeatmapsetCommentOwnerChange {
                    map_discussion_id: Some(0),
                    map_discussion_post_id: Some(1),
                    map_id: 123,
                    version: "epic version".to_owned(),
                    new_user_id: 98,
                    new_username: "new name".into(),
                },
                created_at: get_date(),
                user_id: 99,
                mapset: get_mapset_compact(),
            },
            BeatmapsetEvent::Rank {
                event_id: 7,
                created_at: get_date(),
                mapset: get_mapset_compact(),
            },
            BeatmapsetEvent::Qualify {
                event_id: 8,
                created_at: get_date(),
                mapset: get_mapset_compact(),
            },
        ],
        reviews_config: BeatmapsetReviewsConfig { max_blocks: 100 },
        users: vec![get_user_compact()],
    }
}

fn get_match() -> OsuMatch {
    OsuMatch {
        current_game_id: Some(3),
        end_time: Some(get_date()),
        events: vec![
            MatchEvent::Create {
                event_id: 0,
                timestamp: get_date(),
                user_id: Some(0),
            },
            MatchEvent::Joined {
                event_id: 1,
                timestamp: get_date(),
                user_id: 1,
            },
            MatchEvent::Left {
                event_id: 2,
                timestamp: get_date(),
                user_id: 1,
            },
            MatchEvent::HostChanged {
                event_id: 3,
                timestamp: get_date(),
                user_id: 0,
            },
            MatchEvent::Game {
                event_id: 4,
                game: Box::new(MatchGame {
                    game_id: 0,
                    start_time: get_date(),
                    end_time: Some(get_date()),
                    mode: GameMode::Osu,
                    scoring_type: ScoringType::Score,
                    team_type: TeamType::HeadToHead,
                    mods: GameMods::Hidden | GameMods::HardRock,
                    map: Some(get_map_compact()),
                    scores: vec![get_match_score()],
                }),
                match_name: "other name".to_owned(),
                timestamp: get_date(),
            },
            MatchEvent::Disbanded {
                event_id: 5,
                timestamp: get_date(),
            },
        ],
        first_event_id: 0,
        latest_event_id: 1,
        match_id: 0,
        name: "A: B vs C".to_owned(),
        start_time: get_date(),
        users: {
            let mut map = HashMap::new();
            map.insert(3, get_user_compact());

            map
        },
    }
}

fn get_match_score() -> MatchScore {
    MatchScore {
        user_id: 123456,
        accuracy: 99.5,
        mods: GameMods::ScoreV2 | GameMods::Relax,
        score: 12_345_678,
        max_combo: 1000,
        perfect: false,
        statistics: ScoreStatistics {
            count_geki: 0,
            count_300: 1,
            count_katu: 2,
            count_100: 3,
            count_50: 4,
            count_miss: 5,
        },
        slot: 0,
        team: Team::Red,
        pass: true,
    }
}

fn get_score() -> Score {
    Score {
        accuracy: 98.76,
        ended_at: get_date(),
        grade: Grade::A,
        max_combo: 1234,
        map: Some(get_map()),
        mapset: Some(get_mapset_compact()),
        mode: GameMode::Catch,
        mods: GameMods::Hidden | GameMods::DoubleTime,
        perfect: false,
        pp: Some(456.78),
        rank_country: Some(1),
        rank_global: Some(10),
        replay: Some(true),
        score: 12_345_678,
        score_id: Some(123_456_789_000),
        statistics: ScoreStatistics {
            count_geki: 1,
            count_300: 1000,
            count_katu: 2,
            count_100: 300,
            count_50: 200,
            count_miss: 1,
        },
        user: Some(get_user_compact()),
        user_id: 2,
        weight: Some(ScoreWeight {
            percentage: 1.0,
            pp: 456.78,
        }),
    }
}

fn get_seasonal_backgrounds() -> SeasonalBackgrounds {
    SeasonalBackgrounds {
        ends_at: get_date(),
        backgrounds: vec![SeasonalBackground {
            url: "https://www.bing.com".to_owned(),
            artist: get_user_compact(),
        }],
    }
}

fn get_spotlight() -> Spotlight {
    Spotlight {
        end_date: get_date(),
        mode_specific: true,
        name: "epic spotlight".to_owned(),
        participant_count: Some(3),
        spotlight_id: 2,
        spotlight_type: "idk".to_owned(),
        start_date: get_date(),
    }
}

fn get_user() -> User {
    User {
        avatar_url: String::new(),
        comments_count: 0,
        country: "belgiania".to_owned(),
        country_code: "be".into(),
        cover: UserCover {
            custom_url: Some(String::new()),
            url: String::new(),
            id: Some(String::new()),
        },
        default_group: "default".to_owned(),
        discord: Some(String::new()),
        has_supported: true,
        interests: Some(String::new()),
        is_active: true,
        is_bot: false,
        is_deleted: false,
        is_online: true,
        is_supporter: false,
        join_date: get_date(),
        kudosu: UserKudosu {
            available: 1,
            total: 2,
        },
        last_visit: Some(get_date()),
        location: Some(String::new()),
        max_blocks: 0,
        max_friends: 500,
        mode: GameMode::Taiko,
        occupation: Some(String::new()),
        playstyle: Some(vec![Playstyle::Keyboard, Playstyle::Tablet]),
        pm_friends_only: false,
        forum_post_count: 0,
        profile_color: Some(String::new()),
        profile_order: vec![ProfilePage::Me, ProfilePage::TopRanks],
        title: Some(String::new()),
        title_url: Some(String::new()),
        twitter: Some(String::new()),
        user_id: 12345,
        username: "bob".into(),
        website: Some(String::new()),
        account_history: Some(vec![AccountHistory {
            id: Some(1),
            history_type: HistoryType::Note,
            timestamp: get_date(),
            seconds: 2,
        }]),
        badges: Some(vec![Badge {
            awarded_at: get_date(),
            description: "big boi tourney".to_owned(),
            image_url: String::new(),
            url: String::new(),
        }]),
        beatmap_playcounts_count: Some(3),
        favourite_mapset_count: Some(3),
        follower_count: Some(2),
        graveyard_mapset_count: Some(8),
        groups: Some(vec![Group {
            color: Some("#FFFFFF".to_owned()),
            description: Some("epic group".to_owned()),
            has_modes: true,
            id: 1,
            identifier: String::new(),
            is_probationary: true,
            modes: Some(vec![GameMode::Osu, GameMode::Mania]),
            name: "group".to_owned(),
            short_name: "g".to_owned(),
        }]),
        guest_mapset_count: Some(3),
        is_admin: Some(true),
        is_bng: Some(false),
        is_full_bn: Some(true),
        is_gmt: Some(true),
        is_limited_bn: Some(true),
        is_moderator: Some(true),
        is_nat: Some(true),
        is_silenced: Some(true),
        loved_mapset_count: Some(3),
        mapping_follower_count: Some(5),
        monthly_playcounts: Some(vec![MonthlyCount {
            start_date: Date::from_ordinal_date(2017, 1).unwrap(),
            count: 42,
        }]),
        page: Some(UserPage {
            html: String::new(),
            raw: String::new(),
        }),
        previous_usernames: Some(vec!["b0b".into()]),
        rank_history: Some(vec![50, 40, 30, 35]),
        ranked_mapset_count: Some(800),
        replays_watched_counts: Some(vec![MonthlyCount {
            start_date: Date::from_ordinal_date(2017, 1).unwrap(),
            count: 42,
        }]),
        scores_best_count: Some(13),
        scores_first_count: Some(13),
        scores_recent_count: Some(13),
        statistics: Some(get_user_stats()),
        support_level: Some(3),
        pending_mapset_count: Some(13),
        medals: Some(vec![MedalCompact {
            achieved_at: get_date(),
            medal_id: 1,
        }]),
    }
}

fn get_user_compact() -> UserCompact {
    UserCompact {
        avatar_url: String::new(),
        country_code: "be".into(),
        default_group: "default".to_owned(),
        is_active: true,
        is_bot: false,
        is_deleted: false,
        is_online: true,
        is_supporter: true,
        last_visit: Some(get_date()),
        pm_friends_only: false,
        profile_color: Some("#FFFFFF".to_owned()),
        user_id: 12345,
        username: "bob".into(),
        account_history: Some(vec![AccountHistory {
            id: Some(1),
            history_type: HistoryType::Note,
            timestamp: get_date(),
            seconds: 2,
        }]),
        badges: Some(vec![Badge {
            awarded_at: get_date(),
            description: "big boi tourney".to_owned(),
            image_url: String::new(),
            url: String::new(),
        }]),
        beatmap_playcounts_count: Some(3),
        country: Some("belgiania".to_owned()),
        cover: Some(UserCover {
            custom_url: None,
            url: String::new(),
            id: None,
        }),
        favourite_mapset_count: Some(34),
        follower_count: Some(2),
        graveyard_mapset_count: Some(34),
        groups: Some(vec![Group {
            color: Some("#FFFFFF".to_owned()),
            description: Some("epic group".to_owned()),
            has_modes: true,
            id: 1,
            identifier: String::new(),
            is_probationary: true,
            modes: Some(vec![GameMode::Osu, GameMode::Mania]),
            name: "group".to_owned(),
            short_name: "g".to_owned(),
        }]),
        guest_mapset_count: Some(3),
        is_admin: Some(true),
        is_bng: Some(false),
        is_full_bn: Some(true),
        is_gmt: Some(true),
        is_limited_bn: Some(true),
        is_moderator: Some(false),
        is_nat: Some(false),
        is_silenced: Some(false),
        loved_mapset_count: Some(34),
        medals: Some(vec![MedalCompact {
            achieved_at: get_date(),
            medal_id: 1,
        }]),
        monthly_playcounts: Some(vec![MonthlyCount {
            start_date: Date::from_ordinal_date(2017, 1).unwrap(),
            count: 42,
        }]),
        page: Some(UserPage {
            html: String::new(),
            raw: String::new(),
        }),
        previous_usernames: Some(vec!["b0b".into()]),
        rank_history: Some(vec![50, 40, 30, 35]),
        ranked_mapset_count: Some(34),
        replays_watched_counts: Some(vec![MonthlyCount {
            start_date: Date::from_ordinal_date(2017, 1).unwrap(),
            count: 42,
        }]),
        scores_best_count: Some(34),
        scores_first_count: Some(34),
        scores_recent_count: Some(34),
        statistics: Some(get_user_stats()),
        support_level: Some(1),
        pending_mapset_count: Some(34),
    }
}

fn get_user_stats() -> UserStatistics {
    UserStatistics {
        accuracy: 99.11,
        country_rank: Some(1),
        global_rank: Some(1),
        grade_counts: GradeCounts {
            ss: 1,
            ssh: 2,
            s: 3,
            sh: 4,
            a: 5,
        },
        is_ranked: true,
        level: UserLevel {
            current: 101,
            progress: 96,
        },
        max_combo: 6543,
        playcount: 100_000,
        playtime: 10_000_000,
        pp: 9876.54,
        ranked_score: 111_222_333_444,
        replays_watched: 123,
        total_hits: 123_456_789,
        total_score: 111_222_333_444_555,
    }
}

fn get_map_attributes() -> Vec<BeatmapDifficultyAttributes> {
    vec![
        BeatmapDifficultyAttributes {
            max_combo: 1,
            stars: 2.0,
            attrs: GameModeAttributes::Osu {
                ar: 5.55,
                od: 6.66,
                aim_difficulty: 4.44,
                flashlight_difficulty: 3.33,
                slider_factor: 2.22,
                speed_difficulty: 1.11,
            },
        },
        BeatmapDifficultyAttributes {
            max_combo: 3,
            stars: 4.0,
            attrs: GameModeAttributes::Taiko {
                stamina_difficulty: 7.89,
                rhythm_difficulty: 4.56,
                colour_difficulty: 1.23,
                peak_difficulty: 999.99,
                great_hit_window: 10.0,
            },
        },
        BeatmapDifficultyAttributes {
            max_combo: 5,
            stars: 6.0,
            attrs: GameModeAttributes::Mania {
                great_hit_window: 1.0,
                score_multiplier: 3.0,
            },
        },
    ]
}

#[test]
fn serde_beatmap() {
    ser_de(&get_map());
}

#[test]
fn serde_beatmap_attributes() {
    ser_de(&get_map_attributes());
}

#[test]
fn serde_beatmapset_events() {
    ser_de(&get_mapset_events());
}

#[test]
fn serde_chart_rankings() {
    ser_de(&get_chart_rankings());
}

#[test]
fn serde_country_ranking() {
    ser_de(&get_country_ranking());
}

#[test]
fn serde_forum_posts() {
    ser_de(&get_forum_posts());
}

#[test]
fn serde_match() {
    ser_de(&get_match());
}

#[test]
fn serde_score() {
    ser_de(&get_score());
}

#[test]
fn serde_seasonal_backgrounds() {
    ser_de(&get_seasonal_backgrounds());
}

#[test]
fn serde_user() {
    ser_de(&get_user());
}

#[cfg(feature = "rkyv")]
mod rkyv_tests {
    use std::fmt::Debug;

    use ::rkyv::{
        archived_root, ser::serializers::AllocSerializer, to_bytes, Archive, Deserialize,
        Infallible, Serialize,
    };

    use super::*;

    fn ser_de<T>(val: &T)
    where
        T: PartialEq + Debug + Archive + Serialize<AllocSerializer<512>>,
        <T as Archive>::Archived: Deserialize<T, Infallible>,
    {
        let bytes =
            to_bytes::<_, 512>(val).unwrap_or_else(|e| panic!("Failed to serialize: {}", e));
        let archived = unsafe { archived_root::<T>(&bytes) };
        let deserialized =
            <<T as Archive>::Archived as Deserialize<T, _>>::deserialize(archived, &mut Infallible)
                .unwrap_or_else(|e| panic!("Failed to deserialize: {}", e));

        assert_eq!(val, &deserialized);
    }

    #[test]
    fn serde_beatmap() {
        ser_de(&get_map());
    }

    #[test]
    fn serde_beatmap_attributes() {
        ser_de(&get_map_attributes());
    }

    #[test]
    fn serde_beatmapset_events() {
        ser_de(&get_mapset_events());
    }

    #[test]
    fn serde_chart_rankings() {
        ser_de(&get_chart_rankings());
    }

    #[test]
    fn serde_country_ranking() {
        ser_de(&get_country_ranking());
    }

    // TODO
    // #[test]
    // fn serde_forum_posts() {
    //     ser_de(&get_forum_posts());
    // }

    #[test]
    fn serde_match() {
        ser_de(&get_match());
    }

    #[test]
    fn serde_score() {
        ser_de(&get_score());
    }

    #[test]
    fn serde_seasonal_backgrounds() {
        ser_de(&get_seasonal_backgrounds());
    }

    #[test]
    fn serde_user() {
        ser_de(&get_user());
    }
}
