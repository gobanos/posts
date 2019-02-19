table! {
    posts (id_post) {
        id_post -> Integer,
        id_user -> Integer,
        published_at -> Nullable<Timestamp>,
        title -> Text,
        body -> Text,
    }
}

table! {
    users (id_user) {
        id_user -> Integer,
        name -> Text,
    }
}

joinable!(posts -> users (id_user));

allow_tables_to_appear_in_same_query!(posts, users,);
