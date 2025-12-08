extern crate rosu_v2;

use std::{env, time::Duration};

use dotenvy::dotenv;
use eyre::{Result, WrapErr};

use hyper::StatusCode;
use rosu_v2::{error::OsuError, model::chat::ChatChannelId, prelude::Token, Osu};

#[cfg(feature = "local_oauth")]
use rosu_v2::prelude::Scopes;
use serial_test::serial;
use tokio::time::sleep;

mod common;

async fn osu() -> Result<Osu> {
    common::init_tracing();

    dotenv().ok();

    let client_id = env::var("CLIENT_ID")
        .expect("missing CLIENT_ID")
        .parse()
        .wrap_err("failed to parse client id as u64")?;

    let client_secret = env::var("CLIENT_SECRET").wrap_err("missing CLIENT_SECRET")?;

    // Preventing 429s
    sleep(common::jitter()).await;

    let mut builder = Osu::builder()
        .client_id(client_id)
        .client_secret(client_secret)
        .ratelimit(1);

    #[cfg(feature = "local_oauth")]
    {
        builder = match env::var("ACCESS_TOKEN") {
            Ok(access_token) => builder.with_token(Token::new(&access_token, None), None),
            Err(_) => {
                let redirect_url = env::var("REDIRECT_URL")
                    .expect("Set either ACCESS_TOKEN or REDIRECT_URL in .env");

                let wide_access_scope = Scopes::Public
                    | Scopes::Identify
                    | Scopes::ChatRead
                    | Scopes::ChatWrite
                    | Scopes::ChatWriteManage;

                builder.with_local_authorization(redirect_url, wide_access_scope)
            }
        };
    }

    #[cfg(not(feature = "local_oauth"))]
    {
        let access_token = env::var("ACCESS_TOKEN").expect(
            "Either set ACCESS_TOKEN in .env, or enable 'local_oauth' feature and set REDIRECT_URL",
        );

        builder = builder.with_token(Token::new(&access_token, None), None);
    }

    Ok(builder.build().await?)
}

const OSU_CHANNEL_ID: ChatChannelId = 5;
const LAZER_CHANNEL_ID: ChatChannelId = 14599138;

#[tokio::test]
#[serial]
async fn chat() -> Result<()> {
    let osu = osu().await?;

    let chat_silences = osu.chat_keepalive().await?;
    println!("Chat: {} recent silences", chat_silences.silences.len());

    let public_channels = osu.chat_channels().await?;
    println!(
        "Chat: {} public chats available for joining",
        public_channels.len()
    );

    let chat_channel = osu.chat_channel(OSU_CHANNEL_ID).await?;
    println!(
        "Chat: #osu description: {:?}, last message ID: {:?}",
        chat_channel.channel.description, chat_channel.channel.last_message_id,
    );

    let osu_messages = osu.chat_channel_messages(OSU_CHANNEL_ID).limit(3).await?;
    match osu_messages.as_slice() {
        [first, middle, last] => {
            println!(
                "Chat: Read 3 messages from #osu: {:?}",
                osu_messages
                    .iter()
                    .map(|m| m.message_id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            let filtered_messages = osu
                .chat_channel_messages(OSU_CHANNEL_ID)
                .since_message_id(first.message_id)
                .until_message_id(last.message_id)
                .await?;

            match filtered_messages.as_slice() {
                [another_middle, _] => {
                    assert_eq!(another_middle.message_id, middle.message_id)
                }
                _ => panic!("Chat: Expected to re-read two messages, got: {filtered_messages:?}"),
            };
        }
        _ => println!(
            "Chat: Read {} messages from #osu instead of 3",
            osu_messages.len()
        ),
    }

    Ok(())
}

#[tokio::test]
#[serial]
#[ignore = "chat/updates requires `lazer` OAuth scope"]
async fn chat_updates() -> Result<()> {
    let chat_updates = osu().await.unwrap().chat_updates().await.unwrap();

    println!(
        "Chat: Currently in {} channel(s), {} user silence(s) since last read",
        chat_updates.presence.len(),
        chat_updates.silences.len()
    );

    Ok(())
}

#[tokio::test]
#[serial]
async fn chat_join_leave() -> Result<()> {
    let osu = osu().await?;
    let me = osu.own_data().await?;

    osu.chat_join_channel(LAZER_CHANNEL_ID, me.user_id).await?;
    osu.chat_leave_channel(LAZER_CHANNEL_ID, me.user_id).await?;
    osu.chat_join_channel(LAZER_CHANNEL_ID, me.user_id).await?;

    Ok(())
}

const BANCHOBOT_USER_ID: u32 = 3;

#[tokio::test]
#[serial]
async fn chat_post_messages() -> Result<()> {
    let osu = osu().await?;

    let channel = osu
        .chat_create_private_channel(BANCHOBOT_USER_ID, "!faq ping", false)
        .await?;

    // Awaiting reply
    sleep(Duration::from_secs(3)).await;

    let reply = osu
        .chat_channel_messages(channel.channel.channel_id)
        .limit(1)
        .await?
        .pop()
        .unwrap();

    assert_eq!(
        reply.sender_id, BANCHOBOT_USER_ID,
        "Missing BanchoBot reply"
    );

    // Avoid the "New osu! notifications" email triggered by an unread message from BanchoBot.
    // (At the same time, test the API method.)
    osu.chat_mark_as_read(channel.channel.channel_id, reply.message_id)
        .await?;

    Ok(())
}

#[tokio::test]
#[serial]
async fn chat_create_announcement() -> Result<()> {
    let osu = osu().await?;

    let channel = osu
        .chat_create_announcement(
            "rosu-v2",
            "Test announcement",
            "test message",
            vec![BANCHOBOT_USER_ID],
        )
        .await;

    match channel {
        Ok(announcement) => {
            println!("Chat: Created announcement with yourself and BanchoBot: {announcement:?}");
        }
        Err(OsuError::Response { source, status, .. }) => {
            assert_eq!(
                status,
                StatusCode::FORBIDDEN,
                "Unexpected error -- expected failure due to lack of announcement rights: {source}"
            );

            println!("Chat: EXPECTED failure during announcement creation: {source:?} {status}");
        }
        Err(e) => panic!("Unexpected error: {e}"),
    }

    Ok(())
}
