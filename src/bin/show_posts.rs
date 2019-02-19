extern crate posts as lib_posts;

use diesel::prelude::*;
use lib_posts::establish_connection;
use lib_posts::models::*;

fn main() {
    use lib_posts::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published_at.is_not_null())
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{} - {}", post.title, post.published_at.unwrap());
        println!("----------\n");
        println!("{}", post.body);
    }
}
