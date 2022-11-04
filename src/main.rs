mod godette;

use teloxide::prelude::*;

use godette::commands;
use godette::Godette;

use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let bot = Godette::new();
    let handler = bot.create_handler();
    Dispatcher::builder(bot.bot, handler)
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
