#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use crate::models::{User, Post};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &SqliteConnection, name: &str) /*-> User*/{
    use crate::models::NewUser;
    use schema::users;

    let new_user = NewUser {
        name,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");
}

pub fn create_post<'a>(conn: &SqliteConnection, user: &User, title: &str, body: &str) /*-> Post*/ {
    use crate::models::NewPost;
    use schema::posts;

    let new_post = NewPost {
        title,
        body,
        id_user: user.id_user,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}