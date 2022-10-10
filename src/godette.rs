use teloxide::{
    prelude::*, types::ParseMode::MarkdownV2, utils::command::BotCommands, utils::markdown,
};

use crate::commands::Command;

pub struct Godette {
    pub bot: Bot,
}

impl Godette {
    pub fn new() -> Godette {
        Godette {
            bot: Bot::from_env(),
        }
    }

    pub async fn commands_dispatcher(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => Godette::show_help(bot, msg).await?,
            Command::Me(quote) => Godette::me(bot, msg, quote).await?,
            Command::Warn(reason) => Godette::warn(bot, msg, reason).await?,
            Command::Unwarn => Godette::unwarn(bot, msg).await?,
        };
        Ok(())
    }

    async fn show_help(bot: Bot, msg: Message) -> ResponseResult<Message> {
        bot.send_message(msg.chat.id, Command::descriptions().to_string())
            .await
    }

    async fn me(bot: Bot, msg: Message, quote: String) -> ResponseResult<Message> {
        let name = msg.from().unwrap().to_owned().full_name();
        let esc_username = markdown::escape(&name);
        let esc_quote = markdown::escape(&quote);

        let text = format!("*_{esc_username}_* {esc_quote}").to_string();
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, text)
            .parse_mode(MarkdownV2)
            .await
    }

    async fn warn(bot: Bot, msg: Message, reason: String) -> ResponseResult<Message> {
        match msg.reply_to_message() {
            Some(guilty) => {
                let username = guilty.from().unwrap().to_owned().full_name();
                let username_formatted = markdown::bold(&markdown::escape(&username));
                let reason_formatted = markdown::italic(&markdown::escape(&reason));
                let text = format!(
                    "{username_formatted} получил предупреждение по причине:\n\"{reason_formatted}\""
                );
                bot.send_message(msg.chat.id, text)
                    .parse_mode(MarkdownV2)
                    .await
            }
            None => {
                bot.send_message(
                    msg.chat.id,
                    "Используйте эту команду как ответ на сообщение, требующее действий."
                        .to_string(),
                )
                .await
            }
        }
    }

    async fn unwarn(bot: Bot, msg: Message) -> ResponseResult<Message> {
        bot.send_message(msg.chat.id, "Это разбан".to_string())
            .await
    }
}
