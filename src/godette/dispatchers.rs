use teloxide::{prelude::*, types::ChatMemberKind};

use super::{handlers, utils, Godette};
use crate::commands::{AdminCommand, Command};
use lazy_static::lazy_static;
use regex::Regex;

impl Godette {
    pub async fn commands_dispatcher(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Help => handlers::show_help(bot, msg).await?,
            Command::Me(quote) => handlers::me(bot, msg, quote).await?,
        };
        Ok(())
    }

    pub async fn admin_dispatcher(bot: Bot, msg: Message, cmd: AdminCommand) -> ResponseResult<()> {
        let mut is_admin = true;
        if msg.chat.is_group() || msg.chat.is_supergroup() {
            let sender = msg.from().unwrap();
            let admins = bot.get_chat_administrators(msg.chat.id).await?;
            is_admin = admins.iter().any(|member| member.user.id == sender.id);
        }
        if !is_admin {
            return Ok(());
        }
        match cmd {
            AdminCommand::HelpAdmin => handlers::show_adminhelp(bot, msg).await?,
            AdminCommand::Say(quote) => handlers::say(bot, msg, quote).await?,
            AdminCommand::Warn(reason) => handlers::warn(bot, msg, reason).await?,
            AdminCommand::Unwarn => handlers::unwarn(bot, msg).await?,
        };
        Ok(())
    }

    pub async fn message_dispatcher(bot: Bot, msg: Message) -> ResponseResult<()> {
        // Checking if it's a reply and send it to Reply dispatcher
        if let Some(reply) = msg.reply_to_message() {
            Godette::reply_dispatcher(bot.clone(), msg.clone(), reply.to_owned()).await?;
        }
        let text = utils::get_text_or_empty(&msg);
        if !text.to_lowercase().contains("оффтоп") {
            handlers::offtop(&bot, &msg).await?;
        }

        lazy_static! {
            static ref DOC_RE: Regex =
                Regex::new(r"(?i)док(ументац[а-я]+|[а-я])? ((п)?о )?(?P<topic>@?[\w\d\s]{1,32})")
                    .unwrap();
        }

        if let Some(caps) = DOC_RE.captures(&text) {
            if let Some(topic) = caps.name("topic") {
                handlers::documentation(&bot, &msg, String::from(topic.as_str())).await?
            }
        }

        Ok(())
    }

    pub async fn reply_dispatcher(bot: Bot, msg: Message, reply: Message) -> ResponseResult<()> {
        let text = utils::get_text_or_empty(&msg);
        handlers::karma(&bot, &msg, &reply, &text).await?;

        Ok(())
    }

    pub async fn callback_dispatcher(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
        if let Some(data) = q.data {
            bot.answer_callback_query(q.id).await?;

            if data == "no_thanks" {
                if let Some(Message { id, chat, .. }) = q.message {
                    bot.delete_message(chat.id, id).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn chat_member(bot: Bot, member: ChatMemberUpdated) -> ResponseResult<()> {
        if member.new_chat_member.kind == ChatMemberKind::Member {
            bot.send_message(member.chat.id, member.from.first_name)
                .await?;
        }

        Ok(())
    }
}
