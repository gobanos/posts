extern crate posts as lib_posts;

use lib_posts::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Who are you?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Drop the newline character

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Drop the newline character

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let user = create_user(&connection, name);
    let post = create_post(&connection, &user, title, &body);
    println!(
        "\nSaved draft {} by {} with id {}",
        title, name, post.id_post
    );
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
