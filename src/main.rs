use teloxide::{
    prelude::*, types::ParseMode::MarkdownV2, utils::command::BotCommands, utils::markdown,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::from_env();
    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Вот мои команды:")]
enum Command {
    #[command(description = "Отобразить это сообщение")]
    Help,
    #[command(description = "Написать сообщение от третьего лица")]
    Me(String),
    #[command(description = "Выдать предупреждение пользователю (только для админов)")]
    Warn(String),
    #[command(description = "Снять предупреждения и убрать мут")]
    Unwarn,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Me(quote) => {
            let user = msg.from().unwrap().to_owned();
            let username = user.username.unwrap_or("noname".to_owned());
            let esc_username = markdown::escape(&username);
            let esc_quote = markdown::escape(&quote);

            let text = format!("*_{esc_username}_* {esc_quote}").to_string();
            bot.delete_message(msg.chat.id, msg.id).await?;
            bot.send_message(msg.chat.id, text)
                .parse_mode(MarkdownV2)
                .await?
        }
        Command::Warn(reason) => match msg.reply_to_message() {
            Some(guilty) => {
                let username = guilty
                    .from()
                    .unwrap()
                    .to_owned()
                    .username
                    .unwrap_or("noname".to_string());
                let username_formatted = markdown::bold(&username);
                let reason_formatted = markdown::italic(&markdown::escape(&reason));
                let text = format!(
                    "{username_formatted} получил предупреждение по причине \"{reason_formatted}\""
                );
                bot.send_message(msg.chat.id, text)
                    .parse_mode(MarkdownV2)
                    .await?
            }
            None => {
                bot.send_message(
                    msg.chat.id,
                    "Используйте эту команду как ответ на сообщение, требующее действий."
                        .to_string(),
                )
                .await?
            }
        },
        Command::Unwarn => {
            bot.send_message(msg.chat.id, "Это разбан".to_string())
                .await?
        }
    };
    Ok(())
}
