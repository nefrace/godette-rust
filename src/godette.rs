use std::env;

use diesel::{Connection, PgConnection};
use teloxide::{dispatching::DpHandlerDescription, prelude::*, RequestError};

pub mod commands;

mod dispatchers;
mod handlers;
mod utils;

pub struct Godette {
    pub bot: Bot,
    pub triggers: Vec<Trigger>,
    db: PgConnection,
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
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        Godette {
            bot: Bot::from_env(),
            triggers: vec![],
            db: PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
        }
    }

    pub fn create_handler(
        &self,
    ) -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
        dptree::entry()
            .branch(Update::filter_chat_member().endpoint(Godette::chat_member))
            .branch(Update::filter_callback_query().endpoint(Godette::callback_dispatcher))
            .branch(
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
                    ),
            )
    }
}
