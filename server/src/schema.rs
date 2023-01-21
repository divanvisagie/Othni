// @generated automatically by Diesel CLI.

diesel::table! {
    futhark (id) {
        id -> Int4,
        rune_set -> Varchar,
    }
}

diesel::table! {
    rune (id) {
        id -> Int4,
        name -> Varchar,
        latin -> Varchar,
        futhark_id -> Int4,
    }
}

diesel::joinable!(rune -> futhark (futhark_id));

diesel::allow_tables_to_appear_in_same_query!(
    futhark,
    rune,
);
