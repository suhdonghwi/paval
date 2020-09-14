use std::env;

use chrono::prelude::*;
use tbot::prelude::*;

mod til;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("TBOT_TOKEN").event_loop();
    let api_path = env::var("PAVAL_API_PATH");

    bot.text(|context| async move {
        dbg!(&context);
        let text = &context.text.value;
        let naive = NaiveDateTime::from_timestamp(context.date, 0);
        let date: Date<Utc> = Date::from_utc(naive.date(), Utc);

        let til = til::parse_til(text, date);
        dbg!(til);
        //let echo = til::parse_til(&context.text.value);
    });

    bot.command("register", |context| async move {
    });

    bot.edited_text(|context| async move {
        dbg!(&context);
        let echo = &context.text.value;
        let call_result = context.send_message(echo).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
