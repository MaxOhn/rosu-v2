extern crate rosu_v2;

use dotenv::dotenv;
use rosu_v2::Osu;
use std::env;

#[allow(unused_imports)]
use rosu_v2::model::GameMode;

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

#[tokio::test]
async fn all_requests() {
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
        .unwrap_or_else(|err| panic!("Failed to build osu! client: {}", err));

    #[allow(unused_variables)]
    let adesso_balla: u32 = 171024;
    #[allow(unused_variables)]
    let badewanne3: u32 = 2211396;
    #[allow(unused_variables)]
    let sylas: u32 = 3906405;

    // * osu.beatmap
    // match osu.beatmap(adesso_balla).await {
    //     Ok(map) => println!(
    //         "Received {} - {}",
    //         map.mapset.as_ref().unwrap().artist(),
    //         map.mapset.as_ref().unwrap().title(),
    //     ),
    //     Err(why) => unwind_error!(println, why, "Error while requesting beatmap: {}"),
    // }

    // * osu.beatmap_scores
    // match osu.beatmap_scores(adesso_balla).await {
    //     Ok(scores) => println!(
    //         "Received {} scores | user score: {}",
    //         scores.scores.len(),
    //         scores.user_score.is_some(),
    //     ),
    //     Err(why) => unwind_error!(println, why, "Error while requesting beatmap scores: {}"),
    // }

    // * osu.beatmap_user_score
    // match osu.beatmap_user_score(adesso_balla, badewanne3).await {
    //     Ok(score) => println!(
    //         "Received score, pos={} | mods={}",
    //         score.pos, score.score.mods,
    //     ),
    //     Err(why) => unwind_error!(
    //         println,
    //         why,
    //         "Error while requesting beatmap user scores: {}"
    //     ),
    // }

    // * osu.comments
    // match osu.comments().sort_new().await {
    //     Ok(bundle) => println!(
    //         "Received bundle, {} comments | {} users",
    //         bundle.comments.len(),
    //         bundle.users.len(),
    //     ),
    //     Err(why) => unwind_error!(println, why, "Error while requesting comments: {}"),
    // }

    // * osu.recent_events
    // match osu.recent_events(badewanne3).limit(10).offset(2).await {
    //     Ok(events) => println!("Received {} events", events.len()),
    //     Err(why) => unwind_error!(println, why, "Error while requesting recent events: {}"),
    // }

    // * osu.kudosu
    // match osu.kudosu(sylas).limit(5).offset(1).await {
    //     Ok(history) => {
    //         let sum: i32 = history.iter().map(|entry| entry.amount).sum();

    //         println!("Received {} entries amounting to {}", history.len(), sum);
    //     }
    //     Err(why) => unwind_error!(println, why, "Error while requesting kudosu: {}"),
    // }

    // * osu.rankings
    // match osu
    //     .rankings(GameMode::STD)
    //     .country("be")
    //     .type_performance()
    //     .await
    // {
    //     Ok(rankings) => {
    //         let mapsets = rankings.mapsets.map_or(0, |mapsets| mapsets.len());
    //         let total = rankings.total;
    //         let rankings = rankings.ranking.len();

    //         println!(
    //             "Received value with {} mapsets, {} rankings, and a total of {}",
    //             mapsets, rankings, total
    //         );
    //     }
    //     Err(why) => unwind_error!(println, why, "Error while requesting rankings: {}"),
    // }

    // ? osu.score
    // match osu.score(room, playlist, score_id).await {
    //     Ok(score) => todo!(),
    //     Err(why) => unwind_error!(println, why, "Error while requesting score: {}"),
    // }

    // ? osu.scores
    // match osu.scores(room, playlist).await {
    //     Ok(scores) => todo!(),
    //     Err(why) => unwind_error!(println, why, "Error while requesting scores: {}"),
    // }

    // * osu.spotlights
    // match osu.spotlights().await {
    //     Ok(spotlights) => {
    //         let participants: u32 = spotlights
    //             .iter()
    //             .map(|s| s.participant_count.unwrap_or(0))
    //             .sum();

    //         println!(
    //             "Received {} spotlights with a total of {} participants",
    //             spotlights.len(),
    //             participants
    //         );
    //     }
    //     Err(why) => unwind_error!(println, why, "Error while requesting spotlights: {}"),
    // }

    // * osu.user
    // match osu.user(badewanne3).mode(GameMode::TKO).await {
    //     Ok(user) => println!("Received user who was last active {:?}", user.last_visit),
    //     Err(why) => unwind_error!(println, why, "Error while requesting user: {}"),
    // }

    // * osu.user_beatmapsets
    // match osu
    //     .user_beatmapsets(sylas)
    //     .limit(5)
    //     .ranked_and_approved()
    //     .offset(2)
    //     .await
    // {
    //     Ok(mapsets) => println!("Received {} mapsets of the user", mapsets.len()),
    //     Err(why) => unwind_error!(println, why, "Error while requesting user beatmapsets: {}"),
    // }

    // ? osu.user_highscore
    // match osu.user_highscore(room, playlist, badewanne3).await {
    //     Ok(score) => todo!(),
    //     Err(why) => unwind_error!(println, why, "Error while requesting user highscore: {}"),
    // }

    // ? osu.user_scores
    // match osu
    //     .user_scores(badewanne3)
    //     .mode(GameMode::CTB)
    //     .limit(10)
    //     .offset(1)
    //     .firsts()
    //     .await
    // {
    //     Ok(scores) => todo!(),
    //     Err(why) => unwind_error!(println, why, "Error while requesting user scores: {}"),
    // }

    // ! osu.users
    // match osu.users([badewanne3, sylas].iter().copied()).await {
    //     Ok(users) => todo!(),
    //     Err(why) => unwind_error!(println, why, "Error while requesting users: {}"),
    // }

    // ? osu.wiki
    // match osu.wiki().await {
    //     Ok(page) => println!("{:#?}", page),
    //     Err(why) => unwind_error!(println, why, "Error while requesting wiki: {}"),
    // }
}
