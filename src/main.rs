use tbot::prelude::*;

mod til;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("TBOT_TOKEN").event_loop();

    bot.text(|context| async move {
        dbg!(&context);
        let echo = &context.text.value;
        let call_result = context.send_message(echo).call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
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
