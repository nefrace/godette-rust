use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::ParseMode::MarkdownV2,
    utils::command::BotCommands,
    utils::markdown::{self, bold, escape, italic},
};

use crate::commands::{AdminCommand, Command};

pub async fn show_help(bot: Bot, msg: Message) -> ResponseResult<Message> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await
}
pub async fn show_adminhelp(bot: Bot, msg: Message) -> ResponseResult<Message> {
    bot.send_message(msg.chat.id, AdminCommand::descriptions().to_string())
        .await
}

pub async fn me(bot: Bot, msg: Message, quote: String) -> ResponseResult<Message> {
    let name = msg.from().unwrap().to_owned().full_name();
    let esc_username = markdown::escape(&name);
    let esc_quote = markdown::escape(&quote);

    let text = format!("*_{esc_username}_* {esc_quote}").to_string();
    bot.delete_message(msg.chat.id, msg.id).await?;
    bot.send_message(msg.chat.id, text)
        .parse_mode(MarkdownV2)
        .await
}

pub async fn say(bot: Bot, msg: Message, quote: String) -> ResponseResult<Message> {
    let text = bold(&italic(&escape(&quote)));
    bot.delete_message(msg.chat.id, msg.id).await?;
    let message = bot.send_message(msg.chat.id, text).parse_mode(MarkdownV2);
    match msg.reply_to_message() {
        Some(reply) => message.reply_to_message_id(reply.id).await,
        None => message.await,
    }
}

pub async fn warn(bot: Bot, msg: Message, reason: String) -> ResponseResult<Message> {
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
                "Используйте эту команду как ответ на сообщение, требующее действий.".to_string(),
            )
            .await
        }
    }
}

pub async fn unwarn(bot: Bot, msg: Message) -> ResponseResult<Message> {
    bot.send_message(msg.chat.id, "Это разбан".to_string())
        .await
}

pub async fn karma(bot: &Bot, msg: &Message, reply: &Message, change: i8) -> ResponseResult<()> {
    let giver = msg.from().unwrap();
    let reciever = reply.from().unwrap();
    let change_text = match change {
        1 => "повысил",
        -1 => "понизил",
        _ => "изменил",
    };
    let text = format!(
        "*{}* {} карму *{}*",
        escape(&giver.first_name),
        change_text,
        escape(&reciever.first_name)
    );
    bot.send_message(msg.chat.id, text)
        .parse_mode(MarkdownV2)
        .await?;
    Ok(())
}
