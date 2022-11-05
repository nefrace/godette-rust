use diesel::prelude::*;
use crate::schema::{chats, users};

#[derive(Queryable)]
pub struct Chat {
    pub id: i64,
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub id: i64,
    pub name: &str,
}

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub karma: i32,
    pub messages_count: i32,
}

#[derive(Insertable)]
pub struct NewUser<'a> {
    pub id: i64,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub karma: i32,
    pub messages_count: i32,
}