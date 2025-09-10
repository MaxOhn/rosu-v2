extern crate rosu_v2;

mod types {
    #![allow(unused)]

    use ::serde::{de::DeserializeOwned, Serialize};
    use rosu_v2::prelude::*;
    use std::{collections::HashMap, fmt::Debug};
    use time::{Date, Duration, OffsetDateTime};

    pub(super) fn get_chart_rankings() -> ChartRankings {
        ChartRankings {
            mapsets: vec![get_mapset_extended()],
            ranking: vec![User {
                statistics_modes: UserStatisticsModes {
                    osu: None,
                    taiko: None,
                    catch: None,
                    mania: None,
                },
                ..get_user()
            }],
            spotlight: get_spotlight(),
        }
    }

    pub(super) fn get_country_ranking() -> CountryRanking {
        CountryRanking {
            active_users: 2,
            country: "belgiania".to_owned(),
            country_code: "be".into(),
            playcount: 420,
            pp: 123.45,
            ranked_score: 1_000_000_000_000_000,
        }
    }

    pub(super) fn get_date() -> OffsetDateTime {
        let mut now = OffsetDateTime::now_utc();
        now -= Duration::nanoseconds(now.nanosecond() as i64);

        now
    }

    pub(super) fn get_forum_posts() -> ForumPosts {
        ForumPosts {
            cursor: Some("my cursor".to_owned()),
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

    pub(super) fn get_mapset_covers() -> BeatmapsetCovers {
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

    pub(super) fn get_mapset_extended() -> BeatmapsetExtended {
        BeatmapsetExtended {
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
            creator: Some(Box::new(get_user())),
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
                eligible_main_rulesets: Some(vec![GameMode::Taiko, GameMode::Mania]),
                required_meta: BeatmapsetNominationsRequiredMeta {
                    main_mode: GameMode::Osu,
                    non_main_mode: GameMode::Taiko,
                },
            },
            nsfw: true,
            playcount: 0,
            preview_url: "b.ppy.sh/preview/12345.mp3".to_owned(),
            ratings: Some(vec![1, 2, 3, 4, 5, 6]),
            ranked_date: Some(get_date()),
            recent_favourites: Some(vec![get_user()]),
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

    pub(super) fn get_map_extended() -> BeatmapExtended {
        BeatmapExtended {
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
                exit: Some((1..=100).collect::<Vec<_>>().try_into().unwrap()),
                fail: Some((101..=200).collect::<Vec<_>>().try_into().unwrap()),
            }),
            hp: 7.5,
            is_scoreable: true,
            last_updated: get_date(),
            map_id: 123456,
            mapset: Some(Box::new(get_mapset_extended())),
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

    pub(super) fn get_map() -> Beatmap {
        Beatmap {
            checksum: Some("ABC123".to_owned()),
            creator_id: 456,
            fail_times: None,
            map_id: 123456,
            mapset: Some(Box::new(get_mapset())),
            mapset_id: 2345,
            max_combo: Some(1000),
            mode: GameMode::Catch,
            seconds_total: 120,
            stars: 5.5,
            status: RankStatus::Loved,
            version: "HIAAAA".to_owned(),
        }
    }

    pub(super) fn get_mapset() -> Beatmapset {
        Beatmapset {
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

    pub(super) fn get_mapset_discussion() -> BeatmapsetDiscussion {
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

    pub(super) fn get_mapset_events() -> BeatmapsetEvents {
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
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
                    mapset: Box::new(get_mapset()),
                },
                BeatmapsetEvent::Rank {
                    event_id: 7,
                    created_at: get_date(),
                    mapset: Box::new(get_mapset()),
                },
                BeatmapsetEvent::Qualify {
                    event_id: 8,
                    created_at: get_date(),
                    mapset: Box::new(get_mapset()),
                },
            ],
            reviews_config: BeatmapsetReviewsConfig { max_blocks: 100 },
            users: vec![get_user()],
        }
    }

    pub(super) fn get_match() -> OsuMatch {
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
                        mods: [
                            GameMod::HiddenOsu(HiddenOsu {
                                only_fade_approach_circles: Some(true),
                            }),
                            GameMod::HardRockOsu(HardRockOsu {}),
                        ]
                        .into_iter()
                        .collect(),
                        map: Some(Box::new(get_map())),
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
                map.insert(3, get_user());

                map
            },
        }
    }

    pub(super) fn get_match_score() -> MatchScore {
        MatchScore {
            set_on_lazer: false,
            mods: [
                GameMod::DifficultyAdjustCatch(DifficultyAdjustCatch {
                    circle_size: Some(1.0),
                    approach_rate: None,
                    hard_rock_offsets: Some(false),
                    drain_rate: None,
                    overall_difficulty: Some(10.0),
                    extended_limits: None,
                }),
                GameMod::HardRockCatch(HardRockCatch {}),
            ]
            .into_iter()
            .collect(),
            statistics: ScoreStatistics {
                miss: 1,
                great: 1000,
                large_tick_hit: 300,
                small_tick_hit: 2,
                small_tick_miss: 1,
                meh: 0,
                ok: 0,
                good: 0,
                perfect: 0,
                large_tick_miss: 0,
                ignore_hit: 0,
                ignore_miss: 0,
                large_bonus: 0,
                small_bonus: 0,
                slider_tail_hit: 0,
                combo_break: 0,
                legacy_combo_increase: 0,
            },
            maximum_statistics: ScoreStatistics {
                miss: 0,
                great: 1100,
                large_tick_hit: 350,
                small_tick_hit: 5,
                small_tick_miss: 0,
                meh: 0,
                ok: 0,
                good: 0,
                perfect: 0,
                large_tick_miss: 0,
                ignore_hit: 0,
                ignore_miss: 0,
                large_bonus: 0,
                small_bonus: 0,
                slider_tail_hit: 0,
                combo_break: 0,
                legacy_combo_increase: 0,
            },
            map_id: 53,
            best_id: None,
            id: Some(123),
            grade: Grade::D,
            kind: Box::from("legacy_match_score"),
            user_id: 123456,
            accuracy: 99.5,
            build_id: None,
            ended_at: get_date(),
            has_replay: false,
            is_perfect_combo: false,
            legacy_perfect: Some(true),
            legacy_score_id: Some(123),
            legacy_score: 765_432_198,
            max_combo: 1000,
            passed: false,
            pp: Some(123.45),
            started_at: None,
            score: 12_345_678,
            replay: true,
            current_user_attributes: UserAttributes { pin: None },
            info: MatchScoreInfo {
                slot: 0,
                team: MatchTeam::Red,
                pass: true,
            },
        }
    }

    pub(super) fn get_room() -> Room {
        Room {
            room_id: 123,
            name: "room name".to_owned(),
            description: Some("room description".to_owned()),
            category: RoomCategory::FeaturedArtist,
            status: RoomStatus::Playing,
            type_group: RoomTypeGroup::TagTeamVersus,
            user_id: 456,
            starts_at: get_date(),
            ends_at: Some(get_date()),
            max_attempts: Some(2),
            participant_count: 9001,
            channel_id: Some(1234),
            active: true,
            has_password: false,
            queue_mode: RoomQueueMode::AllPlayersRoundRobin,
            auto_skip: false,
            pinned: true,
            current_playlist_item: Some(get_playlist_item()),
            host: get_user(),
            recent_participants: vec![get_user()],
            playlist_item_stats: Some(PlaylistItemStats {
                count_active: 4,
                count_total: 5,
                modes: vec![GameMode::Taiko, GameMode::Mania],
            }),
            difficulty_range: Some(RoomDifficultyRange {
                min: 12.34,
                max: 3.1419,
            }),
            playlist: vec![get_playlist_item()],
        }
    }

    pub(super) fn get_playlist_item() -> PlaylistItem {
        PlaylistItem {
            map: get_map(),
            map_id: 123,
            created_at: Some(get_date()),
            playlist_item_id: 123,
            owner_id: 456,
            room_id: 789,
            mode: GameMode::Catch,
            freestyle: true,
            expired: false,
            played_at: Some(get_date()),
            playlist_order: Some(0),
            allowed_mods: [
                GameMod::DifficultyAdjustCatch(DifficultyAdjustCatch {
                    circle_size: Some(1.0),
                    approach_rate: None,
                    hard_rock_offsets: Some(false),
                    drain_rate: None,
                    overall_difficulty: Some(10.0),
                    extended_limits: None,
                }),
                GameMod::HardRockCatch(HardRockCatch {}),
            ]
            .into_iter()
            .collect(),
            required_mods: GameMods::new(),
            scores: vec![get_score()],
        }
    }

    pub(super) fn get_score() -> Score {
        Score {
            set_on_lazer: true,
            classic_score: 765_432_199,
            ranked: Some(true),
            preserve: Some(true),
            processed: Some(false),
            accuracy: 98.76,
            ended_at: get_date(),
            passed: true,
            grade: Grade::A,
            map_id: 123,
            max_combo: 1234,
            total_score_without_mods: Some(987_654),
            map: Some(Box::new(get_map_extended())),
            mapset: Some(Box::new(get_mapset())),
            mode: GameMode::Catch,
            mods: [
                GameMod::DifficultyAdjustCatch(DifficultyAdjustCatch {
                    circle_size: Some(1.0),
                    approach_rate: None,
                    hard_rock_offsets: Some(false),
                    drain_rate: None,
                    overall_difficulty: Some(10.0),
                    extended_limits: None,
                }),
                GameMod::HardRockCatch(HardRockCatch {}),
            ]
            .into_iter()
            .collect(),
            is_perfect_combo: false,
            legacy_perfect: Some(false),
            pp: Some(456.78),
            rank_global: Some(10),
            replay: true,
            score: 12_345_678,
            best_id: Some(123_456_789_000),
            id: 4_444_444,
            statistics: ScoreStatistics {
                miss: 1,
                great: 1000,
                large_tick_hit: 300,
                small_tick_hit: 2,
                small_tick_miss: 1,
                meh: 0,
                ok: 0,
                good: 0,
                perfect: 0,
                large_tick_miss: 0,
                ignore_hit: 0,
                ignore_miss: 0,
                large_bonus: 0,
                small_bonus: 0,
                slider_tail_hit: 0,
                combo_break: 0,
                legacy_combo_increase: 0,
            },
            user: Some(Box::new(get_user())),
            user_id: 2,
            weight: Some(ScoreWeight {
                percentage: 1.0,
                pp: 456.78,
            }),
            maximum_statistics: ScoreStatistics {
                miss: 0,
                great: 1100,
                large_tick_hit: 350,
                small_tick_hit: 5,
                small_tick_miss: 0,
                meh: 0,
                ok: 0,
                good: 0,
                perfect: 0,
                large_tick_miss: 0,
                ignore_hit: 0,
                ignore_miss: 0,
                large_bonus: 0,
                small_bonus: 0,
                slider_tail_hit: 0,
                combo_break: 0,
                legacy_combo_increase: 0,
            },
            kind: "solo_score".into(),
            build_id: Some(678),
            has_replay: true,
            legacy_score_id: Some(123),
            legacy_score: 765_432_198,
            started_at: Some(get_date()),
            current_user_attributes: UserAttributes { pin: None },
            playlist_item_id: Some(999),
            room_id: Some(888),
        }
    }

    pub(super) fn get_seasonal_backgrounds() -> SeasonalBackgrounds {
        SeasonalBackgrounds {
            ends_at: get_date(),
            backgrounds: vec![SeasonalBackground {
                url: "https://www.bing.com".to_owned(),
                artist: get_user(),
            }],
        }
    }

    pub(super) fn get_spotlight() -> Spotlight {
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

    pub(super) fn get_team() -> Team {
        Team {
            flag_url: None,
            id: 1024,
            name: "Traceable Enjoyer".to_owned(),
            short_name: "TC".to_owned(),
        }
    }

    pub(super) fn get_user_extended() -> UserExtended {
        UserExtended {
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
            team: Some(get_team()),
            title: Some(String::new()),
            title_url: Some(String::new()),
            twitter: Some(String::new()),
            user_id: 12345,
            username: "bob".into(),
            website: Some(String::new()),
            account_history: Some(vec![AccountHistory {
                id: Some(1),
                description: Some("abc".to_owned()),
                history_type: HistoryType::Note,
                timestamp: get_date(),
                seconds: 2,
                permanent: false,
            }]),
            badges: Some(vec![Badge {
                awarded_at: get_date(),
                description: "big boi tourney".to_owned(),
                image_url: String::new(),
                url: String::new(),
            }]),
            beatmap_playcounts_count: Some(3),
            daily_challenge_stats: DailyChallengeUserStatistics {
                daily_streak_best: 123,
                daily_streak_current: 123,
                last_update: Some(get_date()),
                last_weekly_streak: Some(get_date()),
                playcount: 123,
                top_10p_placements: 123,
                top_50p_placements: 123,
                user_id: 123,
                weekly_streak_best: 123,
                weekly_streak_current: 123,
            },
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
            highest_rank: Some(get_highest_rank()),
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
            statistics_modes: UserStatisticsModes {
                osu: Some(get_user_stats()),
                taiko: None,
                catch: Some(get_user_stats()),
                mania: Some(get_user_stats()),
            },
            support_level: Some(3),
            pending_mapset_count: Some(13),
            medals: Some(vec![MedalCompact {
                achieved_at: get_date(),
                medal_id: 1,
            }]),
        }
    }

    pub(super) fn get_user() -> User {
        User {
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
                description: None,
                history_type: HistoryType::Note,
                timestamp: get_date(),
                seconds: 0,
                permanent: true,
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
            highest_rank: Some(get_highest_rank()),
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
            statistics_modes: UserStatisticsModes {
                osu: Some(get_user_stats()),
                taiko: None,
                catch: Some(get_user_stats()),
                mania: Some(get_user_stats()),
            },
            support_level: Some(1),
            pending_mapset_count: Some(34),
            team: Some(get_team()),
        }
    }

    pub(super) fn get_highest_rank() -> UserHighestRank {
        UserHighestRank {
            rank: 123,
            updated_at: get_date(),
        }
    }

    pub(super) fn get_user_stats() -> UserStatistics {
        UserStatistics {
            accuracy: 99.11,
            count_300: 12,
            count_100: 34,
            count_50: 56,
            count_miss: 78,
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
            rank_change_since_30_days: Some(42),
            replays_watched: 123,
            total_hits: 123_456_789,
            total_score: 111_222_333_444_555,
        }
    }

    pub(super) fn get_map_attributes() -> Vec<BeatmapDifficultyAttributes> {
        vec![
            BeatmapDifficultyAttributes {
                max_combo: 1,
                stars: 2.0,
                attrs: Some(GameModeAttributes::Osu {
                    aim_difficulty: 4.44,
                    slider_factor: 2.22,
                    speed_difficulty: 1.11,
                    speed_note_count: 77.7777,
                    aim_difficult_slider_count: 88.8888,
                    aim_difficult_strain_count: 99.9999,
                    speed_difficult_strain_count: 66.6666,
                }),
            },
            BeatmapDifficultyAttributes {
                max_combo: 3,
                stars: 4.0,
                attrs: Some(GameModeAttributes::Taiko {
                    stamina_difficulty: 7.89,
                    rhythm_difficulty: 4.56,
                    colour_difficulty: 1.23,
                    reading_difficulty: 3.21,
                    mono_stamina_factor: 0.0123,
                    rhythm_difficult_strains: 0.0456,
                    colour_difficult_strains: 0.0789,
                    stamina_difficult_strains: 0.0321,
                }),
            },
            BeatmapDifficultyAttributes {
                max_combo: 5,
                stars: 6.0,
                attrs: None,
            },
        ]
    }
}

#[cfg(feature = "serialize")]
mod serde_tests {
    use serde::{de::DeserializeOwned, Serialize};

    use super::types::*;

    fn roundtrip<T>(val: &T)
    where
        T: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
    {
        let serialized =
            serde_json::to_string(val).unwrap_or_else(|e| panic!("Failed to serialize: {}", e));

        let deserialized: T = serde_json::from_str(&serialized)
            .unwrap_or_else(|e| panic!("Failed to deserialize: {}\n{serialized}", e));

        assert_eq!(val, &deserialized);
    }

    #[test]
    fn serde_beatmap() {
        roundtrip(&get_map());
    }

    #[test]
    fn serde_beatmap_attributes() {
        roundtrip(&get_map_attributes());
    }

    #[test]
    fn serde_beatmapset_events() {
        roundtrip(&get_mapset_events());
    }

    #[test]
    fn serde_chart_rankings() {
        roundtrip(&get_chart_rankings());
    }

    #[test]
    fn serde_country_ranking() {
        roundtrip(&get_country_ranking());
    }

    #[test]
    fn serde_forum_posts() {
        roundtrip(&get_forum_posts());
    }

    #[test]
    fn serde_match() {
        roundtrip(&get_match());
    }

    #[test]
    fn serde_room() {
        roundtrip(&get_room());
    }

    #[test]
    fn serde_score() {
        roundtrip(&get_score());
    }

    #[test]
    fn serde_seasonal_backgrounds() {
        roundtrip(&get_seasonal_backgrounds());
    }

    #[test]
    fn serde_user() {
        roundtrip(&get_user());
    }
}
