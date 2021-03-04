extern crate rosu_v2;

use chrono::{DateTime, TimeZone, Utc};
use rosu_v2::model::*;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

fn ser_de<T: DeserializeOwned + Serialize + PartialEq + Debug>(val: T) {
    let serialized = serde_json::to_string(&val).expect("failed to serialize");
    let deserialized: T = serde_json::from_str(&serialized).expect("failed to deserialize");

    assert_eq!(val, deserialized);
}

fn get_date() -> DateTime<Utc> {
    Utc.timestamp(1_500_000_000, 0)
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
        covers: get_mapset_covers(),
        creator: "god".to_owned(),
        creator_id: 2,
        discussion_enabled: true,
        discussion_locked: false,
        favourite_count: 1_111_111,
        hype: Some(BeatmapsetHype {
            current: 1,
            required: 2,
        }),
        is_scoreable: true,
        last_updated: get_date(),
        legacy_thread_url: None,
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
        ranked_date: None,
        source: String::new(),
        status: RankStatus::WIP,
        storyboard: true,
        submitted_date: None,
        tags: "tags".to_owned(),
        title: "title".to_owned(),
        title_unicode: None,
        video: false,
    }
}

fn get_map() -> Beatmap {
    Beatmap {
        ar: 9.3,
        bpm: 182.3,
        checksum: None,
        convert: false,
        count_circles: 1234,
        count_sliders: 123,
        count_spinners: 1,
        cs: 4.1,
        deleted_at: None,
        fail_times: Some(FailTimes {
            exit: Some(vec![1, 2, 3]),
            fail: Some(vec![4, 5, 6]),
        }),
        hp: 7.5,
        is_scoreable: true,
        last_updated: get_date(),
        map_id: 123456,
        mapset: Some(Mapset::Full(get_mapset())),
        mapset_id: 12345,
        max_combo: Some(1750),
        mode: GameMode::STD,
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

fn get_mapset_compact() -> BeatmapsetCompact {
    BeatmapsetCompact {
        artist: "artist".to_owned(),
        artist_unicode: Some("äöü".to_owned()),
        covers: get_mapset_covers(),
        creator: "god".to_owned(),
        creator_id: 2,
        favourite_count: 1_234_567,
        hype: Some(BeatmapsetHype {
            current: 1,
            required: 2,
        }),
        mapset_id: 12345,
        nsfw: false,
        playcount: 56_789,
        preview_url: "b.ppy.sh/preview/12345.mp3".to_owned(),
        source: String::new(),
        status: RankStatus::Graveyard,
        title: "title".to_owned(),
        title_unicode: None,
        video: true,
    }
}

fn get_score() -> Score {
    Score {
        accuracy: 98.76,
        best_id: None,
        created_at: get_date(),
        grade: Grade::A,
        max_combo: 1234,
        map: Some(get_map()),
        mapset: Some(get_mapset_compact()),
        mode: GameMode::CTB,
        mods: GameMods::Hidden | GameMods::DoubleTime,
        perfect: false,
        pp: Some(456.78),
        rank_country: Some(1),
        rank_global: Some(10),
        replay: true,
        score: 12_345_678,
        score_id: 123_456_789_000,
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

fn get_user() -> User {
    User {
        avatar_url: String::new(),
        comments_count: 0,
        cover_url: String::new(),
        country: Country {
            code: "be".to_owned(),
            name: "belgiania".to_owned(),
        },
        country_code: "be".to_owned(),
        cover: UserCover {
            custom_url: None,
            url: String::new(),
            id: None,
        },
        default_group: "default".to_owned(),
        discord: None,
        has_supported: true,
        interests: None,
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
        location: None,
        max_blocks: 0,
        max_friends: 500,
        mode: GameMode::TKO,
        occupation: None,
        playstyle: Some(vec![Playstyle::Keyboard, Playstyle::Tablet]),
        pm_friends_only: false,
        forum_post_count: 0,
        profile_color: Some(String::new()),
        profile_order: vec![ProfilePage::Me, ProfilePage::TopRanks],
        skype: None,
        title: None,
        title_url: None,
        twitter: None,
        user_id: 12345,
        username: "bob".to_owned(),
        website: None,
        account_history: Some(vec![AccountHistory {
            id: 1,
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
        beatmap_playcounts_count: (None),
        favourite_beatmapset_count: None,
        follower_count: None,
        graveyard_beatmapset_count: None,
        groups: Some(vec![Group {
            id: 1,
            identifier: String::new(),
            is_probationary: true,
            name: "group".to_owned(),
            short_name: "g".to_owned(),
            description: "epic group".to_owned(),
            color: "#FFFFFF".to_owned(),
            modes: Some(vec![GameMode::STD, GameMode::MNA]),
        }]),
        is_admin: Some(true),
        is_bng: Some(false),
        is_full_bn: None,
        is_gmt: None,
        is_limited_bn: None,
        is_moderator: None,
        is_nat: None,
        is_restricted: None,
        is_silenced: None,
        loved_beatmapset_count: None,
        mapping_follower_count: None,
        monthly_playcounts: Some(vec![MonthlyCount {
            start_date: Utc.ymd(2017, 01, 01),
            count: 42,
        }]),
        page: Some(UserPage {
            html: String::new(),
            raw: String::new(),
        }),
        previous_usernames: Some(vec!["b0b".to_owned()]),
        rank_history: Some(vec![50, 40, 30, 35]),
        ranked_and_approved_beatmapset_count: None,
        replays_watched_counts: Some(vec![MonthlyCount {
            start_date: Utc.ymd(2017, 01, 01),
            count: 42,
        }]),
        scores_best_count: None,
        scores_first_count: None,
        scores_recent_count: None,
        statistics: Some(get_user_stats()),
        support_level: None,
        unranked_beatmapset_count: None,
        unread_pm_count: None,
        medals: Some(vec![MedalCompact {
            achieved_at: get_date(),
            medal_id: 1,
        }]),
    }
}

fn get_user_compact() -> UserCompact {
    UserCompact {
        avatar_url: String::new(),
        country_code: "be".to_owned(),
        default_group: "default".to_owned(),
        is_active: true,
        is_bot: false,
        is_deleted: false,
        is_online: true,
        is_supporter: true,
        last_visit: None,
        pm_friends_only: false,
        profile_color: None,
        user_id: 12345,
        username: "bob".to_owned(),
        account_history: Some(vec![AccountHistory {
            id: 1,
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
        beatmap_playcounts_count: None,
        country: Some(Country {
            code: "be".to_owned(),
            name: "belgiania".to_owned(),
        }),
        cover: Some(UserCover {
            custom_url: None,
            url: String::new(),
            id: None,
        }),
        favourite_beatmapset_count: None,
        follower_count: Some(2),
        graveyard_beatmapset_count: None,
        groups: Some(vec![Group {
            id: 1,
            identifier: String::new(),
            is_probationary: true,
            name: "group".to_owned(),
            short_name: "g".to_owned(),
            description: "epic group".to_owned(),
            color: "#FFFFFF".to_owned(),
            modes: Some(vec![GameMode::STD, GameMode::MNA]),
        }]),
        is_admin: Some(true),
        is_bng: Some(false),
        is_full_bn: None,
        is_gmt: None,
        is_limited_bn: None,
        is_moderator: None,
        is_nat: None,
        is_restricted: None,
        is_silenced: None,
        loved_beatmapset_count: None,
        monthly_playcounts: Some(vec![MonthlyCount {
            start_date: Utc.ymd(2017, 01, 01),
            count: 42,
        }]),
        page: Some(UserPage {
            html: String::new(),
            raw: String::new(),
        }),
        previous_usernames: Some(vec!["b0b".to_owned()]),
        rank_history: Some(vec![50, 40, 30, 35]),
        ranked_and_approved_beatmapset_count: None,
        replays_watched_counts: None,
        scores_best_count: None,
        scores_first_count: None,
        scores_recent_count: None,
        statistics: Some(get_user_stats()),
        support_level: None,
        unranked_beatmapset_count: None,
        unread_pm_count: None,
    }
}

fn get_user_stats() -> UserStatistics {
    UserStatistics {
        accuracy: 99.11,
        country_rank: Some(1),
        global_rank: 1,
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
        user: None,
    }
}

#[test]
fn serde_beatmap() {
    ser_de(get_map());
}

#[test]
fn serde_score() {
    ser_de(get_score());
}

#[test]
fn serde_user() {
    ser_de(get_user());
}
