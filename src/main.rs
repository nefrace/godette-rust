mod godette;

use teloxide::prelude::*;

use godette::commands;
use godette::Godette;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Godette::new();
    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<commands::Command>()
                .endpoint(Godette::commands_dispatcher),
        )
        .branch(
            dptree::entry()
                .filter_command::<commands::AdminCommand>()
                .endpoint(Godette::admin_dispatcher),
        )
        .branch(
            dptree::filter(|msg: Message| {
                msg.from()
                    .map(|user| user.id == UserId(60441930))
                    .unwrap_or_default()
            })
            .endpoint(Godette::message_dispatcher),
        );
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
