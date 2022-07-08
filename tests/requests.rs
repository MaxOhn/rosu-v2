extern crate rosu_v2;

use std::{env, error::Error, fmt::Write};

use dotenv::dotenv;
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

#[macro_use]
extern crate log;

macro_rules! unwind_error {
    ($log:ident, $err:ident, $($arg:tt)+) => {
        {
            $log!($($arg)+, $err);
            let mut err: &dyn ::std::error::Error = &$err;
            while let Some(source) = err.source() {
                $log!("  - caused by: {}", source);
                err = source;
            }
        }
    };
}

static OSU: OnceCell<Osu> = OnceCell::new();

// Be sure you pass `--test-threads=1` to `cargo test` when running
async fn init() {
    if OSU.get().is_none() {
        let _ = env_logger::builder().is_test(true).try_init();
        dotenv().ok();

        let client_id = env::var("CLIENT_ID")
            .expect("missing CLIENT_ID")
            .parse()
            .expect("failed to parse client id as u64");

        let client_secret = env::var("CLIENT_SECRET").expect("missing CLIENT_SECRET");

        let osu = Osu::builder()
            .client_id(client_id)
            .client_secret(client_secret)
            .build()
            .await
            .unwrap_or_else(|e| {
                let mut output = format!("failed to build osu! client:\n  - caused by: {}", e);
                let mut err: &dyn Error = &e;

                while let Some(src) = err.source() {
                    let _ = Write::write_fmt(&mut output, format_args!("\n  - caused by: {}", src));
                    err = src;
                }

                panic!("{}", output)
            });

        OSU.set(osu).unwrap_or_else(|_| panic!("failed to set OSU"));
    }
}

const ADESSO_BALLA: u32 = 171024;
const BREEZEBLOCKS: u32 = 3187415;

const HIKOUI_GUMO: u32 = 357161;

const BADEWANNE3: u32 = 2211396;
const SYLAS: u32 = 3906405;

const DE_VS_CA: u32 = 71028303;

const COOKIEZI_FREEDOM_DIVE: u64 = 2177560145;

fn osu() -> &'static Osu {
    OSU.get().expect("OSU not initialized")
}

#[tokio::test]
#[ignore = "specific testing"]
async fn custom() {
    init().await;

    let req_fut = osu().beatmap_user_scores(ADESSO_BALLA, BADEWANNE3);

    let result = req_fut.await.unwrap();
    println!("Result:\n{:#?}", result);
}

