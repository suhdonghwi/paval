use std::env;

use chrono::prelude::*;
use std::sync::Arc;
use tbot::contexts::fields::*;
use tbot::prelude::*;
use tbot::types::chat::Id;

mod til;

fn get_env(env: &str) -> String {
    match env::var(env) {
        Ok(var) => var,
        Err(_) => panic!(format!("Environment variable `{}` does not exist", env)),
    }
}

#[tokio::main]
async fn main() {
    let token = get_env("PAVAL_BOT_TOKEN");
    let mut bot = tbot::Bot::new(token.clone()).event_loop();

    let api_path = get_env("PAVAL_API_PATH");
    let channel_id = Id::from(
        get_env("PAVAL_CHANNEL_ID")
            .parse::<i64>()
            .expect("Invalid PAVAL_CHANNEL_ID"),
    );

    bot.text(move |context| post_handler(context, api_path.clone(), channel_id.clone()));

    let bot_url = get_env("WEBHOOK_URL");
    let port = get_env("PORT").parse().expect("Invalid PORT");

    println!("Starting at {}:{}", bot_url, port);
    bot.webhook(&bot_url, port)
        .accept_updates_on(format!("/{}", token))
        .ip("0.0.0.0".parse().unwrap())
        .http()
        .start()
        .await
        .unwrap();
    //bot.polling().start().await.unwrap();
}

async fn post_til(til: &til::TIL, api_path: &String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let content = format!("# {}\n\n {}", til.title, til.content);

    let res = client
        .post(api_path)
        .query(&[
            ("fields[title]", &til.title),
            ("fields[content]", &content),
            ("fields[category]", &til.category),
        ])
        .send()
        .await?;

    Ok(res)
}

async fn post_handler<T: Text + Message>(
    context: Arc<T>,
    api_path: String,
    channel_id: Id,
) {
    let text = &context.text().value;
    let naive = NaiveDateTime::from_timestamp(context.date(), 0);
    let date: Date<Utc> = Date::from_utc(naive.date(), Utc);

    let send_result = if let Some(til) = til::parse_til(text, date) {
        let post_result = post_til(&til, &api_path).await;

        match post_result {
            Ok(res) => {
                let message = if res.status() != 200 {
                    dbg!(&res);
                    format!("😢 Could not post TIL : status {}", res.status())
                } else if channel_id != context.chat().id {
                    String::from("😠 Channel ID mismatch, how dare you try terrorism!")
                } else {
                    format!("✅ Successfully posted : {}", til.title)
                };

                context.send_message_in_reply(&message).call().await
            }
            Err(post_err) => {
                dbg!(&post_err);
                let error_message = format!("😢 Could not post TIL : {}", post_err);
                context.send_message_in_reply(&error_message).call().await
            }
        }
    } else {
        context
            .send_message_in_reply("😢 Could not parse TIL post")
            .call()
            .await
    };

    if let Err(err) = send_result {
        dbg!(err);
    }
}
