// @generated automatically by Diesel CLI.

diesel::table! {
    hero_types (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    heroes (id) {
        id -> Integer,
        name -> Text,
        hero_type_id -> Integer,
    }
}

diesel::joinable!(heroes -> hero_types (hero_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    hero_types,
    heroes,
);