#[tokio::test]
async fn beatmap() {
    init().await;

    match osu().beatmap().map_id(ADESSO_BALLA).await {
        Ok(map) => println!(
            "Received {} - {}",
            map.mapset.as_ref().unwrap().artist,
            map.mapset.as_ref().unwrap().title,
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting beatmap: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn beatmaps() {
    init().await;

    match osu().beatmaps([ADESSO_BALLA, BREEZEBLOCKS]).await {
        Ok(maps) => println!("Received {} maps", maps.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting beatmap: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn beatmap_scores() {
    init().await;

    match osu().beatmap_scores(ADESSO_BALLA).await {
        Ok(scores) => println!("Received {} scores", scores.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting beatmap scores: {}");
            panic!()
        }
    }
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn beatmap_user_score() {
    init().await;

    match osu()
        .beatmap_user_score(ADESSO_BALLA, BADEWANNE3)
        .mods(GameMods::Hidden | GameMods::HardRock | GameMods::HalfTime)
        .await
    {
        Ok(score) => println!(
            "Received score, pos={} | mods={}",
            score.pos, score.score.mods,
        ),
        Err(why) => {
            unwind_error!(
                println,
                why,
                "Error while requesting beatmap user score: {}"
            );
            panic!()
        }
    }
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn beatmap_user_scores() {
    init().await;

    match osu().beatmap_user_scores(ADESSO_BALLA, BADEWANNE3).await {
        Ok(scores) => println!("Received {} scores", scores.len(),),
        Err(why) => {
            unwind_error!(
                println,
                why,
                "Error while requesting beatmap user scores: {}"
            );
            panic!()
        }
    }
}

#[tokio::test]
async fn beatmapset() {
    init().await;

    match osu().beatmapset(HIKOUI_GUMO).await {
        Ok(mapset) => println!("Received mapset with {} maps", mapset.maps.unwrap().len()),
        Err(why) => {
            unwind_error!(println, why, "Error while requesting beatmapset: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn beatmapset_events() {
    init().await;

    match osu().beatmapset_events().await {
        Ok(events) => println!(
            "Received {} events, {} users",
            events.events.len(),
            events.users.len(),
        ),
        Err(why) => {
            unwind_error!(println, why, "Error while requesting beatmapset events: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn beatmapset_search() {
    init().await;

    let search_fut = osu()
        .beatmapset_search()
        .query("artist=camellia stars>8 ar>9 length<400")
        .status(RankStatus::Graveyard)
        .mode(GameMode::Osu)
        .nsfw(false)
        .sort(BeatmapsetSearchSort::Favourites, false);

    match search_fut.await {
        Ok(result) => println!(
            "Received search result containing {} out of {} mapsets",
            result.mapsets.len(),
            result.total,
        ),
        Err(why) => {
            unwind_error!(println, why, "Error while requesting beatmapset events: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn comments() {
    init().await;

    match osu().comments().sort_new().await {
        Ok(bundle) => println!(
            "Received bundle, {} comments | {} users",
            bundle.comments.len(),
            bundle.users.len(),
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting comments: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn chart_rankings() {
    init().await;

    match osu().chart_rankings(GameMode::Osu).await {
        Ok(rankings) => println!(
            "Received a spotlight with {} mapsets and {} statistics",
            rankings.mapsets.len(),
            rankings.ranking.len(),
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting comments: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn country_rankings() {
    init().await;

    match osu().country_rankings(GameMode::Osu).await {
        Ok(countries) => println!(
            "Received the first {} out of {} countries",
            countries.ranking.len(),
            countries.total
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting comments: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn forum_posts() {
    init().await;

    match osu().forum_posts(1265690).sort_descending().limit(10).await {
        Ok(posts) => println!("Received {} posts", posts.posts.len(),),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting forum posts: {}");
            panic!()
        }
    }
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn recent_events() {
    init().await;

    match osu().recent_events("badewanne3").limit(10).offset(2).await {
        Ok(events) => println!("Received {} events", events.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting recent events: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn kudosu() {
    init().await;

    match osu().kudosu(SYLAS).limit(5).offset(1).await {
        Ok(history) => {
            let sum: i32 = history.iter().map(|entry| entry.amount).sum();

            println!("Received {} entries amounting to {}", history.len(), sum);
        }
        Err(why) => {
            unwind_error!(error, why, "Error while requesting kudosu: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn news() {
    init().await;

    match osu().news().await {
        Ok(news) => println!("Received news, got {} posts", news.posts.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting news: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn osu_match() {
    init().await;

    match osu().osu_match(DE_VS_CA).await {
        Ok(osu_match) => {
            println!(
                "Received match, got {} events and {} users",
                osu_match.events.len(),
                osu_match.users.len()
            );
        }
        Err(why) => {
            unwind_error!(error, why, "Error while requesting match: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn osu_matches() {
    init().await;

    match osu().osu_matches().await {
        Ok(osu_matches) => println!("Received {} matches", osu_matches.matches.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting matches: {}");
            panic!()
        }
    }
}

#[tokio::test]
#[ignore = "requires OAuth to not throw an error"]
async fn own_data() {
    init().await;

    match osu().own_data().mode(GameMode::Taiko).await {
        Ok(user) => println!(
            "Received own data showing a last activity of {:?}",
            user.last_visit
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting own data: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn performance_rankings() {
    init().await;

    match osu()
        .performance_rankings(GameMode::Osu)
        .country("be")
        .await
    {
        Ok(rankings) => {
            println!(
                "Received performance rankings with {} out of {} users",
                rankings.ranking.len(),
                rankings.total
            );
        }
        Err(why) => {
            unwind_error!(error, why, "Error while requesting rankings: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn score() {
    init().await;

    match osu().score(COOKIEZI_FREEDOM_DIVE, GameMode::Osu).await {
        Ok(score) => println!(
            "Received {}'s FREEDOM DIVE score",
            score.user.unwrap().username
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting score: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn score_rankings() {
    init().await;

    match osu().score_rankings(GameMode::Osu).await {
        Ok(rankings) => {
            println!(
                "Received score rankings with {} out of {} users",
                rankings.ranking.len(),
                rankings.total
            );
        }
        Err(why) => {
            unwind_error!(error, why, "Error while requesting rankings: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn seasonal_backgrounds() {
    init().await;

    match osu().seasonal_backgrounds().await {
        Ok(backgrounds) => println!("Received {} backgrounds", backgrounds.backgrounds.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting user: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn spotlights() {
    init().await;

    match osu().spotlights().await {
        Ok(spotlights) => {
            let participants: u32 = spotlights
                .iter()
                .map(|s| s.participant_count.unwrap_or(0))
                .sum();

            println!(
                "Received {} spotlights with a total of {} participants",
                spotlights.len(),
                participants
            );
        }
        Err(why) => {
            unwind_error!(error, why, "Error while requesting spotlights: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn user() {
    init().await;

    match osu().user("freddie benson").mode(GameMode::Taiko).await {
        Ok(user) => println!("Received user who was last active {:?}", user.last_visit),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting user: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn user_beatmapsets() {
    init().await;

    match osu()
        .user_beatmapsets(SYLAS)
        .limit(5)
        .ranked()
        .offset(2)
        .await
    {
        Ok(mapsets) => println!("Received {} mapsets of the user", mapsets.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting user beatmapsets: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn user_most_played() {
    init().await;

    match osu().user_most_played(BADEWANNE3).limit(5).offset(2).await {
        Ok(scores) => println!(
            "Received {} scores, the first is map id {}",
            scores.len(),
            scores[0].map_id
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting user most played: {}");
            panic!()
        }
    }
}

#[cfg(feature = "cache")]
#[tokio::test]
async fn user_scores() {
    init().await;

    match osu()
        .user_scores("Badewanne3")
        .mode(GameMode::Catch)
        .limit(99)
        .offset(1)
        .best()
        .await
    {
        Ok(scores) => assert_eq!(scores.len(), 99),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting user scores: {}");
            panic!()
        }
    }
}

#[tokio::test]
#[ignore = "currently unavailable"]
async fn users() {
    init().await;

    #[allow(deprecated)]
    match osu().users(&[BADEWANNE3, SYLAS]).await {
        Ok(users) => println!("Received {} users", users.len()),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting users: {}");
            panic!()
        }
    }
}

#[tokio::test]
async fn wiki() {
    init().await;

    match osu()
        .wiki("fr")
        .page("Client/File_formats/Osu_%28file_format%29")
        .await
    {
        Ok(page) => println!(
            "Received page {}/{}: {}",
            page.locale, page.path, page.title
        ),
        Err(why) => {
            unwind_error!(error, why, "Error while requesting wiki: {}");
            panic!()
        }
    }
}
