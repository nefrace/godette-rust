use teloxide::utils::command::BotCommands;
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Вот мои команды:")]
pub enum Command {
    #[command(description = "Отобразить это сообщение")]
    Help,
    #[command(description = "Написать сообщение от третьего лица")]
    Me(String),
    #[command(description = "Выдать предупреждение пользователю (только для админов)")]
    Warn(String),
    #[command(description = "Снять предупреждения и убрать мут")]
    Unwarn,
}
