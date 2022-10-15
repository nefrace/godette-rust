use teloxide::types::Message;

pub fn get_text_or_empty(msg: &Message) -> String {
    msg.text()
        .unwrap_or(msg.caption().unwrap_or(""))
        .to_string()
}
