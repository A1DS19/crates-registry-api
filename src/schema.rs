// @generated automatically by Diesel CLI.

diesel::table! {
    #[sql_name = "crate"]
    crate_ (id) {
        id -> Int4,
        rustacean_id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 64]
        version -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rustacean (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(crate_ -> rustacean (rustacean_id));

diesel::allow_tables_to_appear_in_same_query!(
    crate_,
    rustacean,
);
