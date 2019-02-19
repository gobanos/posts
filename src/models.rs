use crate::schema::{posts, users};
use chrono::NaiveDateTime;

type Id = i32;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub id_user: Id,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct Post {
    pub id_post: Id,
    pub id_user: Id,
    pub published_at: Option<NaiveDateTime>,
    pub title: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct User {
    pub id_user: Id,
    pub name: String,
}
