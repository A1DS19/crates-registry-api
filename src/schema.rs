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
    role (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
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

diesel::table! {
    user_role (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(crate_ -> rustacean (rustacean_id));
diesel::joinable!(user_role -> role (role_id));
diesel::joinable!(user_role -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    crate_,
    role,
    rustacean,
    user_role,
    users,
);
