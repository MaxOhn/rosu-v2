extern crate rosu_v2;

use std::{
    env,
    sync::atomic::{AtomicBool, Ordering::SeqCst},
};

use dotenv::dotenv;
use eyre::{Result, WrapErr};
use once_cell::sync::OnceCell;
#[cfg(feature = "cache")]
use rosu_v2::mods;
use rosu_v2::{
    model::{
        beatmap::{BeatmapsetSearchSort, RankStatus},
        GameMode,
    },
    Osu,
};
use tokio::sync::{Mutex, MutexGuard};

struct OsuSingleton {
    initialized: AtomicBool,
    // The mutex is necessary since each test spawns its own async executor and hyper's Client
    // does not like that, see https://github.com/hyperium/hyper/issues/2112
    inner: OnceCell<Mutex<Osu>>,
}

impl OsuSingleton {
    const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            inner: OnceCell::new(),
        }
    }

    async fn get(&self) -> Result<MutexGuard<'_, Osu>> {
        let cmp_res = self
            .initialized
            .compare_exchange(false, true, SeqCst, SeqCst);

        if cmp_res.is_ok() {
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

            if self.inner.set(Mutex::new(osu)).is_err() {
                eyre::bail!("Failed to set inner cell");
            }
        }

        Ok(self.inner.wait().lock().await)
    }
}

static OSU: OsuSingleton = OsuSingleton::new();

const ADESSO_BALLA: u32 = 171024;
const BREEZEBLOCKS: u32 = 3187415;

const HIKOUI_GUMO: u32 = 357161;

const BADEWANNE3: u32 = 2211396;
const SYLAS: u32 = 3906405;

const DE_VS_CA: u32 = 71028303;

const COOKIEZI_FREEDOM_DIVE: u64 = 2177560145;

#[tokio::test]
async fn beatmap() -> Result<()> {
    let map = OSU.get().await?.beatmap().map_id(ADESSO_BALLA).await?;

    println!(
        "Received {} - {}",
        map.mapset.as_ref().unwrap().artist,
        map.mapset.as_ref().unwrap().title,
    );

    Ok(())
}

#[tokio::test]
async fn beatmap_difficulty_attributes() -> Result<()> {
    let attrs = OSU
        .get()
        .await?
        .beatmap_difficulty_attributes(ADESSO_BALLA)
        .mode(GameMode::Taiko)
        .await?;

    println!("{:?}", attrs.attrs);

    Ok(())
}

#[tokio::test]
async fn beatmaps() -> Result<()> {
    let maps = OSU
        .get()
        .await?
        .beatmaps([ADESSO_BALLA, BREEZEBLOCKS])
        .await?;

    println!("Received {} maps", maps.len());

    Ok(())
}

