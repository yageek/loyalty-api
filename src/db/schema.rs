table! {
    card (id) {
        id -> Integer,
        name -> Text,
        color -> Nullable<Text>,
        code -> Text,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Text,
        name -> Text,
        pass -> Text,
    }
}

joinable!(card -> users (user_id));

allow_tables_to_appear_in_same_query!(card, users,);
