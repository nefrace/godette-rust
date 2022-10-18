use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::ParseMode::MarkdownV2,
    utils::command::BotCommands,
    utils::markdown::{self, bold, escape, italic},
};
use url::Url;

use crate::commands::{AdminCommand, Command};

use super::{utils, KarmaTrigger};

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

pub async fn karma(bot: &Bot, msg: &Message, reply: &Message, text: &String) -> ResponseResult<()> {
    let triggers = vec![
        KarmaTrigger::new("спс", 1),
        KarmaTrigger::new("спасибо", 1),
        KarmaTrigger::new("+", 1),
        KarmaTrigger::new("благодарю", 1),
        KarmaTrigger::new("пасиб", 1),
        KarmaTrigger::new("-", -1),
        KarmaTrigger::new("👍", 1),
        KarmaTrigger::new("👎", -1),
    ];
    for trigger in triggers {
        match text.to_lowercase().find(&trigger.text) {
            Some(_id) => {
                let giver = msg.from().unwrap();
                let reciever = reply.from().unwrap();
                let change_text = match trigger.value {
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
                return Ok(());
            }
            None => (),
        }
    }
    Ok(())
}

pub async fn offtop(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Вот вам ссылка на оффтоп")
        .reply_to_message_id(msg.id)
        .reply_markup(utils::make_offtop_keyboard())
        .await?;
    Ok(())
}

pub async fn documentation(bot: &Bot, msg: &Message, topic: String) -> ResponseResult<()> {
    let mut text = format!(
        "Извините, по запросу \"{}\" ничего не найдено\\.",
        escape(&topic)
    );
    let btn_text = format!("Поиск \"{}\"", topic);
    let btn_url = Url::parse(&format!(
        "https://docs.godotengine.org/ru/stable/search.html?q={}",
        topic
    ))
    .unwrap();
    let results = utils::request_docs(&topic).await;

    if results.len() > 0 {
        let links = results
            .iter()
            .take(10)
            .map(|res| format!("\\- [{}]({})", escape(&res.title), res.path))
            .collect::<Vec<String>>()
            .join("\n");
        text = format!(
            "Вот что удалось мне найти в документации по запроу {}:\n\n{}",
            bold(&escape(&topic)),
            links
        );
    }

    bot.send_message(msg.chat.id, text)
        .parse_mode(MarkdownV2)
        .reply_markup(utils::make_docs_keyboard(btn_text, btn_url))
        .reply_to_message_id(msg.id)
        .await?;
    Ok(())
}
