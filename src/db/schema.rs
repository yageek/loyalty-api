table! {
    cards (id) {
        id -> Integer,
        name -> Text,
        color -> Nullable<Text>,
        code -> Text,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        name -> Text,
        pass -> Text,
    }
}

joinable!(cards -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cards,
    users,
);
