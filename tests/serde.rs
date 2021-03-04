extern crate rosu_v2;

use chrono::{TimeZone, Utc};
use rosu_v2::model::*;

#[test]
fn serde_beatmap() {
    let map = Beatmap {
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
        last_updated: Utc.timestamp(1_500_000_000, 0),
        map_id: 123456,
        mapset: Some(Mapset::Compact(BeatmapsetCompact {
            artist: "artist".to_owned(),
            artist_unicode: Some("äöü".to_owned()),
            covers: BeatmapsetCovers {
                cover: "some str".to_owned(),
                cover_2x: "some str".to_owned(),
                card: "some str".to_owned(),
                card_2x: "some str".to_owned(),
                list: "some str".to_owned(),
                list_2x: "some str".to_owned(),
                slim_cover: "some str".to_owned(),
                slim_cover_2x: "some str".to_owned(),
            },
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
        })),
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
    };

    let serialized = serde_json::to_string(&map).expect("failed to serialize map");
    let deserialized: Beatmap =
        serde_json::from_str(&serialized).expect("failed to deserialize map");

    assert_eq!(map, deserialized);
}
