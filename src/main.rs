use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        handle_message(message).await.expect("Something wrong with the bot!");
        respond(())
    })
    .await;
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
) -> TransitionOut<()> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await?;
            next(())
        }
        Some(ans) => {
            log::info!("re text: {}", ans);
            next(())
        },
    }
}