use teloxide::{dispatching::DpHandlerDescription, prelude::*, RequestError};

pub mod commands;
mod dispatchers;
mod handlers;
mod utils;

pub struct Godette {
    pub bot: Bot,
    pub triggers: Vec<Trigger>,
}

pub struct Trigger {
    pub words: Vec<String>,
    pub callback: fn(&Bot, &Message) -> ResponseResult<()>,
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
            triggers: vec![],
        }
    }

    pub fn create_handler(
        &self,
    ) -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
        Update::filter_message()
            // User commands
            .branch(
                dptree::entry()
                    .filter_command::<commands::Command>()
                    .endpoint(Godette::commands_dispatcher),
            )
            // Admin commands
            .branch(
                dptree::entry()
                    .filter_command::<commands::AdminCommand>()
                    .endpoint(Godette::admin_dispatcher),
            )
            // Replies
            // .branch(Message::filter_reply_to_message().endpoint(Godette::reply_dispatcher))
            // Messages
            .branch(
                dptree::filter(|msg: Message| {
                    msg.from()
                        .map(|user| user.id == UserId(60441930))
                        .unwrap_or_default()
                })
                .endpoint(Godette::message_dispatcher),
            )
    }
}
