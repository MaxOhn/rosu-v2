extern crate rosu_v2;

use dotenv::dotenv;
use once_cell::sync::OnceCell;
use rosu_v2::{
    model::{
        beatmap::{BeatmapsetSearchSort, RankStatus},
        GameMode, GameMods,
    },
    Osu,
};
use std::env;

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
            .unwrap_or_else(|err| panic!("failed to build osu! client: {}", err));

        OSU.set(osu).unwrap_or_else(|_| panic!("failed to set OSU"));
    }
}

macro_rules! get_id {
    ($name:ident, $id:literal) => {
        fn $name() -> u32 {
            $id
        }
    };
}

// Map id
get_id!(adesso_balla, 171024);

// Mapset id
get_id!(hikoui_gumo, 357161);

// Player id
get_id!(badewanne3, 2211396);

// Mapper id
get_id!(sylas, 3906405);

// Match id
get_id!(de_vs_ca, 71028303);

fn osu() -> &'static Osu {
    OSU.get().expect("OSU not initialized")
}

#[tokio::test]
#[ignore = "specific testing"]
async fn custom() {
    init().await;

    let req_fut = osu().beatmapset_search();

    let result = req_fut.await.unwrap();
    println!("Result 1: {:#?}", result);
}

#[tokio::test]
async fn beatmap() {
    init().await;

    match osu().beatmap().map_id(adesso_balla()).await {
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
async fn beatmap_scores() {
    init().await;

    match osu().beatmap_scores(adesso_balla()).await {
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
        .beatmap_user_score(adesso_balla(), badewanne3())
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
                "Error while requesting beatmap user scores: {}"
            );
            panic!()
        }
    }
}

#[tokio::test]
async fn beatmapset() {
    init().await;

    match osu().beatmapset(hikoui_gumo()).await {
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
        .mode(GameMode::STD)
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

    match osu().chart_rankings(GameMode::STD).await {
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

    match osu().country_rankings(GameMode::STD).await {
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

    match osu().kudosu(sylas()).limit(5).offset(1).await {
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
#[ignore = "TODO"]
async fn multiplayer_score() {
    init().await;

    // match osu().multiplayer_score(room, playlist, score_id).await {
    //     Ok(score) => todo!(),
    //     Err(why) => {
    //         unwind_error!(error, why, "Error while requesting score: {}");
    //         panic!()
    //     }
    // }
}

#[tokio::test]
#[ignore = "TODO"]
async fn multiplayer_scores() {
    init().await;

    // match osu().multiplayer_scores(room, playlist).await {
    //     Ok(scores) => todo!(),
    //     Err(why) => {
    //         unwind_error!(error, why, "Error while requesting scores: {}");
    //         panic!()
    //     }
    // }
}

#[tokio::test]
#[ignore = "TODO"]
async fn multiplayer_user_highscore() {
    init().await;

    // match osu().multiplayer_user_highscore(room, playlist, user_id).await {
    //     Ok(score) => todo!(),
    //     Err(why) => {
    //         unwind_error!(error, why, "Error while requesting user highscore: {}");
    //         panic!()
    //     }
    // }
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

    match osu().osu_match(de_vs_ca()).await {
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
async fn performance_rankings() {
    init().await;

    match osu()
        .performance_rankings(GameMode::STD)
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
async fn score_rankings() {
    init().await;

    match osu().score_rankings(GameMode::STD).await {
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

    match osu().user("freddie benson").mode(GameMode::TKO).await {
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
        .user_beatmapsets(sylas())
        .limit(5)
        .ranked_and_approved()
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

    match osu()
        .user_most_played(badewanne3())
        .limit(5)
        .offset(2)
        .await
    {
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
        .mode(GameMode::CTB)
        .limit(10)
        .offset(1)
        .best()
        .await
    {
        Ok(scores) => println!("Received {} scores", scores.len()),
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
    match osu().users(&[badewanne3(), sylas()]).await {
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

    match osu().wiki("de").page("Hit_object").await {
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
