use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub mod commands;
mod handlers;
mod utils;
use commands::{AdminCommand, Command};
use url::Url;

use utils::get_text_or_empty;
pub struct Godette {
    pub bot: Bot,
}

pub struct KarmaTrigger {
    pub text: String,
    pub value: i8,
}
impl KarmaTrigger {
    pub fn new(text: &str, value: i8) -> KarmaTrigger {
        KarmaTrigger {
            text: String::from(text),
            value,
        }
    }
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
        let reply = msg.reply_to_message();
        match reply {
            Some(reply) => {
                Godette::reply_dispatcher(bot.clone(), msg.clone(), reply.to_owned()).await?
            }
            None => (),
        };
        let text = utils::get_text_or_empty(&msg).to_lowercase();
        match text.find("оффтоп") {
            Some(_) => {
                bot.send_message(msg.chat.id, "Вот вам ссылка на оффтоп")
                    .reply_to_message_id(msg.id)
                    .reply_markup(Godette::make_offtop_keyboard())
                    .await?;
            }
            None => (),
        }

        Ok(())
    }

    pub async fn reply_dispatcher(bot: Bot, msg: Message, reply: Message) -> ResponseResult<()> {
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
        let text = get_text_or_empty(&msg);
        println!("{:?}", text);
        for trigger in triggers {
            match text.to_lowercase().find(&trigger.text) {
                Some(_id) => {
                    println!("It's a thanks!");
                    return handlers::karma(&bot, &msg, &reply, trigger.value).await;
                }
                None => (),
            }
        }
        Ok(())
    }

    fn make_offtop_keyboard() -> InlineKeyboardMarkup {
        let link = Url::parse("https://t.me/Godot_Engine_Offtop").unwrap();
        let button = InlineKeyboardButton::url("Godot Engine оффтоп чат".to_owned(), link);
        return InlineKeyboardMarkup::new(vec![[button]]);
    }
}
