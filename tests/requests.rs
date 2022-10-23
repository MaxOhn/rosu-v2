extern crate rosu_v2;

use std::env;

use dotenv::dotenv;
use eyre::{Result, WrapErr};
use once_cell::sync::OnceCell;
use rosu_v2::{
    model::{
        beatmap::{BeatmapsetSearchSort, RankStatus},
        GameMode,
    },
    Osu,
};

#[cfg(feature = "cache")]
use rosu_v2::model::GameMods;

static OSU: OnceCell<Osu> = OnceCell::new();

// Be sure you pass `--test-threads=1` to `cargo test` when running
async fn osu() -> Result<&'static Osu> {
    if OSU.get().is_none() {
        let _ = env_logger::builder().is_test(true).try_init();
        dotenv().ok();

        let client_id = env::var("CLIENT_ID")
            .expect("missing CLIENT_ID")
            .parse()
            .wrap_err("failed to parse client id as u64")?;

        let client_secret = env::var("CLIENT_SECRET").wrap_err("missing CLIENT_SECRET")?;

        let osu = Osu::builder()
            .client_id(client_id)
            .client_secret(client_secret)
            .build()
            .await
            .wrap_err("failed to build osu! client")?;

        if OSU.set(osu).is_err() {
            eyre::bail!("Failed to set OSU cell");
        }
    }

    Ok(OSU.wait())
}

const ADESSO_BALLA: u32 = 171024;
const BREEZEBLOCKS: u32 = 3187415;

const HIKOUI_GUMO: u32 = 357161;

const BADEWANNE3: u32 = 2211396;
const SYLAS: u32 = 3906405;

const DE_VS_CA: u32 = 71028303;

const COOKIEZI_FREEDOM_DIVE: u64 = 2177560145;

#[tokio::test]
// #[ignore = "specific testing"]
async fn custom() -> Result<()> {
    let req_fut = osu().await?.beatmapset(3);

    let result = req_fut.await?;
    println!("Result:\n{:#?}", result);

    Ok(())
}

#[tokio::test]
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
async fn beatmap_difficulty_attributes() -> Result<()> {
    let attrs = osu()
        .await?
        .beatmap_difficulty_attributes(ADESSO_BALLA)
        .mode(GameMode::Taiko)
        .await?;

    println!("{:?}", attrs.attrs);

    Ok(())
}

#[tokio::test]
async fn beatmaps() -> Result<()> {
    let maps = osu().await?.beatmaps([ADESSO_BALLA, BREEZEBLOCKS]).await?;
    println!("Received {} maps", maps.len());

    Ok(())
}

#[tokio::test]
async fn beatmap_scores() -> Result<()> {
    let scores = osu().await?.beatmap_scores(ADESSO_BALLA).await?;
    println!("Received {} scores", scores.len());

    Ok(())
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn beatmap_user_score() -> Result<()> {
    let score = osu()
        .await?
        .beatmap_user_score(ADESSO_BALLA, BADEWANNE3)
        .mods(GameMods::Hidden | GameMods::HardRock | GameMods::HalfTime)
        .await?;

    println!(
        "Received score, pos={} | mods={}",
        score.pos, score.score.mods,
    );

    Ok(())
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn beatmap_user_scores() -> Result<()> {
    let scores = osu()
        .await?
        .beatmap_user_scores(ADESSO_BALLA, BADEWANNE3)
        .await?;

    println!("Received {} scores", scores.len());

    Ok(())
}

#[tokio::test]
async fn beatmapset() -> Result<()> {
    let mapset = osu().await?.beatmapset(HIKOUI_GUMO).await?;
    println!("Received mapset with {} maps", mapset.maps.unwrap().len());

    Ok(())
}

#[tokio::test]
async fn beatmapset_from_map_id() -> Result<()> {
    let mapset = osu().await?.beatmapset_from_map_id(ADESSO_BALLA).await?;
    println!("Received mapset with {} maps", mapset.maps.unwrap().len());

    Ok(())
}

#[tokio::test]
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
async fn beatmapset_search() -> Result<()> {
    let search_result = osu()
        .await?
        .beatmapset_search()
        .query("artist=camellia stars>8 ar>9 length<400")
        .status(RankStatus::Graveyard)
        .mode(GameMode::Osu)
        .nsfw(false)
        .sort(BeatmapsetSearchSort::Favourites, false)
        .await?;

    println!(
        "Received search result containing {} out of {} mapsets",
        search_result.mapsets.len(),
        search_result.total,
    );

    Ok(())
}

#[tokio::test]
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

#[cfg(feature = "cache")]
#[tokio::test]
async fn recent_events() -> Result<()> {
    let events = osu()
        .await?
        .recent_events("badewanne3")
        .limit(10)
        .offset(2)
        .await?;

    println!("Received {} events", events.len());

    Ok(())
}

#[tokio::test]
async fn kudosu() -> Result<()> {
    let history = osu().await?.kudosu(SYLAS).limit(5).offset(1).await?;
    let sum: i32 = history.iter().map(|entry| entry.amount).sum();

    println!("Received {} entries amounting to {}", history.len(), sum);

    Ok(())
}

#[tokio::test]
async fn news() -> Result<()> {
    let news = osu().await?.news().await?;
    println!("Received news, got {} posts", news.posts.len());

    Ok(())
}

#[tokio::test]
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
async fn osu_matches() -> Result<()> {
    let osu_matches = osu().await?.osu_matches().await?;
    println!("Received {} matches", osu_matches.matches.len());

    Ok(())
}

#[tokio::test]
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
async fn score() -> Result<()> {
    let score = osu()
        .await?
        .score(COOKIEZI_FREEDOM_DIVE, GameMode::Osu)
        .await?;

    println!(
        "Received {}'s FREEDOM DIVE score",
        score.user.unwrap().username
    );

    Ok(())
}

#[tokio::test]
async fn score_rankings() -> Result<()> {
    let rankings = osu().await?.score_rankings(GameMode::Osu).await?;
    println!(
        "Received score rankings with {} out of {} users",
        rankings.ranking.len(),
        rankings.total
    );

    Ok(())
}

#[tokio::test]
async fn seasonal_backgrounds() -> Result<()> {
    let backgrounds = osu().await?.seasonal_backgrounds().await?;
    println!("Received {} backgrounds", backgrounds.backgrounds.len());

    Ok(())
}

#[tokio::test]
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
async fn user_beatmapsets() -> Result<()> {
    let mapsets = osu()
        .await?
        .user_beatmapsets(SYLAS)
        .limit(5)
        .ranked()
        .offset(2)
        .await?;

    println!("Received {} mapsets of the user", mapsets.len());

    Ok(())
}

#[tokio::test]
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

#[cfg(feature = "cache")]
#[tokio::test]
async fn user_scores() -> Result<()> {
    let scores = osu()
        .await?
        .user_scores("Badewanne3")
        .mode(GameMode::Catch)
        .limit(99)
        .offset(1)
        .best()
        .await?;

    assert_eq!(scores.len(), 99);

    Ok(())
}

#[tokio::test]
#[ignore = "currently unavailable"]
async fn users() -> Result<()> {
    #[allow(deprecated)]
    let users = osu().await?.users(&[BADEWANNE3, SYLAS]).await?;
    println!("Received {} users", users.len());

    Ok(())
}

#[tokio::test]
async fn wiki() -> Result<()> {
    let page = osu()
        .await?
        .wiki("fr")
        .page("Client/File_formats/Osu_%28file_format%29")
        .await?;

    println!(
        "Received page {}/{}: {}",
        page.locale, page.path, page.title
    );

    Ok(())
}
