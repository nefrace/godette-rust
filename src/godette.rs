use teloxide::{prelude::*, repl};

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
        let thanks = vec!["спасибо", "спс", "благодар очка"];
        let text = msg
            .text()
            .unwrap_or(msg.caption().unwrap_or_default())
            .to_string();

        for thank in thanks {
            match text.to_lowercase().find(thank) {
                Some(_id) => {
                    bot.send_message(msg.chat.id, "Не за что!").await?;
                }
                None => (),
            }
        }
        Ok(())
    }

    pub async fn reply_dispatcher(
        bot: Bot,
        msg: Message,
        reply_to_message: Message,
    ) -> ResponseResult<()> {
        let thanks = vec!["спасибо", "спс", "благодар очка"];
        let text = msg
            .text()
            .unwrap_or(msg.caption().unwrap_or_default())
            .to_string();
        println!("{:?}", msg);
        println!("{:?}", bot);
        println!("{:?}", reply_to_message);
        for thank in thanks {
            match text.to_lowercase().find(thank) {
                Some(_id) => {
                    handlers::karma(&bot, &msg, &reply_to_message, 1).await?;
                    ()
                }
                None => (),
            }
        }
        Ok(())
    }
}
