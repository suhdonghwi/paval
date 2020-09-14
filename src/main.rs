use std::env;

use chrono::prelude::*;
use tbot::prelude::*;
use tbot::contexts::fields::*;
use std::sync::Arc;

mod til;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("TBOT_TOKEN").event_loop();

    bot.text(post_handler);
    bot.edited_text(post_handler);

    bot.polling().start().await.unwrap();
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

async fn post_handler<T: Text + Message>(context: Arc<T>) {
    let text = &context.text().value;
    let naive = NaiveDateTime::from_timestamp(context.date(), 0);
    let date: Date<Utc> = Date::from_utc(naive.date(), Utc);

    let api_path = match env::var("PAVAL_API_PATH") {
        Ok(var) => var,
        Err(_) => panic!("Environment variable `PAVAL_API_PATH` does not exist"),
    };

    if let Some(til) = til::parse_til(text, date) {
        let post_result = post_til(&til, &api_path).await;

        match post_result {
            Ok(res) => {
                if res.status() == 200 {
                    println!("✅ Successfully posted : {}", til.title);
                } else {
                    let error_message = format!("😢 Could not post TIL : status {}", res.status());
                    context.send_message_in_reply(&error_message).call().await;
                }
            }
            Err(post_err) => {
                let error_message = format!("😢 Could not post TIL : {}", post_err);
                context.send_message_in_reply(&error_message).call().await;
            }
        };
    } else {
        context
            .send_message_in_reply("😢 Could not parse TIL post")
            .call()
            .await;
    }
}
    
