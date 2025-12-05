extern crate rosu_v2;

use std::{env, time::Duration};

use dotenvy::dotenv;
use eyre::{Result, WrapErr};

use rosu_v2::{
    Osu, model::{
        GameMode, beatmap::{BeatmapsetSearchSort, RankStatus}, event::EventSort
    }, prelude::{PlaylistScoresSort, RoomCategory, RoomTypeGroup, UserBeatmapsetsKind}, request::{RoomsFilter, RoomsTypeGroup}
};
use serial_test::serial;
use tokio::time::sleep;
use tracing_subscriber::{fmt::TestWriter, EnvFilter};

async fn osu() -> Result<Osu> {
    let _ = tracing_subscriber::fmt()
        .with_writer(TestWriter::new())
        .with_env_filter(EnvFilter::builder().parse("rosu_v2=trace,info").unwrap())
        .try_init();

    dotenv().ok();

    let client_id = env::var("CLIENT_ID")
        .expect("missing CLIENT_ID")
        .parse()
        .wrap_err("failed to parse client id as u64")?;

    let client_secret = env::var("CLIENT_SECRET").wrap_err("missing CLIENT_SECRET")?;

    // Preventing 429s
    sleep(Duration::from_secs(1)).await;

    Osu::builder()
        .client_id(client_id)
        .client_secret(client_secret)
        .build()
        .await
        .wrap_err("Failed to build osu! client")
}

const ADESSO_BALLA: u32 = 171024;
const BREEZEBLOCKS: u32 = 3187415;

const HIKOUI_GUMO: u32 = 357161;

const BADEWANNE3: u32 = 2211396;
const SYLAS: u32 = 3906405;

const DE_VS_CA: u32 = 71028303;

const COOKIEZI_FREEDOM_DIVE: u64 = 2177560145;

