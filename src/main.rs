use tbot::prelude::*;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("TBOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let echo = &context.text.value;
        let call_result = context.send_message(echo).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
