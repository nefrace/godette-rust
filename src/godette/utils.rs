use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message};
use url::Url;

pub fn get_text_or_empty(msg: &Message) -> String {
    msg.text()
        .unwrap_or(msg.caption().unwrap_or(""))
        .to_string()
}

pub fn make_offtop_keyboard() -> InlineKeyboardMarkup {
    let link = Url::parse("https://t.me/Godot_Engine_Offtop").unwrap();
    let button = InlineKeyboardButton::url("Godot Engine оффтоп чат".to_owned(), link);
    return InlineKeyboardMarkup::new(vec![[button]]);
}
