use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Message};
use url::Url;

pub fn get_text_or_empty(msg: &Message) -> String {
    msg.text()
        .unwrap_or(msg.caption().unwrap_or(""))
        .to_string()
}

fn no_thanks_button() -> InlineKeyboardButton {
    InlineKeyboardButton::callback("Спасибо, не надо", "no_thanks")
}

pub fn make_offtop_keyboard() -> InlineKeyboardMarkup {
    let link = Url::parse("https://t.me/Godot_Engine_Offtop").unwrap();
    let button = InlineKeyboardButton::url("Godot Engine оффтоп чат".to_owned(), link);
    InlineKeyboardMarkup::new(vec![[button], [no_thanks_button()]])
}

pub fn make_docs_keyboard(text: String, url: Url) -> InlineKeyboardMarkup {
    let button = InlineKeyboardButton::url(text, url);
    InlineKeyboardMarkup::new(vec![[button], [no_thanks_button()]])
}

#[derive(Serialize, Deserialize, Debug)]
struct DocResponse {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<DocResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocResult {
    pub title: String,
    pub path: String,
}

pub async fn request_docs(query: &str) -> Vec<DocResult> {
    let empty_results: Vec<DocResult> = Vec::new();
    let link = format!(
        "https://docs.godotengine.org/_/api/v2/search/?q={}&project=godot-ru&version=stable&language=ru",
        query
    );
    let domain = "https://docs.godotengine.org";
    let response = reqwest::get(link).await;
    match response {
        Ok(response) => {
            let json = response.json::<DocResponse>().await;
            match json {
                Ok(json) => {
                    return json
                        .results
                        .iter()
                        .map(|res| DocResult {
                            path: format!("{}{}", domain, res.path),
                            title: res.title.clone(),
                        })
                        .collect::<Vec<DocResult>>()
                }
                Err(_) => empty_results,
            }
        }
        Err(_) => empty_results,
    }
}
