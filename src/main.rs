use std::sync::Arc;

use chrono::prelude::*;
use tbot::contexts::fields::*;
use tbot::prelude::*;

mod manager;
mod til;
mod env;

use env::*;

#[tokio::main]
async fn main() {
    manager::git_command(&["clone", &*GIT_URL], ".");
    manager::git_command(&["config", "--global", "user.email", &*GIT_EMAIL], ".");
    manager::git_command(&["config", "--global", "user.name", &*GIT_NAME], ".");

    let mut bot = tbot::Bot::new((*BOT_TOKEN).clone()).event_loop();

    bot.text(move |context| post_handler(context));
    bot.edited_text(move |context| post_handler(context));

    println!("Starting at {}:{}", *BOT_URL, *PORT);
    bot.webhook(&*BOT_URL, *PORT)
        .accept_updates_on(format!("/{}", *BOT_TOKEN))
        .ip("0.0.0.0".parse().unwrap())
        .http()
        .start()
        .await
        .unwrap();
    // bot.polling().start().await.unwrap();
}

async fn post_handler<T: Text + Message>(context: Arc<T>) {
    let text = &context.text().value;
    let naive = NaiveDateTime::from_timestamp(context.date(), 0);
    let date: Date<Utc> = Date::from_utc(naive.date(), Utc);

    if *CHANNEL_ID != context.chat().id {
        let message = "😠 Channel ID mismatch, how dare you try terrorism!";
        let send_result = context.send_message_in_reply(message).call().await;

        if let Err(err) = send_result {
            dbg!(err);
        }

        return ();
    }

    let send_result = if let Some(til) = til::parse_til(text, date) {
        let post_result = manager::add_til(&til, &*GIT_URL);

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
