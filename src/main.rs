use std::env;
use std::process::Command;
use std::sync::Arc;

use chrono::prelude::*;
use tbot::contexts::fields::*;
use tbot::prelude::*;
use tbot::types::chat::Id;

mod manager;
mod til;

fn get_env(env: &str) -> String {
    match env::var(env) {
        Ok(var) => var,
        Err(_) => panic!(format!("Environment variable `{}` does not exist", env)),
    }
}

#[tokio::main]
async fn main() {
    let git_url = get_env("PAVAL_GIT_URL");

    let mut clone_command = Command::new(dbg!(format!("git clone {} til", git_url)));
    clone_command
        .spawn()
        .expect("Failed to clone git respository");

    let token = get_env("PAVAL_BOT_TOKEN");
    let mut bot = tbot::Bot::new(token.clone()).event_loop();

    let channel_id = Id::from(
        get_env("PAVAL_CHANNEL_ID")
            .parse::<i64>()
            .expect("Invalid PAVAL_CHANNEL_ID"),
    );

    bot.text(move |context| post_handler(context, channel_id.clone()));

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

async fn post_handler<T: Text + Message>(context: Arc<T>, channel_id: Id) {
    let text = &context.text().value;
    let naive = NaiveDateTime::from_timestamp(context.date(), 0);
    let date: Date<Utc> = Date::from_utc(naive.date(), Utc);

    if channel_id != context.chat().id {
        let message = "😠 Channel ID mismatch, how dare you try terrorism!";
        let send_result = context.send_message_in_reply(message).call().await;

        if let Err(err) = send_result {
            dbg!(err);
        }

        return ();
    }

    let send_result = if let Some(til) = til::parse_til(text, date) {
        let post_result = manager::add_til(&til);

        match post_result {
            Ok(_) => {
                let message = format!("✅ Successfully posted : {}", til.title);
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
