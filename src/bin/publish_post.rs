extern crate posts as lib_posts;

use chrono::Local;
use diesel::prelude::*;
use lib_posts::models::Post;
use lib_posts::*;
use std::env::args;

fn main() {
    use lib_posts::schema::posts::dsl::{posts, published_at};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    diesel::update(posts.find(id))
        .set(published_at.eq(Local::now().naive_local()))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));

    let post: Post = posts
        .find(id)
        .first(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