#[tokio::test]
#[serial]
async fn beatmap() -> Result<()> {
    let map = osu().await?.beatmap().map_id(ADESSO_BALLA).await?;

    println!(
        "Received {} - {}",
        map.mapset.as_ref().unwrap().artist,
        map.mapset.as_ref().unwrap().title,
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmap_difficulty_attributes() -> Result<()> {
    let attrs = osu().await?;

    for mode in [
        GameMode::Osu,
        GameMode::Taiko,
        GameMode::Catch,
        GameMode::Mania,
    ] {
        let attrs = attrs
            .beatmap_difficulty_attributes(ADESSO_BALLA)
            .mode(mode)
            .await?;
        println!("{:?}", attrs.attrs);
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmaps() -> Result<()> {
    let maps = osu().await?.beatmaps([ADESSO_BALLA, BREEZEBLOCKS]).await?;

    println!("Received {} maps", maps.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmap_scores() -> Result<()> {
    let scores = osu().await?.beatmap_scores(ADESSO_BALLA).await?;

    println!(
        "Received {}/{} scores",
        scores.scores.len(),
        scores.score_count
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmap_user_score() -> Result<()> {
    use rosu_v2::model::mods::{GameModIntermode, GameModsIntermode};

    let mods = [
        GameModIntermode::Hidden,
        GameModIntermode::HardRock,
        GameModIntermode::HalfTime,
    ]
    .into_iter()
    .collect::<GameModsIntermode>();

    let score = osu()
        .await?
        .beatmap_user_score(ADESSO_BALLA, BADEWANNE3)
        .mods(mods)
        .await?;

    println!(
        "Received score, pos={} | mods={}",
        score.pos, score.score.mods,
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmap_user_scores() -> Result<()> {
    let scores = osu()
        .await?
        .beatmap_user_scores(ADESSO_BALLA, BADEWANNE3)
        .await?;

    println!("Received {} scores", scores.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmapset() -> Result<()> {
    let mapset = osu().await?.beatmapset(HIKOUI_GUMO).await?;
    println!("Received mapset with {} maps", mapset.maps.unwrap().len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmapset_from_map_id() -> Result<()> {
    let mapset = osu().await?.beatmapset_from_map_id(ADESSO_BALLA).await?;

    println!("Received mapset with {} maps", mapset.maps.unwrap().len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmapset_events() -> Result<()> {
    let events = osu().await?.beatmapset_events().await?;
    println!(
        "Received {} events, {} users",
        events.events.len(),
        events.users.len(),
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn beatmapset_search() -> Result<()> {
    let osu = osu().await?;

    let search_result = osu
        .beatmapset_search()
        .query("artist=camellia stars>8 ar>9 length<400")
        .status(Some(RankStatus::Graveyard))
        .mode(GameMode::Osu)
        .converts(true)
        .featured_artists(true)
        .page(2)
        .recommended(false)
        .nsfw(false)
        .sort(BeatmapsetSearchSort::Favourites, false)
        .await?;

    println!(
        "Received search result containing {} out of {} mapsets",
        search_result.mapsets.len(),
        search_result.total,
    );

    let first_mapset_id = search_result.mapsets[0].mapset_id;

    let search_result = search_result.get_next(&osu).await.unwrap()?;

    println!(
        "Received next search result containing {} out of {} mapsets",
        search_result.mapsets.len(),
        search_result.total,
    );

    let next_mapset_id = search_result.mapsets[0].mapset_id;
    assert_ne!(first_mapset_id, next_mapset_id);

    let search_result = osu
        .beatmapset_search()
        .query("artist='definitely no such name abc'")
        .page(2)
        .await?;

    println!(
        "Received search result containing {} out of {} mapsets",
        search_result.mapsets.len(),
        search_result.total,
    );

    assert!(!search_result.has_more());

    Ok(())
}

#[tokio::test]
#[serial]
async fn comments() -> Result<()> {
    let bundle = osu().await?.comments().sort_new().await?;
    println!(
        "Received bundle, {} comments | {} users",
        bundle.comments.len(),
        bundle.users.len(),
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn chart_rankings() -> Result<()> {
    let rankings = osu().await?.chart_rankings(GameMode::Osu).await?;

    println!(
        "Received a spotlight with {} mapsets and {} statistics",
        rankings.mapsets.len(),
        rankings.ranking.len(),
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn country_rankings() -> Result<()> {
    let countries = osu().await?.country_rankings(GameMode::Osu).await?;

    println!(
        "Received the first {} out of {} countries",
        countries.ranking.len(),
        countries.total
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn events() -> Result<()> {
    let osu = osu().await?;

    let initial = osu.events().sort(EventSort::IdAscending).await?;
    println!("Initial ascending events: {}", initial.events.len());

    let next = initial.get_next(&osu).await.unwrap()?;
    println!("Next ascending events: {}", next.events.len());

    assert!(initial.events.last().unwrap().event_id < next.events.first().unwrap().event_id);

    let initial = osu.events().await?;
    println!("Initial descending events: {}", initial.events.len());

    let next = osu
        .events()
        .cursor(initial.cursor.as_deref().unwrap())
        .sort(EventSort::IdDescending)
        .await?;
    println!("Next descending events: {}", next.events.len());

    assert!(initial.events.last().unwrap().event_id > next.events.first().unwrap().event_id);

    Ok(())
}

#[tokio::test]
#[serial]
async fn forum_posts() -> Result<()> {
    let posts = osu()
        .await?
        .forum_posts(1265690)
        .sort_descending()
        .limit(10)
        .await?;

    println!("Received {} posts", posts.posts.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn recent_activity() -> Result<()> {
    let events = osu()
        .await?
        .recent_activity("badewanne3")
        .limit(10)
        .offset(2)
        .await?;

    println!("Received {} events", events.len());

    Ok(())
}

#[cfg(feature = "replay")]
#[tokio::test]
#[serial]
#[ignore = "requires OAuth to not throw an error"]
async fn replay() -> Result<()> {
    let replay = osu().await?.replay(COOKIEZI_FREEDOM_DIVE).await?;

    println!("Received replay with the following score: {}", replay.score);

    Ok(())
}

#[tokio::test]
#[serial]
async fn kudosu() -> Result<()> {
    let history = osu().await?.kudosu(SYLAS).limit(5).offset(1).await?;
    let sum: i32 = history.iter().map(|entry| entry.amount).sum();

    println!("Received {} entries amounting to {}", history.len(), sum);

    Ok(())
}

#[tokio::test]
#[serial]
async fn news() -> Result<()> {
    let news = osu().await?.news().await?;
    println!("Received news, got {} posts", news.posts.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn osu_match() -> Result<()> {
    let osu_match = osu().await?.osu_match(DE_VS_CA).await?;
    println!(
        "Received match, got {} events and {} users",
        osu_match.events.len(),
        osu_match.users.len()
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn osu_matches() -> Result<()> {
    let osu_matches = osu().await?.osu_matches().await?;
    println!("Received {} matches", osu_matches.matches.len());

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "requires OAuth to not throw an error"]
async fn own_data() -> Result<()> {
    let user = osu().await?.own_data().mode(GameMode::Taiko).await?;

    println!(
        "Received own data showing a last activity of {:?}",
        user.last_visit
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn performance_rankings() -> Result<()> {
    let rankings = osu()
        .await?
        .performance_rankings(GameMode::Osu)
        .country("be")
        .await?;

    println!(
        "Received performance rankings with {} out of {} users",
        rankings.ranking.len(),
        rankings.total
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn playlist_scores() -> Result<()> {
    let osu = osu().await?;

    let room = osu
        .rooms()
        .category(RoomCategory::DailyChallenge)
        .filter(RoomsFilter::Active)
        .await?
        .pop()
        .unwrap();

    let playlist_item_id = room.current_playlist_item.unwrap().playlist_item_id;

    let scores = osu
        .playlist_scores(room.room_id, playlist_item_id)
        .limit(2)
        .sort(PlaylistScoresSort::Ascending)
        .await?;

    let next = scores.get_next(&osu).await.unwrap()?;

    println!(
        "Got {} and {} scores",
        scores.scores.len(),
        next.scores.len()
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn rooms() -> Result<()> {
    let osu = osu().await?;

    let params = [
        (
            RoomCategory::DailyChallenge,
            RoomsFilter::All,
            RoomsTypeGroup::Playlists,
        ),
        (
            RoomCategory::Normal,
            RoomsFilter::Active,
            RoomsTypeGroup::Realtime,
        ),
        (
            RoomCategory::FeaturedArtist,
            RoomsFilter::Ended,
            RoomsTypeGroup::Playlists,
        ),
    ];

    for (category, filter, type_group) in params {
        let rooms = osu
            .rooms()
            .category(category)
            .limit(10)
            .filter(filter)
            .type_group(type_group)
            .await?;

        for room in rooms.iter() {
            assert_eq!(room.category, category);

            match type_group {
                RoomsTypeGroup::Playlists => assert_eq!(room.type_group, RoomTypeGroup::Playlists),
                RoomsTypeGroup::Realtime => assert_ne!(room.type_group, RoomTypeGroup::Playlists),
                _ => {}
            }
        }

        println!(
            "Received {} rooms for \
            category={category:?}, filter={filter:?}, type_group={type_group:?}",
            rooms.len()
        );
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn room() -> Result<()> {
    let room = osu().await?.room(1403108).await?;
    println!("Received room {:?}", room.name);

    Ok(())
}

#[tokio::test]
#[serial]
async fn room_leaderboard() -> Result<()> {
    let res = osu().await?.room_leaderboard(1403108).await?;
    println!("Received {} room scores", res.leaderboard.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn score() -> Result<()> {
    let score = osu().await?.score(COOKIEZI_FREEDOM_DIVE).await?;

    println!(
        "Received {}'s FREEDOM DIVE score",
        score.user.unwrap().username
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn scores() -> Result<()> {
    let osu = osu().await?;

    let scores = osu.scores().mode(GameMode::Osu).await?;
    assert!(scores.scores.len() > 500);
    scores
        .scores
        .iter()
        .for_each(|score| assert_eq!(score.mode, GameMode::Osu));

    let next = scores.get_next(&osu).await?;
    assert!(next.scores.len() < 150, "got {} scores", next.scores.len());
    next.scores
        .iter()
        .for_each(|score| assert_eq!(score.mode, GameMode::Osu));

    println!(
        "Received {} and then {} scores",
        scores.scores.len(),
        next.scores.len()
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn score_rankings() -> Result<()> {
    let osu = osu().await?;

    let global_rankings = osu.score_rankings(GameMode::Osu).await?;

    println!(
        "Received global score rankings with {} out of {} users",
        global_rankings.ranking.len(),
        global_rankings.total
    );

    let national_rankings = osu.score_rankings(GameMode::Osu).country("BE").await?;

    println!(
        "Received national score rankings with {} out of {} users",
        national_rankings.ranking.len(),
        national_rankings.total
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn seasonal_backgrounds() -> Result<()> {
    let backgrounds = osu().await?.seasonal_backgrounds().await?;
    println!("Received {} backgrounds", backgrounds.backgrounds.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn spotlights() -> Result<()> {
    let spotlights = osu().await?.spotlights().await?;

    let participants: u32 = spotlights
        .iter()
        .map(|s| s.participant_count.unwrap_or(0))
        .sum();

    println!(
        "Received {} spotlights with a total of {} participants",
        spotlights.len(),
        participants
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn team_rankings() -> Result<()> {
    let rankings = osu().await?.team_rankings(GameMode::Osu).await?;

    println!(
        "Received team rankings with {} out of {} teams",
        rankings.ranking.len(),
        rankings.total
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn user() -> Result<()> {
    let user = osu()
        .await?
        .user("freddie benson")
        .mode(GameMode::Taiko)
        .await?;

    println!("Received user who was last active {:?}", user.last_visit);

    Ok(())
}

#[tokio::test]
#[serial]
async fn user_beatmapsets() -> Result<()> {
    let kinds = [
        UserBeatmapsetsKind::Favourite,
        UserBeatmapsetsKind::Graveyard,
        UserBeatmapsetsKind::Guest,
        UserBeatmapsetsKind::Loved,
        UserBeatmapsetsKind::Nominated,
        UserBeatmapsetsKind::Pending,
        UserBeatmapsetsKind::Ranked,
    ];

    let osu = osu().await?;

    for kind in kinds {
        let mapsets = osu.user_beatmapsets(SYLAS, kind).limit(5).offset(2).await?;
        println!("Received {} {kind:?} mapsets of the user", mapsets.len());
    }

    Ok(())
}

#[tokio::test]
#[serial]
async fn user_most_played() -> Result<()> {
    let scores = osu()
        .await?
        .user_most_played(BADEWANNE3)
        .limit(5)
        .offset(2)
        .await?;

    println!(
        "Received {} scores, the first is map id {}",
        scores.len(),
        scores[0].map_id
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn user_scores() -> Result<()> {
    let scores = osu()
        .await?
        .user_scores("Badewanne3")
        .mode(GameMode::Catch)
        .limit(9)
        .offset(1)
        .best()
        .await?;

    assert_eq!(scores.len(), 9);

    Ok(())
}

#[cfg(not(feature = "cache"))]
#[tokio::test]
#[serial]
#[ignore = "just making sure it compiles"]
async fn user_scores_no_cache() -> Result<()> {
    let _ = osu().await?.user_scores(BADEWANNE3).best().await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn user_scores_legacy() -> Result<()> {
    let scores = osu()
        .await?
        .user_scores(BADEWANNE3)
        .mode(GameMode::Taiko)
        .limit(9)
        .offset(1)
        .best()
        .legacy_scores(true)
        .await?;

    assert_eq!(scores.len(), 9);

    Ok(())
}

#[tokio::test]
#[serial]
async fn users() -> Result<()> {
    let users = osu().await?.users([BADEWANNE3, SYLAS]).await?;
    println!("Received {} users", users.len());

    Ok(())
}

#[tokio::test]
#[serial]
async fn wiki() -> Result<()> {
    let page = osu()
        .await?
        .wiki("fr")
        .page("Client/File_formats/osu_%28file_format%29")
        .await?;

    println!(
        "Received page {}/{}: {}",
        page.locale, page.path, page.title
    );

    Ok(())
}