#[tokio::test]
async fn beatmap_scores() -> Result<()> {
    let scores = OSU.get().await?.beatmap_scores(ADESSO_BALLA).await?;
    println!("Received {} scores", scores.len());

    Ok(())
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn beatmap_user_score() -> Result<()> {
    let score = OSU
        .get()
        .await?
        .beatmap_user_score(ADESSO_BALLA, BADEWANNE3)
        .mods(mods!(HD HR HT))
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
    let scores = OSU
        .get()
        .await?
        .beatmap_user_scores(ADESSO_BALLA, BADEWANNE3)
        .await?;

    println!("Received {} scores", scores.len());

    Ok(())
}

#[tokio::test]
async fn beatmapset() -> Result<()> {
    let mapset = OSU.get().await?.beatmapset(HIKOUI_GUMO).await?;
    println!("Received mapset with {} maps", mapset.maps.unwrap().len());

    Ok(())
}

#[tokio::test]
async fn beatmapset_from_map_id() -> Result<()> {
    let mapset = OSU
        .get()
        .await?
        .beatmapset_from_map_id(ADESSO_BALLA)
        .await?;

    println!("Received mapset with {} maps", mapset.maps.unwrap().len());

    Ok(())
}

#[tokio::test]
async fn beatmapset_events() -> Result<()> {
    let events = OSU.get().await?.beatmapset_events().await?;
    println!(
        "Received {} events, {} users",
        events.events.len(),
        events.users.len(),
    );

    Ok(())
}

#[tokio::test]
async fn beatmapset_search() -> Result<()> {
    let osu = OSU.get().await?;

    let search_result = osu
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

    let search_result = search_result.get_next(&osu).await.unwrap()?;

    println!(
        "Received next search result containing {} out of {} mapsets",
        search_result.mapsets.len(),
        search_result.total,
    );

    Ok(())
}

#[tokio::test]
async fn comments() -> Result<()> {
    let bundle = OSU.get().await?.comments().sort_new().await?;
    println!(
        "Received bundle, {} comments | {} users",
        bundle.comments.len(),
        bundle.users.len(),
    );

    Ok(())
}

#[tokio::test]
async fn chart_rankings() -> Result<()> {
    let rankings = OSU.get().await?.chart_rankings(GameMode::Osu).await?;

    println!(
        "Received a spotlight with {} mapsets and {} statistics",
        rankings.mapsets.len(),
        rankings.ranking.len(),
    );

    Ok(())
}

#[tokio::test]
async fn country_rankings() -> Result<()> {
    let countries = OSU.get().await?.country_rankings(GameMode::Osu).await?;

    println!(
        "Received the first {} out of {} countries",
        countries.ranking.len(),
        countries.total
    );

    Ok(())
}

#[tokio::test]
async fn forum_posts() -> Result<()> {
    let posts = OSU
        .get()
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
    let events = OSU
        .get()
        .await?
        .recent_events("badewanne3")
        .limit(10)
        .offset(2)
        .await?;

    println!("Received {} events", events.len());

    Ok(())
}

#[cfg(feature = "replay")]
#[tokio::test]
#[ignore = "requires OAuth to not throw an error"]
async fn replay() -> Result<()> {
    let replay = OSU
        .get()
        .await?
        .replay(GameMode::Osu, COOKIEZI_FREEDOM_DIVE)
        .await?;

    println!("Received replay with the following score: {}", replay.score);

    Ok(())
}

#[tokio::test]
async fn kudosu() -> Result<()> {
    let history = OSU.get().await?.kudosu(SYLAS).limit(5).offset(1).await?;
    let sum: i32 = history.iter().map(|entry| entry.amount).sum();

    println!("Received {} entries amounting to {}", history.len(), sum);

    Ok(())
}

#[tokio::test]
async fn news() -> Result<()> {
    let news = OSU.get().await?.news().await?;
    println!("Received news, got {} posts", news.posts.len());

    Ok(())
}

#[tokio::test]
async fn osu_match() -> Result<()> {
    let osu_match = OSU.get().await?.osu_match(DE_VS_CA).await?;
    println!(
        "Received match, got {} events and {} users",
        osu_match.events.len(),
        osu_match.users.len()
    );

    Ok(())
}

#[tokio::test]
async fn osu_matches() -> Result<()> {
    let osu_matches = OSU.get().await?.osu_matches().await?;
    println!("Received {} matches", osu_matches.matches.len());

    Ok(())
}

#[tokio::test]
#[ignore = "requires OAuth to not throw an error"]
async fn own_data() -> Result<()> {
    let user = OSU.get().await?.own_data().mode(GameMode::Taiko).await?;

    println!(
        "Received own data showing a last activity of {:?}",
        user.last_visit
    );

    Ok(())
}

#[tokio::test]
async fn performance_rankings() -> Result<()> {
    let rankings = OSU
        .get()
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
    let score = OSU
        .get()
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
    let rankings = OSU.get().await?.score_rankings(GameMode::Osu).await?;

    println!(
        "Received score rankings with {} out of {} users",
        rankings.ranking.len(),
        rankings.total
    );

    Ok(())
}

#[tokio::test]
async fn seasonal_backgrounds() -> Result<()> {
    let backgrounds = OSU.get().await?.seasonal_backgrounds().await?;
    println!("Received {} backgrounds", backgrounds.backgrounds.len());

    Ok(())
}

#[tokio::test]
async fn spotlights() -> Result<()> {
    let spotlights = OSU.get().await?.spotlights().await?;

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
    let user = OSU
        .get()
        .await?
        .user("freddie benson")
        .mode(GameMode::Taiko)
        .await?;

    println!("Received user who was last active {:?}", user.last_visit);

    Ok(())
}

#[tokio::test]
async fn user_beatmapsets() -> Result<()> {
    let mapsets = OSU
        .get()
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
    let scores = OSU
        .get()
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
    let scores = OSU
        .get()
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

#[tokio::test]
async fn user_scores_legacy() -> Result<()> {
    let scores = OSU
        .get()
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
async fn users() -> Result<()> {
    let users = OSU.get().await?.users([BADEWANNE3, SYLAS]).await?;
    println!("Received {} users", users.len());

    Ok(())
}

#[tokio::test]
async fn wiki() -> Result<()> {
    let page = OSU
        .get()
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
