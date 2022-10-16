use teloxide::prelude::*;

use super::{handlers, utils, Godette, KarmaTrigger, Trigger};
use crate::commands::{AdminCommand, Command};

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
        let reply = msg.reply_to_message();
        match reply {
            Some(reply) => {
                Godette::reply_dispatcher(bot.clone(), msg.clone(), reply.to_owned()).await?
            }
            None => (),
        };
        let text = utils::get_text_or_empty(&msg).to_lowercase();
        match text.find("оффтоп") {
            Some(_) => handlers::offtop(&bot, &msg).await?,
            None => (),
        }

        Ok(())
    }

    pub async fn reply_dispatcher(bot: Bot, msg: Message, reply: Message) -> ResponseResult<()> {
        let text = utils::get_text_or_empty(&msg);
        handlers::karma(&bot, &msg, &reply, &text).await?;

        Ok(())
    }
}
