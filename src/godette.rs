use teloxide::prelude::*;

pub mod commands;
mod handlers;
use commands::{AdminCommand, Command};

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
        // Checking if it's a reply
        let message = msg.clone();
        let reply = message.reply_to_message();
        match reply {
            Some(reply) => return Godette::reply_dispatcher(bot, msg, reply.to_owned()).await,
            None => (),
        };

        Ok(())
    }

    pub async fn reply_dispatcher(bot: Bot, msg: Message, reply: Message) -> ResponseResult<()> {
        let thanks = vec!["спасибо", "спс", "благодар очка"];
        println!("Working on reply");
        let text = msg
            .text()
            .unwrap_or(msg.caption().unwrap_or_default())
            .to_string();
        println!("{:?}", text);
        for thank in thanks {
            match text.to_lowercase().find(thank) {
                Some(_id) => {
                    println!("It's a thanks!");
                    return handlers::karma(&bot, &msg, &reply, 1).await;
                }
                None => (),
            }
        }
        Ok(())
    }
}
