mod commands;
mod godette;

use teloxide::{
    prelude::*, types::ParseMode::MarkdownV2, utils::command::BotCommands, utils::markdown,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = godette::Godette::new();
    teloxide::commands_repl(
        bot.bot,
        godette::Godette::commands_dispatcher,
        commands::Command::ty(),
    )
    .await;
}
